// src/stdlib/http/server.rs
// خادم HTTP المتقدم
// Advanced HTTP Server

use super::{HttpMethod, HttpHeaders, StatusCode, ContentType};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// طلب HTTP الوارد
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// الطريقة
    pub method: HttpMethod,
    /// المسار
    pub path: String,
    /// معاملات الاستعلام
    pub query: HashMap<String, String>,
    /// الرؤوس
    pub headers: HttpHeaders,
    /// الجسم
    pub body: String,
    /// عناوين IP
    pub remote_addr: Option<String>,
    /// ملفات تعريف الارتباط
    pub cookies: HashMap<String, String>,
}

impl HttpRequest {
    /// إنشاء طلب جديد
    pub fn new(method: HttpMethod, path: String) -> Self {
        Self {
            method,
            path,
            query: HashMap::new(),
            headers: HttpHeaders::new(),
            body: String::new(),
            remote_addr: None,
            cookies: HashMap::new(),
        }
    }
    
    /// الحصول على معامل
    pub fn query(&self, key: &str) -> Option<&String> {
        self.query.get(key)
    }
    
    /// الحصول على رأس
    pub fn header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }
    
    /// الحصول على ملف تعريف الارتباط
    pub fn cookie(&self, name: &str) -> Option<&String> {
        self.cookies.get(name)
    }
    
    /// تحليل JSON
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, String> {
        serde_json::from_str(&self.body)
            .map_err(|e| format!("خطأ في تحليل JSON: {}", e))
    }
}

/// استجابة HTTP الصادرة
#[derive(Debug, Clone)]
pub struct HttpResponseBuilder {
    /// رمز الحالة
    pub status: StatusCode,
    /// الرؤوس
    pub headers: HttpHeaders,
    /// الجسم
    pub body: String,
    /// نوع المحتوى
    pub content_type: ContentType,
}

impl HttpResponseBuilder {
    /// إنشاء استجابة جديدة
    pub fn new() -> Self {
        Self {
            status: StatusCode::OK,
            headers: HttpHeaders::new(),
            body: String::new(),
            content_type: ContentType::Text,
        }
    }
    
    /// تعيين رمز الحالة
    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }
    
    /// تعيين الجسم
    pub fn body(mut self, body: String) -> Self {
        self.body = body;
        self
    }
    
    /// تعيين نوع المحتوى
    pub fn content_type(mut self, ct: ContentType) -> Self {
        self.content_type = ct;
        self
    }
    
    /// إضافة رأس
    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
    
    /// تعيين JSON
    pub fn json<T: serde::Serialize>(mut self, data: &T) -> Result<Self, String> {
        self.body = serde_json::to_string(data)
            .map_err(|e| format!("خطأ في تحويل JSON: {}", e))?;
        self.content_type = ContentType::Json;
        Ok(self)
    }
    
    /// تعيين HTML
    pub fn html(mut self, html: String) -> Self {
        self.body = html;
        self.content_type = ContentType::Html;
        self
    }
    
    /// تعيين نص
    pub fn text(mut self, text: String) -> Self {
        self.body = text;
        self.content_type = ContentType::Text;
        self
    }
    
    /// إعادة توجيه
    pub fn redirect(mut self, url: &str) -> Self {
        self.status = StatusCode::FOUND;
        self.headers.insert("Location".to_string(), url.to_string());
        self
    }
    
    /// خطأ 404
    pub fn not_found(mut self) -> Self {
        self.status = StatusCode::NOT_FOUND;
        self.body = "غير موجود".to_string();
        self
    }
    
    /// خطأ 500
    pub fn internal_error(mut self, message: &str) -> Self {
        self.status = StatusCode::INTERNAL_SERVER_ERROR;
        self.body = message.to_string();
        self
    }
    
    /// بناء الاستجابة النهائية
    pub fn build(self) -> String {
        let status_line = format!("HTTP/1.1 {} {}", self.status.0, self.status.reason_phrase());
        let content_type = format!("Content-Type: {}", self.content_type.to_mime());
        let content_length = format!("Content-Length: {}", self.body.len());
        
        let mut headers_str = String::new();
        for (key, value) in self.headers.iter() {
            headers_str.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        format!(
            "{}\r\n{}\r\n{}\r\n{}\r\n\r\n{}",
            status_line, content_type, content_length, headers_str, self.body
        )
    }
}

impl Default for HttpResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// معالج المسار
pub type RouteHandler = Arc<dyn Fn(&HttpRequest) -> HttpResponseBuilder + Send + Sync>;

/// مسار
#[derive(Clone)]
pub struct Route {
    /// الطريقة
    pub method: HttpMethod,
    /// النمط
    pub pattern: String,
    /// المعالج
    pub handler: RouteHandler,
}

