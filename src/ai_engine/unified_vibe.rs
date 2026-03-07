// ═══════════════════════════════════════════════════════════════════════════════
// محرك Vibe Coding الموحد - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// نظام متكامل يجمع بين:
// - مطابقة الأنماط السريعة (VibeCodingEngine)
// - نماذج GGUF للذكاء الاصطناعي الحقيقي
// - نظام Fallback ذكي للتحويل التلقائي
// - واجهة برمجة موحدة للجميع
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use serde::{Deserialize, Serialize};

use super::vibe_advanced::{
    VibeCodingEngine, VibeResult, DetectedIntent, IntentType,
    SemanticAnalysis, ConfidenceLevel, ExecutionContext,
};
use super::gguf_inference::{GGUFEngine, GGUFConfig, GGUFResult};
use super::real_inference::{RealAIEngine, RealModelConfig, RealInferenceResult};

// ═══════════════════════════════════════════════════════════════════════════════
// الثوابت والتعدادات
// ═══════════════════════════════════════════════════════════════════════════════

/// الحد الأقصى لحجم الكاش
const MAX_CACHE_SIZE: usize = 500;
/// عتبة استخدام GGUF (للنصوص المعقدة)
const GGUF_COMPLEXITY_THRESHOLD: f64 = 0.7;
/// عتبة الثقة لاستخدام مطابقة الأنماط
const PATTERN_CONFIDENCE_THRESHOLD: f64 = 0.85;
/// مهلة GGUF بالمللي ثانية
const GGUF_TIMEOUT_MS: u64 = 30000;

/// حالة المحرك
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EngineState {
    /// غير محمّل
    Uninitialized,
    /// يستخدم مطابقة الأنماط فقط
    PatternOnly,
    /// يستخدم GGUF متاح
    GGUFReady,
    /// وضع هجين (أنماط + GGUF)
    Hybrid,
    /// خطأ
    Error,
}

/// نوع المحرك المستخدم
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EngineType {
    /// مطابقة الأنماط السريعة
    PatternMatching,
    /// نموذج GGUF
    GGUFModel,
    /// محرك AI حقيقي
    RealAI,
    /// محاكاة (fallback)
    Simulation,
}

/// نتيجة موحدة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedVibeResult {
    /// النص الأصلي
    pub input: String,
    /// الكود المُنتج
    pub code: String,
    /// الشرح
    pub explanation: String,
    /// النوايا المُكتشفة
    pub intents: Vec<DetectedIntent>,
    /// مستوى الثقة
    pub confidence: f64,
    /// المحرك المستخدم
    pub engine_used: EngineType,
    /// وقت المعالجة بالمللي ثانية
    pub processing_time_ms: u64,
    /// نجح أم لا
    pub success: bool,
    /// اقتراحات التحسين
    pub improvements: Vec<String>,
    /// معلومات إضافية
    pub metadata: HashMap<String, String>,
}

/// إعدادات المحرك الموحد
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedVibeConfig {
    /// تفعيل GGUF
    pub enable_gguf: bool,
    /// تفعيل مطابقة الأنماط
    pub enable_patterns: bool,
    /// تفعيل التخزين المؤقت
    pub enable_cache: bool,
    /// عتبة التعقيد لاستخدام GGUF
    pub complexity_threshold: f64,
    /// عتبة الثقة لاستخدام الأنماط
    pub pattern_confidence_threshold: f64,
    /// مهلة GGUF
    pub gguf_timeout_ms: u64,
    /// مسار نموذج GGUF
    pub gguf_model_path: Option<String>,
    /// درجة حرارة GGUF
    pub gguf_temperature: f32,
    /// الحد الأقصى للتوكنات
    pub max_tokens: usize,
}

