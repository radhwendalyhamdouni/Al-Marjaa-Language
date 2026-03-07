//! ═══════════════════════════════════════════════════════════════════════════════
//! State Management - إدارة الحالة
//! تخزين المستندات والتحليلات المُخبأة
//! متكامل مع المحلل الفعلي للغة المرجع
//! ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

use lsp_types::{Url, Diagnostic, Position, Range, DiagnosticSeverity};

// استيراد المكونات من المشروع الأصلي
use almarjaa::{
    Lexer,
    Parser,
    lint_source_with_config,
    LintConfig,
    LintDiagnostic,
};
use almarjaa::lexer::tokens::TokenType;

/// مستند مفتوح
#[derive(Debug, Clone)]
pub struct Document {
    /// محتوى المستند
    pub content: String,
    /// رقم الإصدار
    pub version: i32,
    /// الأسطر (للوصول السريع)
    lines: Vec<String>,
}

impl Document {
    /// إنشاء مستند جديد
    pub fn new(content: String, version: i32) -> Self {
        let lines = content.lines().map(|s| s.to_string()).collect();
        Self { content, version, lines }
    }
    
    /// الحصول على سطر معين
    pub fn get_line(&self, line: usize) -> Option<&str> {
        self.lines.get(line).map(|s| s.as_str())
    }
    
    /// عدد الأسطر
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
    
    /// تحديث المستند
    pub fn update_content(&mut self, content: String, version: i32) {
        self.content = content;
        self.version = version;
        self.lines = self.content.lines().map(|s| s.to_string()).collect();
    }
}

/// نتيجة التحليل
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// الأخطاء والتشخيصات
    pub diagnostics: Vec<Diagnostic>,
    /// الرموز المُعرّفة
    pub definitions: HashMap<String, DefinitionInfo>,
    /// جميع المراجع
    pub references: HashMap<String, Vec<ReferenceInfo>>,
    /// جميع الـ tokens
    pub tokens: Vec<TokenInfo>,
}

/// معلومات التعريف
#[derive(Debug, Clone)]
pub struct DefinitionInfo {
    pub name: String,
    pub line: usize,
    pub column: usize,
    pub kind: DefinitionKind,
}

/// نوع التعريف
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DefinitionKind {
    Variable,
    Function,
    Constant,
    Parameter,
}

/// معلومات المرجع
#[derive(Debug, Clone)]
pub struct ReferenceInfo {
    pub line: usize,
    pub column: usize,
    pub is_definition: bool,
}

/// معلومات Token
#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub text: String,
    pub line: usize,
    pub column: usize,
    pub kind: String,
}

/// حالة الخادم
pub struct ServerState {
    /// المستندات المفتوحة
    documents: std::sync::RwLock<HashMap<Url, Document>>,
    /// نتائج التحليل المُخبأة
    analysis_cache: std::sync::RwLock<HashMap<Url, AnalysisResult>>,
    /// هل تم التهيئة؟
    pub initialized: std::sync::Mutex<bool>,
    /// إعدادات التهيئة
    pub initialize_params: std::sync::Mutex<Option<lsp_types::InitializeParams>>,
}

impl ServerState {
    /// إنشاء حالة جديدة
    pub fn new() -> Self {
        Self {
            documents: std::sync::RwLock::new(HashMap::new()),
            analysis_cache: std::sync::RwLock::new(HashMap::new()),
            initialized: std::sync::Mutex::new(false),
            initialize_params: std::sync::Mutex::new(None),
        }
    }
    
    /// فتح مستند
    pub fn open_document(&self, uri: Url, content: String, version: i32) {
        let document = Document::new(content, version);
        self.documents.write().unwrap().insert(uri.clone(), document);
        
        // تحليل فوري
        self.analyze_document(&uri);
    }
    
    /// إغلاق مستند
    pub fn close_document(&self, uri: &Url) {
        self.documents.write().unwrap().remove(uri);
        self.analysis_cache.write().unwrap().remove(uri);
    }
    
    /// تحديث مستند
    pub fn update_document(
        &self, 
        uri: &Url, 
        version: i32, 
        content: String,
    ) {
        let mut docs = self.documents.write().unwrap();
        if let Some(doc) = docs.get_mut(uri) {
            doc.update_content(content, version);
        } else {
            docs.insert(uri.clone(), Document::new(content, version));
        }
        drop(docs);
        
        // إعادة التحليل
        self.analyze_document(uri);
    }
    
    /// الحصول على مستند
    pub fn get_document(&self, uri: &Url) -> Option<Document> {
        self.documents.read().unwrap().get(uri).cloned()
    }
    
    /// الحصول على نتيجة التحليل
    pub fn get_analysis(&self, uri: &Url) -> Option<AnalysisResult> {
        self.analysis_cache.read().unwrap().get(uri).cloned()
    }
    
    /// تحليل مستند
    fn analyze_document(&self, uri: &Url) {
        let docs = self.documents.read().unwrap();
        let doc = match docs.get(uri) {
            Some(d) => d,
            None => return,
        };
        
        // تحليل الكود باستخدام المحلل الفعلي
        let result = self.analyze(&doc.content);
        
        self.analysis_cache.write().unwrap().insert(uri.clone(), result);
    }
    
