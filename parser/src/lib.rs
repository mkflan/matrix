#![feature(let_chains)]
#![warn(rust_2018_idioms, clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]
#![allow(unused)]

mod ast;
mod diagnostics;
mod print_ast;

use ast::{Expression, ExpressionKind, ExpressionKind::*, UnaryOpKind};
use diagnostics::{DiagnosticSink, ParseDiagnostic};
use lexer::token::{LiteralKind, Token, TokenKind};
use std::{iter::Peekable, vec::IntoIter};

#[derive(Debug)]
struct Parser {
    /// An iterator over the tokens outputted by the lexer.
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    /// Peek the next token.
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    /// Advance to the next token.
    fn advance(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    /// Check if the parser has reached an end of file.
    fn at_end(&mut self) -> bool {
        self.peek().is_some_and(|t| t.kind == TokenKind::EoF)
    }

    fn parse_primary(&mut self) -> Result<ExpressionKind, ParseDiagnostic> {
        if let Some(&peek) = self.peek() {
            if let TokenKind::Literal(lit) = peek.kind {
                self.advance();
                let lit_kind = lit.into();
                return Ok(ExpressionKind::Literal(lit_kind));
            } else if peek.kind == TokenKind::OpenParen {
                self.advance();
                let expr = self.parse_expr()?;
                self.advance();
                return Ok(ExpressionKind::Grouping(Box::new(expr)));
            } else {
                return Err(ParseDiagnostic::O);
            }
        }
        return Err(ParseDiagnostic::O);
    }

    fn parse_unary(&mut self) -> Result<ExpressionKind, ParseDiagnostic> {
        if let Some(&peek) = self.peek()
            && peek.kind.is_unary_op()
        {
            let operator = self.advance().unwrap().kind.into();
            let operand = self.parse_unary()?;
            return Ok(ExpressionKind::Unary {
                operator,
                operand: Box::new(operand),
            });
        }

        self.parse_primary()
    }

    fn parse_factor(&mut self) -> Result<ExpressionKind, ParseDiagnostic> {
        let mut expr = self.parse_unary()?;

        while let Some(&peek) = self.peek()
            && (peek.kind == TokenKind::Star || peek.kind == TokenKind::Slash)
        {
            let operator = self.advance().unwrap().kind.into();
            let rhs = self.parse_unary()?;
            expr = ExpressionKind::Binary {
                lhs: Box::new(expr),
                operator,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<ExpressionKind, ParseDiagnostic> {
        let mut expr = self.parse_factor()?;

        while let Some(&peek) = self.peek()
            && (peek.kind == TokenKind::Minus || peek.kind == TokenKind::Plus)
        {
            let operator = self.advance().unwrap().kind.into();
            let rhs = self.parse_factor()?;
            expr = ExpressionKind::Binary {
                lhs: Box::new(expr),
                operator,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<ExpressionKind, ParseDiagnostic> {
        let mut expr = self.parse_term()?;

        while let Some(&peek) = self.peek()
            && peek.kind.is_comparison_op()
        {
            let operator = self.advance().unwrap().kind.into();
            let rhs = self.parse_term()?;
            expr = ExpressionKind::Binary {
                lhs: Box::new(expr),
                operator,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<ExpressionKind, ParseDiagnostic> {
        let mut expr = self.parse_comparison()?;

        while let Some(&peek) = self.peek()
            && peek.kind.is_equality_op()
        {
            let operator = self.advance().unwrap().kind.into();
            let rhs = self.parse_comparison()?;
            expr = ExpressionKind::Binary {
                lhs: Box::new(expr),
                operator,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    /// Parse an expression.
    fn parse_expr(&mut self) -> Result<ExpressionKind, ParseDiagnostic> {
        self.parse_equality()
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<ExpressionKind>, DiagnosticSink> {
    let mut parser = Parser::new(tokens);
    let mut nodes = Vec::new();
    let mut diagnostics = DiagnosticSink::new();

    while !parser.at_end() {
        match parser.parse_expr() {
            Ok(expr) => nodes.push(expr),
            Err(e) => diagnostics.push_diagnostic(e),
        }
    }

    if diagnostics.has_diagnostics() {
        return Err(diagnostics);
    }

    Ok(nodes)
}
