# لغة المرجع - Al-Marjaa Language
## أول لغة برمجة عربية بذكاء اصطناعي 🚀

<div align="center">

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/Version-2.0.0-green.svg)](https://github.com/radhwendalyhamdouni/Al-Marjaa-Language)

**اكتب بالعربية، نفذ بالسرعة** ⚡

[البداية السريعة](#-البداية-السريعة) • [الأمثلة](#-أمثلة-vibe-coding) • [التوثيق](#-التوثيق) • [المساهمة](#-المساهمة)

</div>

---

## 🌟 ما هي لغة المرجع؟

**لغة المرجع** هي لغة برمجة عربية متكاملة تجمع بين قوة البرمجة التقليدية وذكاء الـ AI. تتيح لك:

- ✍️ **الكتابة بالعربية**: كود قابل للقراءة بكلمات عربية
- 🧠 **Vibe Coding**: تحويل النص العربي الطبيعي إلى كود تنفيذي
- ⚡ **أداء عالي**: Bytecode VM سريع مع دعم JIT
- 🔒 **Offline بالكامل**: يعمل على أي حاسوب بدون إنترنت

---

## 🚀 البداية السريعة

### التثبيت

```bash
# استنساخ المشروع
git clone https://github.com/radhwendalyhamdouni/Al-Marjaa-Language.git
cd Al-Marjaa-Language

# البناء
cargo build --release
```

### أول برنامج

```almarjaa
// مرحباً بالعالم
اطبع("مرحباً بالعالم!")؛

// المتغيرات
متغير اسم = "أحمد"؛
متغير عمر = 25؛

// الشروط
إذا عمر > 18 {
    اطبع("بالغ")؛
} وإلا {
    اطبع("قاصر")؛
}

// الحلقات
طالما عمر < 30 {
    عمر = عمر + 1؛
    اطبع(عمر)؛
}

// الدوال
دالة ترحيب(اسم) {
    اطبع("مرحباً " + اسم)؛
}

ترحيب("محمد")؛
```

---

## 🤖 Vibe Coding - البرمجة باللغة الطبيعية

### ما هو Vibe Coding؟

Vibe Coding يتيح لك كتابة الكود باللغة العربية الطبيعية، ويقوم الـ AI بتحويله إلى كود تنفيذي:

| النص العربي | الكود المُنتج |
|-------------|---------------|
| "أنشئ متغير س يساوي 5" | `متغير س = 5؛` |
| "اطبع رسالة مرحباً" | `اطبع("مرحباً")؛` |
| "إذا كان س أكبر من 10 اطبع كبير" | `إذا س > 10 { اطبع("كبير"); }` |
| "أنشئ دالة تضيف رقمين" | `دالة اجمع(أ، ب) { أعطِ أ + ب; }` |
| "كرر طباعة مرحبا 3 مرات" | `طالما ع < 3 { ... }` |

### تشغيل Pipeline

```rust
use almarjaa::{run_pipeline, run_example, parse_intent, generate_code};

fn main() {
    // تشغيل Pipeline كامل
    let result = run_pipeline("اطبع مرحباً بالعالم");
    println!("النتيجة: {}", result);
    
    // أو تشغيل مثال مع عرض التفاصيل
    run_example("أنشئ متغير س يساوي 10");
    
    // تحليل النية فقط
    let intent = parse_intent("إذا كان س أكبر من 5 اطبع كبير");
    println!("الإجراء: {}", intent.action);
    
    // توليد الكود من النية
    let code = generate_code(&intent);
    println!("الكود: {}", code);
}
```

---

## 📚 أمثلة Vibe Coding

### مثال 1: إنشاء متغير
```
المدخل: "أنشئ متغير س يساوي 5"
النية: {"action": "variable", "name": "س", "value": "5"}
الكود: متغير س = 5؛
```

### مثال 2: طباعة رسالة
```
المدخل: "اطبع رسالة مرحباً"
النية: {"action": "print", "value": "مرحباً"}
الكود: اطبع("مرحباً")؛
```

### مثال 3: الشرط
```
المدخل: "إذا كان س أكبر من 10 اطبع 'كبير'"
النية: {"action": "condition", "condition": "س > 10"}
الكود: إذا س > 10 { اطبع("كبير"); }
```

### مثال 4: إنشاء دالة
```
المدخل: "أنشئ دالة تضيف رقمين وتعيد النتيجة"
النية: {"action": "function", "name": "اجمع", "params": "أ، ب"}
الكود: دالة اجمع(أ، ب) { أعطِ أ + ب؛ }
```

### مثال 5: التكرار
```
المدخل: "كرر طباعة 'مرحبا' 3 مرات"
النية: {"action": "loop", "count": "3"}
الكود: متغير ع = 0؛ طالما ع < 3 { اطبع("مرحبا"); ع = ع + 1؛ }
```

---

## ⚡ Bytecode VM - أداء عالي

### البنية

```
النص العربي → Lexer → Parser → AST → Compiler → Bytecode → VM → النتيجة
                                                    ↑
                                              Bytecode VM
                                            (50+ تعليمة)
```

### مقارنة الأداء

| الاختبار | المفسر | Bytecode VM | التسريع |
|----------|--------|-------------|---------|
| حلقة 100 | 264 μs | 129 μs | **2.05x** |
| حلقة 500 | 261 μs | 434 μs | 0.60x |
| عمليات حسابية | 174 μs | 314 μs | 0.55x |

### تشغيل Bytecode

```rust
use almarjaa::{run_bytecode, VM, Compiler, Chunk, OpCode};

fn main() {
    // تشغيل مباشر
    let result = run_bytecode("متغير س = 10؛ اطبع(س)؛");
    
    // أو استخدام VM مباشرة
    let chunk = Compiler::compile_source("طالما س < 100 { س = س + 1؛ }");
    let mut vm = VM::new(/* globals */);
    vm.load(chunk.unwrap());
    let result = vm.run();
}
```

---

## 🎓 Fine-tuning محلي

### تخصيص النموذج

```rust
use almarjaa::{fine_tune_model, FineTuningInterface, TrainingConfig};

fn main() {
    // Fine-tuning مباشر
    let inputs = vec!["اطبع مرحبا", "أنشئ متغير س يساوي 5"];
    let outputs = vec![
        r#"{"action":"print","value":"مرحبا"}"#,
        r#"{"action":"variable","name":"س","value":"5"}"#,
    ];
    
    let result = fine_tune_model(inputs, outputs);
    println!("تم التدريب: {} مثال", result.examples_count);
    
    // مع إعدادات مخصصة
    let config = TrainingConfig {
        learning_rate: 0.001,
        epochs: 5,
        batch_size: 8,
        lora_alpha: 32.0,
        lora_rank: 16,
    };
    
    let result = fine_tune_with_config(inputs, outputs, config);
}
```

---

## 📁 هيكل المشروع

```
Al-Marjaa-Language/
├── src/
│   ├── ai_engine/           # محرك الذكاء الاصطناعي
│   │   └── pipeline/
│   │       └── engine.rs    # Pipeline الرئيسي
│   ├── bytecode/            # Bytecode VM
│   │   ├── opcodes.rs       # تعليمات Bytecode
│   │   ├── compiler.rs      # المترجم
│   │   └── vm.rs            # الآلة الافتراضية
│   ├── fine_tuning/         # Fine-tuning
│   │   └── interface.rs     # واجهة التدريب
│   ├── interpreter/         # المفسر
│   ├── lexer/               # المحلل اللغوي
│   ├── parser/              # المحلل النحوي
│   └── lib.rs               # المكتبة الرئيسية
├── examples/
│   ├── vibe_variable.mrj    # مثال المتغير
│   ├── vibe_print.mrj       # مثال الطباعة
│   ├── vibe_condition.mrj   # مثال الشرط
│   ├── vibe_function.mrj    # مثال الدالة
│   └── vibe_loop.mrj        # مثال التكرار
└── docs/
    └── README.md            # هذا الملف
```

---

## 🔧 الميزات

### الميزات الأساسية
- ✅ متغيرات وثوابت عربية
- ✅ دوال مع معاملات وقيم إرجاع
- ✅ شروط (إذا، وإلا، وإلا إذا)
- ✅ حلقات (طالما، لكل، كرر)
- ✅ قوائم وقواميس
- ✅ معالجة الأخطاء

### ميزات AI
- ✅ Vibe Coding (تحويل النص لكود)
- ✅ Pipeline ذكي متكامل
- ✅ Fine-tuning محلي
- ✅ يعمل Offline بالكامل

### ميزات الأداء
- ✅ Bytecode VM سريع
- ✅ 50+ تعليمة Bytecode
- ✅ تسريع 2x للحلقات الصغيرة
- ✅ JIT للأرقام (قيد التطوير)

---

## 🧪 الاختبارات

```bash
# تشغيل جميع الاختبارات
cargo test

# تشغيل اختبارات الأداء
cargo test --release -- --nocapture

# تشغيل اختبارات Pipeline
cargo test test_parse --release -- --nocapture
```

---

## 🤝 المساهمة

نرحب بالمساهمات! يمكنك:

1. الإبلاغ عن أخطاء
2. اقتراح ميزات جديدة
3. إرسال طلبات سحب
4. تحسين التوثيق

---

## 📜 الترخيص

هذا المشروع مرخص تحت رخصة MIT - انظر ملف [LICENSE](LICENSE) للتفاصيل.

---

## 🙏 شكر وتقدير

- مجتمع Rust العربي
- المساهمين في المشروع
- جميع من ساهم في تطوير البرمجة العربية

---

<div align="center">

**صُنع بـ ❤️ للعالم العربي**

[⬆ العودة للأعلى](#-لغة-المرجع---al-marjaa-language)

</div>
