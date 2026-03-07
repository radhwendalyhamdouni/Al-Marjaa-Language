//! Diagnostics - التشخيصات

use lsp_types::*;

/// بناء التشخيصات
pub fn build_diagnostics(
    source: &str,
    line: usize,
    column: usize,
    message: String,
    severity: DiagnosticSeverity,
    code: Option<String>,
) -> Diagnostic {
    Diagnostic {
        range: Range::new(
            Position::new(line.saturating_sub(1) as u32, column.saturating_sub(1) as u32),
            Position::new(line as u32, column as u32),
        ),
        severity: Some(severity),
        code: code.map(|c| NumberOrString::String(c)),
        source: Some(source.to_string()),
        message,
        related_information: None,
        tags: None,
        data: None,
    }
}

/// تصنيف التشخيصات
#[derive(Debug, Clone, Copy)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Information,
    Hint,
}

impl DiagnosticKind {
    pub fn to_severity(&self) -> DiagnosticSeverity {
        match self {
            DiagnosticKind::Error => DiagnosticSeverity::ERROR,
            DiagnosticKind::Warning => DiagnosticSeverity::WARNING,
            DiagnosticKind::Information => DiagnosticSeverity::INFORMATION,
            DiagnosticKind::Hint => DiagnosticSeverity::HINT,
        }
    }
}
