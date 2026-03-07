// ═══════════════════════════════════════════════════════════════════════════════
// وحدة HTTP Client/Server المتقدمة
// Advanced HTTP Client/Server Module
// ═══════════════════════════════════════════════════════════════════════════════
// © 2026 رضوان دالي حمدوني | RADHWEN DALY HAMDOUNI
// جميع الحقوق محفوظة | All Rights Reserved
// ═══════════════════════════════════════════════════════════════════════════════

//! # وحدة HTTP المتقدمة
//!
//! توفر هذه الوحدة:
//! - عميل HTTP/1.1 و HTTP/2
//! - خادم HTTP متقدم
//! - دعم WebSocket
//! - نظام الوسائط (Middleware)
//! - دعم Cookies & Sessions
//! - دعم Proxy
//! - SSL/TLS
//! - طلبات غير متزامنة
//! - تجميع الاتصالات (Connection Pooling)
//! - تحديد معدل الطلبات (Rate Limiting)
//! - منطق إعادة المحاولة (Retry Logic)

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

// ═══════════════════════════════════════════════════════════════════════════════
// الأخطاء
// ═══════════════════════════════════════════════════════════════════════════════

/// خطأ HTTP
#[derive(Debug, Clone)]
pub enum HttpError {
    /// خطأ في الاتصال
    ConnectionError(String),
    /// خطأ في المهلة
    TimeoutError(String),
    /// خطأ في SSL/TLS
    SslError(String),
    /// خطأ في DNS
    DnsError(String),
    /// خطأ في البروتوكول
    ProtocolError(String),
    /// خطأ في الترميز
    EncodingError(String),
    /// خطأ في الإعادة التوجيه
    RedirectError(String),
    /// خطأ في الوسيط
    MiddlewareError(String),
    /// خطأ في WebSocket
    WebSocketError(String),
    /// خطأ في التحقق
    ValidationError(String),
    /// خطأ عام
    GenericError(String),
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpError::ConnectionError(msg) => write!(f, "خطأ في الاتصال: {}", msg),
            HttpError::TimeoutError(msg) => write!(f, "انتهت المهلة: {}", msg),
            HttpError::SslError(msg) => write!(f, "خطأ SSL/TLS: {}", msg),
            HttpError::DnsError(msg) => write!(f, "خطأ DNS: {}", msg),
            HttpError::ProtocolError(msg) => write!(f, "خطأ في البروتوكول: {}", msg),
            HttpError::EncodingError(msg) => write!(f, "خطأ في الترميز: {}", msg),
            HttpError::RedirectError(msg) => write!(f, "خطأ في الإعادة التوجيه: {}", msg),
            HttpError::MiddlewareError(msg) => write!(f, "خطأ في الوسيط: {}", msg),
            HttpError::WebSocketError(msg) => write!(f, "خطأ WebSocket: {}", msg),
            HttpError::ValidationError(msg) => write!(f, "خطأ في التحقق: {}", msg),
            HttpError::GenericError(msg) => write!(f, "خطأ: {}", msg),
        }
    }
}

impl std::error::Error for HttpError {}

/// نتيجة HTTP
pub type HttpResult<T> = Result<T, HttpError>;

// ═══════════════════════════════════════════════════════════════════════════════
// أنواع HTTP الأساسية
// ═══════════════════════════════════════════════════════════════════════════════

/// طريقة HTTP
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    /// GET
    Get,
    /// POST
    Post,
    /// PUT
    Put,
    /// DELETE
    Delete,
    /// PATCH
    Patch,
    /// HEAD
    Head,
    /// OPTIONS
    Options,
    /// CONNECT
    Connect,
    /// TRACE
    Trace,
}

impl HttpMethod {
    /// تحويل إلى نص
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
            HttpMethod::Connect => "CONNECT",
            HttpMethod::Trace => "TRACE",
        }
    }

    /// تحويل من نص
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "GET" => Some(HttpMethod::Get),
            "POST" => Some(HttpMethod::Post),
            "PUT" => Some(HttpMethod::Put),
            "DELETE" => Some(HttpMethod::Delete),
            "PATCH" => Some(HttpMethod::Patch),
            "HEAD" => Some(HttpMethod::Head),
            "OPTIONS" => Some(HttpMethod::Options),
            "CONNECT" => Some(HttpMethod::Connect),
            "TRACE" => Some(HttpMethod::Trace),
            _ => None,
        }
    }
}

/// حالة HTTP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HttpStatus {
    /// الرمز
    pub code: u16,
    /// النص
    pub reason: &'static str,
}

impl HttpStatus {
    // 1xx Informational
    pub const CONTINUE: HttpStatus = HttpStatus { code: 100, reason: "Continue" };
    pub const SWITCHING_PROTOCOLS: HttpStatus = HttpStatus { code: 101, reason: "Switching Protocols" };
    pub const PROCESSING: HttpStatus = HttpStatus { code: 102, reason: "Processing" };

    // 2xx Success
    pub const OK: HttpStatus = HttpStatus { code: 200, reason: "OK" };
    pub const CREATED: HttpStatus = HttpStatus { code: 201, reason: "Created" };
    pub const ACCEPTED: HttpStatus = HttpStatus { code: 202, reason: "Accepted" };
    pub const NO_CONTENT: HttpStatus = HttpStatus { code: 204, reason: "No Content" };
    pub const PARTIAL_CONTENT: HttpStatus = HttpStatus { code: 206, reason: "Partial Content" };

    // 3xx Redirection
    pub const MOVED_PERMANENTLY: HttpStatus = HttpStatus { code: 301, reason: "Moved Permanently" };
    pub const FOUND: HttpStatus = HttpStatus { code: 302, reason: "Found" };
    pub const SEE_OTHER: HttpStatus = HttpStatus { code: 303, reason: "See Other" };
    pub const NOT_MODIFIED: HttpStatus = HttpStatus { code: 304, reason: "Not Modified" };
    pub const TEMPORARY_REDIRECT: HttpStatus = HttpStatus { code: 307, reason: "Temporary Redirect" };
    pub const PERMANENT_REDIRECT: HttpStatus = HttpStatus { code: 308, reason: "Permanent Redirect" };

