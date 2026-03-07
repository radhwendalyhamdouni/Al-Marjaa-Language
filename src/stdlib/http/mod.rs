// src/stdlib/http/mod.rs
// وحدة HTTP المتقدمة للغة المرجع
// Advanced HTTP Module for Al-Marjaa Language

pub mod client;
pub mod server;
pub mod websocket;
pub mod middleware;
pub mod request;
pub mod response;
pub mod cookies;
pub mod session;

pub use client::*;
pub use server::*;
pub use websocket::*;
pub use middleware::*;
pub use request::*;
pub use response::*;
pub use cookies::*;
pub use session::*;

use std::collections::HashMap;

/// طريقة HTTP
#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Connect,
    Trace,
}

impl HttpMethod {
    pub fn from_arabic(name: &str) -> Option<Self> {
        match name {
            "احضر" | "GET" => Some(Self::Get),
            "ارسل" | "POST" => Some(Self::Post),
            "ضع" | "PUT" => Some(Self::Put),
            "احذف" | "DELETE" => Some(Self::Delete),
            "عدل" | "PATCH" => Some(Self::Patch),
            "رأس" | "HEAD" => Some(Self::Head),
            "خيارات" | "OPTIONS" => Some(Self::Options),
            "اتصل" | "CONNECT" => Some(Self::Connect),
            "تتبع" | "TRACE" => Some(Self::Trace),
            _ => None,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self {
            Self::Get => "GET".to_string(),
            Self::Post => "POST".to_string(),
            Self::Put => "PUT".to_string(),
            Self::Delete => "DELETE".to_string(),
            Self::Patch => "PATCH".to_string(),
            Self::Head => "HEAD".to_string(),
            Self::Options => "OPTIONS".to_string(),
            Self::Connect => "CONNECT".to_string(),
            Self::Trace => "TRACE".to_string(),
        }
    }
}

/// رؤوس HTTP
#[derive(Debug, Clone, Default)]
pub struct HttpHeaders {
    headers: HashMap<String, String>,
}

impl HttpHeaders {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, key: String, value: String) {
        self.headers.insert(key.to_lowercase(), value);
    }
    
    pub fn get(&self, key: &str) -> Option<&String> {
        self.headers.get(&key.to_lowercase())
    }
    
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.headers.remove(&key.to_lowercase())
    }
    
    pub fn contains(&self, key: &str) -> bool {
        self.headers.contains_key(&key.to_lowercase())
    }
    
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.headers.iter()
    }
    
    pub fn len(&self) -> usize {
        self.headers.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.headers.is_empty()
    }
}

/// رمز الحالة HTTP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCode(pub u16);

impl StatusCode {
    // 2xx Success
    pub const OK: Self = Self(200);
    pub const CREATED: Self = Self(201);
    pub const ACCEPTED: Self = Self(202);
    pub const NO_CONTENT: Self = Self(204);
    
    // 3xx Redirection
    pub const MOVED_PERMANENTLY: Self = Self(301);
    pub const FOUND: Self = Self(302);
    pub const SEE_OTHER: Self = Self(303);
    pub const NOT_MODIFIED: Self = Self(304);
    pub const TEMPORARY_REDIRECT: Self = Self(307);
    pub const PERMANENT_REDIRECT: Self = Self(308);
    
    // 4xx Client Errors
    pub const BAD_REQUEST: Self = Self(400);
    pub const UNAUTHORIZED: Self = Self(401);
    pub const FORBIDDEN: Self = Self(403);
    pub const NOT_FOUND: Self = Self(404);
    pub const METHOD_NOT_ALLOWED: Self = Self(405);
    pub const REQUEST_TIMEOUT: Self = Self(408);
    pub const CONFLICT: Self = Self(409);
    pub const GONE: Self = Self(410);
    pub const PAYLOAD_TOO_LARGE: Self = Self(413);
    pub const URI_TOO_LONG: Self = Self(414);
    pub const UNSUPPORTED_MEDIA_TYPE: Self = Self(415);
    pub const TOO_MANY_REQUESTS: Self = Self(429);
    
