// src/stdlib/http/middleware.rs
// البرمجيات الوسيطة
// HTTP Middleware

use super::{HttpRequest, HttpResponseBuilder, StatusCode};

/// نوع البرمجية الوسيطة
pub type MiddlewareFn = Box<dyn Fn(&HttpRequest, &mut HttpResponseBuilder) + Send + Sync>;

/// سجل الطلبات
pub fn logger(req: &HttpRequest, _res: &mut HttpResponseBuilder) {
    println!("[{}] {} {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), 
             req.method.to_string(), req.path);
}

/// التحقق من المصادقة
pub fn auth_checker(req: &HttpRequest, res: &mut HttpResponseBuilder) {
    if !req.headers.contains("Authorization") {
        res.status = StatusCode::UNAUTHORIZED;
        res.body = r#"{"خطأ": "مطلوب مصادقة"}"#.to_string();
    }
}

/// CORS - مشاركة الموارد عبر المصادر
pub fn cors(origin: &str) -> impl Fn(&HttpRequest, &mut HttpResponseBuilder) + Send + Sync {
    let origin = origin.to_string();
    move |_req: &HttpRequest, res: &mut HttpResponseBuilder| {
        res.headers.insert("Access-Control-Allow-Origin".to_string(), origin.clone());
        res.headers.insert("Access-Control-Allow-Methods".to_string(), "GET, POST, PUT, DELETE, OPTIONS".to_string());
        res.headers.insert("Access-Control-Allow-Headers".to_string(), "Content-Type, Authorization".to_string());
    }
}

/// ضغط GZIP
pub fn gzip(_req: &HttpRequest, res: &mut HttpResponseBuilder) {
    res.headers.insert("Content-Encoding".to_string(), "gzip".to_string());
}

/// تحديد معدل الطلبات
pub struct RateLimiter {
    requests: std::collections::HashMap<String, Vec<std::time::Instant>>,
    max_requests: usize,
    window_secs: u64,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            requests: std::collections::HashMap::new(),
            max_requests,
            window_secs,
        }
    }
    
    pub fn check(&mut self, ip: &str) -> bool {
        let now = std::time::Instant::now();
        let window = std::time::Duration::from_secs(self.window_secs);
        
        let entry = self.requests.entry(ip.to_string()).or_insert_with(Vec::new);
        
        // إزالة الطلبات القديمة
        entry.retain(|&t| now.duration_since(t) < window);
        
        if entry.len() >= self.max_requests {
            return false;
        }
        
        entry.push(now);
        true
    }
}

/// برمجية وسيطة لتحديد المعدل
pub fn rate_limit(max_requests: usize, window_secs: u64) -> impl Fn(&HttpRequest, &mut HttpResponseBuilder) + Send + Sync {
    let limiter = std::sync::Arc::new(std::sync::Mutex::new(RateLimiter::new(max_requests, window_secs)));
    
    move |req: &HttpRequest, res: &mut HttpResponseBuilder| {
        let ip = req.remote_addr.clone().unwrap_or_else(|| "unknown".to_string());
        let mut limiter = limiter.lock().unwrap();
        
        if !limiter.check(&ip) {
            res.status = StatusCode::TOO_MANY_REQUESTS;
            res.body = r#"{"خطأ": "تجاوزت الحد الأقصى للطلبات"}"#.to_string();
        }
    }
}