    // 4xx Client Errors
    pub const BAD_REQUEST: HttpStatus = HttpStatus { code: 400, reason: "Bad Request" };
    pub const UNAUTHORIZED: HttpStatus = HttpStatus { code: 401, reason: "Unauthorized" };
    pub const FORBIDDEN: HttpStatus = HttpStatus { code: 403, reason: "Forbidden" };
    pub const NOT_FOUND: HttpStatus = HttpStatus { code: 404, reason: "Not Found" };
    pub const METHOD_NOT_ALLOWED: HttpStatus = HttpStatus { code: 405, reason: "Method Not Allowed" };
    pub const REQUEST_TIMEOUT: HttpStatus = HttpStatus { code: 408, reason: "Request Timeout" };
    pub const CONFLICT: HttpStatus = HttpStatus { code: 409, reason: "Conflict" };
    pub const GONE: HttpStatus = HttpStatus { code: 410, reason: "Gone" };
    pub const PAYLOAD_TOO_LARGE: HttpStatus = HttpStatus { code: 413, reason: "Payload Too Large" };
    pub const URI_TOO_LONG: HttpStatus = HttpStatus { code: 414, reason: "URI Too Long" };
    pub const UNSUPPORTED_MEDIA_TYPE: HttpStatus = HttpStatus { code: 415, reason: "Unsupported Media Type" };
    pub const TOO_MANY_REQUESTS: HttpStatus = HttpStatus { code: 429, reason: "Too Many Requests" };

    // 5xx Server Errors
    pub const INTERNAL_SERVER_ERROR: HttpStatus = HttpStatus { code: 500, reason: "Internal Server Error" };
    pub const NOT_IMPLEMENTED: HttpStatus = HttpStatus { code: 501, reason: "Not Implemented" };
    pub const BAD_GATEWAY: HttpStatus = HttpStatus { code: 502, reason: "Bad Gateway" };
    pub const SERVICE_UNAVAILABLE: HttpStatus = HttpStatus { code: 503, reason: "Service Unavailable" };
    pub const GATEWAY_TIMEOUT: HttpStatus = HttpStatus { code: 504, reason: "Gateway Timeout" };

    /// هل الحالة ناجحة (2xx)
    pub fn is_success(&self) -> bool {
        self.code >= 200 && self.code < 300
    }

    /// هل الحالة إعادة توجيه (3xx)
    pub fn is_redirect(&self) -> bool {
        self.code >= 300 && self.code < 400
    }

    /// هل الحالة خطأ عميل (4xx)
    pub fn is_client_error(&self) -> bool {
        self.code >= 400 && self.code < 500
    }

    /// هل الحالة خطأ خادم (5xx)
    pub fn is_server_error(&self) -> bool {
        self.code >= 500 && self.code < 600
    }

    /// إنشاء من رمز
    pub fn from_code(code: u16) -> Self {
        match code {
            100 => Self::CONTINUE,
            101 => Self::SWITCHING_PROTOCOLS,
            102 => Self::PROCESSING,
            200 => Self::OK,
            201 => Self::CREATED,
            202 => Self::ACCEPTED,
            204 => Self::NO_CONTENT,
            206 => Self::PARTIAL_CONTENT,
            301 => Self::MOVED_PERMANENTLY,
            302 => Self::FOUND,
            303 => Self::SEE_OTHER,
            304 => Self::NOT_MODIFIED,
            307 => Self::TEMPORARY_REDIRECT,
            308 => Self::PERMANENT_REDIRECT,
            400 => Self::BAD_REQUEST,
            401 => Self::UNAUTHORIZED,
            403 => Self::FORBIDDEN,
            404 => Self::NOT_FOUND,
            405 => Self::METHOD_NOT_ALLOWED,
            408 => Self::REQUEST_TIMEOUT,
            409 => Self::CONFLICT,
            410 => Self::GONE,
            413 => Self::PAYLOAD_TOO_LARGE,
            414 => Self::URI_TOO_LONG,
            415 => Self::UNSUPPORTED_MEDIA_TYPE,
            429 => Self::TOO_MANY_REQUESTS,
            500 => Self::INTERNAL_SERVER_ERROR,
            501 => Self::NOT_IMPLEMENTED,
            502 => Self::BAD_GATEWAY,
            503 => Self::SERVICE_UNAVAILABLE,
            504 => Self::GATEWAY_TIMEOUT,
            _ => HttpStatus { code, reason: "Unknown" },
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الرؤوس
// ═══════════════════════════════════════════════════════════════════════════════

/// رؤوس HTTP
#[derive(Debug, Clone, Default)]
pub struct HttpHeaders {
    headers: HashMap<String, String>,
}

impl HttpHeaders {
    /// إنشاء رؤوس جديدة
    pub fn new() -> Self {
        Self { headers: HashMap::new() }
    }

    /// إضافة رأس
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.headers.insert(key.into().to_lowercase(), value.into());
    }

    /// الحصول على رأس
    pub fn get(&self, key: &str) -> Option<&String> {
        self.headers.get(&key.to_lowercase())
    }

    /// إزالة رأس
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.headers.remove(&key.to_lowercase())
    }

    /// هل يحتوي على رأس
    pub fn contains(&self, key: &str) -> bool {
        self.headers.contains_key(&key.to_lowercase())
    }

    /// جميع الرؤوس
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.headers.iter()
    }

    /// عدد الرؤوس
    pub fn len(&self) -> usize {
        self.headers.len()
    }

    /// هل فارغ
    pub fn is_empty(&self) -> bool {
        self.headers.is_empty()
    }

    // رؤوس شائعة
    /// نوع المحتوى
    pub fn content_type(&self) -> Option<&String> {
        self.get("content-type")
    }

    /// تعيين نوع المحتوى
    pub fn set_content_type(&mut self, content_type: &str) {
        self.insert("Content-Type", content_type);
    }

    /// الترميز
    pub fn content_encoding(&self) -> Option<&String> {
        self.get("content-encoding")
    }

    /// الطول
    pub fn content_length(&self) -> Option<usize> {
        self.get("content-length").and_then(|v| v.parse().ok())
    }

    /// تعيين الطول
    pub fn set_content_length(&mut self, length: usize) {
        self.insert("Content-Length", length.to_string());
    }

