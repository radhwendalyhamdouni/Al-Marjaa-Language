//! ═══════════════════════════════════════════════════════════════════════════════
//! Request Handlers - معالجات الطلبات الشاملة
//! ═══════════════════════════════════════════════════════════════════════════════
//! معالجة جميع طلبات وإشعارات LSP

use std::sync::Arc;

use lsp_types::*;
use serde_json::Value;

use crate::capabilities::{create_server_capabilities, get_server_info};
use crate::state::{ServerState, DefinitionKind};
use crate::{LspError, LspMessage};
use crate::semantic_tokens::build_semantic_tokens;
use crate::code_actions::get_code_actions;
use crate::code_lens::get_code_lenses;
use crate::inlay_hints::get_inlay_hints;
use crate::call_hierarchy::{get_call_hierarchy_incoming, get_call_hierarchy_outgoing};
use crate::formatting::format_document;
use crate::folding::get_folding_ranges;
use crate::rename::prepare_rename, rename_symbol;
use crate::signature_help::get_signature_help;
use crate::workspace_symbols::search_workspace_symbols;

/// معالج الطلبات
pub struct RequestHandler {
    state: Arc<ServerState>,
}

impl RequestHandler {
    /// إنشاء معالج جديد
    pub fn new(state: Arc<ServerState>) -> Self {
        Self { state }
    }
    
