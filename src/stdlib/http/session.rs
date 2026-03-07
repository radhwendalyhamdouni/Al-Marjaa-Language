// src/stdlib/http/session.rs
// إدارة الجلسات

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// جلسة
#[derive(Debug, Clone)]
pub struct Session {
    /// معرف الجلسة
    pub id: String,
    /// البيانات
    pub data: HashMap<String, String>,
    /// وقت الإنشاء
    pub created_at: Instant,
    /// آخر نشاط
    pub last_activity: Instant,
    /// مدة الصلاحية
    pub ttl: Duration,
}

impl Session {
    /// إنشاء جلسة جديدة
    pub fn new(id: &str, ttl: Duration) -> Self {
        let now = Instant::now();
        Self {
            id: id.to_string(),
            data: HashMap::new(),
            created_at: now,
            last_activity: now,
            ttl,
        }
    }
    
    /// الحصول على قيمة
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    
    /// تعيين قيمة
    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
        self.touch();
    }
    
    /// حذف قيمة
    pub fn remove(&mut self, key: &str) -> Option<String> {
        let result = self.data.remove(key);
        self.touch();
        result
    }
    
    /// تحديث النشاط
    pub fn touch(&mut self) {
        self.last_activity = Instant::now();
    }
    
    /// هل الجلسة منتهية؟
    pub fn is_expired(&self) -> bool {
        self.last_activity.elapsed() > self.ttl
    }
}

/// مدير الجلسات
pub struct SessionManager {
    /// الجلسات
    sessions: HashMap<String, Session>,
    /// مدة الصلاحية الافتراضية
    default_ttl: Duration,
}

impl SessionManager {
    /// إنشاء مدير جديد
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            sessions: HashMap::new(),
            default_ttl,
        }
    }
    
    /// إنشاء جلسة جديدة
    pub fn create(&mut self) -> String {
        let id = self.generate_id();
        let session = Session::new(&id, self.default_ttl);
        self.sessions.insert(id.clone(), session);
        id
    }
    
    /// الحصول على جلسة
    pub fn get(&mut self, id: &str) -> Option<&mut Session> {
        if let Some(session) = self.sessions.get_mut(id) {
            if session.is_expired() {
                self.sessions.remove(id);
                return None;
            }
            session.touch();
            return Some(session);
        }
        None
    }
    
    /// حذف جلسة
    pub fn destroy(&mut self, id: &str) {
        self.sessions.remove(id);
    }
    
    /// تنظيف الجلسات المنتهية
    pub fn cleanup(&mut self) {
        let expired: Vec<String> = self.sessions
            .iter()
            .filter(|(_, s)| s.is_expired())
            .map(|(id, _)| id.clone())
            .collect();
        
        for id in expired {
            self.sessions.remove(&id);
        }
    }
    
    /// توليد معرف عشوائي
    fn generate_id(&self) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::thread_rng();
        
        (0..32)
            .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
            .collect()
    }
    
    /// عدد الجلسات النشطة
    pub fn count(&self) -> usize {
        self.sessions.len()
    }
}