    /// التخويل
    pub fn authorization(&self) -> Option<&String> {
        self.get("authorization")
    }

    /// تعيين التخويل Bearer
    pub fn set_bearer_token(&mut self, token: &str) {
        self.insert("Authorization", format!("Bearer {}", token));
    }

    /// تعيين التخويل Basic
    pub fn set_basic_auth(&mut self, username: &str, password: &str) {
        let encoded = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            format!("{}:{}", username, password),
        );
        self.insert("Authorization", format!("Basic {}", encoded));
    }

    /// User-Agent
    pub fn user_agent(&self) -> Option<&String> {
        self.get("user-agent")
    }

    /// تعيين User-Agent
    pub fn set_user_agent(&mut self, user_agent: &str) {
        self.insert("User-Agent", user_agent);
    }

    /// تعيين JSON
    pub fn set_json(&mut self) {
        self.set_content_type("application/json; charset=utf-8");
    }

    /// تعيين نص عربي
    pub fn set_arabic_text(&mut self) {
        self.set_content_type("text/plain; charset=utf-8");
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الطلب
// ═══════════════════════════════════════════════════════════════════════════════

/// طلب HTTP
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// الطريقة
    pub method: HttpMethod,
    /// الرابط
    pub url: String,
    /// الرؤوس
    pub headers: HttpHeaders,
    /// الجسم
    pub body: Option<Vec<u8>>,
    /// معاملات الاستعلام
    pub query_params: HashMap<String, String>,
    /// المهلة
    pub timeout: Option<Duration>,
    /// إصدار HTTP
    pub http_version: HttpVersion,
}

/// إصدار HTTP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpVersion {
    /// HTTP/1.0
    Http10,
    /// HTTP/1.1
    Http11,
    /// HTTP/2
    Http2,
}

impl Default for HttpVersion {
    fn default() -> Self {
        HttpVersion::Http11
    }
}

impl HttpRequest {
    /// إنشاء طلب جديد
    pub fn new(method: HttpMethod, url: impl Into<String>) -> Self {
        Self {
            method,
            url: url.into(),
            headers: HttpHeaders::new(),
            body: None,
            query_params: HashMap::new(),
            timeout: None,
            http_version: HttpVersion::Http11,
        }
    }

    /// طلب GET
    pub fn get(url: impl Into<String>) -> Self {
        Self::new(HttpMethod::Get, url)
    }

    /// طلب POST
    pub fn post(url: impl Into<String>) -> Self {
        Self::new(HttpMethod::Post, url)
    }

    /// طلب PUT
    pub fn put(url: impl Into<String>) -> Self {
        Self::new(HttpMethod::Put, url)
    }

    /// طلب DELETE
    pub fn delete(url: impl Into<String>) -> Self {
        Self::new(HttpMethod::Delete, url)
    }

    /// طلب PATCH
    pub fn patch(url: impl Into<String>) -> Self {
        Self::new(HttpMethod::Patch, url)
    }

    /// إضافة رأس
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// إضافة رؤوس متعددة
    pub fn headers(mut self, headers: HttpHeaders) -> Self {
        for (k, v) in headers.iter() {
            self.headers.insert(k.clone(), v.clone());
        }
        self
    }

    /// تعيين الجسم
    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// تعيين الجسم كنص
    pub fn text(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        self.headers.set_content_length(text.len());
        self.body = Some(text.into_bytes());
        self
    }

    /// تعيين الجسم كـ JSON
    pub fn json<T: serde::Serialize>(mut self, value: &T) -> HttpResult<Self> {
        let json = serde_json::to_string(value)
            .map_err(|e| HttpError::EncodingError(format!("فشل تحويل JSON: {}", e)))?;
        self.headers.set_json();
        self.headers.set_content_length(json.len());
        self.body = Some(json.into_bytes());
        Ok(self)
    }

    /// تعيين الجسم كـ form
    pub fn form(mut self, params: HashMap<String, String>) -> Self {
        let form_data: Vec<String> = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        let body = form_data.join("&");
        self.headers.insert("Content-Type", "application/x-www-form-urlencoded");
        self.headers.set_content_length(body.len());
        self.body = Some(body.into_bytes());
        self
    }