impl Default for UnifiedVibeConfig {
    fn default() -> Self {
        Self {
            enable_gguf: true,
            enable_patterns: true,
            enable_cache: true,
            complexity_threshold: GGUF_COMPLEXITY_THRESHOLD,
            pattern_confidence_threshold: PATTERN_CONFIDENCE_THRESHOLD,
            gguf_timeout_ms: GGUF_TIMEOUT_MS,
            gguf_model_path: Some("models/qwen2.5-0.5b-instruct-q8_0.gguf".to_string()),
            gguf_temperature: 0.7,
            max_tokens: 256,
        }
    }
}

/// إحصائيات المحرك الموحد
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnifiedStats {
    /// إجمالي الطلبات
    pub total_requests: u64,
    /// الطلبات الناجحة
    pub successful_requests: u64,
    /// استخدام الأنماط
    pub pattern_requests: u64,
    /// استخدام GGUF
    pub gguf_requests: u64,
    /// استخدام المحاكاة
    pub simulation_requests: u64,
    /// إصابات الكاش
    pub cache_hits: u64,
    /// متوسط وقت المعالجة
    pub avg_processing_time_ms: f64,
    /// متوسط الثقة
    pub avg_confidence: f64,
}

/// عنصر الكاش
#[derive(Debug, Clone)]
struct CacheEntry {
    result: UnifiedVibeResult,
    timestamp: Instant,
    hits: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// المحرك الموحد - UnifiedVibeEngine
// ═══════════════════════════════════════════════════════════════════════════════

/// محرك Vibe Coding الموحد
pub struct UnifiedVibeEngine {
    /// محرك مطابقة الأنماط
    pattern_engine: VibeCodingEngine,
    /// محرك GGUF (اختياري)
    gguf_engine: Option<GGUFEngine>,
    /// محرك AI حقيقي (اختياري)
    real_engine: Option<RealAIEngine>,
    /// الإعدادات
    config: UnifiedVibeConfig,
    /// الحالة
    state: EngineState,
    /// الكاش
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
    /// الإحصائيات
    stats: UnifiedStats,
    /// آخر خطأ
    last_error: Option<String>,
}

impl UnifiedVibeEngine {
    /// إنشاء محرك جديد
    pub fn new() -> Self {
        Self {
            pattern_engine: VibeCodingEngine::new(),
            gguf_engine: None,
            real_engine: None,
            config: UnifiedVibeConfig::default(),
            state: EngineState::Uninitialized,
            cache: Arc::new(Mutex::new(HashMap::new())),
            stats: UnifiedStats::default(),
            last_error: None,
        }
    }

    /// إنشاء محرك مع إعدادات مخصصة
    pub fn with_config(config: UnifiedVibeConfig) -> Self {
        Self {
            pattern_engine: VibeCodingEngine::new(),
            gguf_engine: None,
            real_engine: None,
            config,
            state: EngineState::Uninitialized,
            cache: Arc::new(Mutex::new(HashMap::new())),
            stats: UnifiedStats::default(),
            last_error: None,
        }
    }

