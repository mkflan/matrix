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

    /// The length of the current token being processed.
    cur_tok_len: usize,
}

impl<'src> Lexer<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            source: source.chars().peekable(),
            cursor: (1, 0),
            cur_tok_len: 0,
        }
    }

    /// Create a new token.
    fn create_token(&self, token_kind: TokenKind) -> Token {
        Token::new(token_kind, Span::of_token(self.cur_tok_len, self.cursor.1))
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
        while !self.at_end() && self.advance().unwrap().is_alphanumeric() {
            self.cur_tok_len += 1;
        }

        self.create_token(Ident)
    }

    /// Lex a token.
    fn lex_token(&mut self) -> anyhow::Result<Token> {
        let Some(ch) = self.advance() else {
            return Ok(self.create_token(EoF));
        };

        self.cur_tok_len += 1;
        let token = match ch {
            '(' => self.create_token(OpenParen),
            ')' => self.create_token(ClosingParen),
            '{' => self.create_token(OpenCurly),
            '}' => self.create_token(ClosingCurly),
            '[' => self.create_token(OpenSquare),
            ']' => self.create_token(ClosingSquare),
            ':' => self.create_token(Colon),
            ';' => self.create_token(Semicolon),
            '.' => self.create_token(Period),
            '\'' => self.create_token(SingleQuote),
            '"' => self.create_token(DoubleQuote),
            ',' => self.create_token(Comma),
            '=' => self.create_token(Equals),
            '+' => self.create_token(Plus),
            '-' => self.create_token(Minus),
            '*' => self.create_token(Star),
            '/' => self.create_token(Slash),
            '%' => self.create_token(Percent),
            '&' => self.create_token(Ampersand),
            '|' => self.create_token(Bar),
            '<' => self.create_token(Lt),
            '>' => self.create_token(Gt),
            // TODO: whitespace
            ch if ch.is_alphabetic() || ch == '_' => self.lex_ident(),
            _ => bail!("unknown token: {ch}"),
        };

        self.cur_tok_len = 0;
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
            &tokens,
            &[
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
}