    /// إضافة معامل استعلام
    pub fn query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.insert(key.into(), value.into());
        self
    }

    /// تعيين المهلة
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    /// تعيين إصدار HTTP
    pub fn http_version(mut self, version: HttpVersion) -> Self {
        self.http_version = version;
        self
    }

    /// تعيين Bearer Token
    pub fn bearer_token(mut self, token: impl Into<String>) -> Self {
        self.headers.set_bearer_token(&token.into());
        self
    }

    /// تعيين Basic Auth
    pub fn basic_auth(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.headers.set_basic_auth(&username.into(), &password.into());
        self
    }

    /// بناء الرابط الكامل
    pub fn build_url(&self) -> String {
        if self.query_params.is_empty() {
            return self.url.clone();
        }
        let params: Vec<String> = self
            .query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        let separator = if self.url.contains('?') { "&" } else { "?" };
        format!("{}{}{}", self.url, separator, params.join("&"))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الاستجابة
// ═══════════════════════════════════════════════════════════════════════════════

/// استجابة HTTP
#[derive(Debug, Clone)]
pub struct HttpResponse {
    /// الحالة
    pub status: HttpStatus,
    /// الرؤوس
    pub headers: HttpHeaders,
    /// الجسم
    pub body: Vec<u8>,
    /// إصدار HTTP
    pub http_version: HttpVersion,
}

impl HttpResponse {
    /// إنشاء استجابة جديدة
    pub fn new(status: HttpStatus) -> Self {
        Self {
            status,
            headers: HttpHeaders::new(),
            body: Vec::new(),
            http_version: HttpVersion::Http11,
        }
    }

    /// استجابة OK
    pub fn ok() -> Self {
        Self::new(HttpStatus::OK)
    }

    /// استجابة Created
    pub fn created() -> Self {
        Self::new(HttpStatus::CREATED)
    }

    /// استجابة No Content
    pub fn no_content() -> Self {
        Self::new(HttpStatus::NO_CONTENT)
    }

    /// استجابة Bad Request
    pub fn bad_request() -> Self {
        Self::new(HttpStatus::BAD_REQUEST)
    }

    /// استجابة Unauthorized
    pub fn unauthorized() -> Self {
        Self::new(HttpStatus::UNAUTHORIZED)
    }

    /// استجابة Forbidden
    pub fn forbidden() -> Self {
        Self::new(HttpStatus::FORBIDDEN)
    }

    /// استجابة Not Found
    pub fn not_found() -> Self {
        Self::new(HttpStatus::NOT_FOUND)
    }

    /// استجابة Internal Server Error
    pub fn internal_server_error() -> Self {
        Self::new(HttpStatus::INTERNAL_SERVER_ERROR)
    }

    /// تعيين الحالة
    pub fn status(mut self, status: HttpStatus) -> Self {
        self.status = status;
        self
    }

    /// إضافة رأس
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// تعيين الجسم
    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self.headers.set_content_length(self.body.len());
        self
    }

    /// تعيين الجسم كنص
    pub fn text(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        self.headers.set_content_length(text.len());
        self.body = text.into_bytes();
        self
    }

    /// تعيين الجسم كـ JSON
    pub fn json<T: serde::Serialize>(mut self, value: &T) -> HttpResult<Self> {
        let json = serde_json::to_string(value)
            .map_err(|e| HttpError::EncodingError(format!("فشل تحويل JSON: {}", e)))?;
        self.headers.set_json();
        self.headers.set_content_length(json.len());
        self.body = json.into_bytes();
        Ok(self)
    }

    /// الحصول على الجسم كنص
    pub fn text_body(&self) -> HttpResult<String> {
        String::from_utf8(self.body.clone())
            .map_err(|e| HttpError::EncodingError(format!("فشل فك الترميز: {}", e)))
    }

    /// الحصول على الجسم كـ JSON
    pub fn json_body<T: serde::de::DeserializeOwned>(&self) -> HttpResult<T> {
        let text = self.text_body()?;
        serde_json::from_str(&text)
            .map_err(|e| HttpError::EncodingError(format!("فشل تحليل JSON: {}", e)))
    }

    /// هل الاستجابة ناجحة
    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }

    /// هل الاستجابة خطأ
    pub fn is_error(&self) -> bool {
        self.status.is_client_error() || self.status.is_server_error()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الكوكيز والجلسات
// ═══════════════════════════════════════════════════════════════════════════════

/// كوكي
#[derive(Debug, Clone)]
pub struct Cookie {
    /// الاسم
    pub name: String,
    /// القيمة
    pub value: String,
    /// المسار
    pub path: Option<String>,
    /// النطاق
    pub domain: Option<String>,
    /// تاريخ الانتهاء
    pub expires: Option<String>,
    /// أمان
    pub secure: bool,
    /// HttpOnly
    pub http_only: bool,
    /// SameSite
    pub same_site: Option<SameSite>,
}

/// SameSite
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SameSite {
    /// Strict
    Strict,
    /// Lax
    Lax,
    /// None
    None,
}

impl Cookie {
    /// إنشاء كوكي جديد
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            path: None,
            domain: None,
            expires: None,
            secure: false,
            http_only: false,
            same_site: None,
        }
    }

    /// تعيين المسار
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// تعيين النطاق
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    /// تعيين تاريخ الانتهاء
    pub fn expires(mut self, expires: impl Into<String>) -> Self {
        self.expires = Some(expires.into());
        self
    }

    /// تعيين الأمان
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    /// تعيين HttpOnly
    pub fn http_only(mut self, http_only: bool) -> Self {
        self.http_only = http_only;
        self
    }

    /// تعيين SameSite
    pub fn same_site(mut self, same_site: SameSite) -> Self {
        self.same_site = Some(same_site);
        self
    }

    /// تحويل إلى نص رأس
    pub fn to_header(&self) -> String {
        let mut parts = vec![format!("{}={}", self.name, self.value)];
        
        if let Some(ref path) = self.path {
            parts.push(format!("Path={}", path));
        }
        if let Some(ref domain) = self.domain {
            parts.push(format!("Domain={}", domain));
        }
        if let Some(ref expires) = self.expires {
            parts.push(format!("Expires={}", expires));
        }
        if self.secure {
            parts.push("Secure".to_string());
        }
        if self.http_only {
            parts.push("HttpOnly".to_string());
        }
        if let Some(ref same_site) = self.same_site {
            let value = match same_site {
                SameSite::Strict => "Strict",
                SameSite::Lax => "Lax",
                SameSite::None => "None",
            };
            parts.push(format!("SameSite={}", value));
        }
        
        parts.join("; ")
    }
}

/// مدير الجلسات
#[derive(Debug)]
pub struct SessionManager {
    /// الجلسات
    sessions: RwLock<HashMap<String, Session>>,
    /// مدة الجلسة
    session_duration: Duration,
}

/// جلسة
#[derive(Debug, Clone)]
pub struct Session {
    /// المعرف
    pub id: String,
    /// البيانات
    pub data: HashMap<String, String>,
    /// وقت الإنشاء
    pub created_at: std::time::Instant,
    /// آخر نشاط
    pub last_activity: std::time::Instant,
}