    /// تهيئة المحرك
    pub fn initialize(&mut self) -> Result<EngineState, String> {
        println!("\n🚀 تهيئة محرك Vibe Coding الموحد...");
        
        // تهيئة محرك الأنماط (دائماً متاح)
        if self.config.enable_patterns {
            println!("   ✅ محرك مطابقة الأنماط جاهز");
        }

        // محاولة تهيئة GGUF
        if self.config.enable_gguf {
            if let Some(model_path) = &self.config.gguf_model_path {
                let gguf_config = GGUFConfig {
                    model_path: std::path::PathBuf::from(model_path),
                    temperature: self.config.gguf_temperature,
                    max_tokens: self.config.max_tokens,
                    ..Default::default()
                };
                
                let mut gguf_engine = GGUFEngine::with_config(gguf_config);
                match gguf_engine.load() {
                    Ok(_) => {
                        println!("   ✅ محرك GGUF جاهز");
                        self.gguf_engine = Some(gguf_engine);
                        self.state = EngineState::Hybrid;
                    }
                    Err(e) => {
                        println!("   ⚠️ GGUF غير متاح: {}", e);
                        self.state = EngineState::PatternOnly;
                    }
                }
            } else {
                self.state = EngineState::PatternOnly;
            }
        } else {
            self.state = EngineState::PatternOnly;
        }

        // محاولة تهيئة RealAI Engine
        if self.config.enable_gguf && self.gguf_engine.is_none() {
            let mut real_engine = RealAIEngine::new();
            match real_engine.load() {
                Ok(_) => {
                    println!("   ✅ محرك Real AI جاهز");
                    self.real_engine = Some(real_engine);
                }
                Err(_) => {
                    println!("   ⚠️ Real AI غير متاح");
                }
            }
        }

        // تحديث الحالة
        if self.gguf_engine.is_some() || self.real_engine.is_some() {
            self.state = EngineState::Hybrid;
        } else if self.config.enable_patterns {
            self.state = EngineState::PatternOnly;
        } else {
            self.state = EngineState::Error;
            return Err("لا يوجد محرك متاح".to_string());
        }

        println!("   📊 الحالة: {:?}\n", self.state);
        Ok(self.state)
    }

    /// معالجة نص Vibe Coding
    pub fn process(&mut self, text: &str) -> UnifiedVibeResult {
        let start = Instant::now();
        self.stats.total_requests += 1;

        // التحقق من الكاش
        if self.config.enable_cache {
            if let Some(cached) = self.check_cache(text) {
                self.stats.cache_hits += 1;
                return cached;
            }
        }

        // تحليل التعقيد
        let analysis = self.pattern_engine.analyze_semantics(text);
        let complexity = self.calculate_complexity(&analysis);
        
        // اختيار المحرك المناسب
        let engine_type = self.select_engine(text, complexity, &analysis);
        
        // المعالجة
        let result = match engine_type {
            EngineType::PatternMatching => self.process_with_patterns(text),
            EngineType::GGUFModel => self.process_with_gguf(text),
            EngineType::RealAI => self.process_with_real_ai(text),
            EngineType::Simulation => self.process_with_simulation(text),
        };

        let processing_time = start.elapsed().as_millis() as u64;
        
        // تحديث النتيجة
        let mut final_result = result;
        final_result.processing_time_ms = processing_time;
        final_result.engine_used = engine_type;

        // تحديث الإحصائيات
        self.update_stats(&final_result);

        // تخزين في الكاش
        if self.config.enable_cache {
            self.store_cache(text, final_result.clone());
        }

        final_result
    }

    /// حساب التعقيد
    fn calculate_complexity(&self, analysis: &SemanticAnalysis) -> f64 {
        let entity_factor = analysis.entities.len() as f64 / 10.0;
        let relation_factor = analysis.relations.len() as f64 / 5.0;
        let structure_factor = match analysis.structure.sentence_type {
            super::vibe_advanced::SentenceType::Imperative => 0.2,
            super::vibe_advanced::SentenceType::Declarative => 0.3,
            super::vibe_advanced::SentenceType::Conditional => 0.5,
            super::vibe_advanced::SentenceType::Iterative => 0.6,
            super::vibe_advanced::SentenceType::Interrogative => 0.4,
            super::vibe_advanced::SentenceType::Compound => 0.8,
        };

        let complexity_score = (entity_factor + relation_factor + structure_factor) / 3.0;
        complexity_score.min(1.0).max(0.0)
    }

    /// اختيار المحرك المناسب
    fn select_engine(&self, text: &str, complexity: f64, _analysis: &SemanticAnalysis) -> EngineType {
        // للنصوص البسيطة والثقة العالية، استخدم الأنماط
        if complexity < self.config.complexity_threshold {
            let intents = self.pattern_engine.detect_intents(text);
            let avg_confidence = if !intents.is_empty() {
                intents.iter().map(|i| i.confidence).sum::<f64>() / intents.len() as f64
            } else {
                0.0
            };

            if avg_confidence >= self.config.pattern_confidence_threshold {
                return EngineType::PatternMatching;
            }
        }

        // للنصوص المعقدة أو الثقة المنخفضة، استخدم GGUF
        if self.gguf_engine.is_some() {
            return EngineType::GGUFModel;
        }

        // محاولة Real AI
        if self.real_engine.is_some() {
            return EngineType::RealAI;
        }

        // Fallback للأنماط
        EngineType::PatternMatching
    }