    /// معالجة طلب
    pub fn handle_request(&self, id: u64, method: &str, params: Value) -> LspMessage {
        let result = match method {
            // ═══════════════════════════════════════════════════════════
            // الطلبات الأساسية
            // ═══════════════════════════════════════════════════════════
            "initialize" => self.handle_initialize(params),
            "shutdown" => self.handle_shutdown(),
            
            // ═══════════════════════════════════════════════════════════
            // التنقل
            // ═══════════════════════════════════════════════════════════
            "textDocument/definition" => self.handle_definition(params),
            "textDocument/typeDefinition" => self.handle_type_definition(params),
            "textDocument/implementation" => self.handle_definition(params),
            "textDocument/references" => self.handle_references(params),
            "textDocument/documentSymbol" => self.handle_document_symbol(params),
            "workspace/symbol" => self.handle_workspace_symbol(params),
            
            // ═══════════════════════════════════════════════════════════
            // المعلومات
            // ═══════════════════════════════════════════════════════════
            "textDocument/hover" => self.handle_hover(params),
            "textDocument/completion" => self.handle_completion(params),
            "completionItem/resolve" => self.handle_completion_resolve(params),
            "textDocument/signatureHelp" => self.handle_signature_help(params),
            "textDocument/documentHighlight" => self.handle_document_highlight(params),
            
            // ═══════════════════════════════════════════════════════════
            // الإجراءات
            // ═══════════════════════════════════════════════════════════
            "textDocument/codeAction" => self.handle_code_action(params),
            "codeAction/resolve" => self.handle_code_action_resolve(params),
            "textDocument/codeLens" => self.handle_code_lens(params),
            "codeLens/resolve" => self.handle_code_lens_resolve(params),
            
            // ═══════════════════════════════════════════════════════════
            // التنسيق والتحويل
            // ═══════════════════════════════════════════════════════════
            "textDocument/formatting" => self.handle_formatting(params),
            "textDocument/rangeFormatting" => self.handle_range_formatting(params),
            "textDocument/onTypeFormatting" => self.handle_on_type_formatting(params),
            "textDocument/foldingRange" => self.handle_folding_range(params),
            "textDocument/selectionRange" => self.handle_selection_range(params),
            
            // ═══════════════════════════════════════════════════════════
            // إعادة التسمية
            // ═══════════════════════════════════════════════════════════
            "textDocument/prepareRename" => self.handle_prepare_rename(params),
            "textDocument/rename" => self.handle_rename(params),
            
            // ═══════════════════════════════════════════════════════════
            // Semantic Tokens
            // ═══════════════════════════════════════════════════════════
            "textDocument/semanticTokens/full" => self.handle_semantic_tokens_full(params),
            "textDocument/semanticTokens/range" => self.handle_semantic_tokens_range(params),
            
            // ═══════════════════════════════════════════════════════════
            // Inlay Hints
            // ═══════════════════════════════════════════════════════════
            "textDocument/inlayHint" => self.handle_inlay_hint(params),
            "inlayHint/resolve" => self.handle_inlay_hint_resolve(params),
            
            // ═══════════════════════════════════════════════════════════
            // Call Hierarchy
            // ═══════════════════════════════════════════════════════════
            "textDocument/prepareCallHierarchy" => self.handle_prepare_call_hierarchy(params),
            "callHierarchy/incomingCalls" => self.handle_call_hierarchy_incoming(params),
            "callHierarchy/outgoingCalls" => self.handle_call_hierarchy_outgoing(params),
            
            // ═══════════════════════════════════════════════════════════
            // Linked Editing
            // ═══════════════════════════════════════════════════════════
            "textDocument/linkedEditingRange" => self.handle_linked_editing(params),
            
            // ═══════════════════════════════════════════════════════════
            // الأوامر
            // ═══════════════════════════════════════════════════════════
            "workspace/executeCommand" => self.handle_execute_command(params),
            
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
                *self.state.initialized.write() = true;
                log::info!("Client initialized successfully");
                None
            }
            "textDocument/didOpen" => {
                self.handle_did_open(params);
                None
            }
            "textDocument/didChange" => {
                self.handle_did_change(params);
                None
            }
            "textDocument/didClose" => {
                self.handle_did_close(params);
                None
            }
            "textDocument/didSave" => {
                // إعادة التحليل عند الحفظ
                None
            }
            "textDocument/willSave" => {
                None
            }
            "workspace/didChangeConfiguration" => {
                self.handle_config_change(params);
                None
            }
            "workspace/didChangeWorkspaceFolders" => {
                None
            }
            "workspace/didCreateFiles" => {
                None
            }
            "workspace/didRenameFiles" => {
                None
            }
            "workspace/didDeleteFiles" => {
                None
            }
            "$/cancelRequest" => {
                None
            }
            "$/setTrace" => {
                None
            }
            "exit" => {
                log::info!("Exit notification received");
                None
            }
            _ => {
                log::debug!("Unknown notification: {}", method);
                None
            }
        }
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // طلبات LSP - التهيئة
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// تهيئة الخادم
    fn handle_initialize(&self, params: Value) -> Option<Value> {
        let params: InitializeParams = serde_json::from_value(params)
            .unwrap_or_else(|_| InitializeParams::default());
        
        *self.state.initialize_params.write() = Some(params.clone());
        
        // تطبيق الإعدادات
        if let Some(init_opts) = params.initialization_options {
            if let Some(obj) = init_opts.as_object() {
                let mut settings = self.state.settings.write();
                if let Some(enabled) = obj.get("semanticTokens").and_then(|v| v.as_bool()) {
                    settings.semantic_tokens_enabled = enabled;
                }
                if let Some(enabled) = obj.get("inlayHints").and_then(|v| v.as_bool()) {
                    settings.inlay_hints_enabled = enabled;
                }
                if let Some(enabled) = obj.get("codeLens").and_then(|v| v.as_bool()) {
                    settings.code_lens_enabled = enabled;
                }
            }
        }
        
        let result = InitializeResult {
            capabilities: create_server_capabilities(),
            server_info: Some(get_server_info()),
        };
        
        serde_json::to_value(result).ok()
    }
    
