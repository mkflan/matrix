#![warn(rust_2018_idioms, clippy::nursery)]
#![allow(clippy::missing_const_for_fn)]
#![allow(unused)]

mod ast;

use lexer::token::{Token, TokenKind};

pub fn parse(tokens: &[Token]) -> anyhow::Result<()> {
    Ok(())
}
