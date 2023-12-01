#![warn(rust_2018_idioms)]
#![allow(unused)]

pub mod token;

use anyhow::bail;
use std::{iter::Peekable, str::Chars};
use token::{
    IntegerBase::*,
    LiteralKind::*,
    Span, Token,
    TokenKind::{self, *},
};

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

    /// Check if the end of the source has been reached.
    fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    /// Lex an identifier.
    fn lex_ident(&mut self) -> Token {
        let mut len = 1; // the first character of the identifier has already been consumed.

        while !self.at_end() && self.peek().unwrap().is_alphanumeric() {
            self.advance();
            len += 1;
        }

        self.create_token(Ident, len)
    }

    /// Lex a character literal.
    fn lex_char_literal(&mut self) -> Token {
        if self.at_end() {
            panic!("unterminated character literal. expected closing quote found EoF");
        }

        self.create_token(Literal(Character), 3)
    }

    /// Lex a string literal.
    fn lex_string_literal(&mut self) -> Token {
        let mut len = 1; // the opening quote has already been consumed.

        while !self.at_end() && self.peek().unwrap() != &'"' {
            self.advance();
            len += 1;
        }

        if self.at_end() {
            panic!("unterminated string literal. expected closing quote found EoF")
        }

        self.advance();
        len += 1;
        self.create_token(Literal(String), len)
    }

    /// Lex a numerical literal.
    /// TODO: lex decimals
    fn lex_numerical_literal(&mut self, first_digit: char) -> Token {
        let mut len = 1; // the first digit has already been consumed.
        let base = match (first_digit, self.advance_with_callback(|| len += 1)) {
            ('0', Some('b')) => Binary,
            ('0', Some('o')) => Octal,
            ('0', Some('x')) => Hexadecimal,
            ('0', Some(ch)) => panic!("invalid integer base specifier"),
            _ => Decimal,
        };

        while !self.at_end()
            && (self.peek().unwrap().is_numeric()
                || self.peek().unwrap() == &'_'
                || ('A'..='F').contains(&self.peek().unwrap().to_ascii_uppercase()))
        {
            self.advance_with_callback(|| len += 1);
        }

        self.create_token(Literal(Integer { base }), len)
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
            '=' => self.create_token(Equals, 1),
            '+' => self.create_token(Plus, 1),
            '-' => self.create_token(Minus, 1),
            '*' => self.create_token(Star, 1),
            '/' => self.create_token(Slash, 1),
            '%' => self.create_token(Percent, 1),
            '&' => self.create_token(Ampersand, 1),
            '|' => self.create_token(Bar, 1),
            '<' => self.create_token(Lt, 1),
            '>' => self.create_token(Gt, 1),
            '"' => self.lex_string_literal(),
            '\'' => self.lex_char_literal(),
            ch if ch.is_alphabetic() || ch == '_' => self.lex_ident(),
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

        if let EoF = token.kind {
            break;
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use crate::token::{IntegerBase::*, LiteralKind::*, Token, TokenKind::*};
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
        let source = "=+-*/%&|<>";
        let tokens = super::lex(source)?;

        pretty_assert_eq!(
            tokens,
            [
                Token {
                    kind: Equals,
                    span: (1..2).into()
                },
                Token {
                    kind: Plus,
                    span: (2..3).into()
                },
                Token {
                    kind: Minus,
                    span: (3..4).into()
                },
                Token {
                    kind: Star,
                    span: (4..5).into()
                },
                Token {
                    kind: Slash,
                    span: (5..6).into()
                },
                Token {
                    kind: Percent,
                    span: (6..7).into()
                },
                Token {
                    kind: Ampersand,
                    span: (7..8).into()
                },
                Token {
                    kind: Bar,
                    span: (8..9).into()
                },
                Token {
                    kind: Lt,
                    span: (9..10).into()
                },
                Token {
                    kind: Gt,
                    span: (10..11).into()
                },
                Token {
                    kind: EoF,
                    span: (12..12).into()
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_lex_keywords() -> anyhow::Result<()> {
        let source = "proc let void int ret float";
        let tokens = super::lex(source)?;

        pretty_assert_eq!(
            tokens,
            [
                Token {
                    kind: Ident,
                    span: (1..5).into(),
                },
                Token {
                    kind: Ident,
                    span: (6..9).into(),
                },
                Token {
                    kind: Ident,
                    span: (10..14).into(),
                },
                Token {
                    kind: Ident,
                    span: (15..18).into(),
                },
                Token {
                    kind: Ident,
                    span: (19..22).into(),
                },
                Token {
                    kind: Ident,
                    span: (23..28).into()
                },
                Token {
                    kind: EoF,
                    span: (29..29).into(),
                },
            ]
        );

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
                    kind: Ident,
                    span: (1..3).into(),
                },
                Token {
                    kind: Ident,
                    span: (4..5).into(),
                },
                Token {
                    kind: Ident,
                    span: (6..7).into(),
                },
                Token {
                    kind: Ident,
                    span: (8..12).into(),
                },
                Token {
                    kind: Ident,
                    span: (13..16).into(),
                },
                Token {
                    kind: Ident,
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
        let source = "1 100 0b0101 0b1111_0000 0xFF 0xab2 0o20";
        let tokens = super::lex(source)?;

        pretty_assert_eq!(
            tokens,
            [
                Token {
                    kind: Literal(Integer { base: Decimal }),
                    span: (1..2).into()
                },
                Token {
                    kind: Literal(Integer { base: Decimal }),
                    span: (3..6).into(),
                },
                Token {
                    kind: Literal(Integer { base: Binary }),
                    span: (8..14).into(),
                },
                Token {
                    kind: Literal(Integer { base: Binary }),
                    span: (15..26).into()
                },
                Token {
                    kind: Literal(Integer { base: Hexadecimal }),
                    span: (27..31).into(),
                },
                Token {
                    kind: Literal(Integer { base: Hexadecimal }),
                    span: (32..37).into()
                },
                Token {
                    kind: Literal(Integer { base: Octal }),
                    span: (38..42).into(),
                },
                Token {
                    kind: EoF,
                    span: (42..42).into()
                }
            ]
        );

        Ok(())
    }
}
