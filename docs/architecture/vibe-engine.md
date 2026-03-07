# نظام Vibe Coding الموحد - لغة المرجع

## نظرة عامة

نظام Vibe Coding الموحد هو محرك متكامل يجمع بين:
- **مطابقة الأنماط السريعة** (VibeCodingEngine)
- **نماذج الذكاء الاصطناعي GGUF** (GGUFEngine)
- **نظام Fallback ذكي** للتحويل التلقائي بين المحركات
- **واجهة برمجة موحدة** للاستخدام السهل

## الميزات الرئيسية

### 1. التكامل مع GGUF
- دعم نماذج GGUF عبر llama.cpp server/cli
- تكوين مرن (درجة الحرارة، الحد الأقصى للتوكنات، إلخ)
- Fallback تلقائي عند عدم توفر GGUF

### 2. نظام Fallback الذكي
- للنصوص البسيطة: استخدام مطابقة الأنماط (سريع ودقيق)
- للنصوص المعقدة: استخدام GGUF (قوي ومرن)
- عند فشل GGUF: الرجوع للأنماط تلقائياً

### 3. التخزين المؤقت (Caching)
- كاش LRU ذكي للنتائج
- تحسين الأداء للطلبات المتكررة
- تنظيف تلقائي عند امتلاء الكاش

### 4. إحصائيات ومراقبة
- تتبع الطلبات والنجاح
- معدل استخدام كل محرك
- متوسط الثقة ووقت المعالجة

## الاستخدام

### الاستخدام الأساسي

```rust
use almarjaa::ai_engine::UnifiedVibeEngine;

// إنشاء المحرك
let mut engine = UnifiedVibeEngine::new();
engine.initialize().unwrap();

// معالجة نص
let result = engine.process("اطبع مرحبا بالعالم");
println!("الكود: {}", result.code);
println!("الثقة: {}%", result.confidence * 100.0);
```

### دوال سهلة

```rust
use almarjaa::ai_engine::{unified_text_to_code, unified_vibe_process};

// تحويل مباشر
let code = unified_text_to_code("أنشئ متغير س يساوي 10");

// معالجة كاملة
let result = unified_vibe_process("أنشئ دالة تجمع رقمين");
```

### معالجة دفعة

```rust
let texts = vec!["اطبع أ", "أنشئ متغير ب", "كرر 5 مرات"];
let results = engine.process_batch(&texts);
```

### تكوين مخصص

```rust
use almarjaa::ai_engine::UnifiedVibeConfig;

let config = UnifiedVibeConfig {
    enable_gguf: true,
    enable_cache: true,
    gguf_temperature: 0.7,
    max_tokens: 256,
    ..Default::default()
};

let mut engine = UnifiedVibeEngine::with_config(config);
```

## حالات المحرك

| الحالة | الوصف |
|--------|-------|
| `Uninitialized` | غير مُهيأ |
| `PatternOnly` | مطابقة الأنماط فقط |
| `GGUFReady` | GGUF متاح |
| `Hybrid` | وضع هجين (أنماط + GGUF) |
| `Error` | خطأ في التهيئة |

## أنواع المحركات

| النوع | الاستخدام |
|-------|----------|
| `PatternMatching` | نصوص بسيطة، ثقة عالية |
| `GGUFModel` | نصوص معقدة، GGUF متاح |
| `RealAI` | محرك AI حقيقي |
| `Simulation` | محاكاة (fallback) |

## الاختبارات

### تشغيل الاختبارات

```bash
# اختبارات المحرك الموحد
cargo test --lib unified_vibe

# اختبارات GGUF
cargo test --lib gguf

# اختبارات التكامل
cargo test --test unified_vibe_integration
```

### النتائج

- **14** اختبار وحدة للمحرك الموحد ✓
- **6** اختبارات GGUF ✓
- **23** اختبار تكامل شامل ✓

## البنية

```
src/ai_engine/
├── unified_vibe.rs      # المحرك الموحد
├── vibe_advanced.rs     # محرك مطابقة الأنماط
├── gguf_inference.rs    # محرك GGUF
├── real_inference.rs    # محرك AI حقيقي
└── mod.rs               # التصديرات

tests/
└── unified_vibe_integration.rs  # اختبارات التكامل
```

## الأداء

- **معالجة الأنماط**: < 1ms (عادي)
- **معالجة GGUF**: 50-500ms (حسب النموذج)
- **إصابة الكاش**: فوري (< 0.1ms)
- **استهلاك الذاكرة**: ~10MB + نموذج GGUF

## التطوير المستقبلي

- [ ] دعم المزيد من نماذج GGUF
- [ ] تحسين خوارزمية اختيار المحرك
- [ ] إضافة دعم multi-threading
- [ ] تكامل مع Ollama
- [ ] واجهة REST API
