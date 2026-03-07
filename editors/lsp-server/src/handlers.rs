//! ═══════════════════════════════════════════════════════════════════════════════
//! Request Handlers - معالجات الطلبات
//! معالجة جميع طلبات وإشعارات LSP
//! متكامل مع المحلل الفعلي للغة المرجع
//! ═══════════════════════════════════════════════════════════════════════════════

use lsp_types::*;
use serde_json::Value;

use crate::state::{ServerState, DefinitionKind};
use crate::{LspError, LspMessage};

/// معالج الطلبات
pub struct RequestHandler {
    state: std::sync::Arc<ServerState>,
}

impl RequestHandler {
    /// إنشاء معالج جديد
    pub fn new(state: std::sync::Arc<ServerState>) -> Self {
        Self { state }
    }
    
    /// معالجة طلب
    pub fn handle_request(&self, id: u64, method: &str, params: Value) -> LspMessage {
        let result = match method {
            "initialize" => self.handle_initialize(params),
            "shutdown" => self.handle_shutdown(),
            "textDocument/definition" => self.handle_definition(params),
            "textDocument/references" => self.handle_references(params),
            "textDocument/hover" => self.handle_hover(params),
            "textDocument/completion" => self.handle_completion(params),
            "textDocument/documentSymbol" => self.handle_document_symbol(params),
            "textDocument/documentHighlight" => self.handle_document_highlight(params),
            "textDocument/implementation" => self.handle_definition(params), // نفس التعريف
            _ => return LspMessage::Response {
                id,
                result: None,
                error: Some(LspError::method_not_found(method)),
            },
        };
        
        LspMessage::Response { id, result, error: None }
    }
    