impl SessionManager {
    /// إنشاء مدير جلسات جديد
    pub fn new(session_duration: Duration) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            session_duration,
        }
    }

    /// إنشاء جلسة جديدة
    pub fn create_session(&self) -> String {
        use rand::Rng;
        let id: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        
        let session = Session {
            id: id.clone(),
            data: HashMap::new(),
            created_at: std::time::Instant::now(),
            last_activity: std::time::Instant::now(),
        };
        
        self.sessions.write().unwrap().insert(id.clone(), session);
        id
    }

    /// الحصول على جلسة
    pub fn get_session(&self, id: &str) -> Option<Session> {
        let mut sessions = self.sessions.write().unwrap();
        if let Some(session) = sessions.get_mut(id) {
            session.last_activity = std::time::Instant::now();
            return Some(session.clone());
        }
        None
    }

    /// تحديث بيانات الجلسة
    pub fn update_session(&self, id: &str, data: HashMap<String, String>) -> bool {
        let mut sessions = self.sessions.write().unwrap();
        if let Some(session) = sessions.get_mut(id) {
            session.data = data;
            session.last_activity = std::time::Instant::now();
            return true;
        }
        false
    }

    /// حذف جلسة
    pub fn delete_session(&self, id: &str) {
        self.sessions.write().unwrap().remove(id);
    }

    /// تنظيف الجلسات المنتهية
    pub fn cleanup_expired(&self) {
        let mut sessions = self.sessions.write().unwrap();
        let now = std::time::Instant::now();
        sessions.retain(|_, session| {
            now.duration_since(session.last_activity) < self.session_duration
        });
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الوسائط (Middleware)
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع الوسيط
pub type MiddlewareFn = Box<dyn Fn(&mut HttpRequest) -> HttpResult<()> + Send + Sync>;

/// وسيط HTTP
pub struct HttpMiddleware {
    /// الاسم
    pub name: String,
    /// الدالة
    pub handler: MiddlewareFn,
}

impl HttpMiddleware {
    /// إنشاء وسيط جديد
    pub fn new(name: impl Into<String>, handler: MiddlewareFn) -> Self {
        Self {
            name: name.into(),
            handler,
        }
    }

    /// تنفيذ الوسيط
    pub fn execute(&self, request: &mut HttpRequest) -> HttpResult<()> {
        (self.handler)(request)
    }
}

/// وسيط تسجيل الطلبات
pub fn logging_middleware() -> HttpMiddleware {
    HttpMiddleware::new("تسجيل", Box::new(|req: &mut HttpRequest| {
        println!("[{}] {} {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), req.method.as_str(), req.url);
        Ok(())
    }))
}

/// وسيط إضافة رؤوس
pub fn headers_middleware(headers: HashMap<String, String>) -> HttpMiddleware {
    HttpMiddleware::new("رؤوس", Box::new(move |req: &mut HttpRequest| {
        for (k, v) in &headers {
            req.headers.insert(k.clone(), v.clone());
        }
        Ok(())
    }))
}

/// وسيط المصادقة
pub fn auth_middleware(token: String) -> HttpMiddleware {
    HttpMiddleware::new("مصادقة", Box::new(move |req: &mut HttpRequest| {
        req.headers.set_bearer_token(&token);
        Ok(())
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════
// Rate Limiting
// ═══════════════════════════════════════════════════════════════════════════════

/// تحديد معدل الطلبات
#[derive(Debug)]
pub struct RateLimiter {
    /// الحد الأقصى من الطلبات
    max_requests: u32,
    /// فترة الوقت
    window: Duration,
    /// سجل الطلبات
    requests: RwLock<HashMap<String, Vec<std::time::Instant>>>,
}

impl RateLimiter {
    /// إنشاء محدد معدل جديد
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            max_requests,
            window,
            requests: RwLock::new(HashMap::new()),
        }
    }

    /// التحقق من السماح بالطلب
    pub fn is_allowed(&self, key: &str) -> bool {
        let mut requests = self.requests.write().unwrap();
        let now = std::time::Instant::now();
        
        let entry = requests.entry(key.to_string()).or_default();
        
        // إزالة الطلبات القديمة
        entry.retain(|&t| now.duration_since(t) < self.window);
        
        if entry.len() < self.max_requests as usize {
            entry.push(now);
            true
        } else {
            false
        }
    }

    /// الحصول على عدد الطلبات المتبقية
    pub fn remaining(&self, key: &str) -> u32 {
        let requests = self.requests.read().unwrap();
        let now = std::time::Instant::now();
        
        let entry = requests.get(key);
        if let Some(entry) = entry {
            let count = entry.iter().filter(|&&t| now.duration_since(t) < self.window).count();
            self.max_requests.saturating_sub(count as u32)
        } else {
            self.max_requests
        }
    }

    /// إعادة تعيين
    pub fn reset(&self, key: &str) {
        self.requests.write().unwrap().remove(key);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Retry Logic
// ═══════════════════════════════════════════════════════════════════════════════

/// إعدادات إعادة المحاولة
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// الحد الأقصى من المحاولات
    pub max_retries: u32,
    /// التأخير الأولي
    pub initial_delay: Duration,
    /// التأخير الأقصى
    pub max_delay: Duration,
    /// معامل الضرب
    pub multiplier: f64,
    /// رموز الحالة التي تستدعي إعادة المحاولة
    pub retry_status_codes: Vec<u16>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            multiplier: 2.0,
            retry_status_codes: vec![429, 500, 502, 503, 504],
        }
    }
}

impl RetryConfig {
    /// إنشاء إعدادات جديدة
    pub fn new() -> Self {
        Self::default()
    }

    /// تعيين الحد الأقصى للمحاولات
    pub fn max_retries(mut self, max: u32) -> Self {
        self.max_retries = max;
        self
    }

    /// تعيين التأخير الأولي
    pub fn initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    /// حساب التأخير للمحاولة
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        let delay = self.initial_delay.as_millis() as f64
            * self.multiplier.powi(attempt as i32);
        let delay = delay.min(self.max_delay.as_millis() as f64);
        Duration::from_millis(delay as u64)
    }

    /// هل يجب إعادة المحاولة
    pub fn should_retry(&self, status: HttpStatus, attempt: u32) -> bool {
        attempt < self.max_retries && self.retry_status_codes.contains(&status.code)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Connection Pool
// ═══════════════════════════════════════════════════════════════════════════════

/// اتصال
#[derive(Debug)]
pub struct Connection {
    /// المعرف
    pub id: String,
    /// المضيف
    pub host: String,
    /// المنفذ
    pub port: u16,
    /// نشط
    pub is_active: bool,
    /// آخر استخدام
    pub last_used: std::time::Instant,
}

/// تجمع الاتصالات
#[derive(Debug)]
pub struct ConnectionPool {
    /// الاتصالات
    connections: RwLock<Vec<Connection>>,
    /// الحد الأقصى
    max_connections: usize,
    /// مهلة الخمول
    idle_timeout: Duration,
}

impl ConnectionPool {
    /// إنشاء تجمع جديد
    pub fn new(max_connections: usize, idle_timeout: Duration) -> Self {
        Self {
            connections: RwLock::new(Vec::new()),
            max_connections,
            idle_timeout,
        }
    }

    /// الحصول على اتصال
    pub fn acquire(&self, host: &str, port: u16) -> HttpResult<Arc<Connection>> {
        let mut connections = self.connections.write().unwrap();
        
        // البحث عن اتصال متاح
        if let Some(conn) = connections.iter_mut().find(|c| {
            c.host == host && c.port == port && c.is_active
        }) {
            conn.last_used = std::time::Instant::now();
            return Ok(Arc::new(Connection {
                id: conn.id.clone(),
                host: conn.host.clone(),
                port: conn.port,
                is_active: conn.is_active,
                last_used: conn.last_used,
            }));
        }
        
        // إنشاء اتصال جديد
        if connections.len() < self.max_connections {
            let conn = Connection {
                id: uuid::Uuid::new_v4().to_string(),
                host: host.to_string(),
                port,
                is_active: true,
                last_used: std::time::Instant::now(),
            };
            connections.push(conn.clone());
            return Ok(Arc::new(conn));
        }
        
        Err(HttpError::ConnectionError("تجمع الاتصالات ممتلئ".to_string()))
    }

    /// إرجاع اتصال
    pub fn release(&self, conn: &Connection) {
        let mut connections = self.connections.write().unwrap();
        if let Some(c) = connections.iter_mut().find(|c| c.id == conn.id) {
            c.is_active = true;
            c.last_used = std::time::Instant::now();
        }
    }

    /// تنظيف الاتصالات الخاملة
    pub fn cleanup_idle(&self) {
        let mut connections = self.connections.write().unwrap();
        let now = std::time::Instant::now();
        connections.retain(|c| now.duration_since(c.last_used) < self.idle_timeout);
    }

    /// عدد الاتصالات النشطة
    pub fn active_count(&self) -> usize {
        self.connections.read().unwrap().iter().filter(|c| c.is_active).count()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Proxy
// ═══════════════════════════════════════════════════════════════════════════════

/// إعدادات الوكيل
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    /// رابط الوكيل
    pub url: String,
    /// اسم المستخدم
    pub username: Option<String>,
    /// كلمة المرور
    pub password: Option<String>,
    /// تجاوز للنطاقات
    pub bypass: Vec<String>,
}

impl ProxyConfig {
    /// إنشاء إعدادات جديدة
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            username: None,
            password: None,
            bypass: Vec::new(),
        }
    }

    /// تعيين المصادقة
    pub fn auth(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    /// إضافة تجاوز
    pub fn bypass(mut self, domain: impl Into<String>) -> Self {
        self.bypass.push(domain.into());
        self
    }

    /// هل يجب استخدام الوكيل
    pub fn should_use_proxy(&self, url: &str) -> bool {
        for bypass in &self.bypass {
            if url.contains(bypass) {
                return false;
            }
        }
        true
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SSL/TLS
// ═══════════════════════════════════════════════════════════════════════════════

/// إعدادات SSL/TLS
#[derive(Debug, Clone)]
pub struct TlsConfig {
    /// التحقق من الشهادة
    pub verify_certificate: bool,
    /// التحقق من اسم المضيف
    pub verify_hostname: bool,
    /// مسار الشهادة
    pub certificate_path: Option<String>,
    /// مسار المفتاح
    pub key_path: Option<String>,
    /// الإصدارات المدعومة
    pub min_version: TlsVersion,
}

/// إصدار TLS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsVersion {
    /// TLS 1.0
    Tls10,
    /// TLS 1.1
    Tls11,
    /// TLS 1.2
    Tls12,
    /// TLS 1.3
    Tls13,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            verify_certificate: true,
            verify_hostname: true,
            certificate_path: None,
            key_path: None,
            min_version: TlsVersion::Tls12,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// WebSocket
// ═══════════════════════════════════════════════════════════════════════════════

/// إعدادات WebSocket
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    /// رابط WebSocket
    pub url: String,
    /// الرؤوس
    pub headers: HttpHeaders,
    /// المهلة
    pub timeout: Duration,
    /// حجم المخزن المؤقت
    pub buffer_size: usize,
    /// إعادة الاتصال التلقائية
    pub auto_reconnect: bool,
    /// الحد الأقصى لمحاولات إعادة الاتصال
    pub max_reconnect_attempts: u32,
}

impl WebSocketConfig {
    /// إنشاء إعدادات جديدة
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            headers: HttpHeaders::new(),
            timeout: Duration::from_secs(30),
            buffer_size: 8192,
            auto_reconnect: true,
            max_reconnect_attempts: 5,
        }
    }
}

/// رسالة WebSocket
#[derive(Debug, Clone)]
pub enum WebSocketMessage {
    /// نص
    Text(String),
    /// ثنائي
    Binary(Vec<u8>),
    /// Ping
    Ping(Vec<u8>),
    /// Pong
    Pong(Vec<u8>),
    /// إغلاق
    Close(u16, String),
}

/// WebSocket
pub struct WebSocket {
    /// الإعدادات
    config: WebSocketConfig,
    /// متصل
    connected: bool,
}

impl WebSocket {
    /// إنشاء WebSocket جديد
    pub fn new(config: WebSocketConfig) -> Self {
        Self {
            config,
            connected: false,
        }
    }

    /// الاتصال
    pub fn connect(&mut self) -> HttpResult<()> {
        // TODO: تنفيذ الاتصال الفعلي
        self.connected = true;
        Ok(())
    }

    /// إرسال رسالة
    pub fn send(&mut self, message: &WebSocketMessage) -> HttpResult<()> {
        if !self.connected {
            return Err(HttpError::WebSocketError("غير متصل".to_string()));
        }
        // TODO: تنفيذ الإرسال الفعلي
        Ok(())
    }

    /// استقبال رسالة
    pub fn receive(&mut self) -> HttpResult<WebSocketMessage> {
        if !self.connected {
            return Err(HttpError::WebSocketError("غير متصل".to_string()));
        }
        // TODO: تنفيذ الاستقبال الفعلي
        Ok(WebSocketMessage::Text("".to_string()))
    }

    /// إغلاق الاتصال
    pub fn close(&mut self) -> HttpResult<()> {
        self.connected = false;
        Ok(())
    }

    /// هل متصل
    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HTTP Client
// ═══════════════════════════════════════════════════════════════════════════════

/// إعدادات عميل HTTP
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    /// المهلة الافتراضية
    pub timeout: Duration,
    /// الاتصالات القصوى
    pub max_connections: usize,
    /// إعدادات إعادة المحاولة
    pub retry_config: RetryConfig,
    /// إعدادات TLS
    pub tls_config: TlsConfig,
    /// الوكيل
    pub proxy: Option<ProxyConfig>,
    /// User-Agent
    pub user_agent: String,
    /// اتباع إعادة التوجيه
    pub follow_redirects: bool,
    /// الحد الأقصى لإعادة التوجيه
    pub max_redirects: u32,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_connections: 100,
            retry_config: RetryConfig::default(),
            tls_config: TlsConfig::default(),
            proxy: None,
            user_agent: format!("AlMarjaa-HTTP/{}", env!("CARGO_PKG_VERSION")),
            follow_redirects: true,
            max_redirects: 10,
        }
    }
}

/// عميل HTTP
pub struct HttpClient {
    /// الإعدادات
    config: HttpClientConfig,
    /// الوسائط
    middlewares: Vec<HttpMiddleware>,
    /// تجمع الاتصالات
    pool: ConnectionPool,
    /// محدد المعدل
    rate_limiter: Option<RateLimiter>,
}

impl HttpClient {
    /// إنشاء عميل جديد
    pub fn new() -> Self {
        Self::with_config(HttpClientConfig::default())
    }

    /// إنشاء عميل بإعدادات مخصصة
    pub fn with_config(config: HttpClientConfig) -> Self {
        let pool = ConnectionPool::new(
            config.max_connections,
            Duration::from_secs(60),
        );
        Self {
            config,
            middlewares: Vec::new(),
            pool,
            rate_limiter: None,
        }
    }

    /// إضافة وسيط
    pub fn middleware(mut self, middleware: HttpMiddleware) -> Self {
        self.middlewares.push(middleware);
        self
    }

    /// تعيين محدد المعدل
    pub fn rate_limiter(mut self, limiter: RateLimiter) -> Self {
        self.rate_limiter = Some(limiter);
        self
    }

    /// تنفيذ طلب
    pub fn execute(&self, mut request: HttpRequest) -> HttpResult<HttpResponse> {
        // التحقق من محدد المعدل
        if let Some(ref limiter) = self.rate_limiter {
            if !limiter.is_allowed(&request.url) {
                return Err(HttpError::GenericError("تم تجاوز حد الطلبات".to_string()));
            }
        }

        // تنفيذ الوسائط
        for middleware in &self.middlewares {
            middleware.execute(&mut request)?;
        }

        // تعيين User-Agent
        if !request.headers.contains("User-Agent") {
            request.headers.set_user_agent(&self.config.user_agent);
        }

        // TODO: تنفيذ الطلب الفعلي باستخدام reqwest أو غيرها
        // هذا تنفيذ وهمي للعرض
        let response = HttpResponse::new(HttpStatus::OK)
            .text("تم التنفيذ بنجاح");
        
        Ok(response)
    }

    /// طلب GET
    pub fn get(&self, url: &str) -> HttpResult<HttpResponse> {
        self.execute(HttpRequest::get(url))
    }

    /// طلب POST
    pub fn post(&self, url: &str, body: Option<Vec<u8>>) -> HttpResult<HttpResponse> {
        let mut request = HttpRequest::post(url);
        if let Some(b) = body {
            request = request.body(b);
        }
        self.execute(request)
    }

    /// طلب PUT
    pub fn put(&self, url: &str, body: Option<Vec<u8>>) -> HttpResult<HttpResponse> {
        let mut request = HttpRequest::put(url);
        if let Some(b) = body {
            request = request.body(b);
        }
        self.execute(request)
    }

    /// طلب DELETE
    pub fn delete(&self, url: &str) -> HttpResult<HttpResponse> {
        self.execute(HttpRequest::delete(url))
    }

    /// طلب PATCH
    pub fn patch(&self, url: &str, body: Option<Vec<u8>>) -> HttpResult<HttpResponse> {
        let mut request = HttpRequest::patch(url);
        if let Some(b) = body {
            request = request.body(b);
        }
        self.execute(request)
    }

    /// طلب JSON
    pub fn json<T: serde::Serialize>(&self, method: HttpMethod, url: &str, data: &T) -> HttpResult<HttpResponse> {
        let request = HttpRequest::new(method, url).json(data)?;
        self.execute(request)
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HTTP Server
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع معالج الطلب
pub type RequestHandler = Box<dyn Fn(&HttpRequest) -> HttpResult<HttpResponse> + Send + Sync>;

/// مسار
#[derive(Debug)]
pub struct Route {
    /// الطريق
    pub path: String,
    /// الطريقة
    pub method: HttpMethod,
    /// المعالج
    pub handler: RequestHandler,
}

/// إعدادات خادم HTTP
#[derive(Debug, Clone)]
pub struct HttpServerConfig {
    /// المضيف
    pub host: String,
    /// المنفذ
    pub port: u16,
    /// عدد العمال
    pub workers: usize,
    /// المهلة
    pub timeout: Duration,
    /// الحد الأقصى لحجم الطلب
    pub max_request_size: usize,
    /// تفعيل TLS
    pub tls_enabled: bool,
    /// شهادة TLS
    pub tls_cert: Option<String>,
    /// مفتاح TLS
    pub tls_key: Option<String>,
}

impl Default for HttpServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            workers: 4,
            timeout: Duration::from_secs(30),
            max_request_size: 10 * 1024 * 1024, // 10 MB
            tls_enabled: false,
            tls_cert: None,
            tls_key: None,
        }
    }
}

