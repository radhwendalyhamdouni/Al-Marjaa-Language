//! ═══════════════════════════════════════════════════════════════════════════════
//! State Management - إدارة الحالة المتقدمة
//! ═══════════════════════════════════════════════════════════════════════════════
//! تخزين المستندات والتحليلات مع تخزين مؤقت ذكي
//! دعم التزامن العالي مع الأداء الأمثل

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use lsp_types::{Url, Diagnostic, Position, Range, DiagnosticSeverity};
use parking_lot::{RwLock, Mutex};
use dashmap::DashMap;
use lru::LruCache;

use almarjaa::{
    Lexer,
    Parser,
    lint_source_with_config,
    LintConfig,
    LintDiagnostic,
};
use almarjaa::lexer::tokens::TokenType;

/// الحد الأقصى لحجم الذاكرة المؤقتة
const CACHE_SIZE: usize = 100;

/// مستند مفتوح
#[derive(Debug, Clone)]
pub struct Document {
    /// محتوى المستند
    pub content: String,
    /// رقم الإصدار
    pub version: i32,
    /// الأسطر (للوصول السريع)
    lines: Vec<String>,
    /// وقت آخر تحليل
    last_analysis: Option<Instant>,
    /// الهاش للمحتوى
    content_hash: u64,
}

impl Document {
    /// إنشاء مستند جديد
    pub fn new(content: String, version: i32) -> Self {
        let lines = content.lines().map(|s| s.to_string()).collect();
        let content_hash = Self::calculate_hash(&content);
        Self { 
            content, 
            version, 
            lines,
            last_analysis: None,
            content_hash,
        }
    }
    
    /// حساب هاش المحتوى
    fn calculate_hash(content: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        hasher.finish()
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
        let new_hash = Self::calculate_hash(&content);
        
        // تحديث فقط إذا تغير المحتوى
        if new_hash != self.content_hash {
            self.content = content;
            self.version = version;
            self.lines = self.content.lines().map(|s| s.to_string()).collect();
            self.content_hash = new_hash;
            self.last_analysis = None;
        }
    }
    
    /// هل يحتاج لإعادة تحليل؟
    pub fn needs_analysis(&self) -> bool {
        self.last_analysis.is_none()
    }
    
    /// تعيين وقت التحليل
    pub fn set_analyzed(&mut self) {
        self.last_analysis = Some(Instant::now());
    }
}

/// نتيجة التحليل الشاملة
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
    /// شجرة الاستدعاءات
    pub call_hierarchy: HashMap<String, Vec<CallInfo>>,
    /// معلومات الأنواع
    pub type_info: HashMap<String, TypeInfo>,
    /// الرموز الدلالية
    pub semantic_tokens: Vec<SemanticTokenInfo>,
    /// النطاقات القابلة للطي
    pub folding_ranges: Vec<FoldingRangeInfo>,
    /// التواقيع
    pub signatures: HashMap<String, SignatureInfo>,
}

impl Default for AnalysisResult {
    fn default() -> Self {
        Self {
            diagnostics: Vec::new(),
            definitions: HashMap::new(),
            references: HashMap::new(),
            tokens: Vec::new(),
            call_hierarchy: HashMap::new(),
            type_info: HashMap::new(),
            semantic_tokens: Vec::new(),
            folding_ranges: Vec::new(),
            signatures: HashMap::new(),
        }
    }
}

/// معلومات التعريف
#[derive(Debug, Clone)]
pub struct DefinitionInfo {
    pub name: String,
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
    pub kind: DefinitionKind,
    pub type_annotation: Option<String>,
    pub documentation: Option<String>,
}

/// نوع التعريف
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefinitionKind {
    Variable,
    Function,
    Constant,
    Parameter,
    Class,
    Method,
    Property,
    Enum,
    EnumVariant,
    Interface,
    Module,
    Namespace,
}