    /// معالجة إشعار
    pub fn handle_notification(&self, method: &str, params: Value) -> Option<LspMessage> {
        match method {
            "initialized" => {
                *self.state.initialized.lock().unwrap() = true;
                eprintln!("[LSP] Client initialized successfully");
                None
            }
            "textDocument/didOpen" => {
                self.handle_did_open(params);
                Some(self.create_diagnostics_notification())
            }
            "textDocument/didChange" => {
                self.handle_did_change(params);
                Some(self.create_diagnostics_notification())
            }
            "textDocument/didClose" => {
                self.handle_did_close(params);
                None
            }
            "textDocument/didSave" => {
                // إعادة التحليل عند الحفظ
                None
            }
            "exit" => {
                eprintln!("[LSP] Exit notification received");
                None
            }
            _ => {
                eprintln!("[LSP] Unknown notification: {}", method);
                None
            }
        }
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // طلبات LSP
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// تهيئة الخادم
    fn handle_initialize(&self, params: Value) -> Option<Value> {
        let params: InitializeParams = serde_json::from_value(params)
            .unwrap_or_else(|_| InitializeParams::default());
        
        *self.state.initialize_params.lock().unwrap() = Some(params);
        
        let capabilities = ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                TextDocumentSyncKind::INCREMENTAL
            )),
            definition_provider: Some(OneOf::Left(true)),
            references_provider: Some(OneOf::Left(true)),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            completion_provider: Some(CompletionOptions {
                trigger_characters: Some(vec![
                    ".".to_string(), 
                    " ".to_string(),
                    "(".to_string()
                ]),
                resolve_provider: Some(false),
                all_commit_characters: None,
                completion_item: None,
            }),
            document_symbol_provider: Some(OneOf::Left(true)),
            document_highlight_provider: Some(OneOf::Left(true)),
            signature_help_provider: Some(SignatureHelpOptions {
                trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
                retrigger_characters: None,
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: None,
                },
            }),
            workspace_symbol_provider: Some(OneOf::Left(true)),
            code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
            execute_command_provider: Some(ExecuteCommandOptions {
                commands: vec![
                    "almarjaa.runFile".to_string(),
                    "almarjaa.format".to_string(),
                ],
                work_done_progress_options: WorkDoneProgressOptions {
                    work_done_progress: None,
                },
            }),
            ..ServerCapabilities::default()
        };
        
        let result = InitializeResult {
            capabilities,
            server_info: Some(ServerInfo {
                name: "Al-Marjaa Language Server".to_string(),
                version: Some("3.0.0".to_string()),
            }),
        };
        
        serde_json::to_value(result).ok()
    }
    
    /// إيقاف الخادم
    fn handle_shutdown(&self) -> Option<Value> {
        Some(Value::Null)
    }
    
    /// الانتقال للتعريف
    fn handle_definition(&self, params: Value) -> Option<Value> {
        let params: TextDocumentPositionParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        let position = params.position;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        // البحث عن الكلمة عند الموضع
        let line = doc.get_line(position.line as usize)?;
        let word = find_word_at_position(line, position.character as usize)?;
        
        // البحث عن التعريف
        if let Some(def) = analysis.definitions.get(&word) {
            let location = Location {
                uri,
                range: Range::new(
                    Position::new(def.line.saturating_sub(1) as u32, def.column as u32),
                    Position::new(def.line.saturating_sub(1) as u32, (def.column + word.len()) as u32),
                ),
            };
            return serde_json::to_value(GotoDefinitionResponse::Scalar(location)).ok();
        }
        
        None
    }
    
    /// إيجاد المراجع
    fn handle_references(&self, params: Value) -> Option<Value> {
        let params: ReferenceParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        // البحث عن الكلمة
        let line = doc.get_line(position.line as usize)?;
        let word = find_word_at_position(line, position.character as usize)?;
        
        // الحصول على جميع المراجع
        if let Some(refs) = analysis.references.get(&word) {
            let include_declaration = params.context.include_declaration;
            
            let locations: Vec<Location> = refs.iter()
                .filter(|r| include_declaration || !r.is_definition)
                .map(|r| Location {
                    uri: uri.clone(),
                    range: Range::new(
                        Position::new(r.line.saturating_sub(1) as u32, r.column as u32),
                        Position::new(r.line.saturating_sub(1) as u32, (r.column + word.len()) as u32),
                    ),
                })
                .collect();
            
            return serde_json::to_value(locations).ok();
        }
        
        Some(Value::Array(vec![]))
    }
    
    /// معلومات التمرير (Hover)
    fn handle_hover(&self, params: Value) -> Option<Value> {
        let params: TextDocumentPositionParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        let position = params.position;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        let line = doc.get_line(position.line as usize)?;
        let word = find_word_at_position(line, position.character as usize)?;
        
        // بناء رسالة المعلومات
        let (message, kind_str) = if let Some(def) = analysis.definitions.get(&word) {
            let kind = match def.kind {
                DefinitionKind::Variable => "متغير",
                DefinitionKind::Function => "دالة",
                DefinitionKind::Constant => "ثابت",
                DefinitionKind::Parameter => "معامل",
            };
            let location = format!("مُعرّف عند السطر {}", def.line);
            (location, kind.to_string())
        } else {
            let usage_count = analysis.references.get(&word)
                .map(|r| r.len())
                .unwrap_or(0);
            (format!("مستخدم {} مرة", usage_count), "معرف".to_string())
        };
        
        // إضافة معلومات إضافية
        let docs = get_documentation_for_word(&word);
        
        let hover = Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!(
                    "```almarjaa\n{}\n```\n\n**{}**\n\n{}\n\n{}",
                    word,
                    kind_str,
                    message,
                    docs
                ),
            }),
            range: None,
        };
        
        serde_json::to_value(hover).ok()
    }
    
    /// الإكمال التلقائي
    fn handle_completion(&self, params: Value) -> Option<Value> {
        let params: CompletionParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        let line = doc.get_line(position.line as usize)?;
        let prefix = find_word_at_position(line, position.character as usize)
            .unwrap_or_default();
        
        let mut items: Vec<CompletionItem> = Vec::new();
        
        // 1. الكلمات المفتاحية
        let keywords = get_keywords();
        for kw in keywords {
            if kw.starts_with(&prefix) || prefix.is_empty() {
                items.push(CompletionItem {
                    label: kw.to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some("كلمة محجوزة".to_string()),
                    documentation: Some(Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: get_keyword_documentation(kw),
                    })),
                    insert_text: Some(get_keyword_insert_text(kw)),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..CompletionItem::default()
                });
            }
        }
        
        // 2. الدوال المدمجة
        let builtins = get_builtin_functions();
        for (name, sig, doc) in builtins {
            if name.starts_with(&prefix) || prefix.is_empty() {
                items.push(CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::FUNCTION),
                    detail: Some(sig),
                    documentation: Some(Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: doc,
                    })),
                    insert_text: Some(format!("{}($1)", name)),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..CompletionItem::default()
                });
            }
        }
        
        // 3. الرموز المُعرّفة في الملف
        for (name, def) in &analysis.definitions {
            if name.starts_with(&prefix) || prefix.is_empty() {
                let kind = match def.kind {
                    DefinitionKind::Variable => CompletionItemKind::VARIABLE,
                    DefinitionKind::Function => CompletionItemKind::FUNCTION,
                    DefinitionKind::Constant => CompletionItemKind::CONSTANT,
                    DefinitionKind::Parameter => CompletionItemKind::VARIABLE,
                };
                
                items.push(CompletionItem {
                    label: name.clone(),
                    kind: Some(kind),
                    detail: Some(format!("السطر {}", def.line)),
                    ..CompletionItem::default()
                });
            }
        }
        
        // ترتيب النتائج
        items.sort_by(|a, b| a.label.cmp(&b.label));
        
        let response = CompletionResponse::Array(items);
        serde_json::to_value(response).ok()
    }
    
    /// رموز المستند
    fn handle_document_symbol(&self, params: Value) -> Option<Value> {
        let params: DocumentSymbolParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        let analysis = self.state.get_analysis(&uri)?;
        
        let symbols: Vec<DocumentSymbol> = analysis.definitions.values()
            .map(|def| {
                let kind = match def.kind {
                    DefinitionKind::Variable => SymbolKind::VARIABLE,
                    DefinitionKind::Function => SymbolKind::FUNCTION,
                    DefinitionKind::Constant => SymbolKind::CONSTANT,
                    DefinitionKind::Parameter => SymbolKind::VARIABLE,
                };
                
                DocumentSymbol {
                    name: def.name.clone(),
                    detail: None,
                    kind,
                    tags: None,
                    deprecated: None,
                    range: Range::new(
                        Position::new(def.line.saturating_sub(1) as u32, 0),
                        Position::new(def.line.saturating_sub(1) as u32, 100),
                    ),
                    selection_range: Range::new(
                        Position::new(def.line.saturating_sub(1) as u32, def.column as u32),
                        Position::new(def.line.saturating_sub(1) as u32, (def.column + def.name.len()) as u32),
                    ),
                    children: None,
                }
            })
            .collect();
        
        serde_json::to_value(symbols).ok()
    }
    
    /// تمييز المستند
    fn handle_document_highlight(&self, params: Value) -> Option<Value> {
        let params: TextDocumentPositionParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        let position = params.position;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        let line = doc.get_line(position.line as usize)?;
        let word = find_word_at_position(line, position.character as usize)?;
        
        let highlights: Vec<DocumentHighlight> = analysis.references.get(&word)
            .map(|refs| {
                refs.iter().map(|r| DocumentHighlight {
                    range: Range::new(
                        Position::new(r.line.saturating_sub(1) as u32, r.column as u32),
                        Position::new(r.line.saturating_sub(1) as u32, (r.column + word.len()) as u32),
                    ),
                    kind: if r.is_definition {
                        Some(DocumentHighlightKind::WRITE)
                    } else {
                        Some(DocumentHighlightKind::READ)
                    },
                }).collect()
            })
            .unwrap_or_default();
        
        serde_json::to_value(highlights).ok()
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // إشعارات LSP
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// فتح مستند
    fn handle_did_open(&self, params: Value) {
        if let Ok(params) = serde_json::from_value::<DidOpenTextDocumentParams>(params) {
            let doc = params.text_document;
            self.state.open_document(doc.uri, doc.text, doc.version);
            eprintln!("[LSP] Opened document: {}", doc.uri);
        }
    }
    
    /// تغيير مستند
    fn handle_did_change(&self, params: Value) {
        if let Ok(params) = serde_json::from_value::<DidChangeTextDocumentParams>(params) {
            let uri = params.text_document.uri;
            let version = params.text_document.version.unwrap_or(0);
            
            // للحصول على المحتوى الكامل
            if let Some(change) = params.content_changes.first() {
                if change.range.is_none() {
                    // تحديث كامل
                    self.state.update_document(&uri, version, change.text.clone());
                }
            }
            
            eprintln!("[LSP] Changed document: {} (v{})", uri, version);
        }
    }
    
    /// إغلاق مستند
    fn handle_did_close(&self, params: Value) {
        if let Ok(params) = serde_json::from_value::<DidCloseTextDocumentParams>(params) {
            self.state.close_document(&params.text_document.uri);
            eprintln!("[LSP] Closed document: {}", params.text_document.uri);
        }
    }
    
    /// إنشاء إشعار التشخيصات
    fn create_diagnostics_notification(&self) -> LspMessage {
        LspMessage::Notification {
            method: "textDocument/publishDiagnostics".to_string(),
            params: Value::Null,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال مساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// إيجاد الكلمة عند موضع معين
fn find_word_at_position(line: &str, column: usize) -> Option<String> {
    let chars: Vec<char> = line.chars().collect();
    if chars.is_empty() {
        return None;
    }
    
    let column = column.min(chars.len().saturating_sub(1));
    
    // البحث عن بداية الكلمة
    let mut start = column;
    while start > 0 && is_arabic_word_char(chars[start - 1]) {
        start -= 1;
    }
    
    // البحث عن نهاية الكلمة
    let mut end = column;
    while end < chars.len() && is_arabic_word_char(chars[end]) {
        end += 1;
    }
    
    if start == end {
        return None;
    }
    
    Some(chars[start..end].iter().collect())
}

/// هل الحرف جزء من كلمة عربية
fn is_arabic_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// الحصول على الكلمات المفتاحية
fn get_keywords() -> Vec<&'static str> {
    vec![
        "متغير", "ثابت", "دالة", "إذا", "وإلا", "وإذا",
        "طالما", "لكل", "في", "أرجع", "أعطِ", "توقف", "أكمل",
        "صح", "خطأ", "لا_شيء", "جديد",
        "حاول", "امسك", "أخيراً", "ألقِ",
        "استورد", "من", "كـ", "صدر",
        "اطبع", "ادخل", "نوع", "طول",
        "و", "أو", "ليس",
        "تأكد", "كرر", "مرة",
        "طابق", "حالة", "افتراضي",
    ]
}

/// الحصول على نص الإدراج للكلمة المفتاحية
fn get_keyword_insert_text(kw: &str) -> String {
    match kw {
        "متغير" => "متغير ${1:الاسم} = ${2:القيمة}؛".to_string(),
        "ثابت" => "ثابت ${1:الاسم} = ${2:القيمة}؛".to_string(),
        "دالة" => "دالة ${1:الاسم}(${2:المعاملات}) {\n\t${3}\n}".to_string(),
        "إذا" => "إذا ${1:الشرط} {\n\t${2}\n}".to_string(),
        "وإلا" => "وإلا {\n\t${1}\n}".to_string(),
        "وإذا" => "وإذا ${1:الشرط} {\n\t${2}\n}".to_string(),
        "طالما" => "طالما ${1:الشرط} {\n\t${2}\n}".to_string(),
        "لكل" => "لكل ${1:العنصر} في ${2:المجموعة} {\n\t${3}\n}".to_string(),
        "اطبع" => "اطبع($1)；".to_string(),
        "أرجع" | "أعطِ" => "أعطِ ${1:القيمة}؛".to_string(),
        "حاول" => "حاول {\n\t${1}\n} امسك(${2:الخطأ}) {\n\t${3}\n}".to_string(),
        _ => kw.to_string(),
    }
}

/// الحصول على توثيق الكلمة المفتاحية
fn get_keyword_documentation(kw: &str) -> String {
    match kw {
        "متغير" => "**متغير** - تعريف متغير جديد\n\n```\nمتغير اسم = قيمة؛\n```".to_string(),
        "ثابت" => "**ثابت** - تعريف ثابت\n\n```\nثابت اسم = قيمة؛\n```".to_string(),
        "دالة" => "**دالة** - تعريف دالة\n\n```\nدالة اسم(معاملات) {\n    // الكود\n}\n```".to_string(),
        "إذا" => "**إذا** - جملة شرطية\n\n```\nإذا شرط {\n    // الكود\n}\n```".to_string(),
        _ => format!("**{}** - كلمة محجوزة", kw),
    }
}

/// الحصول على الدوال المدمجة
fn get_builtin_functions() -> Vec<(&'static str, String, String)> {
    vec![
        ("اطبع", "(نص)".to_string(), "**اطبع** - طباعة نص أو قيمة\n\n```\nاطبع(\"مرحباً\")؛\n```".to_string()),
        ("طول", "(قائمة)".to_string(), "**طول** - الحصول على طول القائمة أو النص".to_string()),
        ("نوع", "(قيمة)".to_string(), "**نوع** - الحصول على نوع القيمة".to_string()),
        ("أدخل", "()".to_string(), "**أدخل** - قراءة إدخال من المستخدم".to_string()),
    ]
}

/// الحصول على توثيق الكلمة
fn get_documentation_for_word(word: &str) -> String {
    // يمكن إضافة توثيق خاص بكل كلمة
    if let Some(doc) = get_builtin_docs().get(word) {
        return doc.clone();
    }
    String::new()
}

/// توثيق الدوال المدمجة
fn get_builtin_docs() -> std::collections::HashMap<&'static str, String> {
    let mut map = std::collections::HashMap::new();
    map.insert("اطبع", "اطبع النص المحدد في وحدة الإخراج".to_string());
    map.insert("طول", "يعيد عدد العناصر في القائمة أو عدد الأحرف في النص".to_string());
    map.insert("نوع", "يعيد نوع القيمة كنص".to_string());
    map
}