    /// المعالجة بمحرك الأنماط
    fn process_with_patterns(&mut self, text: &str) -> UnifiedVibeResult {
        self.stats.pattern_requests += 1;
        
        let vibe_result = self.pattern_engine.process(text);
        
        UnifiedVibeResult {
            input: text.to_string(),
            code: vibe_result.code,
            explanation: vibe_result.explanation,
            intents: vibe_result.intents,
            confidence: vibe_result.overall_confidence,
            engine_used: EngineType::PatternMatching,
            processing_time_ms: 0,
            success: vibe_result.success,
            improvements: vibe_result.improvements,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("method".to_string(), "pattern_matching".to_string());
                meta.insert("complexity".to_string(), "simple".to_string());
                meta
            },
        }
    }

    /// المعالجة بـ GGUF
    fn process_with_gguf(&mut self, text: &str) -> UnifiedVibeResult {
        self.stats.gguf_requests += 1;
        
        if let Some(ref engine) = self.gguf_engine {
            match engine.infer(text) {
                Ok(gguf_result) => {
                    let code = gguf_result.text;
                    
                    // تحليل النتيجة
                    let intents = self.pattern_engine.detect_intents(text);
                    let confidence = if intents.is_empty() { 0.7 } else {
                        intents.iter().map(|i| i.confidence).sum::<f64>() / intents.len() as f64
                    };
                    
                    return UnifiedVibeResult {
                        input: text.to_string(),
                        code,
                        explanation: "تم توليد الكود باستخدام نموذج الذكاء الاصطناعي".to_string(),
                        intents,
                        confidence,
                        engine_used: EngineType::GGUFModel,
                        processing_time_ms: gguf_result.duration_ms,
                        success: true,
                        improvements: vec![
                            "الكود مُنتج من نموذج AI - راجعه للتأكد من صحته".to_string()
                        ],
                        metadata: {
                            let mut meta = HashMap::new();
                            meta.insert("method".to_string(), "gguf_model".to_string());
                            meta.insert("tokens_generated".to_string(), gguf_result.tokens_generated.to_string());
                            meta.insert("tokens_per_second".to_string(), format!("{:.2}", gguf_result.tokens_per_second));
                            meta
                        },
                    };
                }
                Err(e) => {
                    self.last_error = Some(e.clone());
                    println!("   ⚠️ خطأ GGUF: {} - استخدام الأنماط", e);
                }
            }
        }
        
        // Fallback للأنماط
        self.process_with_patterns(text)
    }

    /// المعالجة بـ Real AI
    fn process_with_real_ai(&mut self, text: &str) -> UnifiedVibeResult {
        if let Some(ref mut engine) = self.real_engine {
            match engine.infer(text) {
                Ok(result) => {
                    let intents = self.pattern_engine.detect_intents(text);
                    let confidence = if intents.is_empty() { 0.7 } else {
                        intents.iter().map(|i| i.confidence).sum::<f64>() / intents.len() as f64
                    };
                    
                    return UnifiedVibeResult {
                        input: text.to_string(),
                        code: result.text,
                        explanation: "تم توليد الكود باستخدام محرك AI حقيقي".to_string(),
                        intents,
                        confidence,
                        engine_used: EngineType::RealAI,
                        processing_time_ms: result.duration_ms,
                        success: true,
                        improvements: vec![],
                        metadata: {
                            let mut meta = HashMap::new();
                            meta.insert("method".to_string(), "real_ai".to_string());
                            meta.insert("model_used".to_string(), result.model_used);
                            meta
                        },
                    };
                }
                Err(e) => {
                    self.last_error = Some(e.clone());
                }
            }
        }
        
        // Fallback للأنماط
        self.process_with_patterns(text)
    }

