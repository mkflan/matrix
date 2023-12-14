#![feature(let_chains, lazy_cell)]
#![warn(rust_2018_idioms, clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]

pub mod token;

use std::{collections::HashMap, iter::Peekable, str::Chars, sync::LazyLock};
use token::{
    IdentKind::*,
    IntegerBase::*,
    LiteralKind::*,
    Span, Token,
    TokenKind::{self, *},
};
use unicode_xid::UnicodeXID;

static KEYWORDS: LazyLock<HashMap<&str, TokenKind>> = LazyLock::new(|| {
    use crate::token::Keyword::*;

    HashMap::from([
        ("proc", Ident(Keyword(Proc))),
        ("let", Ident(Keyword(Let))),
        ("void", Ident(Keyword(Void))),
        ("int", Ident(Keyword(Int))),
        ("ret", Ident(Keyword(Ret))),
        ("float", Ident(Keyword(Float))),
        ("if", Ident(Keyword(If))),
        ("elif", Ident(Keyword(Elif))),
        ("else", Ident(Keyword(Else))),
        ("for", Ident(Keyword(For))),
        ("while", Ident(Keyword(While))),
        ("do", Ident(Keyword(Do))),
        ("bool", Ident(Keyword(Bool))),
        ("str", Ident(Keyword(Str))),
    ])
});

#[derive(Debug)]
struct Lexer<'src> {
    /// An iterator over the characters of the source code.
    source: Peekable<Chars<'src>>,

    /// Indicates where the lexer currently is in the source code.
    cursor: (usize, usize),
}