    // 5xx Server Errors
    pub const INTERNAL_SERVER_ERROR: Self = Self(500);
    pub const NOT_IMPLEMENTED: Self = Self(501);
    pub const BAD_GATEWAY: Self = Self(502);
    pub const SERVICE_UNAVAILABLE: Self = Self(503);
    pub const GATEWAY_TIMEOUT: Self = Self(504);
    pub const HTTP_VERSION_NOT_SUPPORTED: Self = Self(505);
    
    pub fn is_success(&self) -> bool {
        self.0 >= 200 && self.0 < 300
    }
    
    pub fn is_redirect(&self) -> bool {
        self.0 >= 300 && self.0 < 400
    }
    
    pub fn is_client_error(&self) -> bool {
        self.0 >= 400 && self.0 < 500
    }
    
    pub fn is_server_error(&self) -> bool {
        self.0 >= 500 && self.0 < 600
    }
    
    pub fn reason_phrase(&self) -> &'static str {
        match self.0 {
            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            204 => "No Content",
            301 => "Moved Permanently",
            302 => "Found",
            304 => "Not Modified",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            408 => "Request Timeout",
            409 => "Conflict",
            429 => "Too Many Requests",
            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Timeout",
            _ => "Unknown",
        }
    }
    
    pub fn reason_phrase_arabic(&self) -> &'static str {
        match self.0 {
            200 => "نجاح",
            201 => "تم الإنشاء",
            202 => "مقبول",
            204 => "بدون محتوى",
            301 => "نقل دائم",
            302 => "وجد",
            304 => "لم يعدل",
            400 => "طلب غير صالح",
            401 => "غير مصرح",
            403 => "ممنوع",
            404 => "غير موجود",
            405 => "الطريقة غير مسموحة",
            408 => "انتهت مهلة الطلب",
            409 => "تعارض",
            429 => "طلبات كثيرة جداً",
            500 => "خطأ داخلي في الخادم",
            501 => "غير منفذ",
            502 => "بوابة سيئة",
            503 => "الخدمة غير متاحة",
            504 => "انتهت مهلة البوابة",
            _ => "غير معروف",
        }
    }
}

/// نوع المحتوى
#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    Json,
    Xml,
    Html,
    Text,
    Binary,
    FormData,
    Multipart,
    UrlEncoded,
    Custom(String),
}

impl ContentType {
    pub fn to_mime(&self) -> String {
        match self {
            Self::Json => "application/json".to_string(),
            Self::Xml => "application/xml".to_string(),
            Self::Html => "text/html".to_string(),
            Self::Text => "text/plain".to_string(),
            Self::Binary => "application/octet-stream".to_string(),
            Self::FormData => "multipart/form-data".to_string(),
            Self::Multipart => "multipart/mixed".to_string(),
            Self::UrlEncoded => "application/x-www-form-urlencoded".to_string(),
            Self::Custom(s) => s.clone(),
        }
    }
    
    pub fn from_mime(mime: &str) -> Self {
        match mime.to_lowercase().as_str() {
            "application/json" => Self::Json,
            "application/xml" | "text/xml" => Self::Xml,
            "text/html" => Self::Html,
            "text/plain" => Self::Text,
            "application/octet-stream" => Self::Binary,
            "multipart/form-data" => Self::FormData,
            "multipart/mixed" => Self::Multipart,
            "application/x-www-form-urlencoded" => Self::UrlEncoded,
            _ => Self::Custom(mime.to_string()),
        }
    }
    
    pub fn from_arabic(name: &str) -> Self {
        match name {
            "json" | "جسون" => Self::Json,
            "xml" | "إكس إم إل" => Self::Xml,
            "html" | "إتش تي إم إل" => Self::Html,
            "نص" | "text" => Self::Text,
            "ثنائي" | "binary" => Self::Binary,
            _ => Self::Custom(name.to_string()),
        }
    }
}