/// خادم HTTP
pub struct HttpServer {
    /// المنفذ
    pub port: u16,
    /// المضيف
    pub host: String,
    /// المسارات
    routes: Arc<Mutex<Vec<Route>>>,
    /// البرمجيات الوسيطة
    middleware: Arc<Mutex<Vec<Box<dyn Fn(&HttpRequest, &mut HttpResponseBuilder) + Send + Sync>>>>,
}

impl HttpServer {
    /// إنشاء خادم جديد
    pub fn new() -> Self {
        Self {
            port: 8080,
            host: "0.0.0.0".to_string(),
            routes: Arc::new(Mutex::new(Vec::new())),
            middleware: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// تعيين المنفذ
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    
    /// تعيين المضيف
    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }
    
    /// إضافة مسار GET
    pub fn get<F>(&self, pattern: &str, handler: F)
    where
        F: Fn(&HttpRequest) -> HttpResponseBuilder + Send + Sync + 'static,
    {
        self.add_route(HttpMethod::Get, pattern, handler);
    }
    
    /// إضافة مسار POST
    pub fn post<F>(&self, pattern: &str, handler: F)
    where
        F: Fn(&HttpRequest) -> HttpResponseBuilder + Send + Sync + 'static,
    {
        self.add_route(HttpMethod::Post, pattern, handler);
    }
    
    /// إضافة مسار PUT
    pub fn put<F>(&self, pattern: &str, handler: F)
    where
        F: Fn(&HttpRequest) -> HttpResponseBuilder + Send + Sync + 'static,
    {
        self.add_route(HttpMethod::Put, pattern, handler);
    }
    
    /// إضافة مسار DELETE
    pub fn delete<F>(&self, pattern: &str, handler: F)
    where
        F: Fn(&HttpRequest) -> HttpResponseBuilder + Send + Sync + 'static,
    {
        self.add_route(HttpMethod::Delete, pattern, handler);
    }
    
    /// إضافة مسار عام
    fn add_route<F>(&self, method: HttpMethod, pattern: &str, handler: F)
    where
        F: Fn(&HttpRequest) -> HttpResponseBuilder + Send + Sync + 'static,
    {
        let route = Route {
            method,
            pattern: pattern.to_string(),
            handler: Arc::new(handler),
        };
        
        self.routes.lock().unwrap().push(route);
    }
    
    /// إضافة برمجية وسيطة
    pub fn use_middleware<F>(&self, middleware: F)
    where
        F: Fn(&HttpRequest, &mut HttpResponseBuilder) + Send + Sync + 'static,
    {
        self.middleware.lock().unwrap().push(Box::new(middleware));
    }
    
    /// معالجة الطلب
    pub fn handle(&self, request: &HttpRequest) -> HttpResponseBuilder {
        let routes = self.routes.lock().unwrap();
        
        for route in routes.iter() {
            if route.method == request.method && self.match_pattern(&route.pattern, &request.path) {
                let mut response = (route.handler)(request);
                
                // تطبيق البرمجيات الوسيطة
                let middleware = self.middleware.lock().unwrap();
                for mw in middleware.iter() {
                    mw(request, &mut response);
                }
                
                return response;
            }
        }
        
        // لم يتم العثور على مسار
        HttpResponseBuilder::new().not_found()
    }
    
    /// مطابقة النمط
    fn match_pattern(&self, pattern: &str, path: &str) -> bool {
        if pattern == path {
            return true;
        }
        
        // دعم المعاملات الديناميكية مثل /user/:id
        let pattern_parts: Vec<&str> = pattern.split('/').collect();
        let path_parts: Vec<&str> = path.split('/').collect();
        
        if pattern_parts.len() != path_parts.len() {
            return false;
        }
        
        for (p, actual) in pattern_parts.iter().zip(path_parts.iter()) {
            if p.starts_with(':') {
                continue; // معامل ديناميكي
            }
            if p != actual {
                return false;
            }
        }
        
        true
    }
    
    /// تشغيل الخادم
    pub fn run(&self) -> Result<(), String> {
        println!("🚀 الخادم يعمل على http://{}:{}", self.host, self.port);
        println!("📖 لغة المرجع - الخادم العربي المتقدم");
        
        // في الإصدار الحقيقي، سيتم استخدام tokio أو async-std
        // هذا تنفيذ مبسط للعرض
        
        Ok(())
    }
}

impl Default for HttpServer {
    fn default() -> Self {
        Self::new()
    }
}

// ===== دوال عربية =====

/// إنشاء خادم جديد
pub fn خادم_جديد() -> HttpServer {
    HttpServer::new()
}

/// استجابة جديدة
pub fn استجابة() -> HttpResponseBuilder {
    HttpResponseBuilder::new()
}