impl<'src> Lexer<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            source: source.chars().peekable(),
            cursor: (1, 1),
        }
    }

    /// Create a new token.
    fn create_token(&self, token_kind: TokenKind, token_len: usize) -> Token {
        Token::new(token_kind, Span::of_token(token_len, self.cursor.1))
    }

    /// Peek the next character in the source.
    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    /// Advance to the next character in the source.
    fn advance(&mut self) -> Option<char> {
        self.cursor.1 += 1;
        self.source.next()
    }

    /// Advance and invoke a callback.
    fn advance_with_callback(&mut self, mut cb: impl FnMut()) -> Option<char> {
        let next = self.advance();
        cb();
        next
    }

    /// Check if the next character is a specified character, returning whether it was consumed or not.
    fn next_is(&mut self, next: char) -> bool {
        if let Some(&c) = self.peek() {
            if c == next {
                self.advance();
                return true;
            }
        }

        false
    }

    /// Check if the end of the source has been reached.
    fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    /// Lex a potentially longer operator.
    fn lex_potentially_longer_operator(
        &mut self,
        check_next: char,
        if_next: TokenKind,
        fallback: TokenKind,
    ) -> Token {
        self.next_is(check_next)
            .then(|| self.create_token(if_next, 2))
            .unwrap_or_else(|| self.create_token(fallback, 1))
    }

    /// Lex an identifier.
    fn lex_ident(&mut self, first_char: char) -> Token {
        let mut ident = std::string::String::from(first_char);

        while !self.at_end() && UnicodeXID::is_xid_continue(*self.peek().unwrap()) {
            ident.push(self.advance().unwrap());
        }

        let token_kind = KEYWORDS
            .get_key_value(ident.as_str())
            .map(|(_, tk)| *tk)
            .unwrap_or(Ident(NonReserved));
        self.create_token(token_kind, ident.len())
    }

    /// Lex a character literal.
    // TODO: handle when the  literal contains an escaped single quote. currently,
    // it will panic as it believes the escaped quote is the closing quote.
    fn lex_char_literal(&mut self) -> Token {
        let mut len = 1; // The opening quote has already been consumed.

        while !self.at_end() && self.peek().unwrap() != &'\'' {
            self.advance_with_callback(|| len += 1);
        }

        // The character contains more than one codepoint.
        if len > 2 {
            panic!("character literals can only contain one codepoint");
        }

        if !self.next_is('\'') {
            panic!("unterminated character literal. expected closing quote");
        }

        self.create_token(Literal(Character), 3)
    }

    /// Lex a string literal.
    // TODO: handle when the literal contains an escaped double quote. currently,
    // it will panic as it believes the escaped quote is the closing quote.
    fn lex_string_literal(&mut self) -> Token {
        let mut len = 1; // The opening quote has already been consumed.

        while !self.at_end() && self.peek().unwrap() != &'"' {
            self.advance_with_callback(|| len += 1);
        }

        if !self.next_is('"') {
            panic!("unterminated string literal. expected closing quote");
        }

        len += 1;
        self.create_token(Literal(String), len)
    }

    /// Lex a numerical literal.
    fn lex_numerical_literal(&mut self, first_digit: char) -> Token {
        let mut len = 1; // The first digit has already been consumed.

        // The literal is an integer with a base specified.
        if first_digit == '0'
            && self
                .peek()
                .is_some_and(|&c| c == 'b' || c == 'o' || c == 'x')
        {
            let base = match self.advance_with_callback(|| len += 1).unwrap() {
                'b' => Binary,
                'o' => Octal,
                'x' => Hexadecimal,
                _ => unreachable!(),
            };

            while !self.at_end()
                && (self.peek().unwrap().is_numeric()
                    || self.peek().unwrap().is_ascii_hexdigit()
                    || self.peek().unwrap() == &'_')
            {
                self.advance_with_callback(|| len += 1);
            }

            self.create_token(Literal(Integer { base }), len)
        } else {
            while !self.at_end() && self.peek().unwrap().is_numeric() {
                self.advance_with_callback(|| len += 1);
            }

            // We have a float.
            if let Some(&next) = self.peek()
                && next == '.'
            {
                self.advance_with_callback(|| len += 1); // Consume the dot.

                while !self.at_end() && self.peek().unwrap().is_numeric() {
                    self.advance_with_callback(|| len += 1);
                }

                return self.create_token(Literal(Float), len);
            }

            self.create_token(Literal(Integer { base: Decimal }), len)
        }
    }

    /// Lex a token.
    fn lex_token(&mut self) -> anyhow::Result<Token> {
        let Some(ch) = self.advance() else {
            return Ok(self.create_token(EoF, 0));
        };

        let token = match ch {
            '(' => self.create_token(OpenParen, 1),
            ')' => self.create_token(ClosingParen, 1),
            '{' => self.create_token(OpenCurly, 1),
            '}' => self.create_token(ClosingCurly, 1),
            '[' => self.create_token(OpenSquare, 1),
            ']' => self.create_token(ClosingSquare, 1),
            ':' => self.create_token(Colon, 1),
            ';' => self.create_token(Semicolon, 1),
            '.' => self.create_token(Period, 1),
            ',' => self.create_token(Comma, 1),
            '=' => self.lex_potentially_longer_operator('=', EqualEqual, Equal),
            '+' => self.lex_potentially_longer_operator('=', PlusEqual, Plus),
            '-' => self.lex_potentially_longer_operator('=', MinusEqual, Minus),
            '*' => self.lex_potentially_longer_operator('=', StarEqual, Star),
            '/' => self.lex_potentially_longer_operator('=', SlashEqual, Slash),
            '%' => self.lex_potentially_longer_operator('=', PercentEqual, Percent),
            '&' => self.create_token(Ampersand, 1),
            '|' => self.create_token(Bar, 1),
            '!' => self.lex_potentially_longer_operator('=', BangEqual, Bang),
            '<' => self.create_token(Lt, 1),
            '>' => self.create_token(Gt, 1),
            '"' => self.lex_string_literal(),
            '\'' => self.lex_char_literal(),
            ch if UnicodeXID::is_xid_start(ch) || ch == '_' => self.lex_ident(ch),
            ch if ch.is_numeric() => self.lex_numerical_literal(ch),
            ch if ch.is_whitespace() => self.lex_token()?,
            _ => panic!("unknown token: {ch}"),
        };

        Ok(token)
    }
}