impl DefinitionKind {
    /// تحويل إلى SymbolKind لـ LSP
    pub fn to_symbol_kind(&self) -> lsp_types::SymbolKind {
        match self {
            DefinitionKind::Variable => lsp_types::SymbolKind::VARIABLE,
            DefinitionKind::Function => lsp_types::SymbolKind::FUNCTION,
            DefinitionKind::Constant => lsp_types::SymbolKind::CONSTANT,
            DefinitionKind::Parameter => lsp_types::SymbolKind::VARIABLE,
            DefinitionKind::Class => lsp_types::SymbolKind::CLASS,
            DefinitionKind::Method => lsp_types::SymbolKind::METHOD,
            DefinitionKind::Property => lsp_types::SymbolKind::PROPERTY,
            DefinitionKind::Enum => lsp_types::SymbolKind::ENUM,
            DefinitionKind::EnumVariant => lsp_types::SymbolKind::ENUM_MEMBER,
            DefinitionKind::Interface => lsp_types::SymbolKind::INTERFACE,
            DefinitionKind::Module => lsp_types::SymbolKind::MODULE,
            DefinitionKind::Namespace => lsp_types::SymbolKind::NAMESPACE,
        }
    }
    
    /// تحويل إلى CompletionItemKind
    pub fn to_completion_kind(&self) -> lsp_types::CompletionItemKind {
        match self {
            DefinitionKind::Variable => lsp_types::CompletionItemKind::VARIABLE,
            DefinitionKind::Function => lsp_types::CompletionItemKind::FUNCTION,
            DefinitionKind::Constant => lsp_types::CompletionItemKind::CONSTANT,
            DefinitionKind::Parameter => lsp_types::CompletionItemKind::VARIABLE,
            DefinitionKind::Class => lsp_types::CompletionItemKind::CLASS,
            DefinitionKind::Method => lsp_types::CompletionItemKind::METHOD,
            DefinitionKind::Property => lsp_types::CompletionItemKind::PROPERTY,
            DefinitionKind::Enum => lsp_types::CompletionItemKind::ENUM,
            DefinitionKind::EnumVariant => lsp_types::CompletionItemKind::ENUM_MEMBER,
            DefinitionKind::Interface => lsp_types::CompletionItemKind::INTERFACE,
            DefinitionKind::Module => lsp_types::CompletionItemKind::MODULE,
            DefinitionKind::Namespace => lsp_types::CompletionItemKind::MODULE,
        }
    }
}

/// معلومات المرجع
#[derive(Debug, Clone)]
pub struct ReferenceInfo {
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
    pub is_definition: bool,
    pub is_write: bool,
}

/// معلومات Token
#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub text: String,
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
    pub kind: TokenKind,
}

/// نوع Token
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Keyword,
    Identifier,
    Number,
    String,
    Boolean,
    Operator,
    Punctuation,
    Comment,
    Whitespace,
    Unknown,
}

/// معلومات الاستدعاء
#[derive(Debug, Clone)]
pub struct CallInfo {
    pub caller: String,
    pub callee: String,
    pub line: usize,
    pub column: usize,
}

/// معلومات النوع
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub kind: TypeKind,
    pub line: usize,
    pub column: usize,
}

/// نوع البيانات
#[derive(Debug, Clone)]
pub enum TypeKind {
    Number,
    String,
    Boolean,
    List,
    Dict,
    Function,
    Class,
    Null,
    Unknown,
}

/// معلومات الرمز الدلالي
#[derive(Debug, Clone)]
pub struct SemanticTokenInfo {
    pub line: usize,
    pub column: usize,
    pub length: usize,
    pub token_type: u32,
    pub token_modifiers: u32,
}

/// معلومات نطاق الطي
#[derive(Debug, Clone)]
pub struct FoldingRangeInfo {
    pub start_line: usize,
    pub end_line: usize,
    pub kind: FoldingKind,
}

/// نوع الطي
#[derive(Debug, Clone, Copy)]
pub enum FoldingKind {
    Function,
    Class,
    Block,
    Comment,
    Region,
}

/// معلومات التوقيع
#[derive(Debug, Clone)]
pub struct SignatureInfo {
    pub name: String,
    pub label: String,
    pub documentation: Option<String>,
    pub parameters: Vec<ParameterInfo>,
}

/// معلومات المعامل
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub type_annotation: Option<String>,
    pub documentation: Option<String>,
    pub optional: bool,
}

