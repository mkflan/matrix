use ariadne::ReportKind;
use miette::Diagnostic;
use span::Span;
use thiserror::Error;

/// Diagnostics that can happen within the lexer.
#[derive(Debug, Clone, Error, Diagnostic)]
pub enum LexDiagnostic {
    #[diagnostic(code(lexer::unexpected_character))]
    #[error("Encountered unexpected character with no corresponding token.")]
    UnexpectedCharacter(char, #[label("unexpected character here")] Span),

    #[diagnostic(
        code(lexer::empty_character_literal),
        help("add a singular codepoint within the single quotes")
    )]
    #[error("Empty character literal")]
    EmptyCharacterLiteral(#[label("empty character literal here")] Span),

    #[diagnostic(
        code(lexer::unterminated_character_literal),
        help("add a closing single quote")
    )]
    #[error("Unterminated character literal. Expected closing quote")]
    UnterminatedCharacterLiteral(#[label("unterminated character literal here")] Span),

    #[diagnostic(
        code(lexer::character_lit_one_codepoint),
        help("use double quotes if you meant to write a string literal")
    )]
    #[error("Encountered character literal with more than one codepoint")]
    CharacterLiteralOneCodePoint(#[label("here")] Span),

    #[diagnostic(
        code(lexer::unterminated_string_literal),
        help("add a closing double quote")
    )]
    #[error("Unterminated string literal. Expected closing quote")]
    UnterminatedStringLiteral(#[label("unterminated string literal here")] Span),
}

#[derive(Debug, Default, Error, Diagnostic)]
#[diagnostic(code(lexer::failure))]
#[error("lexing failed with {} diagnostic{}", diagnostics.len(), if diagnostics.len() != 1 { "s" } else { "" })]
pub struct DiagnosticSink {
    #[related]
    diagnostics: Vec<LexDiagnostic>,
}

impl DiagnosticSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_diagnostic(&mut self, diagnostic: LexDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn has_diagnostics(&self) -> bool {
        !self.diagnostics.is_empty()
    }
}