    /// إيقاف الخادم
    fn handle_shutdown(&self) -> Option<Value> {
        Some(Value::Null)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // طلبات LSP - التنقل
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// الانتقال للتعريف
    fn handle_definition(&self, params: Value) -> Option<Value> {
        let params: TextDocumentPositionParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        let position = params.position;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        let line = doc.get_line(position.line as usize)?;
        let word = find_word_at_position(line, position.character as usize)?;
        
        if let Some(def) = analysis.definitions.get(&word) {
            let location = Location {
                uri,
                range: Range::new(
                    Position::new(def.line.saturating_sub(1) as u32, def.column as u32),
                    Position::new(def.line.saturating_sub(1) as u32, def.end_column as u32),
                ),
            };
            
            // تحديث الإحصائيات
            self.state.stats.write().definitions_found += 1;
            
            return serde_json::to_value(GotoDefinitionResponse::Scalar(location)).ok();
        }
        
        None
    }
    
    /// الانتقال لتعريف النوع
    fn handle_type_definition(&self, params: Value) -> Option<Value> {
        // يمكن توسيع هذا لاحقاً عند إضافة نظام أنواع كامل
        self.handle_definition(params)
    }
    
    /// إيجاد المراجع
    fn handle_references(&self, params: Value) -> Option<Value> {
        let params: ReferenceParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        let line = doc.get_line(position.line as usize)?;
        let word = find_word_at_position(line, position.character as usize)?;
        
        if let Some(refs) = analysis.references.get(&word) {
            let include_declaration = params.context.include_declaration;
            
            let locations: Vec<Location> = refs.iter()
                .filter(|r| include_declaration || !r.is_definition)
                .map(|r| Location {
                    uri: uri.clone(),
                    range: Range::new(
                        Position::new(r.line.saturating_sub(1) as u32, r.column as u32),
                        Position::new(r.line.saturating_sub(1) as u32, r.end_column as u32),
                    ),
                })
                .collect();
            
            // تحديث الإحصائيات
            self.state.stats.write().references_found += locations.len() as u64;
            
            return serde_json::to_value(locations).ok();
        }
        
        Some(Value::Array(vec![]))
    }
    
    /// رموز المستند
    fn handle_document_symbol(&self, params: Value) -> Option<Value> {
        let params: DocumentSymbolParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        let analysis = self.state.get_analysis(&uri)?;
        
        let symbols: Vec<DocumentSymbol> = analysis.definitions.values()
            .map(|def| {
                DocumentSymbol {
                    name: def.name.clone(),
                    detail: def.type_annotation.clone(),
                    kind: def.kind.to_symbol_kind(),
                    tags: None,
                    deprecated: None,
                    range: Range::new(
                        Position::new(def.line.saturating_sub(1) as u32, 0),
                        Position::new(def.line.saturating_sub(1) as u32, 100),
                    ),
                    selection_range: Range::new(
                        Position::new(def.line.saturating_sub(1) as u32, def.column as u32),
                        Position::new(def.line.saturating_sub(1) as u32, def.end_column as u32),
                    ),
                    children: None,
                }
            })
            .collect();
        
        serde_json::to_value(symbols).ok()
    }
    
    /// رموز مساحة العمل
    fn handle_workspace_symbol(&self, params: Value) -> Option<Value> {
        let params: WorkspaceSymbolParams = serde_json::from_value(params).ok()?;
        let query = params.query;
        
        let symbols = search_workspace_symbols(&self.state, &query);
        serde_json::to_value(symbols).ok()
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // طلبات LSP - المعلومات
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// معلومات التمرير (Hover)
    fn handle_hover(&self, params: Value) -> Option<Value> {
        let params: TextDocumentPositionParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        let position = params.position;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        let line = doc.get_line(position.line as usize)?;
        let word = find_word_at_position(line, position.character as usize)?;
        
        let (message, kind_str) = if let Some(def) = analysis.definitions.get(&word) {
            let kind = match def.kind {
                DefinitionKind::Variable => "متغير",
                DefinitionKind::Function => "دالة",
                DefinitionKind::Constant => "ثابت",
                DefinitionKind::Parameter => "معامل",
                DefinitionKind::Class => "صنف",
                DefinitionKind::Method => "طريقة",
                DefinitionKind::Property => "خاصية",
                DefinitionKind::Enum => "تعداد",
                DefinitionKind::Interface => "واجهة",
                _ => "معرف",
            };
            let location = format!("مُعرّف عند السطر {}", def.line);
            if let Some(type_ann) = &def.type_annotation {
                (format!("{}: {}", location, type_ann), format!("{} ({})", kind, type_ann))
            } else {
                (location, kind.to_string())
            }
        } else {
            let usage_count = analysis.references.get(&word)
                .map(|r| r.len())
                .unwrap_or(0);
            (format!("مستخدم {} مرة", usage_count), "معرف".to_string())
        };
        
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
        
        // تحديث الإحصائيات
        self.state.stats.write().completions_provided += 1;
        
        let mut items: Vec<CompletionItem> = Vec::new();
        
        // 1. الكلمات المفتاحية
        for kw in get_keywords() {
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
                    sort_text: Some(format!("1{}", kw)),
                    ..CompletionItem::default()
                });
            }
        }
        
