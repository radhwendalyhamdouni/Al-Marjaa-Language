// src/error/mod.rs
// Professional Error System for Al-Marjaa Language
// Provides: Spans, Error Codes, Source Context, Arabic Messages, Suggestions

use std::fmt;

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Warning,
    Error,
    Critical,
}

/// Error categories with codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    // Lexer errors (E1xx)
    E100, // Unknown character
    E101, // Unclosed string
    E102, // Unclosed comment
    E103, // Invalid number

    // Parser errors (E2xx)
    E200, // Unexpected token
    E201, // Expected token
    E202, // Invalid expression
    E203, // Missing semicolon
    E204, // Invalid statement

    // Runtime errors (E3xx)
    E300, // Undefined variable
    E301, // Type error
    E302, // Division by zero
    E303, // Index out of bounds
    E304, // Call non-function
    E305, // Wrong arguments

    // General errors (E9xx)
    E900, // Internal error
    E901, // File not found
    E902, // Permission denied
}

impl ErrorCode {
    /// Get error code as string
    pub fn code(&self) -> &'static str {
        match self {
            ErrorCode::E100 => "E100",
            ErrorCode::E101 => "E101",
            ErrorCode::E102 => "E102",
            ErrorCode::E103 => "E103",
            ErrorCode::E200 => "E200",
            ErrorCode::E201 => "E201",
            ErrorCode::E202 => "E202",
            ErrorCode::E203 => "E203",
            ErrorCode::E204 => "E204",
            ErrorCode::E300 => "E300",
            ErrorCode::E301 => "E301",
            ErrorCode::E302 => "E302",
            ErrorCode::E303 => "E303",
            ErrorCode::E304 => "E304",
            ErrorCode::E305 => "E305",
            ErrorCode::E900 => "E900",
            ErrorCode::E901 => "E901",
            ErrorCode::E902 => "E902",
        }
    }

    /// Get Arabic message for error code
    pub fn arabic_message(&self) -> &'static str {
        match self {
            ErrorCode::E100 => "رمز غير معروف",
            ErrorCode::E101 => "نص غير مغلق",
            ErrorCode::E102 => "تعليق غير مغلق",
            ErrorCode::E103 => "رقم غير صالح",
            ErrorCode::E200 => "رمز غير متوقع",
            ErrorCode::E201 => "توقع رمزاً معيناً",
            ErrorCode::E202 => "تعبير غير صالح",
            ErrorCode::E203 => "نقطة فاصلة مفقودة",
            ErrorCode::E204 => "تعليمة غير صالحة",
            ErrorCode::E300 => "متغير غير معرف",
            ErrorCode::E301 => "خطأ في النوع",
            ErrorCode::E302 => "القسمة على صفر",
            ErrorCode::E303 => "الفهرس خارج النطاق",
            ErrorCode::E304 => "النوع ليس دالة",
            ErrorCode::E305 => "معاملات خاطئة",
            ErrorCode::E900 => "خطأ داخلي",
            ErrorCode::E901 => "الملف غير موجود",
            ErrorCode::E902 => "لا توجد صلاحيات",
        }
    }
}

/// Source code span (start to end position)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Span { start, end }
    }

    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start,
            end: other.end,
        }
    }
}

/// Source position (line, column, byte offset)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Position {
            line,
            column,
            offset,
        }
    }

    pub fn zero() -> Self {
        Position {
            line: 1,
            column: 1,
            offset: 0,
        }
    }
}

/// Main error struct with comprehensive information
#[derive(Debug, Clone)]
pub struct AlMarjaaError {
    pub code: ErrorCode,
    pub message: String,
    pub span: Option<Span>,
    pub severity: Severity,
    pub source_context: Option<String>,
    pub suggestion: Option<String>,
    pub help: Vec<String>,
}

