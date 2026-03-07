//! ═══════════════════════════════════════════════════════════════════════════════
//! LSP Server للغة المرجع
//! خادم Language Server Protocol كامل مع JSON-RPC
//! متكامل مع المحلل الفعلي للغة المرجع
//! ═══════════════════════════════════════════════════════════════════════════════

mod transport;
mod server;
mod handlers;
mod state;

use std::io::{self, BufReader, BufWriter};

use crossbeam_channel::{bounded, Sender};

use crate::server::LspServer;
use crate::transport::Transport;

/// نقطة الدخول الرئيسية
fn main() {
    // طباعة معلومات البدء
    eprintln!();
    eprintln!("╔═══════════════════════════════════════════════════════════════╗");
    eprintln!("║         لغة المرجع - Al-Marjaa Language Server               ║");
    eprintln!("║         الإصدار 3.0.0                                         ║");
    eprintln!("╠═══════════════════════════════════════════════════════════════╣");
    eprintln!("║  🧠 LSP Server: ✅ جاهز                                       ║");
    eprintln!("║  📦 التحليل المعجمي: ✅ مفعّل                                 ║");
    eprintln!("║  🌳 التحليل النحوي: ✅ مفعّل                                  ║");
    eprintln!("║  🔍 Linter: ✅ مفعّل                                           ║");
    eprintln!("║  💡 الإكمال التلقائي: ✅ مفعّل                                 ║");
    eprintln!("║  🎯 التنقل: ✅ مفعّل                                           ║");
    eprintln!("╚═══════════════════════════════════════════════════════════════╝");
    eprintln!();

    // إنشاء قنوات الاتصال
    let (sender, receiver) = bounded(256);
    
    // إنشاء النقل (stdin/stdout)
    let stdin = BufReader::new(io::stdin());
    let stdout = BufWriter::new(io::stdout());
    let transport = Transport::new(stdin, stdout, sender.clone());
    
    // بدء خيوط المعالجة
    let transport_handle = transport.start();
    let server_handle = LspServer::new(receiver, sender).start();
    
    // انتظار الانتهاء
    transport_handle.join().unwrap();
    server_handle.join().unwrap();
}

/// رسالة LSP
#[derive(Debug, Clone)]
pub enum LspMessage {
    /// طلب من العميل
    Request {
        id: u64,
        method: String,
        params: serde_json::Value,
    },
    /// إشعار من العميل
    Notification {
        method: String,
        params: serde_json::Value,
    },
    /// استجابة للعميل
    Response {
        id: u64,
        result: Option<serde_json::Value>,
        error: Option<LspError>,
    },
}

impl LspMessage {
    /// إنشاء طلب جديد
    pub fn request(id: u64, method: impl Into<String>, params: serde_json::Value) -> Self {
        LspMessage::Request {
            id,
            method: method.into(),
            params,
        }
    }
    
    /// إنشاء إشعار جديد
    pub fn notification(method: impl Into<String>, params: serde_json::Value) -> Self {
        LspMessage::Notification {
            method: method.into(),
            params,
        }
    }
    
    /// إنشاء استجابة ناجحة
    pub fn success(id: u64, result: serde_json::Value) -> Self {
        LspMessage::Response {
            id,
            result: Some(result),
            error: None,
        }
    }
    
    /// إنشاء استجابة خطأ
    pub fn error(id: u64, error: LspError) -> Self {
        LspMessage::Response {
            id,
            result: None,
            error: Some(error),
        }
    }
}

/// خطأ LSP
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LspError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl LspError {
    /// خطأ التحليل
    pub fn parse_error(message: &str) -> Self {
        Self { 
            code: -32700, 
            message: message.to_string(), 
            data: None 
        }
    }
    
    /// طلب غير صالح
    pub fn invalid_request(message: &str) -> Self {
        Self { 
            code: -32600, 
            message: message.to_string(), 
            data: None 
        }
    }
    
    /// طريقة غير موجودة
    pub fn method_not_found(method: &str) -> Self {
        Self { 
            code: -32601, 
            message: format!("Method not found: {}", method), 
            data: None 
        }
    }
    
    /// معاملات غير صالحة
    pub fn invalid_params(message: &str) -> Self {
        Self { 
            code: -32602, 
            message: message.to_string(), 
            data: None 
        }
    }
    
    /// خطأ داخلي
    pub fn internal_error(message: &str) -> Self {
        Self { 
            code: -32603, 
            message: message.to_string(), 
            data: None 
        }
    }
    
    /// خطأ مخصص
    pub fn custom(code: i32, message: &str) -> Self {
        Self { 
            code, 
            message: message.to_string(), 
            data: None 
        }
    }
}

impl std::fmt::Display for LspError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LspError({}): {}", self.code, self.message)
    }
}

impl std::error::Error for LspError {}

/// استجابة JSON-RPC
#[derive(serde::Serialize)]
struct JsonRpcResponse {
    jsonrpc: &'static str,
    id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<LspError>,
}

/// إشعار JSON-RPC
#[derive(serde::Serialize)]
struct JsonRpcNotification {
    jsonrpc: &'static str,
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<serde_json::Value>,
}
