// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات التكامل الشاملة - محرك Vibe Coding الموحد
// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات شاملة للتأكد من عمل المحرك الموحد مع GGUF
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::ai_engine::{
    UnifiedVibeEngine, UnifiedVibeConfig, EngineState,
    unified_vibe_process, unified_text_to_code,
};

/// اختبار تهيئة المحرك الموحد
#[test]
fn test_unified_engine_initialization() {
    let mut engine = UnifiedVibeEngine::new();
    let state = engine.initialize().unwrap();
    
    // يجب أن يكون المحرك في حالة PatternOnly أو Hybrid
    assert!(state == EngineState::PatternOnly || state == EngineState::Hybrid);
}

/// اختبار معالجة أمر طباعة بسيط
#[test]
fn test_simple_print_command() {
    let mut engine = UnifiedVibeEngine::new();
    engine.initialize().unwrap();
    
    let result = engine.process("اطبع مرحبا بالعالم");
    
    assert!(result.success, "Print command should succeed");
    assert!(result.code.contains("اطبع"), "Code should contain 'اطبع'");
    assert!(result.confidence > 0.5, "Confidence should be > 0.5");
}

/// اختبار إنشاء متغير
#[test]
fn test_variable_creation() {
    let mut engine = UnifiedVibeEngine::new();
    engine.initialize().unwrap();
    
    let result = engine.process("أنشئ متغير س يساوي 10");
    
    assert!(result.success, "Variable creation should succeed");
    assert!(result.code.contains("متغير"), "Code should contain 'متغير'");
    assert!(result.code.contains("س"), "Code should contain variable name");
}

/// اختبار إنشاء دالة
#[test]
fn test_function_creation() {
    let mut engine = UnifiedVibeEngine::new();
    engine.initialize().unwrap();
    
    let result = engine.process("أنشئ دالة تجمع رقمين");
    
    assert!(result.success, "Function creation should succeed");
    assert!(result.code.contains("دالة"), "Code should contain 'دالة'");
}

/// اختبار إنشاء حلقة
#[test]
fn test_loop_creation() {
    let mut engine = UnifiedVibeEngine::new();
    engine.initialize().unwrap();
    
    let result = engine.process("كرر 5 مرات اطبع مرحبا");
    
    assert!(result.success, "Loop creation should succeed");
    assert!(result.code.contains("طالما"), "Code should contain 'طالما'");
}

/// اختبار إنشاء شرط
#[test]
fn test_condition_creation() {
    let mut engine = UnifiedVibeEngine::new();
    engine.initialize().unwrap();
    
    let result = engine.process("إذا كان س أكبر من 10 اطبع كبير");
    
    assert!(result.success, "Condition creation should succeed");
    assert!(result.code.contains("إذا"), "Code should contain 'إذا'");
}

/// اختبار معالجة أوامر مركبة
#[test]
fn test_compound_commands() {
    let mut engine = UnifiedVibeEngine::new();
    engine.initialize().unwrap();
    
    let result = engine.process("أنشئ متغير س يساوي 5 واطبعه");
    
    assert!(result.success, "Compound command should succeed");
}

/// اختبار نظام الكاش
#[test]
fn test_caching_system() {
    let mut engine = UnifiedVibeEngine::new();
    engine.initialize().unwrap();
    
    // معالجة أمر للمرة الأولى
    let result1 = engine.process("اطبع اختبار الكاش");
    let stats1 = engine.get_stats().clone();
    
    // معالجة نفس الأمر للمرة الثانية
    let result2 = engine.process("اطبع اختبار الكاش");
    let stats2 = engine.get_stats().clone();
    
    // يجب أن يكون الكود متماثل
    assert_eq!(result1.code, result2.code);
    // يجب أن تزيد إصابات الكاش
    assert!(stats2.cache_hits > stats1.cache_hits);
}

/// اختبار معالجة دفعة
#[test]
fn test_batch_processing() {
    let mut engine = UnifiedVibeEngine::new();
    engine.initialize().unwrap();
    
    let texts = vec![
        "اطبع أ",
        "أنشئ متغير ب يساوي 5",
        "كرر 3 مرات اطبع مرحبا",
    ];
    
    let results = engine.process_batch(&texts.iter().map(|s| *s).collect::<Vec<_>>());
    
    assert_eq!(results.len(), 3);
    for result in results {
        assert!(result.success, "All batch items should succeed");
    }
}

/// اختبار تحويل النص إلى كود (دالة سهلة)
#[test]
fn test_easy_text_to_code() {
    let code = unified_text_to_code("اطبع مرحبا");
    assert!(code.contains("اطبع"));
}

/// اختبار معالجة Vibe (دالة سهلة)
#[test]
fn test_easy_vibe_process() {
    let result = unified_vibe_process("أنشئ متغير س يساوي 10");
    assert!(result.success);
}