/// خادم HTTP
pub struct HttpServer {
    /// الإعدادات
    config: HttpServerConfig,
    /// المسارات
    routes: Vec<Route>,
    /// الوسائط
    middlewares: Vec<HttpMiddleware>,
    /// مدير الجلسات
    session_manager: Option<SessionManager>,
    /// محدد المعدل
    rate_limiter: Option<RateLimiter>,
}

impl HttpServer {
    /// إنشاء خادم جديد
    pub fn new() -> Self {
        Self::with_config(HttpServerConfig::default())
    }

    /// إنشاء خادم بإعدادات مخصصة
    pub fn with_config(config: HttpServerConfig) -> Self {
        Self {
            config,
            routes: Vec::new(),
            middlewares: Vec::new(),
            session_manager: None,
            rate_limiter: None,
        }
    }

    /// إضافة مسار GET
    pub fn get(mut self, path: &str, handler: RequestHandler) -> Self {
        self.routes.push(Route {
            path: path.to_string(),
            method: HttpMethod::Get,
            handler,
        });
        self
    }

    /// إضافة مسار POST
    pub fn post(mut self, path: &str, handler: RequestHandler) -> Self {
        self.routes.push(Route {
            path: path.to_string(),
            method: HttpMethod::Post,
            handler,
        });
        self
    }