        // 2. الدوال المدمجة
        for (name, sig, doc) in get_builtin_functions() {
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
                    sort_text: Some(format!("2{}", name)),
                    ..CompletionItem::default()
                });
            }
        }
        
        // 3. الرموز المُعرّفة في الملف
        for (name, def) in &analysis.definitions {
            if name.starts_with(&prefix) || prefix.is_empty() {
                let kind = def.kind.to_completion_kind();
                
                items.push(CompletionItem {
                    label: name.clone(),
                    kind: Some(kind),
                    detail: Some(format!("السطر {}", def.line)),
                    sort_text: Some(format!("0{}", name)), // أولوية عالية
                    ..CompletionItem::default()
                });
            }
        }
        
        // 4. وحدات المكتبة القياسية
        for (name, doc) in get_stdlib_modules() {
            if name.starts_with(&prefix) || prefix.is_empty() {
                items.push(CompletionItem {
                    label: name.clone(),
                    kind: Some(CompletionItemKind::MODULE),
                    detail: Some("مكتبة قياسية".to_string()),
                    documentation: Some(Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: doc,
                    })),
                    sort_text: Some(format!("3{}", name)),
                    ..CompletionItem::default()
                });
            }
        }
        
        // ترتيب النتائج حسب sort_text
        items.sort_by(|a, b| a.sort_text.cmp(&b.sort_text));
        
        let response = CompletionResponse::Array(items);
        serde_json::to_value(response).ok()
    }
    
    /// حل الإكمال
    fn handle_completion_resolve(&self, params: Value) -> Option<Value> {
        let item: CompletionItem = serde_json::from_value(params).ok()?;
        // يمكن إضافة توثيق إضافي هنا
        Some(serde_json::to_value(item).unwrap())
    }
    
    /// مساعدة التوقيع
    fn handle_signature_help(&self, params: Value) -> Option<Value> {
        let params: SignatureHelpParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        let signature_help = get_signature_help(&doc, &analysis, position);
        serde_json::to_value(signature_help).ok()
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
                        Position::new(r.line.saturating_sub(1) as u32, r.end_column as u32),
                    ),
                    kind: if r.is_definition {
                        Some(DocumentHighlightKind::WRITE)
                    } else if r.is_write {
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
    // طلبات LSP - الإجراءات
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Code Action
    fn handle_code_action(&self, params: Value) -> Option<Value> {
        let params: CodeActionParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        let range = params.range;
        
        let analysis = self.state.get_analysis(&uri)?;
        let actions = get_code_actions(&uri, &range, &analysis, &params.context);
        
        serde_json::to_value(actions).ok()
    }
    
    /// حل Code Action
    fn handle_code_action_resolve(&self, params: Value) -> Option<Value> {
        let action: CodeAction = serde_json::from_value(params).ok()?;
        Some(serde_json::to_value(action).unwrap())
    }
    
    /// Code Lens
    fn handle_code_lens(&self, params: Value) -> Option<Value> {
        let params: CodeLensParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        let lenses = get_code_lenses(&uri, &doc, &analysis);
        serde_json::to_value(lenses).ok()
    }
    
    /// حل Code Lens
    fn handle_code_lens_resolve(&self, params: Value) -> Option<Value> {
        let lens: CodeLens = serde_json::from_value(params).ok()?;
        Some(serde_json::to_value(lens).unwrap())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // طلبات LSP - التنسيق
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// تنسيق المستند
    fn handle_formatting(&self, params: Value) -> Option<Value> {
        let params: DocumentFormattingParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        
        let doc = self.state.get_document(&uri)?;
        let edits = format_document(&doc.content, &params.options);
        
        serde_json::to_value(edits).ok()
    }
    
    /// تنسيق نطاق
    fn handle_range_formatting(&self, params: Value) -> Option<Value> {
        let params: DocumentRangeFormattingParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        
        let doc = self.state.get_document(&uri)?;
        let edits = format_document(&doc.content, &params.options);
        
        serde_json::to_value(edits).ok()
    }
    
    /// تنسيق عند الكتابة
    fn handle_on_type_formatting(&self, params: Value) -> Option<Value> {
        let params: DocumentOnTypeFormattingParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        
        let doc = self.state.get_document(&uri)?;
        let edits = format_document(&doc.content, &params.options);
        
        serde_json::to_value(edits).ok()
    }
    
    /// نطاقات الطي
    fn handle_folding_range(&self, params: Value) -> Option<Value> {
        let params: FoldingRangeParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        
        let analysis = self.state.get_analysis(&uri)?;
        let ranges = get_folding_ranges(&analysis);
        
        serde_json::to_value(ranges).ok()
    }
    
    /// نطاقات التحديد
    fn handle_selection_range(&self, params: Value) -> Option<Value> {
        let params: SelectionRangeParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        let ranges: Vec<SelectionRange> = params.positions.iter()
            .filter_map(|pos| {
                let line = doc.get_line(pos.line as usize)?;
                let word = find_word_at_position(line, pos.character as usize)?;
                
                if let Some(refs) = analysis.references.get(&word) {
                    if let Some(first) = refs.first() {
                        return Some(SelectionRange {
                            range: Range::new(
                                Position::new(first.line.saturating_sub(1) as u32, first.column as u32),
                                Position::new(first.line.saturating_sub(1) as u32, first.end_column as u32),
                            ),
                            parent: None,
                        });
                    }
                }
                None
            })
            .collect();
        
        serde_json::to_value(ranges).ok()
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // طلبات LSP - إعادة التسمية
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// تجهيز إعادة التسمية
    fn handle_prepare_rename(&self, params: Value) -> Option<Value> {
        let params: TextDocumentPositionParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        let position = params.position;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        let result = prepare_rename(&doc, &analysis, position);
        serde_json::to_value(result).ok()
    }
    
    /// إعادة التسمية
    fn handle_rename(&self, params: Value) -> Option<Value> {
        let params: RenameParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let new_name = params.new_name;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        let result = rename_symbol(&doc, &analysis, position, &new_name);
        serde_json::to_value(result).ok()
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // طلبات LSP - Semantic Tokens
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Semantic Tokens كامل
    fn handle_semantic_tokens_full(&self, params: Value) -> Option<Value> {
        let params: SemanticTokensParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        
        let analysis = self.state.get_analysis(&uri)?;
        let tokens = build_semantic_tokens(&analysis.semantic_tokens);
        
        serde_json::to_value(tokens).ok()
    }
    
    /// Semantic Tokens نطاق
    fn handle_semantic_tokens_range(&self, params: Value) -> Option<Value> {
        let params: SemanticTokensRangeParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        let range = params.range;
        
        let analysis = self.state.get_analysis(&uri)?;
        
        let tokens: Vec<_> = analysis.semantic_tokens.iter()
            .filter(|t| {
                t.line >= range.start.line as usize && t.line <= range.end.line as usize
            })
            .cloned()
            .collect();
        
        let tokens = build_semantic_tokens(&tokens);
        serde_json::to_value(tokens).ok()
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // طلبات LSP - Inlay Hints
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Inlay Hints
    fn handle_inlay_hint(&self, params: Value) -> Option<Value> {
        let params: InlayHintParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        let range = params.range;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        let hints = get_inlay_hints(&doc, &analysis, range);
        serde_json::to_value(hints).ok()
    }
    
    /// حل Inlay Hint
    fn handle_inlay_hint_resolve(&self, params: Value) -> Option<Value> {
        let hint: InlayHint = serde_json::from_value(params).ok()?;
        Some(serde_json::to_value(hint).unwrap())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // طلبات LSP - Call Hierarchy
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// تجهيز Call Hierarchy
    fn handle_prepare_call_hierarchy(&self, params: Value) -> Option<Value> {
        let params: CallHierarchyPrepareParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        let position = params.position;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        let line = doc.get_line(position.line as usize)?;
        let word = find_word_at_position(line, position.character as usize)?;
        
        if let Some(def) = analysis.definitions.get(&word) {
            if def.kind == DefinitionKind::Function || def.kind == DefinitionKind::Method {
                let item = CallHierarchyItem {
                    name: word.clone(),
                    kind: def.kind.to_symbol_kind(),
                    tags: None,
                    detail: None,
                    uri: uri.clone(),
                    range: Range::new(
                        Position::new(def.line.saturating_sub(1) as u32, 0),
                        Position::new(def.line.saturating_sub(1) as u32, 100),
                    ),
                    selection_range: Range::new(
                        Position::new(def.line.saturating_sub(1) as u32, def.column as u32),
                        Position::new(def.line.saturating_sub(1) as u32, def.end_column as u32),
                    ),
                    data: None,
                };
                
                return serde_json::to_value(vec![item]).ok();
            }
        }
        
        Some(Value::Array(vec![]))
    }
    
    /// الاستدعاءات الواردة
    fn handle_call_hierarchy_incoming(&self, params: Value) -> Option<Value> {
        let params: CallHierarchyIncomingCallsParams = serde_json::from_value(params).ok()?;
        let calls = get_call_hierarchy_incoming(&self.state, &params.item);
        serde_json::to_value(calls).ok()
    }
    
    /// الاستدعاءات الصادرة
    fn handle_call_hierarchy_outgoing(&self, params: Value) -> Option<Value> {
        let params: CallHierarchyOutgoingCallsParams = serde_json::from_value(params).ok()?;
        let calls = get_call_hierarchy_outgoing(&self.state, &params.item);
        serde_json::to_value(calls).ok()
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // طلبات LSP - Linked Editing
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// نطاق التحرير المرتبط
    fn handle_linked_editing(&self, params: Value) -> Option<Value> {
        let params: LinkedEditingRangeParams = serde_json::from_value(params).ok()?;
        let uri = params.text_document.uri;
        let position = params.position;
        
        let doc = self.state.get_document(&uri)?;
        let analysis = self.state.get_analysis(&uri)?;
        
        let line = doc.get_line(position.line as usize)?;
        let word = find_word_at_position(line, position.character as usize)?;
        
        if let Some(refs) = analysis.references.get(&word) {
            let ranges: Vec<Range> = refs.iter()
                .map(|r| Range::new(
                    Position::new(r.line.saturating_sub(1) as u32, r.column as u32),
                    Position::new(r.line.saturating_sub(1) as u32, r.end_column as u32),
                ))
                .collect();
            
            if !ranges.is_empty() {
                return serde_json::to_value(LinkedEditingRanges {
                    ranges,
                    word_pattern: None,
                }).ok();
            }
        }
        
        None
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // طلبات LSP - الأوامر
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// تنفيذ أمر
    fn handle_execute_command(&self, params: Value) -> Option<Value> {
        let params: ExecuteCommandParams = serde_json::from_value(params).ok()?;
        let command = params.command;
        let _arguments = params.arguments;
        
        log::info!("Executing command: {}", command);
        
        // يمكن إضافة منطق الأوامر هنا
        match command.as_str() {
            "almarjaa.runFile" => {
                // تشغيل الملف
            }
            "almarjaa.format" => {
                // تنسيق
            }
            "almarjaa.fixAll" => {
                // إصلاح الكل
            }
            _ => {
                log::warn!("Unknown command: {}", command);
            }
        }
        
        Some(Value::Null)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // إشعارات LSP
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// فتح مستند
    fn handle_did_open(&self, params: Value) {
        if let Ok(params) = serde_json::from_value::<DidOpenTextDocumentParams>(params) {
            let doc = params.text_document;
            self.state.open_document(doc.uri, doc.text, doc.version.unwrap_or(0));
            log::info!("Opened document: {}", doc.uri);
        }
    }
    
    /// تغيير مستند
    fn handle_did_change(&self, params: Value) {
        if let Ok(params) = serde_json::from_value::<DidChangeTextDocumentParams>(params) {
            let uri = params.text_document.uri;
            let version = params.text_document.version.unwrap_or(0);
            
            if let Some(change) = params.content_changes.first() {
                if change.range.is_none() {
                    self.state.update_document(&uri, version, change.text.clone());
                }
            }
            
            log::debug!("Changed document: {} (v{})", uri, version);
        }
    }
    
    /// إغلاق مستند
    fn handle_did_close(&self, params: Value) {
        if let Ok(params) = serde_json::from_value::<DidCloseTextDocumentParams>(params) {
            self.state.close_document(&params.text_document.uri);
            log::info!("Closed document: {}", params.text_document.uri);
        }
    }
    
    /// تغيير الإعدادات
    fn handle_config_change(&self, params: Value) {
        if let Ok(params) = serde_json::from_value::<DidChangeConfigurationParams>(params) {
            if let Some(settings) = params.settings.get("almarjaa") {
                if let Some(obj) = settings.as_object() {
                    let mut srv_settings = self.state.settings.write();
                    
                    if let Some(enabled) = obj.get("semanticTokens").and_then(|v| v.as_bool()) {
                        srv_settings.semantic_tokens_enabled = enabled;
                    }
                    if let Some(enabled) = obj.get("inlayHints").and_then(|v| v.as_bool()) {
                        srv_settings.inlay_hints_enabled = enabled;
                    }
                    if let Some(enabled) = obj.get("codeLens").and_then(|v| v.as_bool()) {
                        srv_settings.code_lens_enabled = enabled;
                    }
                    if let Some(max) = obj.get("maxDiagnostics").and_then(|v| v.as_u64()) {
                        srv_settings.max_diagnostics = max as usize;
                    }
                }
            }
            log::info!("Configuration updated");
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال مساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// إيجاد الكلمة عند موضع معين
pub fn find_word_at_position(line: &str, column: usize) -> Option<String> {
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
        // الأساسية
        "متغير", "ثابت", "دالة", "إذا", "وإلا", "وإذا",
        "طالما", "لكل", "في", "أرجع", "أعطِ", "توقف", "أكمل",
        "صح", "خطأ", "لا_شيء", "جديد",
        
        // الاستثناءات
        "حاول", "امسك", "أخيراً", "ألقِ",
        
        // الوحدات
        "استورد", "من", "كـ", "صدر", "وحدة",
        
        // الإدخال/الإخراج
        "اطبع", "ادخل", "نوع", "طول",
        
        // المنطقية
        "و", "أو", "ليس",
        
        // التأكيد
        "تأكد", "كرر", "مرة",
        
        // المطابقة
        "طابق", "حالة", "افتراضي",
        
        // الصنف
        "صنف", "هذا", "أصل",
        
        // التزامن
        "غير_متزامن", "انتظر",
        
        // اللامدا
        "لامدا",
        
        // التعداد
        "تعداد", "بيانات",
        
        // معالجة السياق
        "مع",
        
        // ONNX والذكاء الاصطناعي
        "أونكس", "نموذج", "حمّل", "احفظ", "استدل",
        "موتر", "شكل", "مخرج", "طبقة",
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
        "صنف" => "صنف ${1:الاسم} {\n\t${2}\n}".to_string(),
        "تعداد" => "تعداد ${1:الاسم} {\n\t${2}\n}".to_string(),
        "بيانات" => "بيانات ${1:الاسم} {\n\t${2}\n}".to_string(),
        _ => kw.to_string(),
    }
}

/// الحصول على توثيق الكلمة المفتاحية
fn get_keyword_documentation(kw: &str) -> String {
    match kw {
        "متغير" => "**متغير** - تعريف متغير جديد\n\n```almarjaa\nمتغير اسم = قيمة؛\n```\n\nالمتغير يمكن تغيير قيمته لاحقاً.".to_string(),
        "ثابت" => "**ثابت** - تعريف ثابت\n\n```almarjaa\nثابت اسم = قيمة؛\n```\n\nالثابت لا يمكن تغيير قيمته بعد التعريف.".to_string(),
        "دالة" => "**دالة** - تعريف دالة\n\n```almarjaa\ndالة اسم(معاملات) {\n    // الكود\n}\n```\n\nالدوال يمكنها إرجاع قيم باستخدام `أعطِ`.".to_string(),
        "إذا" => "**إذا** - جملة شرطية\n\n```almarjaa\nإذا شرط {\n    // الكود\n}\n```\n\nيمكن إضافة `وإلا` و `وإذا` للفروع الإضافية.".to_string(),
        "طالما" => "**طالما** - حلقة تكرار\n\n```almarjaa\nطالما شرط {\n    // الكود\n}\n```\n\nتستمر الحلقة طالما الشرط صحيح.".to_string(),
        "لكل" => "**لكل** - حلقة تكرار على عناصر مجموعة\n\n```almarjaa\nلكل عنصر في مجموعة {\n    // الكود\n}\n```\n\nتكرر على كل عنصر في المجموعة.".to_string(),
        "حاول" => "**حاول** - معالجة الاستثناءات\n\n```almarjaa\nحاول {\n    // كود قد يسبب خطأ\n} امسك(خطأ) {\n    // معالجة الخطأ\n}\n```\n\nيمكن إضافة `أخيراً` للتنفيذ دائماً.".to_string(),
        "صنف" => "**صنف** - تعريف صنف (class)\n\n```almarjaa\nصنف اسم {\n    // الخصائص والطرق\n}\n```\n\nالأصناف هي قوالب لإنشاء كائنات.".to_string(),
        _ => format!("**{}** - كلمة محجوزة في لغة المرجع", kw),
    }
}

/// الحصول على الدوال المدمجة
fn get_builtin_functions() -> Vec<(&'static str, String, String)> {
    vec![
        ("اطبع", "(نص)".to_string(), "**اطبع** - طباعة نص أو قيمة\n\n```almarjaa\nاطبع(\"مرحباً بالعالم\")؛\nاطبع(متغير)؛\n```".to_string()),
        ("طول", "(قائمة)".to_string(), "**طول** - الحصول على طول القائمة أو النص\n\n```almarjaa\nطول([١، ٢، ٣]) // ٣\nttطول(\"مرحبا\") // ٥\n```".to_string()),
        ("نوع", "(قيمة)".to_string(), "**نوع** - الحصول على نوع القيمة\n\n```almarjaa\nنوع(١٠) // \"رقم\"\nنوع(\"نص\") // \"نص\"\n```".to_string()),
        ("أدخل", "()".to_string(), "**أدخل** - قراءة إدخال من المستخدم\n\n```almarjaa\nمتغير اسم = أدخل()؛\n```".to_string()),
        ("مدى", "(بداية، نهاية)".to_string(), "**مدى** - إنشاء نطاق من الأرقام\n\n```almarjaa\nلكل س في مدى(١، ١٠) {\n    اطبع(س)؛\n}\n```".to_string()),
        ("قائمة", "()".to_string(), "**قائمة** - إنشاء قائمة فارغة\n\n```almarjaa\nمتجر عناصر = قائمة()؛\n```".to_string()),
        ("قاموس", "()".to_string(), "**قاموس** - إنشاء قاموس فارغ\n\n```almarjaa\nمتجر بيانات = قاموس()؛\n```".to_string()),
    ]
}

/// الحصول على وحدات المكتبة القياسية
fn get_stdlib_modules() -> Vec<(&'static str, String)> {
    vec![
        ("http", "**http** - مكتبة HTTP\n\nتوفر دوال للتعامل مع الطلبات والاستجابات HTTP.".to_string()),
        ("قاعدة_بيانات", "**قاعدة_بيانات** - مكتبة قواعد البيانات\n\nدعم SQLite و PostgreSQL و MySQL و MongoDB.".to_string()),
        ("تعبير_نمطي", "**تعبير_نمطي** - مكتبة التعابير النمطية\n\nمطابقة النصوص والبحث والاستبدال.".to_string()),
        ("تشفير", "**تشفير** - مكتبة التشفير\n\nدوال التشفير والهاش و JWT.".to_string()),
    ]
}

/// الحصول على توثيق الكلمة
fn get_documentation_for_word(word: &str) -> String {
    let builtin_docs = get_builtin_docs();
    if let Some(doc) = builtin_docs.get(word) {
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
    map.insert("أدخل", "يقرأ سطر من الإدخال من المستخدم".to_string());
    map.insert("مدى", "ينشئ نطاقاً من الأرقام للتكرار".to_string());
    map
}