pub fn lex(code: &str) -> anyhow::Result<Vec<Token>> {
    let mut tokens = Vec::<Token>::new();
    let mut lexer = Lexer::new(code);

    while let Ok(token) = lexer.lex_token() {
        tokens.push(token);

        if token.kind == EoF {
            break;
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::token::{IdentKind::*, IntegerBase::*, Token, TokenKind::*};
    use pretty_assertions::assert_eq as pretty_assert_eq;

    #[test]
    fn test_lex_delimiters() -> anyhow::Result<()> {
        let source = "(){}[]:;.,";
        let tokens = super::lex(source)?;

        pretty_assert_eq!(
            tokens,
            [
                Token {
                    kind: OpenParen,
                    span: (1..2).into(),
                },
                Token {
                    kind: ClosingParen,
                    span: (2..3).into(),
                },
                Token {
                    kind: OpenCurly,
                    span: (3..4).into(),
                },
                Token {
                    kind: ClosingCurly,
                    span: (4..5).into(),
                },
                Token {
                    kind: OpenSquare,
                    span: (5..6).into()
                },
                Token {
                    kind: ClosingSquare,
                    span: (6..7).into(),
                },
                Token {
                    kind: Colon,
                    span: (7..8).into()
                },
                Token {
                    kind: Semicolon,
                    span: (8..9).into()
                },
                Token {
                    kind: Period,
                    span: (9..10).into()
                },
                Token {
                    kind: Comma,
                    span: (10..11).into()
                },
                Token {
                    kind: EoF,
                    span: (12..12).into(),
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_lex_operators() -> anyhow::Result<()> {
        let source = "= == + += - -= * *= / /= % %= & | ! != < >";
        let tokens = super::lex(source)?;

        pretty_assert_eq!(
            tokens,
            [
                Token {
                    kind: Equal,
                    span: (1..2).into(),
                },
                Token {
                    kind: EqualEqual,
                    span: (3..5).into(),
                },
                Token {
                    kind: Plus,
                    span: (6..7).into(),
                },
                Token {
                    kind: PlusEqual,
                    span: (8..10).into(),
                },
                Token {
                    kind: Minus,
                    span: (11..12).into(),
                },
                Token {
                    kind: MinusEqual,
                    span: (13..15).into(),
                },
                Token {
                    kind: Star,
                    span: (16..17).into(),
                },
                Token {
                    kind: StarEqual,
                    span: (18..20).into(),
                },
                Token {
                    kind: Slash,
                    span: (21..22).into(),
                },
                Token {
                    kind: SlashEqual,
                    span: (23..25).into(),
                },
                Token {
                    kind: Percent,
                    span: (26..27).into(),
                },
                Token {
                    kind: PercentEqual,
                    span: (28..30).into(),
                },
                Token {
                    kind: Ampersand,
                    span: (31..32).into(),
                },
                Token {
                    kind: Bar,
                    span: (33..34).into(),
                },
                Token {
                    kind: Bang,
                    span: (35..36).into(),
                },
                Token {
                    kind: BangEqual,
                    span: (37..39).into(),
                },
                Token {
                    kind: Lt,
                    span: (40..41).into()
                },
                Token {
                    kind: Gt,
                    span: (42..43).into(),
                },
                Token {
                    kind: EoF,
                    span: (44..44).into(),
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn test_lex_keywords() -> anyhow::Result<()> {
        use crate::token::Keyword::*;

        let source = "proc let void int ret float if elif else for while do";
        let tokens = super::lex(source)?;

        pretty_assert_eq!(
            tokens,
            [
                Token {
                    kind: Ident(Keyword(Proc)),
                    span: (1..5).into(),
                },
                Token {
                    kind: Ident(Keyword(Let)),
                    span: (6..9).into(),
                },
                Token {
                    kind: Ident(Keyword(Void)),
                    span: (10..14).into(),
                },
                Token {
                    kind: Ident(Keyword(Int)),
                    span: (15..18).into(),
                },
                Token {
                    kind: Ident(Keyword(Ret)),
                    span: (19..22).into(),
                },
                Token {
                    kind: Ident(Keyword(Float)),
                    span: (23..28).into(),
                },
                Token {
                    kind: Ident(Keyword(If)),
                    span: (29..31).into(),
                },
                Token {
                    kind: Ident(Keyword(Elif)),
                    span: (32..36).into(),
                },
                Token {
                    kind: Ident(Keyword(Else)),
                    span: (37..41).into(),
                },
                Token {
                    kind: Ident(Keyword(For)),
                    span: (42..45).into(),
                },
                Token {
                    kind: Ident(Keyword(While)),
                    span: (46..51).into(),
                },
                Token {
                    kind: Ident(Keyword(Do)),
                    span: (52..54).into(),
                },
                Token {
                    kind: EoF,
                    span: (55..55).into(),
                },
            ]
        );

        // pretty_assert_eq!(
        //     tokens,
        //     [
        //         Token {
        //             kind: Ident,
        //             span: (1..5).into(),
        //         },
        //         Token {
        //             kind: Ident,
        //             span: (6..9).into(),
        //         },
        //         Token {
        //             kind: Ident,
        //             span: (10..14).into(),
        //         },
        //         Token {
        //             kind: Ident,
        //             span: (15..18).into(),
        //         },
        //         Token {
        //             kind: Ident,
        //             span: (19..22).into(),
        //         },
        //         Token {
        //             kind: Ident,
        //             span: (23..28).into()
        //         },
        //         Token {
        //             kind: Ident,
        //             span: (29..31).into(),
        //         },
        //         Token {
        //             kind: Ident,
        //             span: (32..36).into(),
        //         },
        //         Token {
        //             kind: Ident,
        //             span: (37..41).into(),
        //         },
        //         Token {
        //             kind: Ident,
        //             span: (42..45).into(),
        //         },
        //         Token {
        //             kind: Ident,
        //             span: (46..51).into(),
        //         },
        //         Token {
        //             kind: Ident,
        //             span: (52..54).into(),
        //         },
        //         Token {
        //             kind: EoF,
        //             span: (55..55).into(),
        //         },
        //     ]
        // );

        Ok(())
    }

    #[test]
    fn test_lex_identifiers() -> anyhow::Result<()> {
        let source = "_x y z _foo bar baz";
        let tokens = super::lex(source)?;

        pretty_assert_eq!(
            tokens,
            [
                Token {
                    kind: Ident(NonReserved),
                    span: (1..3).into(),
                },
                Token {
                    kind: Ident(NonReserved),
                    span: (4..5).into(),
                },
                Token {
                    kind: Ident(NonReserved),
                    span: (6..7).into(),
                },
                Token {
                    kind: Ident(NonReserved),
                    span: (8..12).into(),
                },
                Token {
                    kind: Ident(NonReserved),
                    span: (13..16).into(),
                },
                Token {
                    kind: Ident(NonReserved),
                    span: (17..20).into(),
                },
                Token {
                    kind: EoF,
                    span: (21..21).into(),
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_lex_literals() -> anyhow::Result<()> {
        use crate::token::LiteralKind::*;

        let source = "1 100 0b10000001 0b1000_0001 0xFF 0xAB_CD 0xAB2 0o25 20.0 15.2587 'a' \"hi\"";
        let tokens = super::lex(source)?;

        pretty_assert_eq!(
            tokens,
            [
                Token {
                    kind: Literal(Integer { base: Decimal }),
                    span: (1..2).into(),
                },
                Token {
                    kind: Literal(Integer { base: Decimal }),
                    span: (3..6).into(),
                },
                Token {
                    kind: Literal(Integer { base: Binary }),
                    span: (7..17).into(),
                },
                Token {
                    kind: Literal(Integer { base: Binary }),
                    span: (18..29).into(),
                },
                Token {
                    kind: Literal(Integer { base: Hexadecimal }),
                    span: (30..34).into(),
                },
                Token {
                    kind: Literal(Integer { base: Hexadecimal }),
                    span: (35..42).into(),
                },
                Token {
                    kind: Literal(Integer { base: Hexadecimal }),
                    span: (43..48).into(),
                },
                Token {
                    kind: Literal(Integer { base: Octal }),
                    span: (49..53).into(),
                },
                Token {
                    kind: Literal(Float),
                    span: (54..58).into(),
                },
                Token {
                    kind: Literal(Float),
                    span: (59..66).into(),
                },
                Token {
                    kind: Literal(Character),
                    span: (67..70).into()
                },
                Token {
                    kind: Literal(String),
                    span: (71..75).into(),
                },
                Token {
                    kind: EoF,
                    span: (76..76).into(),
                },
            ]
        );

        Ok(())
    }
}
