//! Cache - التخزين المؤقت

use lru::LruCache;
use parking_lot::Mutex;
use std::num::NonZeroUsize;
use std::sync::Arc;

/// ذاكرة مؤقتة عالمية
pub struct GlobalCache {
    /// ذاكرة الإكمال
    completion_cache: Mutex<LruCache<String, Vec<String>>>,
    /// ذاكرة التحليل
    analysis_cache: Mutex<LruCache<String, AnalysisCacheEntry>>,
    /// ذاكرة التوثيق
    docs_cache: Mutex<LruCache<String, String>>,
}

/// مدخل ذاكرة التحليل
#[derive(Clone)]
pub struct AnalysisCacheEntry {
    pub hash: u64,
    pub timestamp: std::time::Instant,
}

impl GlobalCache {
    pub fn new() -> Self {
        Self {
            completion_cache: Mutex::new(LruCache::new(NonZeroUsize::new(1000).unwrap())),
            analysis_cache: Mutex::new(LruCache::new(NonZeroUsize::new(100).unwrap())),
            docs_cache: Mutex::new(LruCache::new(NonZeroUsize::new(500).unwrap())),
        }
    }
    
    /// إضافة إلى ذاكرة الإكمال
    pub fn put_completion(&self, key: String, value: Vec<String>) {
        self.completion_cache.lock().put(key, value);
    }
    
    /// الحصول من ذاكرة الإكمال
    pub fn get_completion(&self, key: &str) -> Option<Vec<String>> {
        self.completion_cache.lock().get(key).cloned()
    }
    
    /// إضافة إلى ذاكرة التحليل
    pub fn put_analysis(&self, key: String, hash: u64) {
        self.analysis_cache.lock().put(key, AnalysisCacheEntry {
            hash,
            timestamp: std::time::Instant::now(),
        });
    }
    
    /// الحصول من ذاكرة التحليل
    pub fn get_analysis(&self, key: &str) -> Option<(u64, std::time::Instant)> {
        self.analysis_cache.lock().get(key).map(|e| (e.hash, e.timestamp))
    }
    
    /// التحقق من صلاحية التحليل
    pub fn is_analysis_valid(&self, key: &str, current_hash: u64) -> bool {
        if let Some((hash, timestamp)) = self.get_analysis(key) {
            // التحليل صالح إذا تطابق الهاش ولم يمضِ عليه وقت طويل
            hash == current_hash && timestamp.elapsed().as_secs() < 300 // 5 دقائق
        } else {
            false
        }
    }
    
    /// إضافة توثيق
    pub fn put_doc(&self, key: String, doc: String) {
        self.docs_cache.lock().put(key, doc);
    }
    
    /// الحصول على توثيق
    pub fn get_doc(&self, key: &str) -> Option<String> {
        self.docs_cache.lock().get(key).cloned()
    }
    
    /// مسح كل الذاكرات
    pub fn clear_all(&self) {
        self.completion_cache.lock().clear();
        self.analysis_cache.lock().clear();
        self.docs_cache.lock().clear();
    }
    
    /// الحصول على إحصائيات
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            completion_entries: self.completion_cache.lock().len(),
            analysis_entries: self.analysis_cache.lock().len(),
            docs_entries: self.docs_cache.lock().len(),
        }
    }
}

impl Default for GlobalCache {
    fn default() -> Self {
        Self::new()
    }
}

/// إحصائيات الذاكرة المؤقتة
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub completion_entries: usize,
    pub analysis_entries: usize,
    pub docs_entries: usize,
}
