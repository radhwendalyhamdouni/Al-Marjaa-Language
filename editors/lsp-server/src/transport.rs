//! ═══════════════════════════════════════════════════════════════════════════════
//! Transport Layer - طبقة النقل لـ JSON-RPC
//! ═══════════════════════════════════════════════════════════════════════════════
//! التعامل مع stdin/stdout للتواصل مع VS Code

use std::io::{BufRead, BufWriter, Write};
use std::thread::{self, JoinHandle};

use crossbeam_channel::Sender;

use crate::{LspMessage, LspError, JsonRpcResponse, JsonRpcNotification};

/// طبقة النقل
pub struct Transport<R: BufRead, W: Write> {
    reader: R,
    writer: W,
    sender: Sender<LspMessage>,
}

impl<R: BufRead + Send + 'static, W: Write + Send + 'static> Transport<R, W> {
    /// إنشاء طبقة نقل جديدة
    pub fn new(reader: R, writer: W, sender: Sender<LspMessage>) -> Self {
        Self { reader, writer, sender }
    }
    
    /// بدء قراءة الرسائل
    pub fn start(self) -> JoinHandle<()> {
        thread::spawn(move || {
            self.read_loop();
        })
    }
    
    /// حلقة القراءة الرئيسية
    fn read_loop(mut self) {
        log::info!("[Transport] Starting read loop...");
        
        loop {
            match self.read_message() {
                Ok(Some(message)) => {
                    if self.sender.send(message).is_err() {
                        log::info!("[Transport] Channel closed, exiting");
                        break;
                    }
                }
                Ok(None) => {
                    log::info!("[Transport] EOF reached, exiting");
                    break;
                }
                Err(e) => {
                    log::error!("[Transport] Error reading message: {}", e);
                    
                    // إرسال خطأ للعميل
                    let error = LspError::parse_error(&e);
                    let response = LspMessage::Response {
                        id: 0,
                        result: None,
                        error: Some(error),
                    };
                    
                    if self.sender.send(response).is_err() {
                        break;
                    }
                }
            }
        }
        
        log::info!("[Transport] Read loop ended");
    }
    
    /// قراءة رسالة واحدة
    fn read_message(&mut self) -> Result<Option<LspMessage>, String> {
        // قراءة رأس Content-Length
        let content_length = self.read_headers()?;
        
        if content_length == 0 {
            return Ok(None);
        }
        
        // قراءة المحتوى
        let mut buffer = vec![0u8; content_length];
        self.reader.read_exact(&mut buffer)
            .map_err(|e| format!("Failed to read content: {}", e))?;
        
        let content = String::from_utf8(buffer)
            .map_err(|e| format!("Invalid UTF-8: {}", e))?;
        
        // تحليل JSON
        let json: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("Invalid JSON: {}", e))?;
        
        // استخراج الرسالة
        self.parse_message(json)
    }
    
    /// قراءة الرؤوس
    fn read_headers(&mut self) -> Result<usize, String> {
        let mut content_length = 0;
        
        loop {
            let mut line = String::new();
            let bytes = self.reader.read_line(&mut line)
                .map_err(|e| format!("Failed to read header: {}", e))?;
            
            if bytes == 0 {
                return Ok(0);
            }
            
            let line = line.trim();
            
            // سطر فارغ يعني نهاية الرؤوس
            if line.is_empty() {
                break;
            }
            
            // استخراج Content-Length
            if line.to_lowercase().starts_with("content-length:") {
                let value = line.split(':').nth(1)
                    .ok_or("Invalid Content-Length header")?
                    .trim();
                content_length = value.parse()
                    .map_err(|e| format!("Invalid Content-Length value: {}", e))?;
            }
        }
        
        Ok(content_length)
    }
    
    /// تحليل رسالة JSON
    fn parse_message(&self, json: serde_json::Value) -> Result<LspMessage, String> {
        let obj = json.as_object()
            .ok_or("Message must be an object")?;
        
        let method = obj.get("method")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        // التحقق من نوع الرسالة
        if let Some(id) = obj.get("id") {
            // هذا طلب
            let id = id.as_u64()
                .ok_or_else(|| "Invalid request id".to_string())?;
            
            let method = method.ok_or_else(|| "Request missing method".to_string())?;
            let params = obj.get("params").cloned().unwrap_or(serde_json::Value::Null);
            
            Ok(LspMessage::Request { id, method, params })
        } else if let Some(method) = method {
            // هذا إشعار
            let params = obj.get("params").cloned().unwrap_or(serde_json::Value::Null);
            Ok(LspMessage::Notification { method, params })
        } else {
            Err("Invalid message: missing method and id".to_string())
        }
    }
    
    /// إرسال رسالة
    pub fn send_message(&mut self, message: &LspMessage) -> Result<(), String> {
        let json = match message {
            LspMessage::Response { id, result, error } => {
                let response = JsonRpcResponse {
                    jsonrpc: "2.0",
                    id: *id,
                    result: result.clone(),
                    error: error.clone(),
                };
                serde_json::to_value(response)
                    .map_err(|e| format!("Failed to serialize response: {}", e))?
            }
            LspMessage::Notification { method, params } => {
                let notification = JsonRpcNotification {
                    jsonrpc: "2.0",
                    method: method.clone(),
                    params: Some(params.clone()),
                };
                serde_json::to_value(notification)
                    .map_err(|e| format!("Failed to serialize notification: {}", e))?
            }
            LspMessage::Request { .. } => {
                return Err("Cannot send request from server".to_string());
            }
        };
        
        let content = serde_json::to_string(&json)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?;
        
        // كتابة الرأس
        write!(self.writer, "Content-Length: {}\r\n\r\n", content.len())
            .map_err(|e| format!("Failed to write header: {}", e))?;
        
        // كتابة المحتوى
        self.writer.write_all(content.as_bytes())
            .map_err(|e| format!("Failed to write content: {}", e))?;
        
        self.writer.flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    
    #[test]
    fn test_parse_request() {
        let json = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {}
        });
        
        let transport = Transport::new(
            BufReader::new(Cursor::new("")),
            BufWriter::new(Vec::new()),
            crossbeam_channel::bounded(1).0,
        );
        
        let message = transport.parse_message(json).unwrap();
        match message {
            LspMessage::Request { id, method, .. } => {
                assert_eq!(id, 1);
                assert_eq!(method, "initialize");
            }
            _ => panic!("Expected Request"),
        }
    }
    
    #[test]
    fn test_parse_notification() {
        let json = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "initialized",
            "params": {}
        });
        
        let transport = Transport::new(
            BufReader::new(Cursor::new("")),
            BufWriter::new(Vec::new()),
            crossbeam_channel::bounded(1).0,
        );
        
        let message = transport.parse_message(json).unwrap();
        match message {
            LspMessage::Notification { method, .. } => {
                assert_eq!(method, "initialized");
            }
            _ => panic!("Expected Notification"),
        }
    }
}
