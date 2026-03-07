// src/stdlib/http/client.rs
// عميل HTTP المتقدم
// Advanced HTTP Client

use super::{HttpMethod, HttpHeaders, StatusCode, ContentType};
use std::collections::HashMap;
use std::time::Duration;

/// إعدادات عميل HTTP
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    /// مهلة الاتصال بالثواني
    pub timeout: Duration,
    /// مهلة الاتصال بالخادم
    pub connect_timeout: Duration,
    /// الحد الأقصى لإعادة المحاولة
    pub max_retries: u32,
    /// التأخير بين المحاولات
    pub retry_delay: Duration,
    /// استخدام الاتصالات المستمرة
    pub keep_alive: bool,
    /// الحد الأقصى للاتصالات في المجمع
    pub max_connections: usize,
    /// اتبع إعادات التوجيه تلقائياً
    pub follow_redirects: bool,
    /// الحد الأقصى لإعادات التوجيه
    pub max_redirects: u32,
    /// التحقق من شهادات SSL
    pub verify_ssl: bool,
    /// وكيل HTTP
    pub proxy: Option<String>,
    /// رؤوس افتراضية
    pub default_headers: HttpHeaders,
    /// معرف المستخدم الافتراضي
    pub user_agent: String,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            max_retries: 3,
            retry_delay: Duration::from_millis(500),
            keep_alive: true,
            max_connections: 10,
            follow_redirects: true,
            max_redirects: 10,
            verify_ssl: true,
            proxy: None,
            default_headers: HttpHeaders::new(),
            user_agent: "AlMarjaa/3.2.0".to_string(),
        }
    }
}

impl HttpClientConfig {
    /// إنشاء إعدادات جديدة
    pub fn new() -> Self {
        Self::default()
    }
    
    /// تعيين المهلة
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout = Duration::from_secs(seconds);
        self
    }
    
    /// تعيين مهلة الاتصال
    pub fn with_connect_timeout(mut self, seconds: u64) -> Self {
        self.connect_timeout = Duration::from_secs(seconds);
        self
    }
    
    /// تعيين الحد الأقصى للمحاولات
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }
    
    /// تعيين الوكيل
    pub fn with_proxy(mut self, proxy: String) -> Self {
        self.proxy = Some(proxy);
        self
    }
    
    /// تعيين معرف المستخدم
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }
    
    /// تعطيل التحقق من SSL (غير آمن!)
    pub fn insecure(mut self) -> Self {
        self.verify_ssl = false;
        self
    }
}

/// نتيجة HTTP
#[derive(Debug, Clone)]
pub struct HttpResponse {
    /// رمز الحالة
    pub status: StatusCode,
    /// الرؤوس
    pub headers: HttpHeaders,
    /// المحتوى
    pub body: String,
    /// المحتوى الثنائي
    pub body_bytes: Option<Vec<u8>>,
    /// وقت الاستجابة
    pub response_time: Duration,
    /// عنوان URL النهائي (بعد إعادات التوجيه)
    pub final_url: Option<String>,
}

impl HttpResponse {
    /// هل نجح الطلب؟
    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }
    
    /// هل فشل الطلب؟
    pub fn is_error(&self) -> bool {
        self.status.is_client_error() || self.status.is_server_error()
    }
    
    /// تحليل JSON
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, String> {
        serde_json::from_str(&self.body)
            .map_err(|e| format!("خطأ في تحليل JSON: {}", e))
    }
    
    /// الحصول على المحتوى الثنائي
    pub fn bytes(&self) -> &[u8] {
        self.body_bytes.as_deref().unwrap_or_default()
    }
    
    /// الحصول على المحتوى كنص
    pub fn text(&self) -> &str {
        &self.body
    }
}

/// عميل HTTP
pub struct HttpClient {
    config: HttpClientConfig,
}

impl HttpClient {
    /// إنشاء عميل جديد
    pub fn new() -> Self {
        Self {
            config: HttpClientConfig::default(),
        }
    }
    
    /// إنشاء عميل بإعدادات مخصصة
    pub fn with_config(config: HttpClientConfig) -> Self {
        Self { config }
    }
    
    /// طلب GET
    pub fn get(&self, url: &str) -> Result<HttpResponse, String> {
        self.request(HttpMethod::Get, url, None, None)
    }
    
    /// طلب POST
    pub fn post(&self, url: &str, body: Option<&str>) -> Result<HttpResponse, String> {
        self.request(HttpMethod::Post, url, body, None)
    }
    
    /// طلب PUT
    pub fn put(&self, url: &str, body: Option<&str>) -> Result<HttpResponse, String> {
        self.request(HttpMethod::Put, url, body, None)
    }
    
    /// طلب DELETE
    pub fn delete(&self, url: &str) -> Result<HttpResponse, String> {
        self.request(HttpMethod::Delete, url, None, None)
    }
    
    /// طلب PATCH
    pub fn patch(&self, url: &str, body: Option<&str>) -> Result<HttpResponse, String> {
        self.request(HttpMethod::Patch, url, body, None)
    }
    