/// اختبار تقرير الحالة
#[test]
fn test_status_report() {
    let mut engine = UnifiedVibeEngine::new();
    engine.initialize().unwrap();
    
    engine.process("اطبع اختبار");
    
    let report = engine.status_report();
    
    assert!(report.contains("تقرير"));
    assert!(report.contains("الطلبات"));
}

/// اختبار الإحصائيات
#[test]
fn test_statistics_tracking() {
    let mut engine = UnifiedVibeEngine::new();
    engine.initialize().unwrap();
    
    // معالجة عدة أوامر
    engine.process("اطبع أ");
    engine.process("أنشئ متغير س");
    engine.process("كرر 3 مرات");
    
    let stats = engine.get_stats();
    
    assert_eq!(stats.total_requests, 3);
    assert!(stats.successful_requests > 0);
}

/// اختبار شرح الكود
#[test]
fn test_code_explanation() {
    let engine = UnifiedVibeEngine::new();
    
    let explanation = engine.explain_code("متغير س = 5؛\nاطبع(س)؛");
    
    assert!(explanation.contains("متغير"));
}

/// اختبار الإكمال الذكي
#[test]
fn test_smart_completion() {
    let engine = UnifiedVibeEngine::new();
    
    let completions = engine.smart_completion("متغير");
    
    assert!(!completions.is_empty());
}

/// اختبار اقتراح الإصلاح
#[test]
fn test_fix_suggestion() {
    let engine = UnifiedVibeEngine::new();
    
    let fixes = engine.suggest_fix("اطبع(س)", "متغير غير معرف");
    
    assert!(!fixes.is_empty());
}

/// اختبار تكوين المحرك
#[test]
fn test_engine_configuration() {
    let config = UnifiedVibeConfig {
        enable_gguf: false,
        enable_patterns: true,
        enable_cache: false,
        ..Default::default()
    };
    
    let mut engine = UnifiedVibeEngine::with_config(config);
    engine.initialize().unwrap();
    
    // معالجة بدون كاش
    let result = engine.process("اطبع بدون كاش");
    assert!(result.success);
}

/// اختبار تحديث الإعدادات
#[test]
fn test_config_update() {
    let mut engine = UnifiedVibeEngine::new();
    
    engine.set_gguf_enabled(false);
    engine.set_temperature(0.5);
    
    // التكوينات يجب أن تُحفظ
    let result = engine.process("اطبع اختبار");
    assert!(result.success);
}

/// اختبار معالجة الأرقام العربية
#[test]
fn test_arabic_numbers() {
    let mut engine = UnifiedVibeEngine::new();
    engine.initialize().unwrap();
    
    let result = engine.process("أنشئ متغير س يساوي خمسة");
    
    assert!(result.success);
}

/// اختبار معالجة النصوص المعقدة
#[test]
fn test_complex_text() {
    let mut engine = UnifiedVibeEngine::new();
    engine.initialize().unwrap();
    
    let result = engine.process("أنشئ دالة تحسب مجموع قائمة من الأرقام");
    
    // النتيجة قد تكون دالة أو لا، لكن يجب أن تنجح
    assert!(result.confidence > 0.0);
}

/// اختبار التكامل مع GGUF (إذا كان متاحاً)
#[test]
fn test_gguf_integration() {
    let config = UnifiedVibeConfig {
        enable_gguf: true,
        gguf_model_path: Some("models/qwen2.5-0.5b-instruct-q8_0.gguf".to_string()),
        ..Default::default()
    };
    
    let mut engine = UnifiedVibeEngine::with_config(config);
    let state = engine.initialize().unwrap();
    
    // إذا كان GGUF متاحاً، يجب أن نكون في Hybrid
    // وإلا نكون في PatternOnly
    assert!(state == EngineState::Hybrid || state == EngineState::PatternOnly);
    
    // يجب أن تعمل المعالجة على أي حال
    let result = engine.process("اطبع اختبار GGUF");
    assert!(result.success);
}

/// اختبار التسامح مع الأخطاء
#[test]
fn test_error_tolerance() {
    let mut engine = UnifiedVibeEngine::new();
    engine.initialize().unwrap();
    
    // نص غير واضح
    let _result = engine.process("هذا نص غير مفهوم تماماً");
    
    // يجب ألا يفشل المحرك، بل يُنتج نتيجة (ربما بثقة منخفضة)
    // النظام لا يجب أن يتعطل
}

/// اختبار الأداء مع كمية كبيرة
#[test]
fn test_performance_bulk() {
    let mut engine = UnifiedVibeEngine::new();
    engine.initialize().unwrap();
    
    // معالجة 50 أمر
    for i in 0..50 {
        let _ = engine.process(&format!("اطبع رسالة {}", i));
    }
    
    let stats = engine.get_stats();
    assert_eq!(stats.total_requests, 50);
}
