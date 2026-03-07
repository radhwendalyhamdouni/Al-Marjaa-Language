// src/stdlib/http/websocket.rs
// دعم WebSocket
// WebSocket Support

use std::collections::HashMap;

/// حالة WebSocket
#[derive(Debug, Clone, PartialEq)]
pub enum WebSocketState {
    Connecting,
    Open,
    Closing,
    Closed,
}

/// رسالة WebSocket
#[derive(Debug, Clone)]
pub struct WebSocketMessage {
    /// هل هي نصية؟
    pub is_text: bool,
    /// المحتوى
    pub data: Vec<u8>,
}

impl WebSocketMessage {
    /// إنشاء رسالة نصية
    pub fn text(content: &str) -> Self {
        Self {
            is_text: true,
            data: content.as_bytes().to_vec(),
        }
    }
    
    /// إنشاء رسالة ثنائية
    pub fn binary(data: Vec<u8>) -> Self {
        Self {
            is_text: false,
            data,
        }
    }
    
    /// الحصول على النص
    pub fn as_text(&self) -> Option<&str> {
        if self.is_text {
            std::str::from_utf8(&self.data).ok()
        } else {
            None
        }
    }
    
    /// الحصول على البيانات الثنائية
    pub fn as_binary(&self) -> &[u8] {
        &self.data
    }
}

/// اتصال WebSocket
pub struct WebSocketConnection {
    /// المعرف
    pub id: String,
    /// الحالة
    pub state: WebSocketState,
    /// الرؤوس
    pub headers: HashMap<String, String>,
    /// المسار
    pub path: String,
}

impl WebSocketConnection {
    /// إنشاء اتصال جديد
    pub fn new(id: &str, path: &str) -> Self {
        Self {
            id: id.to_string(),
            state: WebSocketState::Connecting,
            headers: HashMap::new(),
            path: path.to_string(),
        }
    }
    
    /// إرسال رسالة
    pub fn send(&mut self, message: &WebSocketMessage) -> Result<(), String> {
        if self.state != WebSocketState::Open {
            return Err("الاتصال غير مفتوح".to_string());
        }
        // في التنفيذ الحقيقي، سيتم الإرسال عبر الشبكة
        Ok(())
    }
    
    /// إغلاق الاتصال
    pub fn close(&mut self) {
        self.state = WebSocketState::Closed;
    }
    
    /// هل الاتصال مفتوح؟
    pub fn is_open(&self) -> bool {
        self.state == WebSocketState::Open
    }
}

/// خادم WebSocket
pub struct WebSocketServer {
    /// المنفذ
    pub port: u16,
    /// الاتصالات النشطة
    connections: HashMap<String, WebSocketConnection>,
}

impl WebSocketServer {
    /// إنشاء خادم جديد
    pub fn new(port: u16) -> Self {
        Self {
            port,
            connections: HashMap::new(),
        }
    }
    
    /// بث رسالة لجميع الاتصالات
    pub fn broadcast(&mut self, message: &WebSocketMessage) {
        for conn in self.connections.values_mut() {
            if conn.is_open() {
                let _ = conn.send(message);
            }
        }
    }
    
    /// إرسال لاتصال محدد
    pub fn send_to(&mut self, id: &str, message: &WebSocketMessage) -> Result<(), String> {
        if let Some(conn) = self.connections.get_mut(id) {
            conn.send(message)
        } else {
            Err(format!("الاتصال {} غير موجود", id))
        }
    }
    
    /// عدد الاتصالات
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}