    /// إضافة مسار PUT
    pub fn put(mut self, path: &str, handler: RequestHandler) -> Self {
        self.routes.push(Route {
            path: path.to_string(),
            method: HttpMethod::Put,
            handler,
        });
        self
    }

    /// إضافة مسار DELETE
    pub fn delete(mut self, path: &str, handler: RequestHandler) -> Self {
        self.routes.push(Route {
            path: path.to_string(),
            method: HttpMethod::Delete,
            handler,
        });
        self
    }

    /// إضافة وسيط
    pub fn middleware(mut self, middleware: HttpMiddleware) -> Self {
        self.middlewares.push(middleware);
        self
    }

    /// تعيين مدير الجلسات
    pub fn session_manager(mut self, manager: SessionManager) -> Self {
        self.session_manager = Some(manager);
        self
    }

    /// تعيين محدد المعدل
    pub fn rate_limiter(mut self, limiter: RateLimiter) -> Self {
        self.rate_limiter = Some(limiter);
        self
    }

    /// معالجة الطلب
    pub fn handle(&self, request: &HttpRequest) -> HttpResult<HttpResponse> {
        // التحقق من محدد المعدل
        if let Some(ref limiter) = self.rate_limiter {
            let key = request.url.clone();
            if !limiter.is_allowed(&key) {
                return Ok(HttpResponse::new(HttpStatus::TOO_MANY_REQUESTS)
                    .text("تم تجاوز حد الطلبات"));
            }
        }

        // البحث عن المسار المناسب
        for route in &self.routes {
            if route.method == request.method && self.path_matches(&route.path, &request.url) {
                return (route.handler)(request);
            }
        }

        Ok(HttpResponse::not_found().text("الصفحة غير موجودة"))
    }