    /// تحليل الكود باستخدام المحلل الفعلي
    fn analyze(&self, content: &str) -> AnalysisResult {
        let mut diagnostics = Vec::new();
        let mut definitions = HashMap::new();
        let mut references = HashMap::new();
        let mut tokens_info = Vec::new();
        
        // ═══════════════════════════════════════════════════════════════
        // 1. التحليل المعجمي باستخدام Lexer الأصلي
        // ═══════════════════════════════════════════════════════════════
        
        let tokens = match Lexer::new(content).tokenize() {
            Ok(t) => t,
            Err(e) => {
                // خطأ في التحليل المعجمي
                diagnostics.push(Diagnostic::new_simple(
                    Range::new(Position::new(0, 0), Position::new(0, 0)),
                    format!("خطأ معجمي: {}", e),
                ));
                return AnalysisResult {
                    diagnostics,
                    definitions,
                    references,
                    tokens: tokens_info,
                };
            }
        };
        
        // تخزين الـ tokens
        for token in &tokens {
            let text = match &token.token_type {
                TokenType::Identifier(s) => s.clone(),
                TokenType::Number(n) => n.to_string(),
                TokenType::String(s) => s.clone(),
                TokenType::Boolean(b) => b.to_string(),
                _ => format!("{}", token.token_type),
            };
            
            let kind = match &token.token_type {
                TokenType::Identifier(_) => "identifier",
                TokenType::Number(_) => "number",
                TokenType::String(_) => "string",
                TokenType::Boolean(_) => "boolean",
                TokenType::Function | TokenType::Let | TokenType::Const |
                TokenType::If | TokenType::Else | TokenType::While |
                TokenType::For | TokenType::Return | TokenType::Print => "keyword",
                _ => "other",
            };
            
            tokens_info.push(TokenInfo {
                text,
                line: token.line,
                column: token.column,
                kind: kind.to_string(),
            });
        }
        
        // ═══════════════════════════════════════════════════════════════
        // 2. استخراج التعريفات والمراجع
        // ═══════════════════════════════════════════════════════════════
        
        let mut idx = 0;
        while idx + 1 < tokens.len() {
            let token = &tokens[idx];
            let next = &tokens[idx + 1];
            
            // فحص التعريفات
            let is_decl = matches!(
                token.token_type,
                TokenType::Let | TokenType::Const | TokenType::Function
            );
            
            if is_decl {
                if let TokenType::Identifier(name) = &next.token_type {
                    let kind = match &token.token_type {
                        TokenType::Let => DefinitionKind::Variable,
                        TokenType::Const => DefinitionKind::Constant,
                        TokenType::Function => DefinitionKind::Function,
                        _ => DefinitionKind::Variable,
                    };
                    
                    definitions.insert(name.clone(), DefinitionInfo {
                        name: name.clone(),
                        line: next.line,
                        column: next.column,
                        kind,
                    });
                    
                    references.entry(name.clone())
                        .or_insert_with(Vec::new)
                        .push(ReferenceInfo {
                            line: next.line,
                            column: next.column,
                            is_definition: true,
                        });
                }
            }
            
            // تسجيل استخدامات المعرفات
            if let TokenType::Identifier(name) = &token.token_type {
                if !definitions.contains_key(name) {
                    references.entry(name.clone())
                        .or_insert_with(Vec::new)
                        .push(ReferenceInfo {
                            line: token.line,
                            column: token.column,
                            is_definition: false,
                        });
                } else if !matches!(
                    tokens.get(idx.wrapping_sub(1)).map(|t| &t.token_type),
                    Some(TokenType::Let | TokenType::Const | TokenType::Function)
                ) {
                    // استخدام وليس تعريف
                    references.entry(name.clone())
                        .or_insert_with(Vec::new)
                        .push(ReferenceInfo {
                            line: token.line,
                            column: token.column,
                            is_definition: false,
                        });
                }
            }
            
            idx += 1;
        }
        
        // ═══════════════════════════════════════════════════════════════
        // 3. التحليل النحوي باستخدام Parser الأصلي
        // ═══════════════════════════════════════════════════════════════
        
        if let Err(err) = Parser::parse(content) {
            diagnostics.push(Diagnostic {
                range: Range::new(
                    Position::new((err.line.saturating_sub(1)) as u32, (err.column.saturating_sub(1)) as u32),
                    Position::new(err.line as u32, err.column as u32),
                ),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(lsp_types::NumberOrString::String("E200".to_string())),
                source: Some("almarjaa-parser".to_string()),
                message: err.message,
                ..Diagnostic::default()
            });
        }
        
        // ═══════════════════════════════════════════════════════════════
        // 4. التحليل باستخدام Linter الأصلي
        // ═══════════════════════════════════════════════════════════════
        
        if let Ok(lints) = lint_source_with_config(content, &LintConfig::default()) {
            for lint in lints {
                diagnostics.push(Diagnostic {
                    range: Range::new(
                        Position::new((lint.line.saturating_sub(1)) as u32, 0),
                        Position::new(lint.line as u32, 100),
                    ),
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: Some(lsp_types::NumberOrString::String(lint.code.to_string())),
                    source: Some("almarjaa-linter".to_string()),
                    message: lint.message,
                    ..Diagnostic::default()
                });
            }
        }
        
        AnalysisResult {
            diagnostics,
            definitions,
            references,
            tokens: tokens_info,
        }
    }
}

impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_analyze_simple_code() {
        let state = ServerState::new();
        let code = r#"
متغير س = ١٠؛
اطبع(س)؛
"#;
        let result = state.analyze(code);
        
        // يجب أن يكون هناك تعريف واحد
        assert!(result.definitions.contains_key("س"));
        
        // يجب أن تكون هناك مراجع
        assert!(result.references.contains_key("س"));
    }
    
    #[test]
    fn test_analyze_function() {
        let state = ServerState::new();
        let code = r#"
دالة مجموع(أ، ب) {
    أعطِ أ + ب؛
}
"#;
        let result = state.analyze(code);
        
        assert!(result.definitions.contains_key("مجموع"));
    }
}
