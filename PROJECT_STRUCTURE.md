# 📁 بنية المشروع - Project Structure

دليل كامل لبنية مشروع لغة المرجع للمبتدئين.

---

## 🗂️ البنية الكاملة

```
Al-Marjaa-Language/
│
├── 📄 Cargo.toml              # إعدادات مشروع Rust
├── 📄 Cargo.lock              # إصدارات المكتبات
├── 📄 README.md               # وصف المشروع
├── 📄 LICENSE                 # الترخيص وحقوق الملكية
├── 📄 PROJECT_STRUCTURE.md    # هذا الملف
├── 📄 .gitignore              # ملفات مستثناة من Git
│
├── 📂 src/                    # الكود المصدري (Source Code)
│   │
│   ├── 📄 main.rs             # نقطة الدخول الرئيسية
│   ├── 📄 lib.rs              # المكتبة الرئيسية
│   │
│   ├── 📂 lexer/              # المحلل المعجمي (Lexer)
│   │   ├── mod.rs             # الوحدة الرئيسية
│   │   └── tokens.rs          # تعريف الرموز
│   │
│   ├── 📂 parser/             # المحلل النحوي (Parser)
│   │   ├── mod.rs             # الوحدة الرئيسية
│   │   └── ast.rs             # شجرة البنية المجردة
│   │
│   ├── 📂 interpreter/        # المفسر (Interpreter)
│   │   ├── mod.rs             # الوحدة الرئيسية
│   │   ├── value.rs           # أنواع القيم (Value types)
│   │   ├── native_io.rs       # عمليات الإدخال/الإخراج
│   │   ├── native_stdlib.rs   # المكتبة القياسية
│   │   ├── autograd.rs        # الاشتقاق التلقائي
│   │   ├── gpu.rs             # دعم GPU
│   │   └── jit.rs             # JIT للمفسر
│   │
│   ├── 📂 bytecode/           # الآلة الافتراضية (Bytecode VM)
│   │   ├── mod.rs             # الوحدة الرئيسية
│   │   ├── opcodes.rs         # تعريف العمليات (57 opcode)
│   │   ├── compiler.rs        # تحويل AST → Bytecode
│   │   ├── vm.rs              # الآلة الافتراضية
│   │   ├── jit.rs             # المترجم الفوري (JIT)
│   │   ├── advanced_jit.rs    # JIT المتقدم (5 مستويات)
│   │   ├── optimizer.rs       # تحسين الكود
│   │   ├── gc.rs              # جامع القمامة (GC)
│   │   └── benchmarks.rs      # اختبارات الأداء
│   │
│   ├── 📂 ai_engine/          # محرك الذكاء الاصطناعي
│   │   ├── mod.rs             # الوحدة الرئيسية
│   │   ├── inference.rs       # محرك الاستدلال
│   │   ├── gguf_inference.rs  # استدلال GGUF
│   │   ├── arabic_nlp.rs      # معالجة اللغة العربية
│   │   └── 📂 pipeline/       # خط المعالجة
│   │       ├── mod.rs
│   │       └── engine.rs      # محرك Vibe Coding
│   │
│   ├── 📂 gui/                # محرك الواجهات (GUI)
│   │   └── mod.rs             # بناء الواجهات
│   │
│   ├── 📂 exporter/           # نظام التصدير
│   │   └── mod.rs             # تصدير للتطبيقات
│   │
│   ├── 📂 fine_tuning/        # تدريب النماذج
│   │   ├── mod.rs
│   │   └── interface.rs       # واجهة التدريب
│   │
│   ├── 📂 cli/                # واجهة سطر الأوامر
│   │   ├── mod.rs
│   │   ├── args.rs            # تحليل المعاملات
│   │   ├── commands.rs        # الأوامر
│   │   └── repl.rs            # الوضع التفاعلي
│   │
│   ├── 📂 linter/             # فاحص الكود
│   │   └── mod.rs
│   │
│   ├── 📂 formatter/          # منسق الكود
│   │   └── mod.rs
│   │
│   ├── 📂 lsp_bridge/         # جسر LSP للمحررات
│   │   └── mod.rs
│   │
│   ├── 📂 package_manager/    # مدير الحزم
│   │   ├── mod.rs
│   │   ├── registry.rs        # سجل الحزم
│   │   ├── installer.rs       # مثبت الحزم
│   │   └── dependency.rs      # حل التبعيات
│   │
│   ├── 📂 runtime/            # بيئة التشغيل
│   │   └── mod.rs
│   │
│   ├── 📂 error/              # معالجة الأخطاء
│   │   └── mod.rs
│   │
│   ├── 📂 integration/        # تكامل المكونات
│   │   └── mod.rs
│   │
│   └── 📂 bin/                # برامج إضافية
│       ├── test_advanced.rs
│       ├── test_gguf.rs
│       └── ...
│
├── 📂 examples/               # أمثلة البرمجة
│   ├── hello.mrj              # أول برنامج
│   ├── variables.mrj          # المتغيرات
│   ├── conditions.mrj         # الشروط
│   ├── loops.mrj              # الحلقات
│   ├── functions.mrj          # الدوال
│   ├── calculator.mrj         # حاسبة
│   ├── neural_network.mrj     # شبكة عصبية
│   ├── vibe_*.mrj             # أمثلة Vibe Coding
│   └── ...
│
├── 📂 tests/                  # الاختبارات
│   ├── lexer_tests.rs         # اختبارات المحلل المعجمي
│   ├── parser_tests.rs        # اختبارات المحلل النحوي
│   ├── interpreter_tests.rs   # اختبارات المفسر
│   ├── bytecode_*.rs          # اختبارات Bytecode
│   ├── 📂 corpus/             # ملفات اختبار
│   └── 📂 golden/             # نتائج متوقعة
│
├── 📂 docs/                   # التوثيق
│   ├── SPEC.md                # مواصفة اللغة
│   ├── ARCHITECTURE.md        # البنية المعمارية
│   ├── TECHNICAL_DOCUMENTATION.md  # التوثيق التقني
│   ├── JIT_COMPILER.md        # وثائق JIT
│   ├── PARALLEL_GC.md         # وثائق GC
│   ├── ROADMAP.md             # خارطة الطريق
│   └── ...
│
├── 📂 models/                 # نماذج AI (تحميل منفصل)
│   └── README.md              # تعليمات التحميل
│
└── 📂 fine_tuning/            # بيانات التدريب
    ├── README.md
    └── 📂 data/
        └── README.md
```

