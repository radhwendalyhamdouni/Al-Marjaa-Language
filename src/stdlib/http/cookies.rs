// src/stdlib/http/cookies.rs
// ملفات تعريف الارتباط

use std::collections::HashMap;

/// ملف تعريف الارتباط
#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub expires: Option<String>,
    pub max_age: Option<u64>,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<String>,
}

impl Cookie {
    /// إنشاء ملف تعريف جديد
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            expires: None,
            max_age: None,
            domain: None,
            path: None,
            secure: false,
            http_only: false,
            same_site: None,
        }
    }
    
    /// تعيين تاريخ الانتهاء
    pub fn expires(mut self, expires: &str) -> Self {
        self.expires = Some(expires.to_string());
        self
    }
    
    /// تعيين العمر الأقصى
    pub fn max_age(mut self, seconds: u64) -> Self {
        self.max_age = Some(seconds);
        self
    }
    
    /// تعيين النطاق
    pub fn domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_string());
        self
    }
    
    /// تعيين المسار
    pub fn path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }
    
    /// جعله آمناً
    pub fn secure(mut self) -> Self {
        self.secure = true;
        self
    }
    
    /// جعله HTTP فقط
    pub fn http_only(mut self) -> Self {
        self.http_only = true;
        self
    }
    
    /// تحويل إلى نص للرأس
    pub fn to_header(&self) -> String {
        let mut parts = format!("{}={}", self.name, self.value);
        
        if let Some(ref expires) = self.expires {
            parts.push_str(&format!("; Expires={}", expires));
        }
        
        if let Some(max_age) = self.max_age {
            parts.push_str(&format!("; Max-Age={}", max_age));
        }
        
        if let Some(ref domain) = self.domain {
            parts.push_str(&format!("; Domain={}", domain));
        }
        
        if let Some(ref path) = self.path {
            parts.push_str(&format!("; Path={}", path));
        }
        
        if self.secure {
            parts.push_str("; Secure");
        }
        
        if self.http_only {
            parts.push_str("; HttpOnly");
        }
        
        if let Some(ref same_site) = self.same_site {
            parts.push_str(&format!("; SameSite={}", same_site));
        }
        
        parts
    }
}

/// محلل ملفات تعريف الارتباط
pub fn parse_cookies(header: &str) -> HashMap<String, String> {
    let mut cookies = HashMap::new();
    
    for part in header.split(';') {
        let part = part.trim();
        if let Some(pos) = part.find('=') {
            let name = part[..pos].trim().to_string();
            let value = part[pos + 1..].trim().to_string();
            cookies.insert(name, value);
        }
    }
    
    cookies
}