    /// المعالجة بالمحاكاة
    fn process_with_simulation(&mut self, text: &str) -> UnifiedVibeResult {
        self.stats.simulation_requests += 1;
        
        // استخدام محرك الأنماط كمحاكاة
        let mut result = self.process_with_patterns(text);
        result.engine_used = EngineType::Simulation;
        result.metadata.insert("method".to_string(), "simulation".to_string());
        result
    }

    /// التحقق من الكاش
    fn check_cache(&self, text: &str) -> Option<UnifiedVibeResult> {
        let cache = self.cache.lock().unwrap();
        if let Some(entry) = cache.get(&text.to_lowercase()) {
            let mut result = entry.result.clone();
            result.metadata.insert("cache_hit".to_string(), "true".to_string());
            return Some(result);
        }
        None
    }

    /// تخزين في الكاش
    fn store_cache(&self, text: &str, result: UnifiedVibeResult) {
        let mut cache = self.cache.lock().unwrap();
        
        // إزالة العناصر القديمة إذا امتلأ الكاش
        if cache.len() >= MAX_CACHE_SIZE {
            // إزالة العناصر الأقل استخداماً
            let mut entries: Vec<_> = cache.iter()
                .map(|(k, v)| (k.clone(), v.hits, v.timestamp))
                .collect();
            entries.sort_by(|a, b| {
                a.1.cmp(&b.1).then(a.2.cmp(&b.2))
            });
            
            // إزالة 10% من العناصر
            let to_remove = entries.iter()
                .take(MAX_CACHE_SIZE / 10)
                .map(|(k, _, _)| k.clone())
                .collect::<Vec<_>>();
            
            for key in to_remove {
                cache.remove(&key);
            }
        }
        
        cache.insert(text.to_lowercase(), CacheEntry {
            result,
            timestamp: Instant::now(),
            hits: 0,
        });
    }

    /// تحديث الإحصائيات
    fn update_stats(&mut self, result: &UnifiedVibeResult) {
        if result.success {
            self.stats.successful_requests += 1;
        }
        
        let n = self.stats.total_requests as f64;
        self.stats.avg_processing_time_ms = 
            (self.stats.avg_processing_time_ms * (n - 1.0) + result.processing_time_ms as f64) / n;
        self.stats.avg_confidence = 
            (self.stats.avg_confidence * (n - 1.0) + result.confidence) / n;
    }

    // ═══════════════════════════════════════════════════════════════
    // دوال إضافية
    // ═══════════════════════════════════════════════════════════════

    /// الحصول على الحالة
    pub fn get_state(&self) -> EngineState {
        self.state
    }

    /// الحصول على الإحصائيات
    pub fn get_stats(&self) -> &UnifiedStats {
        &self.stats
    }

    /// الحصول على آخر خطأ
    pub fn get_last_error(&self) -> Option<&String> {
        self.last_error.as_ref()
    }

    /// مسح الكاش
    pub fn clear_cache(&self) {
        self.cache.lock().unwrap().clear();
    }

    /// تحديث الإعدادات
    pub fn update_config(&mut self, config: UnifiedVibeConfig) {
        self.config = config;
    }

    /// تفعيل/تعطيل GGUF
    pub fn set_gguf_enabled(&mut self, enabled: bool) {
        self.config.enable_gguf = enabled;
    }

    /// تعيين درجة الحرارة
    pub fn set_temperature(&mut self, temp: f32) {
        self.config.gguf_temperature = temp;
    }

    /// شرح كود
    pub fn explain_code(&self, code: &str) -> String {
        self.pattern_engine.explain_code(code)
    }

    /// اقتراح إصلاح
    pub fn suggest_fix(&self, error_code: &str, error_message: &str) -> Vec<String> {
        self.pattern_engine.suggest_fix(error_code, error_message)
    }

