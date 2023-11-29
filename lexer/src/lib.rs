#![warn(rust_2018_idioms)]

pub mod token;

use anyhow::bail;
use std::{iter::Peekable, str::Chars};
use token::{
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
            cursor: (1, 0),
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
            '\'' => self.create_token(SingleQuote, 1),
            '"' => self.create_token(DoubleQuote, 1),
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
            ch if ch.is_alphabetic() || ch == '_' => self.lex_ident(),
            ' ' => self.lex_token()?,
            // TODO: \n
            _ => bail!("unknown token: {ch}"),
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
    use crate::token::{Token, TokenKind::*};
    use pretty_assertions::assert_eq as pretty_assert_eq;

    #[test]
    fn test_lex_delimiters() -> anyhow::Result<()> {
        let source = "(){}[]:;.'\",";
        let tokens = super::lex(source)?;

        pretty_assert_eq!(
            tokens,
            [
                Token {
                    kind: OpenParen,
                    span: (0..1).into(),
                },
                Token {
                    kind: ClosingParen,
                    span: (1..2).into(),
                },
                Token {
                    kind: OpenCurly,
                    span: (2..3).into(),
                },
                Token {
                    kind: ClosingCurly,
                    span: (3..4).into(),
                },
                Token {
                    kind: OpenSquare,
                    span: (4..5).into()
                },
                Token {
                    kind: ClosingSquare,
                    span: (5..6).into(),
                },
                Token {
                    kind: Colon,
                    span: (6..7).into()
                },
                Token {
                    kind: Semicolon,
                    span: (7..8).into()
                },
                Token {
                    kind: Period,
                    span: (8..9).into()
                },
                Token {
                    kind: SingleQuote,
                    span: (9..10).into(),
                },
                Token {
                    kind: DoubleQuote,
                    span: (10..11).into()
                },
                Token {
                    kind: Comma,
                    span: (11..12).into()
                },
                Token {
                    kind: EoF,
                    span: (13..13).into(),
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
                    span: (0..1).into()
                },
                Token {
                    kind: Plus,
                    span: (1..2).into()
                },
                Token {
                    kind: Minus,
                    span: (2..3).into()
                },
                Token {
                    kind: Star,
                    span: (3..4).into()
                },
                Token {
                    kind: Slash,
                    span: (4..5).into()
                },
                Token {
                    kind: Percent,
                    span: (5..6).into()
                },
                Token {
                    kind: Ampersand,
                    span: (6..7).into()
                },
                Token {
                    kind: Bar,
                    span: (7..8).into()
                },
                Token {
                    kind: Lt,
                    span: (8..9).into()
                },
                Token {
                    kind: Gt,
                    span: (9..10).into()
                },
                Token {
                    kind: EoF,
                    span: (11..11).into()
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_lex_keywords() -> anyhow::Result<()> {
        let source = "proc let void int ret";
        let tokens = super::lex(source)?;

        pretty_assert_eq!(
            tokens,
            [
                Token {
                    kind: Ident,
                    span: (0..4).into(),
                },
                Token {
                    kind: Ident,
                    span: (5..8).into(),
                },
                Token {
                    kind: Ident,
                    span: (9..13).into(),
                },
                Token {
                    kind: Ident,
                    span: (14..17).into(),
                },
                Token {
                    kind: Ident,
                    span: (18..21).into(),
                },
                Token {
                    kind: EoF,
                    span: (22..22).into(),
                },
            ]
        );

        Ok(())
    }
}