---

## 🔄 مسار التنفيذ (Execution Flow)

```
الشفرة المصدرية (.mrj)
         │
         ▼
    ┌─────────┐
    │  Lexer  │ ── تحويل النص إلى رموز (Tokens)
    └─────────┘
         │
         ▼
    ┌─────────┐
    │ Parser  │ ─ـ تحويل الرموز إلى AST
    └─────────┘
         │
         ▼
    ┌───────────┐
    │ Compiler  │ ─ـ تحويل AST إلى Bytecode
    └───────────┘
         │
         ▼
    ┌─────────────────────────────────────┐
    │           Bytecode VM               │
    │  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐   │
    │  │ T0  │→│ T1  │→│ T2  │→│ T3  │   │
    │  │Inter│ │Base │ │Opt  │ │SIMD │   │
    │  └─────┘ └─────┘ └─────┘ └─────┘   │
    │                    │                │
    │                    ▼                │
    │              ┌─────────┐            │
    │              │   T4    │            │
    │              │ Tracing │            │
    │              └─────────┘            │
    └─────────────────────────────────────┘
         │
         ▼
    ┌─────────┐
    │   GC    │ ─ـ إدارة الذاكرة
    └─────────┘
         │
         ▼
      النتيجة
```

---

## 📚 الأدلة حسب المستوى

### 🟢 مبتدئ
1. ابدأ بـ `README.md`
2. اقرأ `docs/SPEC.md` لمواصفة اللغة
3. جرب الأمثلة في `examples/`

### 🟡 متوسط
1. ادرس `docs/ARCHITECTURE.md`
2. اقرأ كود `src/lexer/` و `src/parser/`
3. افحص `src/interpreter/`

### 🔴 متقدم
1. ادرس `docs/JIT_COMPILER.md`
2. اقرأ `src/bytecode/` بالكامل
3. افحص `docs/PARALLEL_GC.md`

---

## 🚀 البدء السريع

```bash
# 1. استنساخ المشروع
git clone https://github.com/radhwendalyhamdouni/Al-Marjaa-Language.git
cd Al-Marjaa-Language

# 2. البناء
cargo build --release

# 3. تشغيل مثال
./target/release/almarjaa examples/hello.mrj

# 4. الوضع التفاعلي
./target/release/almarjaa
```

---

## 📞 للمساعدة

- 📧 البريد: almarjaa.project@hotmail.com
- 🐛 المشاكل: [GitHub Issues](https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/issues)

---

**المؤلف**: رضوان دالي حمدوني
**الإصدار**: 3.0.0