/// حالة الخادم
pub struct ServerState {
    /// المستندات المفتوحة
    documents: DashMap<Url, Document>,
    /// نتائج التحليل المُخبأة
    analysis_cache: DashMap<Url, AnalysisResult>,
    /// ذاكرة مؤقتة للإكمال
    completion_cache: Mutex<LruCache<String, Vec<lsp_types::CompletionItem>>>,
    /// هل تم التهيئة؟
    pub initialized: RwLock<bool>,
    /// إعدادات التهيئة
    pub initialize_params: RwLock<Option<lsp_types::InitializeParams>>,
    /// الإعدادات
    pub settings: RwLock<ServerSettings>,
    /// إحصائيات
    pub stats: RwLock<ServerStats>,
}

/// إعدادات الخادم
#[derive(Debug, Clone)]
pub struct ServerSettings {
    /// تفعيل Semantic Tokens
    pub semantic_tokens_enabled: bool,
    /// تفعيل Inlay Hints
    pub inlay_hints_enabled: bool,
    /// تفعيل Code Lens
    pub code_lens_enabled: bool,
    /// الحد الأقصى للمشاكل
    pub max_diagnostics: usize,
    /// تفعيل التحليل التلقائي
    pub auto_analysis: bool,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            semantic_tokens_enabled: true,
            inlay_hints_enabled: true,
            code_lens_enabled: true,
            max_diagnostics: 100,
            auto_analysis: true,
        }
    }
}

/// إحصائيات الخادم
#[derive(Debug, Clone, Default)]
pub struct ServerStats {
    pub files_opened: u64,
    pub files_analyzed: u64,
    pub completions_provided: u64,
    pub definitions_found: u64,
    pub references_found: u64,
    pub diagnostics_sent: u64,
    pub total_analysis_time_ms: u64,
}

impl ServerState {
    /// إنشاء حالة جديدة
    pub fn new() -> Self {
        Self {
            documents: DashMap::new(),
            analysis_cache: DashMap::new(),
            completion_cache: Mutex::new(LruCache::new(
                std::num::NonZeroUsize::new(CACHE_SIZE).unwrap()
            )),
            initialized: RwLock::new(false),
            initialize_params: RwLock::new(None),
            settings: RwLock::new(ServerSettings::default()),
            stats: RwLock::new(ServerStats::default()),
        }
    }
    
    /// فتح مستند
    pub fn open_document(&self, uri: Url, content: String, version: i32) {
        let document = Document::new(content, version);
        self.documents.insert(uri.clone(), document);
        
        // تحديث الإحصائيات
        self.stats.write().files_opened += 1;
        
        // تحليل فوري
        self.analyze_document(&uri);
    }
    
    /// إغلاق مستند
    pub fn close_document(&self, uri: &Url) {
        self.documents.remove(uri);
        self.analysis_cache.remove(uri);
    }
    
    /// تحديث مستند
    pub fn update_document(&self, uri: &Url, version: i32, content: String) {
        let mut needs_analysis = false;
        
        if let Some(mut doc) = self.documents.get_mut(uri) {
            if doc.needs_analysis() {
                needs_analysis = true;
            }
            doc.update_content(content, version);
        } else {
            self.documents.insert(uri.clone(), Document::new(content, version));
            needs_analysis = true;
        }
        
        if needs_analysis {
            self.analyze_document(uri);
        }
    }
    
    /// الحصول على مستند
    pub fn get_document(&self, uri: &Url) -> Option<Document> {
        self.documents.get(uri).map(|d| d.clone())
    }
    
    /// الحصول على نتيجة التحليل
    pub fn get_analysis(&self, uri: &Url) -> Option<AnalysisResult> {
        self.analysis_cache.get(uri).map(|a| a.clone())
    }
    
    /// تحليل مستند
    fn analyze_document(&self, uri: &Url) {
        let start = Instant::now();
        
        let doc = match self.documents.get(uri) {
            Some(d) => d.clone(),
            None => return,
        };
        
        // تحليل الكود
        let result = self.analyze(&doc.content);
        
        // تخزين النتيجة
        self.analysis_cache.insert(uri.clone(), result);
        
        // تحديث الإحصائيات
        let mut stats = self.stats.write();
        stats.files_analyzed += 1;
        stats.total_analysis_time_ms += start.elapsed().as_millis() as u64;
        
        // تحديث وقت التحليل
        if let Some(mut doc) = self.documents.get_mut(uri) {
            doc.set_analyzed();
        }
    }
    