    /// طلب عام
    pub fn request(
        &self,
        method: HttpMethod,
        url: &str,
        body: Option<&str>,
        headers: Option<&HttpHeaders>,
    ) -> Result<HttpResponse, String> {
        let start = std::time::Instant::now();
        
        // تنفيذ الطلب
        let result = self.execute_request(&method, url, body, headers);
        
        // إذا فشل، حاول مرة أخرى
        let mut retries = 0;
        let mut result = result;
        
        while result.is_err() && retries < self.config.max_retries {
            retries += 1;
            std::thread::sleep(self.config.retry_delay);
            result = self.execute_request(&method, url, body, headers);
        }
        
        let response_time = start.elapsed();
        
        result.map(|mut res| {
            res.response_time = response_time;
            res
        })
    }
    
    /// تنفيذ الطلب فعلياً
    fn execute_request(
        &self,
        method: &HttpMethod,
        url: &str,
        body: Option<&str>,
        headers: Option<&HttpHeaders>,
    ) -> Result<HttpResponse, String> {
        // تنفيذ الطلب باستخدام reqwest إذا كان متاحاً
        #[cfg(feature = "network")]
        {
            use reqwest::blocking::Client;
            use std::time::Duration;
            
            let mut client_builder = Client::builder()
                .timeout(Duration::from_secs(30))
                .danger_accept_invalid_certs(!self.config.verify_ssl);
            
            if let Some(ref proxy) = self.config.proxy {
                let proxy = reqwest::Proxy::all(proxy)
                    .map_err(|e| format!("خطأ في إعداد الوكيل: {}", e))?;
                client_builder = client_builder.proxy(proxy);
            }
            
            let client = client_builder.build()
                .map_err(|e| format!("خطأ في إنشاء العميل: {}", e))?;
            
            let mut request = match method {
                HttpMethod::Get => client.get(url),
                HttpMethod::Post => client.post(url),
                HttpMethod::Put => client.put(url),
                HttpMethod::Delete => client.delete(url),
                HttpMethod::Patch => client.patch(url),
                HttpMethod::Head => client.head(url),
                _ => return Err(format!("الطريقة غير مدعومة: {:?}", method)),
            };
            
            // إضافة الرؤوس
            if let Some(h) = headers {
                for (key, value) in h.iter() {
                    request = request.header(key, value);
                }
            }
            
            // إضافة الجسم
            if let Some(b) = body {
                request = request.body(b.to_string());
            }
            
            let response = request.send()
                .map_err(|e| format!("خطأ في إرسال الطلب: {}", e))?;
            
            let status = StatusCode(response.status().as_u16());
            let mut headers_resp = HttpHeaders::new();
            
            for (key, value) in response.headers() {
                if let Ok(v) = value.to_str() {
                    headers_resp.insert(key.to_string(), v.to_string());
                }
            }
            
            let body_bytes = response.bytes()
                .map_err(|e| format!("خطأ في قراءة الاستجابة: {}", e))?;
            
            let body = String::from_utf8_lossy(&body_bytes).to_string();
            
            Ok(HttpResponse {
                status,
                headers: headers_resp,
                body,
                body_bytes: Some(body_bytes.to_vec()),
                response_time: Duration::from_secs(0),
                final_url: None,
            })
        }
        
        #[cfg(not(feature = "network"))]
        {
            Err("ميزة الشبكة غير مفعّلة. أضف --features network عند البناء.".to_string())
        }
    }
    
    /// طلب مع JSON
    pub fn json<T: serde::Serialize>(
        &self,
        method: HttpMethod,
        url: &str,
        data: &T,
    ) -> Result<HttpResponse, String> {
        let body = serde_json::to_string(data)
            .map_err(|e| format!("خطأ في تحويل JSON: {}", e))?;
        
        let mut headers = HttpHeaders::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        self.request(method, url, Some(&body), Some(&headers))
    }
    
    /// تحميل ملف
    pub fn download(&self, url: &str, path: &str) -> Result<(), String> {
        let response = self.get(url)?;
        
        if !response.is_success() {
            return Err(format!("فشل التحميل: {}", response.status.0));
        }
        
        let bytes = response.body_bytes.ok_or("لا يوجد محتوى")?;
        
        std::fs::write(path, bytes)
            .map_err(|e| format!("خطأ في كتابة الملف: {}", e))
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

// ===== دوال العربية =====

/// إنشاء عميل HTTP جديد
pub fn عميل_جديد() -> HttpClient {
    HttpClient::new()
}

/// طلب GET
pub fn احضر(url: &str) -> Result<HttpResponse, String> {
    HttpClient::new().get(url)
}

/// طلب POST
pub fn ارسل(url: &str, body: &str) -> Result<HttpResponse, String> {
    HttpClient::new().post(url, Some(body))
}

/// طلب PUT
pub fn ضع(url: &str, body: &str) -> Result<HttpResponse, String> {
    HttpClient::new().put(url, Some(body))
}

/// طلب DELETE
pub fn احذف(url: &str) -> Result<HttpResponse, String> {
    HttpClient::new().delete(url)
}
