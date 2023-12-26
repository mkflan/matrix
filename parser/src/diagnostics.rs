use miette::Diagnostic;
use span::Span;
use thiserror::Error;

/// Diagnostics that can happen within the parser.
#[derive(Debug, Clone, Error, Diagnostic)]
pub enum ParseDiagnostic {
    #[error("O")]
    O,
}

#[derive(Debug, Default, Error, Diagnostic)]
#[diagnostic(code(parser::failure))]
#[error("parsing failed with {} diagnostic{}", diagnostics.len(), if diagnostics.len() != 1 { "s" } else { "" })]
pub struct DiagnosticSink {
    #[related]
    diagnostics: Vec<ParseDiagnostic>,
}

impl DiagnosticSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_diagnostic(&mut self, diagnostic: ParseDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn has_diagnostics(&self) -> bool {
        !self.diagnostics.is_empty()
    }
}