impl AlMarjaaError {
    /// Create a new error
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        AlMarjaaError {
            code,
            message: message.into(),
            span: None,
            severity: Severity::Error,
            source_context: None,
            suggestion: None,
            help: Vec::new(),
        }
    }

    /// Create with span
    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    /// Create with severity
    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    /// Add source context (the problematic line)
    pub fn with_source_context(mut self, context: impl Into<String>) -> Self {
        self.source_context = Some(context.into());
        self
    }

    /// Add suggestion for fixing
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Add help message
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help.push(help.into());
        self
    }

    /// Format error for display
    pub fn format(&self, filename: &str) -> String {
        let mut output = String::new();

        // Error header
        let severity_str = match self.severity {
            Severity::Warning => "تحذير",
            Severity::Error => "خطأ",
            Severity::Critical => "خطأ حرج",
        };

        output.push_str(&format!("{}: {}\n", severity_str, self.code.code()));

        // Location
        if let Some(span) = &self.span {
            output.push_str(&format!(
                " --> {}:{}:{}\n",
                filename, span.start.line, span.start.column
            ));

            // Source context
            if let Some(ctx) = &self.source_context {
                output.push_str(&format!(" |\n{} | {}\n", span.start.line, ctx));
                // Caret pointer
                let spaces = span.start.column.saturating_sub(1);
                output.push_str(&format!(" |{}^", " ".repeat(spaces)));
                if span.end.column > span.start.column + 1 {
                    output.push_str(&"~".repeat(span.end.column - span.start.column - 1));
                }
                output.push('\n');
            }
        }

        // Main message
        output.push_str(&format!(" | {}\n", self.message));

        // Arabic explanation
        output.push_str(&format!(" | {}\n", self.code.arabic_message()));

        // Suggestion
        if let Some(suggestion) = &self.suggestion {
            output.push_str(&format!(" = هل تقصد: {}؟\n", suggestion));
        }

        // Help messages
        for help in &self.help {
            output.push_str(&format!(" = {}\n", help));
        }

        output
    }
}

impl fmt::Display for AlMarjaaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AlMarjaaError {}

/// Helper function to create common errors
pub mod helpers {
    use super::*;

    pub fn unexpected_token(found: &str, expected: &str, span: Span) -> AlMarjaaError {
        AlMarjaaError::new(
            ErrorCode::E200,
            format!(
                "رمز غير متوقع: وجد '{}' بينما السياق يحتاج '{}'",
                found, expected
            ),
        )
        .with_span(span)
        .with_help(format!("تأكد من ترتيب الرموز قرب '{}'", found))
    }

    pub fn expected_token(found: &str, expected: &str, span: Span) -> AlMarjaaError {
        AlMarjaaError::new(
            ErrorCode::E201,
            format!("توقع '{}' لكن وجد '{}'", expected, found),
        )
        .with_span(span)
        .with_help(format!(
            "أكمل البنية بإضافة '{}' في الموضع المناسب",
            expected
        ))
    }

    pub fn undefined_variable(name: &str, span: Span) -> AlMarjaaError {
        AlMarjaaError::new(ErrorCode::E300, format!("المتغير '{}' غير معرف", name))
            .with_span(span)
            .with_help("تحقق من كتابة اسم المتغير بشكل صحيح".to_string())
    }

    pub fn division_by_zero(span: Span) -> AlMarjaaError {
        AlMarjaaError::new(ErrorCode::E302, "القسمة على صفر غير مسموحة")
            .with_span(span)
            .with_help("تحقق من أن المقسوم عليه ليس صفراً".to_string())
    }

    pub fn index_out_of_bounds(index: i64, length: usize, span: Span) -> AlMarjaaError {
        AlMarjaaError::new(
            ErrorCode::E303,
            format!("الفهرس {} خارج النطاق [0..{}]", index, length - 1),
        )
        .with_span(span)
        .with_suggestion(format!("استخدم فهرساً بين 0 و {}", length - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = AlMarjaaError::new(ErrorCode::E300, "متغير غير معرف")
            .with_span(Span::new(Position::new(1, 5, 4), Position::new(1, 10, 9)))
            .with_severity(Severity::Error)
            .with_source_context("متغير س = ١٠")
            .with_suggestion("س");

        assert!(error.span.is_some());
        assert_eq!(error.severity, Severity::Error);
        assert!(error.source_context.is_some());
        assert!(error.suggestion.is_some());
    }

    #[test]
    fn test_error_code_messages() {
        assert_eq!(ErrorCode::E300.code(), "E300");
        assert_eq!(ErrorCode::E300.arabic_message(), "متغير غير معرف");
    }

    #[test]
    fn test_span_merge() {
        let span1 = Span::new(Position::new(1, 1, 0), Position::new(1, 5, 4));
        let span2 = Span::new(Position::new(1, 8, 7), Position::new(1, 10, 9));
        let merged = span1.merge(span2);

        assert_eq!(merged.start.column, 1);
        assert_eq!(merged.end.column, 10);
    }
}