    /// إكمال ذكي
    pub fn smart_completion(&self, partial: &str) -> Vec<String> {
        self.pattern_engine.get_smart_completion(partial)
    }

    /// معالجة دفعة من النصوص
    pub fn process_batch(&mut self, texts: &[&str]) -> Vec<UnifiedVibeResult> {
        texts.iter().map(|t| self.process(t)).collect()
    }

    /// تحويل نص إلى كود (دالة سهلة)
    pub fn text_to_code(&mut self, text: &str) -> String {
        let result = self.process(text);
        result.code
    }

    /// تقرير الحالة
    pub fn status_report(&self) -> String {
        let stats = &self.stats;
        let success_rate = if stats.total_requests > 0 {
            (stats.successful_requests as f64 / stats.total_requests as f64) * 100.0
        } else {
            0.0
        };

        let pattern_rate = if stats.total_requests > 0 {
            (stats.pattern_requests as f64 / stats.total_requests as f64) * 100.0
        } else {
            0.0
        };

        let gguf_rate = if stats.total_requests > 0 {
            (stats.gguf_requests as f64 / stats.total_requests as f64) * 100.0
        } else {
            0.0
        };

        format!(
            r#"╔══════════════════════════════════════════════════════════════╗
║             📊 تقرير محرك Vibe Coding الموحد                ║
╠══════════════════════════════════════════════════════════════╣
║ الحالة: {:?}
║ ─────────────────────────────────────────────────────────────
║ 📈 الإحصائيات:
║    • إجمالي الطلبات: {}
║    • الطلبات الناجحة: {} ({:.1}%)
║    • إصابات الكاش: {}
║    • متوسط الثقة: {:.2}%
║    • متوسط الوقت: {:.2}ms
║ ─────────────────────────────────────────────────────────────
║ 🔧 استخدام المحركات:
║    • مطابقة الأنماط: {} ({:.1}%)
║    • GGUF: {} ({:.1}%)
║    • محاكاة: {}
║ ─────────────────────────────────────────────────────────────
║ ⚙️ الإعدادات:
║    • GGUF مفعّل: {}
║    • الكاش مفعّل: {}
║    • عتبة التعقيد: {:.2}
║    • عتبة الثقة: {:.2}
╚══════════════════════════════════════════════════════════════╝"#,
            self.state,
            stats.total_requests,
            stats.successful_requests,
            success_rate,
            stats.cache_hits,
            stats.avg_confidence * 100.0,
            stats.avg_processing_time_ms,
            stats.pattern_requests,
            pattern_rate,
            stats.gguf_requests,
            gguf_rate,
            stats.simulation_requests,
            self.config.enable_gguf,
            self.config.enable_cache,
            self.config.complexity_threshold,
            self.config.pattern_confidence_threshold,
        )
    }
}

impl Default for UnifiedVibeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال سهلة الاستخدام
// ═══════════════════════════════════════════════════════════════════════════════

/// معالجة نص Vibe Coding (دالة سهلة)
pub fn unified_vibe_process(text: &str) -> UnifiedVibeResult {
    let mut engine = UnifiedVibeEngine::new();
    let _ = engine.initialize();
    engine.process(text)
}

/// تحويل نص إلى كود (دالة سهلة)
pub fn unified_text_to_code(text: &str) -> String {
    let mut engine = UnifiedVibeEngine::new();
    let _ = engine.initialize();
    engine.text_to_code(text)
}