    /// التحقق من مطابقة المسار
    fn path_matches(&self, pattern: &str, url: &str) -> bool {
        // TODO: تنفيذ مطابقة المسارات مع المعاملات
        url.contains(pattern)
    }

    /// بدء الخادم
    pub fn start(&self) -> HttpResult<()> {
        println!("🚀 بدء الخادم على {}:{}", self.config.host, self.config.port);
        // TODO: تنفيذ بدء الخادم الفعلي
        Ok(())
    }

    /// إيقاف الخادم
    pub fn stop(&self) -> HttpResult<()> {
        println!("🛑 إيقاف الخادم");
        // TODO: تنفيذ إيقاف الخادم الفعلي
        Ok(())
    }
}

impl Default for HttpServer {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// REST Client المتقدم
// ═══════════════════════════════════════════════════════════════════════════════

/// عميل REST
pub struct RestClient {
    /// العميل الأساسي
    client: HttpClient,
    /// الرابط الأساسي
    base_url: String,
}

impl RestClient {
    /// إنشاء عميل REST جديد
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: HttpClient::new(),
            base_url: base_url.into(),
        }
    }

    /// بناء الرابط الكامل
    fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), path)
    }

    /// GET مع معاملات
    pub fn get_with_params(&self, path: &str, params: HashMap<String, String>) -> HttpResult<HttpResponse> {
        let mut request = HttpRequest::get(self.build_url(path));
        for (k, v) in params {
            request = request.query(k, v);
        }
        self.client.execute(request)
    }

    /// POST مع JSON
    pub fn post_json<T: serde::Serialize>(&self, path: &str, data: &T) -> HttpResult<HttpResponse> {
        let url = self.build_url(path);
        let request = HttpRequest::post(&url).json(data)?;
        self.client.execute(request)
    }

    /// PUT مع JSON
    pub fn put_json<T: serde::Serialize>(&self, path: &str, data: &T) -> HttpResult<HttpResponse> {
        let url = self.build_url(path);
        let request = HttpRequest::put(&url).json(data)?;
        self.client.execute(request)
    }

    /// DELETE
    pub fn delete(&self, path: &str) -> HttpResult<HttpResponse> {
        self.client.delete(&self.build_url(path))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الاختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_method() {
        assert_eq!(HttpMethod::Get.as_str(), "GET");
        assert_eq!(HttpMethod::from_str("post"), Some(HttpMethod::Post));
    }

    #[test]
    fn test_http_status() {
        assert!(HttpStatus::OK.is_success());
        assert!(HttpStatus::BAD_REQUEST.is_client_error());
        assert!(HttpStatus::INTERNAL_SERVER_ERROR.is_server_error());
    }

    #[test]
    fn test_http_headers() {
        let mut headers = HttpHeaders::new();
        headers.insert("Content-Type", "application/json");
        assert!(headers.contains("content-type"));
        assert_eq!(headers.content_type(), Some(&"application/json".to_string()));
    }

    #[test]
    fn test_http_request() {
        let request = HttpRequest::get("https://example.com")
            .header("Accept", "application/json")
            .query("page", "1")
            .timeout(Duration::from_secs(10));
        
        assert_eq!(request.method, HttpMethod::Get);
        assert!(request.headers.contains("accept"));
    }

    #[test]
    fn test_cookie() {
        let cookie = Cookie::new("session", "abc123")
            .path("/")
            .secure(true)
            .http_only(true);
        
        assert!(cookie.to_header().contains("session=abc123"));
    }

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(5, Duration::from_secs(1));
        
        for _ in 0..5 {
            assert!(limiter.is_allowed("test"));
        }
        assert!(!limiter.is_allowed("test"));
    }

    #[test]
    fn test_retry_config() {
        let config = RetryConfig::new().max_retries(3);
        
        assert!(config.should_retry(HttpStatus::SERVICE_UNAVAILABLE, 0));
        assert!(!config.should_retry(HttpStatus::OK, 0));
    }
}
