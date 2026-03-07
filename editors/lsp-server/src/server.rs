//! ═══════════════════════════════════════════════════════════════════════════════
//! LSP Server - الخادم الرئيسي
//! حلقة المعالجة الرئيسية وتوزيع الطلبات
//! متكامل مع المحلل الفعلي للغة المرجع
//! ═══════════════════════════════════════════════════════════════════════════════

use std::sync::Arc;
use std::thread::{self, JoinHandle};

use crossbeam_channel::{Receiver, Sender};
use lsp_types::{PublishDiagnosticsParams, Url, Diagnostic};

use crate::handlers::RequestHandler;
use crate::state::ServerState;
use crate::{LspMessage, LspError};

/// خادم LSP
pub struct LspServer {
    receiver: Receiver<LspMessage>,
    sender: Sender<LspMessage>,
    state: Arc<ServerState>,
}

impl LspServer {
    /// إنشاء خادم جديد
    pub fn new(receiver: Receiver<LspMessage>, sender: Sender<LspMessage>) -> Self {
        Self {
            receiver,
            sender,
            state: Arc::new(ServerState::new()),
        }
    }
    
    /// بدء الخادم
    pub fn start(self) -> JoinHandle<()> {
        thread::spawn(move || {
            self.run();
        })
    }
    
    /// حلقة المعالجة الرئيسية
    fn run(&self) {
        eprintln!("[LSP] ════════════════════════════════════════════════════════════");
        eprintln!("[LSP]    Al-Marjaa Language Server v3.0.0");
        eprintln!("[LSP]    لغة برمجة عربية متكاملة مع الذكاء الاصطناعي");
        eprintln!("[LSP] ════════════════════════════════════════════════════════════");
        eprintln!("[LSP] Server started, waiting for messages...");
        eprintln!("[LSP]");
        
        loop {
            match self.receiver.recv() {
                Ok(message) => {
                    self.handle_message(message);
                }
                Err(_) => {
                    eprintln!("[LSP] Channel closed, shutting down...");
                    break;
                }
            }
        }
        
        eprintln!("[LSP] Server stopped.");
    }
    
    /// معالجة رسالة
    fn handle_message(&self, message: LspMessage) {
        let handler = RequestHandler::new(self.state.clone());
        
        match message {
            LspMessage::Request { id, method, params } => {
                eprintln!("[LSP] → Request #{}: {}", id, method);
                
                let start = std::time::Instant::now();
                let response = handler.handle_request(id, &method, params);
                let elapsed = start.elapsed();
                
                eprintln!("[LSP] ← Response #{}: {:?} ({}ms)", id, 
                    if response.is_success() { "OK" } else { "Error" },
                    elapsed.as_millis()
                );
                
                if self.sender.send(response).is_err() {
                    eprintln!("[LSP] ⚠ Failed to send response");
                }
            }
            LspMessage::Notification { method, params } => {
                eprintln!("[LSP] → Notification: {}", method);
                
                if let Some(response) = handler.handle_notification(&method, params.clone()) {
                    // نشر التشخيصات
                    if method == "textDocument/didOpen" || method == "textDocument/didChange" {
                        if let Some(uri) = self.extract_uri(&params) {
                            self.publish_diagnostics(&uri);
                        }
                    } else if response.is_notification() {
                        if self.sender.send(response).is_err() {
                            eprintln!("[LSP] ⚠ Failed to send notification");
                        }
                    }
                }
            }
            LspMessage::Response { .. } => {
                eprintln!("[LSP] ⚠ Unexpected response from client");
            }
        }
    }
    
    /// استخراج URI من المعاملات
    fn extract_uri(&self, params: &serde_json::Value) -> Option<Url> {
        if let Some(obj) = params.as_object() {
            if let Some(td) = obj.get("textDocument") {
                if let Some(uri_str) = td.get("uri").and_then(|u| u.as_str()) {
                    return Url::parse(uri_str).ok();
                }
            }
        }
        None
    }
    
    /// نشر التشخيصات
    fn publish_diagnostics(&self, uri: &Url) {
        let diagnostics = self.state.get_analysis(uri)
            .map(|a| a.diagnostics.clone())
            .unwrap_or_default();
        
        let params = PublishDiagnosticsParams {
            uri: uri.clone(),
            diagnostics,
            version: None,
        };
        
        let notification = LspMessage::Notification {
            method: "textDocument/publishDiagnostics".to_string(),
            params: serde_json::to_value(params).unwrap_or(serde_json::Value::Null),
        };
        
        if self.sender.send(notification).is_err() {
            eprintln!("[LSP] ⚠ Failed to publish diagnostics for: {}", uri);
        } else {
            eprintln!("[LSP] ✓ Published diagnostics for: {}", uri);
        }
    }
}

impl LspMessage {
    /// هل هي استجابة ناجحة؟
    fn is_success(&self) -> bool {
        match self {
            LspMessage::Response { error, .. } => error.is_none(),
            _ => true,
        }
    }
    
    /// هل هي إشعار؟
    fn is_notification(&self) -> bool {
        matches!(self, LspMessage::Notification { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_server_creation() {
        let (sender, receiver) = crossbeam_channel::bounded(16);
        let _server = LspServer::new(receiver, sender);
    }
}