/// معالجة دفعة من النصوص
pub fn unified_process_batch(texts: &[&str]) -> Vec<UnifiedVibeResult> {
    let mut engine = UnifiedVibeEngine::new();
    let _ = engine.initialize();
    engine.process_batch(texts)
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_engine_creation() {
        let engine = UnifiedVibeEngine::new();
        assert_eq!(engine.state, EngineState::Uninitialized);
    }

    #[test]
    fn test_unified_engine_initialization() {
        let mut engine = UnifiedVibeEngine::new();
        let result = engine.initialize();
        assert!(result.is_ok());
        assert_ne!(engine.state, EngineState::Uninitialized);
    }

    #[test]
    fn test_process_simple_text() {
        let mut engine = UnifiedVibeEngine::new();
        engine.initialize().unwrap();
        
        let result = engine.process("اطبع مرحبا");
        assert!(result.success);
        assert!(result.code.contains("اطبع"));
    }

    #[test]
    fn test_process_variable() {
        let mut engine = UnifiedVibeEngine::new();
        engine.initialize().unwrap();
        
        let result = engine.process("أنشئ متغير س يساوي 10");
        assert!(result.success);
        assert!(result.code.contains("متغير"));
    }

    #[test]
    fn test_process_function() {
        let mut engine = UnifiedVibeEngine::new();
        engine.initialize().unwrap();
        
        let result = engine.process("أنشئ دالة تجمع رقمين");
        assert!(result.success);
        assert!(result.code.contains("دالة"));
    }

    #[test]
    fn test_process_loop() {
        let mut engine = UnifiedVibeEngine::new();
        engine.initialize().unwrap();
        
        let result = engine.process("كرر 5 مرات اطبع مرحبا");
        assert!(result.success);
        assert!(result.code.contains("طالما"));
    }

    #[test]
    fn test_cache_functionality() {
        let mut engine = UnifiedVibeEngine::new();
        engine.initialize().unwrap();
        
        // معالجة مرة
        let result1 = engine.process("اطبع اختبار الكاش");
        
        // معالجة مرة أخرى (يجب أن تصيب الكاش)
        let result2 = engine.process("اطبع اختبار الكاش");
        
        assert_eq!(result1.code, result2.code);
        assert!(engine.stats.cache_hits > 0);
    }

    #[test]
    fn test_batch_processing() {
        let mut engine = UnifiedVibeEngine::new();
        engine.initialize().unwrap();
        
        let texts = vec!["اطبع أ", "اطبع ب", "اطبع ج"];
        let results = engine.process_batch(&texts);
        
        assert_eq!(results.len(), 3);
        for result in results {
            assert!(result.success);
        }
    }

    #[test]
    fn test_stats_tracking() {
        let mut engine = UnifiedVibeEngine::new();
        engine.initialize().unwrap();
        
        engine.process("اطبع أ");
        engine.process("أنشئ متغير س");
        engine.process("كرر 3 مرات");
        
        assert_eq!(engine.stats.total_requests, 3);
        assert!(engine.stats.successful_requests > 0);
    }

    #[test]
    fn test_config_modification() {
        let mut engine = UnifiedVibeEngine::new();
        
        engine.set_gguf_enabled(false);
        assert!(!engine.config.enable_gguf);
        
        engine.set_temperature(0.5);
        assert_eq!(engine.config.gguf_temperature, 0.5);
    }

    #[test]
    fn test_status_report() {
        let mut engine = UnifiedVibeEngine::new();
        engine.initialize().unwrap();
        engine.process("اطبع اختبار");
        
        let report = engine.status_report();
        assert!(report.contains("تقرير"));
        assert!(report.contains("الطلبات"));
    }

    #[test]
    fn test_text_to_code() {
        let mut engine = UnifiedVibeEngine::new();
        engine.initialize().unwrap();
        
        let code = engine.text_to_code("اطبع مرحبا بالعالم");
        assert!(code.contains("اطبع"));
    }

    #[test]
    fn test_explain_code() {
        let engine = UnifiedVibeEngine::new();
        let explanation = engine.explain_code("متغير س = 5؛");
        assert!(explanation.contains("متغير"));
    }

    #[test]
    fn test_smart_completion() {
        let engine = UnifiedVibeEngine::new();
        let completions = engine.smart_completion("متغير");
        assert!(!completions.is_empty());
    }
}
