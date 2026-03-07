//! ═══════════════════════════════════════════════════════════════════════════════
//! LSP Server المتقدم للغة المرجع
//! ═══════════════════════════════════════════════════════════════════════════════
//! الإصدار 3.3.0 - متكامل مع المحلل الأصلي
//! 
//! الميزات:
//! ✅ الإكمال التلقائي الذكي مع Type Inference
//! ✅ التنقل للتعريف والمراجع
//! ✅ التشخيصات والأخطاء
//! ✅ Semantic Tokens للتلوين الدلالي
//! ✅ Code Actions للإصلاح السريع
//! ✅ Code Lens للعمليات السريعة
//! ✅ Inlay Hints للتلميحات المضمنة
//! ✅ Call Hierarchy لتتبع الاستدعاءات
//! ✅ Rename لإعادة التسمية
//! ✅ Formatting للتنسيق
//! ✅ Folding Ranges للطي
//! ✅ Signature Help للمساعدة
//! ✅ دعم محسن للعربية RTL
//! ═══════════════════════════════════════════════════════════════════════════════

mod transport;
mod server;
mod handlers;
mod state;
mod capabilities;
mod semantic_tokens;
mod code_actions;
mod code_lens;
mod inlay_hints;
mod call_hierarchy;
mod formatting;
mod folding;
mod rename;
mod signature_help;
mod workspace_symbols;
mod diagnostics;
mod arabic_support;
mod type_inference;
mod cache;

use std::io::{self, BufReader, BufWriter};
use std::sync::Arc;

use crossbeam_channel::{bounded, Sender};
use parking_lot::RwLock;

use crate::server::LspServer;
use crate::transport::Transport;
use crate::state::ServerState;

/// نقطة الدخول الرئيسية
fn main() {
    // تهيئة السجلات
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();
    
    // طباعة معلومات البدء
    print_banner();
    
    // إنشاء الحالة المشتركة
    let state = Arc::new(ServerState::new());
    
    // إنشاء قنوات الاتصال
    let (sender, receiver) = bounded(1024);
    
    // إنشاء النقل (stdin/stdout)
    let stdin = BufReader::new(io::stdin());
    let stdout = BufWriter::new(io::stdout());
    let transport = Transport::new(stdin, stdout, sender.clone());
    
    // بدء خيوط المعالجة
    let transport_handle = transport.start();
    let server_handle = LspServer::new(receiver, sender, state).start();
    
    // انتظار الانتهاء
    transport_handle.join().unwrap();
    server_handle.join().unwrap();
}

/// طباعة اللافتة
fn print_banner() {
    eprintln!();
    eprintln!("╔═══════════════════════════════════════════════════════════════════════════════╗");
    eprintln!("║             🌙 لغة المرجع - Al-Marjaa Language Server                       ║");
    eprintln!("║                        الإصدار 3.3.0                                         ║");
    eprintln!("╠═══════════════════════════════════════════════════════════════════════════════╣");
    eprintln!("║  🧠 LSP Server:          ✅ جاهز ومتقدم                                      ║");
    eprintln!("║  📦 التحليل المعجمي:     ✅ مفعّل مع Type Inference                          ║");
    eprintln!("║  🌳 التحليل النحوي:      ✅ مفعّل مع AST كامل                                ║");
    eprintln!("║  🔍 Linter:              ✅ مفعّل مع 9 قواعد                                 ║");
    eprintln!("║  💡 الإكمال التلقائي:    ✅ ذكي مع سياق                                      ║");
    eprintln!("║  🎯 التنقل:              ✅ تعريف + مراجع + استدعاءات                        ║");
    eprintln!("║  🎨 Semantic Tokens:     ✅ تلوين دلالي متقدم                                ║");
    eprintln!("║  ⚡ Code Actions:        ✅ إصلاح سريع + تحسينات                             ║");
    eprintln!("║  📊 Code Lens:           ✅ عدسات تفاعلية                                    ║");
    eprintln!("║  💬 Inlay Hints:         ✅ تلميحات الأنواع                                  ║");
    eprintln!("║  🔄 Call Hierarchy:      ✅ تتبع الاستدعاءات                                 ║");
    eprintln!("║  ✏️ Rename:              ✅ إعادة تسمية ذكية                                 ║");
    eprintln!("║  📝 Formatting:          ✅ تنسيق تلقائي                                     ║");
    eprintln!("║  📁 Folding Ranges:      ✅ طي الكود                                         ║");
    eprintln!("║  🖊️ Signature Help:      ✅ مساعدة التوقيعات                                 ║");
    eprintln!("║  🔤 Arabic Support:      ✅ دعم محسن RTL                                     ║");
    eprintln!("╠═══════════════════════════════════════════════════════════════════════════════╣");
    eprintln!("║  📚 التوثيق: https://docs.almarjaa.io                                        ║");
    eprintln!("║  🐦 GitHub: github.com/radhwendalyhamdouni/Al-Marjaa-Language                ║");
    eprintln!("╚═══════════════════════════════════════════════════════════════════════════════╝");
    eprintln!();
    eprintln!("[LSP] 🚀 Server started, waiting for connections...");
    eprintln!();
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
    
    /// هل هي استجابة ناجحة؟
    pub fn is_success(&self) -> bool {
        match self {
            LspMessage::Response { error, .. } => error.is_none(),
            _ => true,
        }
    }
    
    /// هل هي إشعار؟
    pub fn is_notification(&self) -> bool {
        matches!(self, LspMessage::Notification { .. })
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
    
    /// خطأ الطلب الملغي
    pub fn request_cancelled() -> Self {
        Self {
            code: -32800,
            message: "Request cancelled".to_string(),
            data: None,
        }
    }
    
    /// خطأ المحتوى المعدل
    pub fn content_modified() -> Self {
        Self {
            code: -32801,
            message: "Content modified".to_string(),
            data: None,
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