    /// تحليل الكود باستخدام المحلل الفعلي
    fn analyze(&self, content: &str) -> AnalysisResult {
        let mut result = AnalysisResult::default();
        
        // ═══════════════════════════════════════════════════════════════
        // 1. التحليل المعجمي
        // ═══════════════════════════════════════════════════════════════
        
        let tokens = match Lexer::new(content).tokenize() {
            Ok(t) => t,
            Err(e) => {
                result.diagnostics.push(Diagnostic::new_simple(
                    Range::new(Position::new(0, 0), Position::new(0, 0)),
                    format!("خطأ معجمي: {}", e),
                ));
                return result;
            }
        };
        
        // ═══════════════════════════════════════════════════════════════
        // 2. استخراج التعريفات والمراجع والرموز الدلالية
        // ═══════════════════════════════════════════════════════════════
        
        let mut brace_stack: Vec<(usize, usize)> = Vec::new(); // (line, column)
        let mut function_starts: Vec<(String, usize, usize)> = Vec::new(); // (name, line, column)
        
        for (idx, token) in tokens.iter().enumerate() {
            // تخزين الـ tokens
            let (text, kind) = Self::extract_token_info(&token.token_type);
            let token_kind = Self::token_type_to_kind(&token.token_type);
            
            result.tokens.push(TokenInfo {
                text: text.clone(),
                line: token.line,
                column: token.column,
                end_column: token.column + text.chars().count(),
                kind: token_kind,
            });
            
            // إضافة Semantic Token
            if let Some((sem_type, sem_mod)) = Self::to_semantic_token(&token.token_type) {
                result.semantic_tokens.push(SemanticTokenInfo {
                    line: token.line,
                    column: token.column,
                    length: text.chars().count(),
                    token_type: sem_type,
                    token_modifiers: sem_mod,
                });
            }
            
            // تتبع الأقواس للطي
            match &token.token_type {
                TokenType::LBrace => {
                    brace_stack.push((token.line, token.column));
                }
                TokenType::RBrace => {
                    if let Some((start_line, _)) = brace_stack.pop() {
                        result.folding_ranges.push(FoldingRangeInfo {
                            start_line,
                            end_line: token.line,
                            kind: FoldingKind::Block,
                        });
                    }
                }
                _ => {}
            }
            
            // فحص التعريفات
            let next_token = tokens.get(idx + 1);
            
            let is_decl = matches!(
                token.token_type,
                TokenType::Let | TokenType::Const | TokenType::Function | TokenType::Class
            );
            
            if is_decl {
                if let Some(next) = next_token {
                    if let TokenType::Identifier(name) = &next.token_type {
                        let kind = match &token.token_type {
                            TokenType::Let => DefinitionKind::Variable,
                            TokenType::Const => DefinitionKind::Constant,
                            TokenType::Function => DefinitionKind::Function,
                            TokenType::Class => DefinitionKind::Class,
                            _ => DefinitionKind::Variable,
                        };
                        
                        let type_annotation = Self::extract_type_annotation(&tokens, idx + 1);
                        
                        result.definitions.insert(name.clone(), DefinitionInfo {
                            name: name.clone(),
                            line: next.line,
                            column: next.column,
                            end_column: next.column + name.chars().count(),
                            kind,
                            type_annotation,
                            documentation: None,
                        });
                        
                        result.references.entry(name.clone())
                            .or_insert_with(Vec::new)
                            .push(ReferenceInfo {
                                line: next.line,
                                column: next.column,
                                end_column: next.column + name.chars().count(),
                                is_definition: true,
                                is_write: true,
                            });
                        
                        // تسجيل بداية الدالة
                        if kind == DefinitionKind::Function {
                            function_starts.push((name.clone(), next.line, next.column));
                        }
                    }
                }
            }
            
            // تسجيل استخدامات المعرفات
            if let TokenType::Identifier(name) = &token.token_type {
                // تحقق من التعيين (كتابة)
                let is_write = tokens.get(idx + 1).map(|t| {
                    matches!(t.token_type, TokenType::Assign | TokenType::PlusAssign | TokenType::MinusAssign)
                }).unwrap_or(false);
                
                // إذا لم يكن تعريف
                let is_definition = result.definitions.contains_key(name) && {
                    tokens.get(idx.wrapping_sub(1)).map(|t| {
                        matches!(t.token_type, TokenType::Let | TokenType::Const | TokenType::Function | TokenType::Class)
                    }).unwrap_or(false)
                };
                
                if !is_definition {
                    result.references.entry(name.clone())
                        .or_insert_with(Vec::new)
                        .push(ReferenceInfo {
                            line: token.line,
                            column: token.column,
                            end_column: token.column + name.chars().count(),
                            is_definition: false,
                            is_write,
                        });
                }
            }
            
            // استخراج الاستدعاءات
            if let TokenType::Identifier(name) = &token.token_type {
                if let Some(next) = tokens.get(idx + 1) {
                    if matches!(next.token_type, TokenType::LParen) {
                        // هذا استدعاء دالة
                        result.call_hierarchy.entry(name.clone())
                            .or_insert_with(Vec::new)
                            .push(CallInfo {
                                caller: String::new(), // سيتم ملؤه لاحقاً
                                callee: name.clone(),
                                line: token.line,
                                column: token.column,
                            });
                    }
                }
            }
        }
        
        // ═══════════════════════════════════════════════════════════════
        // 3. التحليل النحوي
        // ═══════════════════════════════════════════════════════════════
        
        if let Err(err) = Parser::parse(content) {
            result.diagnostics.push(Diagnostic {
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
        // 4. التحليل باستخدام Linter
        // ═══════════════════════════════════════════════════════════════
        
        if let Ok(lints) = lint_source_with_config(content, &LintConfig::default()) {
            for lint in lints {
                result.diagnostics.push(Diagnostic {
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
        
        // ═══════════════════════════════════════════════════════════════
        // 5. استخراج التواقيع
        // ═══════════════════════════════════════════════════════════════
        
        for (name, def) in &result.definitions {
            if def.kind == DefinitionKind::Function {
                result.signatures.insert(name.clone(), SignatureInfo {
                    name: name.clone(),
                    label: format!("{}(...)", name),
                    documentation: def.documentation.clone(),
                    parameters: Vec::new(), // يمكن استخراجها من AST
                });
            }
        }
        
        result
    }
    
    /// استخراج معلومات Token
    fn extract_token_info(token_type: &TokenType) -> (String, TokenKind) {
        match token_type {
            TokenType::Identifier(s) => (s.clone(), TokenKind::Identifier),
            TokenType::Number(n) => (n.to_string(), TokenKind::Number),
            TokenType::String(s) => (format!("\"{}\"", s), TokenKind::String),
            TokenType::Boolean(b) => (b.to_string(), TokenKind::Boolean),
            TokenType::Comment(c) => (c.clone(), TokenKind::Comment),
            _ => (format!("{}", token_type), TokenKind::Unknown),
        }
    }
    
    /// تحويل نوع Token
    fn token_type_to_kind(token_type: &TokenType) -> TokenKind {
        match token_type {
            TokenType::Identifier(_) => TokenKind::Identifier,
            TokenType::Number(_) => TokenKind::Number,
            TokenType::String(_) => TokenKind::String,
            TokenType::Boolean(_) => TokenKind::Boolean,
            TokenType::Comment(_) => TokenKind::Comment,
            TokenType::Function | TokenType::Let | TokenType::Const |
            TokenType::If | TokenType::Else | TokenType::While |
            TokenType::For | TokenType::Return | TokenType::Print |
            TokenType::Class | TokenType::Import | TokenType::Export => TokenKind::Keyword,
            TokenType::Plus | TokenType::Minus | TokenType::Multiply |
            TokenType::Divide | TokenType::Equal | TokenType::Less |
            TokenType::Greater | TokenType::And | TokenType::Or => TokenKind::Operator,
            TokenType::Semicolon | TokenType::Comma | TokenType::Dot |
            TokenType::LParen | TokenType::RParen | TokenType::LBrace |
            TokenType::RBrace | TokenType::LBracket | TokenType::RBracket => TokenKind::Punctuation,
            _ => TokenKind::Unknown,
        }
    }
    
    /// تحويل إلى Semantic Token
    fn to_semantic_token(token_type: &TokenType) -> Option<(u32, u32)> {
        // (token_type, token_modifiers)
        // الأنواع: 0=namespace, 1=type, 2=class, 3=enum, 4=interface, 
        // 5=struct, 6=typeParameter, 7=parameter, 8=variable, 9=property,
        // 10=enumMember, 11=event, 12=function, 13=method, 14=macro,
        // 15=keyword, 16=modifier, 17=comment, 18=string, 19=number,
        // 20=regexp, 21=operator
        
        match token_type {
            TokenType::Identifier(_) => Some((8, 0)), // variable
            TokenType::Number(_) => Some((19, 0)), // number
            TokenType::String(_) => Some((18, 0)), // string
            TokenType::Boolean(_) => Some((19, 0)), // number (boolean as number)
            TokenType::Function | TokenType::Let | TokenType::Const |
            TokenType::If | TokenType::Else | TokenType::ElseIf |
            TokenType::While | TokenType::For | TokenType::Return |
            TokenType::Break | TokenType::Continue | TokenType::Print |
            TokenType::Class | TokenType::Import | TokenType::Export |
            TokenType::Try | TokenType::Catch | TokenType::Throw |
            TokenType::New | TokenType::This | TokenType::Super => Some((15, 0)), // keyword
            TokenType::Comment(_) => Some((17, 0)), // comment
            TokenType::Plus | TokenType::Minus | TokenType::Multiply |
            TokenType::Divide | TokenType::Modulo | TokenType::Power |
            TokenType::Equal | TokenType::NotEqual | TokenType::Less |
            TokenType::Greater | TokenType::LessEqual | TokenType::GreaterEqual |
            TokenType::And | TokenType::Or | TokenType::Not |
            TokenType::Assign | TokenType::PlusAssign | TokenType::MinusAssign => Some((21, 0)), // operator
            _ => None,
        }
    }
    
    /// استخراج نوع المعامل
    fn extract_type_annotation(tokens: &[almarjaa::lexer::tokens::Token], start_idx: usize) -> Option<String> {
        // البحث عن Colon متبوع بالنوع
        let mut idx = start_idx;
        while idx < tokens.len() {
            match &tokens[idx].token_type {
                TokenType::Colon => {
                    if let Some(next) = tokens.get(idx + 1) {
                        match &next.token_type {
                            TokenType::Identifier(type_name) => return Some(type_name.clone()),
                            _ => {}
                        }
                    }
                }
                TokenType::Semicolon | TokenType::LBrace | TokenType::Assign => break,
                _ => {}
            }
            idx += 1;
        }
        None
    }
    
    /// إضافة اقتراحات للإكمال
    pub fn add_completion_to_cache(&self, prefix: String, items: Vec<lsp_types::CompletionItem>) {
        let mut cache = self.completion_cache.lock();
        cache.put(prefix, items);
    }
    
    /// الحصول على اقتراحات من الذاكرة المؤقتة
    pub fn get_completion_from_cache(&self, prefix: &str) -> Option<Vec<lsp_types::CompletionItem>> {
        let mut cache = self.completion_cache.lock();
        cache.get(prefix).cloned()
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
        
        assert!(result.definitions.contains_key("س"));
        assert!(result.references.contains_key("س"));
        assert!(!result.semantic_tokens.is_empty());
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
        assert!(result.signatures.contains_key("مجموع"));
    }
    
    #[test]
    fn test_folding_ranges() {
        let state = ServerState::new();
        let code = r#"
دالة اختبار() {
    إذا صح {
        اطبع("مرحبا")؛
    }
}
"#;
        let result = state.analyze(code);
        
        assert!(!result.folding_ranges.is_empty());
    }
}
