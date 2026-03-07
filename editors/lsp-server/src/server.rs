//! ═══════════════════════════════════════════════════════════════════════════════
//! LSP Server - الخادم الرئيسي
//! ═══════════════════════════════════════════════════════════════════════════════
//! حلقة المعالجة الرئيسية وتوزيع الطلبات

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
    pub fn new(
        receiver: Receiver<LspMessage>, 
        sender: Sender<LspMessage>,
        state: Arc<ServerState>,
    ) -> Self {
        Self {
            receiver,
            sender,
            state,
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
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("   Al-Marjaa Language Server v3.3.0");
        log::info!("   لغة برمجة عربية متكاملة مع الذكاء الاصطناعي");
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("Server started, waiting for messages...");
        
        loop {
            match self.receiver.recv() {
                Ok(message) => {
                    self.handle_message(message);
                }
                Err(_) => {
                    log::info!("Channel closed, shutting down...");
                    break;
                }
            }
        }
        
        log::info!("Server stopped.");
    }
    
    /// معالجة رسالة
    fn handle_message(&self, message: LspMessage) {
        let handler = RequestHandler::new(self.state.clone());
        
        match message {
            LspMessage::Request { id, method, params } => {
                let start = std::time::Instant::now();
                log::debug!("→ Request #{}: {}", id, method);
                
                let response = handler.handle_request(id, &method, params);
                let elapsed = start.elapsed();
                
                let status = if response.is_success() { "OK" } else { "Error" };
                log::debug!("← Response #{}: {} ({}ms)", id, status, elapsed.as_millis());
                
                if self.sender.send(response).is_err() {
                    log::error!("Failed to send response");
                }
            }
            LspMessage::Notification { method, params } => {
                log::debug!("→ Notification: {}", method);
                
                match handler.handle_notification(&method, params.clone()) {
                    Some(response) => {
                        // نشر التشخيصات
                        if method == "textDocument/didOpen" || method == "textDocument/didChange" {
                            if let Some(uri) = self.extract_uri(&params) {
                                self.publish_diagnostics(&uri);
                            }
                        } else if response.is_notification() {
                            if self.sender.send(response).is_err() {
                                log::error!("Failed to send notification");
                            }
                        }
                    }
                    None => {
                        // نشر التشخيصات للملفات المفتوحة/المعدلة
                        if method == "textDocument/didOpen" || method == "textDocument/didChange" {
                            if let Some(uri) = self.extract_uri(&params) {
                                self.publish_diagnostics(&uri);
                            }
                        }
                    }
                }
            }
            LspMessage::Response { .. } => {
                log::warn!("Unexpected response from client");
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
        
        // تحديث الإحصائيات
        self.state.stats.write().diagnostics_sent += 1;
        
        let notification = LspMessage::Notification {
            method: "textDocument/publishDiagnostics".to_string(),
            params: serde_json::to_value(params).unwrap_or(serde_json::Value::Null),
        };
        
        if self.sender.send(notification).is_err() {
            log::error!("Failed to publish diagnostics for: {}", uri);
        } else {
            log::debug!("✓ Published diagnostics for: {}", uri);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_server_creation() {
        let (sender, receiver) = crossbeam_channel::bounded(16);
        let state = Arc::new(ServerState::new());
        let _server = LspServer::new(receiver, sender, state);
    }
}
