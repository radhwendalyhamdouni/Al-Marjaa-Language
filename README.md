<div align="center">

<img src="banner.png" alt="Al-Marjaa Language Banner" width="100%"/>

# لغة المرجع | Al-Marjaa Language

### أول لغة برمجة عربية متكاملة مع الذكاء الاصطناعي
### The First AI-Native Arabic Programming Language

[![Version](https://img.shields.io/badge/version-3.2.0-blue.svg)](https://github.com/radhwendalyhamdouni/Al-Marjaa-Language)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Coverage](https://img.shields.io/badge/coverage-99.4%25-brightgreen.svg)]()
[![Stars](https://img.shields.io/github/stars/radhwendalyhamdouni/Al-Marjaa-Language?style=social)](https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/stargazers)

<p align="center">
  <a href="#-نظرة-عامة">نظرة عامة</a> •
  <a href="#-المميزات">المميزات</a> •
  <a href="#-التثبيت">التثبيت</a> •
  <a href="#-أمثلة">أمثلة</a> •
  <a href="#-التوثيق">التوثيق</a> •
  <a href="#-المساهمة">المساهمة</a>
</p>

<p align="center">
  <a href="docs/README.md">English</a> | 
  <a href="README.md">العربية</a>
</p>

</div>

---

## 🌟 نظرة عامة | Overview

**لغة المرجع** هي لغة برمجة عربية متكاملة مصممة خصيصاً للمتحدثين باللغة العربية، مع دعم كامل للذكاء الاصطناعي. تجمع بين سهولة الاستخدام وقوة الأداء، مع ميزة **Vibe Coding** الثورية التي تتيح البرمجة باللغة الطبيعية.

**Al-Marjaa** is a comprehensive Arabic programming language designed specifically for Arabic speakers, with full AI integration. It combines ease of use with powerful performance, featuring the revolutionary **Vibe Coding** for natural language programming.

---

## ✨ المميزات | Features

### 🎯 دعم عربي كامل | Full Arabic Support

```almarjaa
// كلمات مفتاحية عربية
متغير الاسم = "مرحباً"؛
ثابت باي = 3.14159؛

// أرقام عربية
متغير عدد = ١٢٣؛

// حلقات عربية
لكل رقم في مدى(١، ١٠) {
    اطبع(رقم)؛
}
```

### 🤖 Vibe Coding - البرمجة باللغة الطبيعية

```almarjaa
// اكتب ما تريد بالعربية!
// Write what you want in Arabic!

"أنشئ متغير اسم يساوي أحمد"
// ← متغير اسم = "أحمد"؛

"إذا كان العمر أكبر من 18 اطبع بالغ"
// ← إذا العمر > 18 { اطبع("بالغ")؛ }
```

### 🧠 تكامل الذكاء الاصطناعي | AI Integration

```almarjaa
// إنشاء شبكة عصبية
متغير شبكة = شبكة_عصبية()؛
شبكة.أضف_طبقة(128، "relu")؛
شبكة.أضف_طبقة(10، "softmax")؛

// التدريب والتقييم
شبكة.درّب(بيانات_التدريب، 100، 0.01)؛
متغير نتيجة = شبكة.مرر(المدخل)؛
```

### 🔌 دعم ONNX | ONNX Support (جديد!)

```almarjaa
// تحميل نموذج ONNX
نموذج = أونكس.حمّل("resnet50.onnx")؛

// إنشاء موتر
موتر مدخل = موتر.عشوائي([1، 3، 224، 224])؛

// تشغيل الاستدلال
نتيجة = نموذج.استدل({"مدخل": مدخل})؛
اطبع(نتيجة["مخرج"])؛

// تصدير شبكة عصبية إلى ONNX
شبكة.صدّر("mymodel.onnx")؛
```

### 🎨 نظام واجهات المستخدم | UI Framework (جديد! v3.2.0)

```almarjaa
// نظام التخطيط التلقائي
صف {
    فجوة: 10،
    محاذاة: "وسط"،
    
    زر("اضغط هنا") {
        نقر: () => اطبع("تم الضغط!")،
    }،
    
    نص("مرحباً بالعالم") {
        لون: "#3498db"،
        حجم: 18،
    }،
}

// ربط البيانات التلقائي
متغير العدد = قابل_للملاحظة(0)؛

زر("+") {
    نقر: () => العدد.قيمة += 1،
}

نص(العدد.قيمة) {
    // يُحدّث تلقائياً عند تغيير العدد
}

// المكونات القابلة لإعادة الاستخدام
مكون بطاقة_منتج {
    خصائص: [الاسم، السعر، الصورة]،
    
    صف {
        صورة(الصورة) { عرض: 100، ارتفاع: 100 }،
        عمود {
            نص(الاسم) { وزن_الخط: "غامق" }،
            نص(السعر + " ر.س") { لون: "#27ae60" }،
        }،
    }،
}

// التصميم المتجاوب
شبكة {
    أعمدة: متجاوب({
        هاتف: 1،
        لوحي: 2،
        حاسوب: 4،
    })،
    فجوة: 15،
    
    لكل منتج في المنتجات {
        بطاقة_منتج(منتج.الاسم، منتج.السعر، منتج.الصورة)،
    }،
}

// نظام الثيمات
الثيم.الأساسي = "#3498db"؛
الثيم.الخلفية = "#ffffff"؛
الثيم.النص = "#2c3e50"؛

// الرسوم البيانية
رسم_خطي {
    بيانات: المبيعات،
    عنوان_س: "الأشهر"،
    عنوان_ص: "المبيعات"،
    ألوان: ["#3498db"، "#e74c3c"]،
}

// النوافذ المنبثقة
نافذة {
    عنوان: "تأكيد الحذف"،
    محتوى: "هل أنت متأكد؟"،
    أزرار: ["إلغاء"، "حذف"]،
    عند_الإغلاق: (النتيجة) => {
        إذا النتيجة == "حذف" {
            احذف_العنصر()؛
        }
    }،
}

// الرسوم المتحركة
حرك(العنصر) {
    من: { شفافية: 0، موقع_ص: 20 }،
    إلى: { شفافية: 1، موقع_ص: 0 }،
    مدة: 300،
    تخفيف: "سهولة_خروج"؛
}
```

### ⚡ أداء عالي | High Performance

| المكون | التقنية | الأداء |
|--------|---------|--------|
| Lexer | Rust | < 1ms لكل 1000 سطر |
| Parser | Recursive Descent | < 5ms لكل 1000 سطر |
| Interpreter | Tree-Walking | ~33ms لكل 1000 تكرار |
| Bytecode VM | Stack-based | 0.49 تعليمة/μs |
| **JIT Compiler** | **5-Tier Optimization** | **5.08x تسريع** ⚡ |

#### 🚀 JIT Compiler (جديد!)

```almarjaa
// المستويات الخمسة للتحسين:
// Tier 0: Interpreter Baseline (0 تنفيذ)
// Tier 1: Baseline JIT (50 تنفيذ)
// Tier 2: Optimizing JIT (200 تنفيذ)
// Tier 3: SIMD Optimizations (1000 تنفيذ)
// Tier 4: Tracing JIT (5000 تنفيذ)

// نتائج الأداء:
// - العمليات الحسابية: 3.6M عملية/ثانية
// - اختبار الضغط: 19.9M عملية/ثانية
// - نسبة التسريع: 5.08x
```

انظر [JIT_COMPLETION_REPORT.md](JIT_COMPLETION_REPORT.md) للتفاصيل.

---

## 📦 التثبيت | Installation

### التثبيت السريع | Quick Install

```bash
# Linux / macOS
curl -sSL https://raw.githubusercontent.com/radhwendalyhamdouni/Al-Marjaa-Language/main/setup.sh | bash

# Windows (PowerShell)
iwr -useb https://raw.githubusercontent.com/radhwendalyhamdouni/Al-Marjaa-Language/main/install/windows/install.ps1 | iex
```

### التثبيت من المصدر | Build from Source

```bash
# استنساخ المستودع
git clone https://github.com/radhwendalyhamdouni/Al-Marjaa-Language.git
cd Al-Marjaa-Language

# البناء
cargo build --release

# التثبيت
cargo install --path .
```

### التحقق من التثبيت | Verify Installation

```bash
almarjaa --version
# الناتج: لغة المرجع - الإصدار 3.1.0
```

---

## 🚀 أمثلة | Examples

### Hello World

```almarjaa
// ملف: hello.mrj
اطبع("مرحباً بالعالم!")؛
```

```bash
almarjaa hello.mrj
# الناتج: مرحباً بالعالم!
```

### دوال رياضية | Mathematical Functions

```almarjaa
// حساب المضروب
دالة مضروب(ن) {
    إذا ن <= 1 {
        أرجع 1؛
    }
    أرجع ن * مضروب(ن - 1)؛
}

اطبع(مضروب(5))؛    // 120
```

### البرمجة الكائنية | Object-Oriented

```almarjaa
صنف حيوان {
    متغير الاسم؛
    
    دالة حيوان(الاسم) {
        هذا.الاسم = الاسم؛
    }
    
    دالة صوت() {
        أرجع "صوت عام"؛
    }
}

صنف كلب: حيوان {
    دالة صوت() {
        أرجع "نباح!"؛
    }
}

متغير كلبي = جديد كلب("بوبي")؛
اطبع(كلبي.صوت())؛    // نباح!
```

### معالجة البيانات | Data Processing

```almarjaa
// قوائم استيعابية
متغير أرقام = [1، 2، 3، 4، 5، 6، 7، 8، 9، 10]؛
متغير زوجية = [س لكل س في أرقام إذا س % 2 == 0]؛
متغير مربعات = [س * س لكل س في أرقام]؛

// معالجة القواميس
متغير إحصائيات = {
    المجموع: 0،
    العدد: 0
}؛

لكل رقم في أرقام {
    إحصائيات["المجموع"] += رقم؛
    إحصائيات["العدد"] += 1؛
}

اطبع("المتوسط: " + إحصائيات["المجموع"] / إحصائيات["العدد"])؛
```

---

## 📚 التوثيق | Documentation

| الملف | الوصف |
|-------|-------|
| [دليل المستخدم](docs/USER_GUIDE_COMPREHENSIVE.md) | دليل شامل لاستخدام اللغة |
| [مرجع API](docs/API.md) | مرجع الدوال والأنواع |
| [دعم ONNX](docs/ONNX_SUPPORT.md) | دعم نماذج ONNX |
| [نظام واجهات المستخدم](docs/UI_SUPPORT.md) | نظام UI Framework (جديد! v3.2.0) |
| [الهيكلية](docs/ARCHITECTURE.md) | هيكلية المشروع التقنية |
| [تقييم Vibe Coding](docs/VIBE_CODING_SCIENTIFIC_EVALUATION.md) | تقييم علمي للميزة |
| [نتائج الاختبارات](docs/TEST_RESULTS.md) | نتائج الاختبارات الشاملة |
| [معايير الأداء](docs/BENCHMARK_RESULTS.md) | مقاييس الأداء |
| [JIT Compiler](JIT_COMPLETION_REPORT.md) | تقرير إكمال JIT ⚡ |
| [سجل تغييرات JIT](CHANGELOG_JIT.md) | توثيق تفصيلي للتغييرات |

---

## 📊 نتائج الاختبارات | Test Results

```
╔═══════════════════════════════════════════════════════════════╗
║              🧪 نتائج الاختبارات الشاملة                      ║
╠═══════════════════════════════════════════════════════════════╣
║ الفئة               │ الاختبارات │ النجاح │ نسبة النجاح       ║
╠═══════════════════════════════════════════════════════════════╣
║ Lexer Tests         │     33     │   33   │     100%   ✅     ║
║ Parser Tests        │     68     │   68   │     100%   ✅     ║
║ Interpreter Tests   │    215     │  215   │     100%   ✅     ║
║ CLI Tests           │     18     │   18   │     100%   ✅     ║
║ Integration Tests   │     9      │    9   │     100%   ✅     ║
╠═══════════════════════════════════════════════════════════════╣
║ الإجمالي            │    343     │  343   │     100%   🎉     ║
╚═══════════════════════════════════════════════════════════════╝
```

---

## 🏗️ هيكلية المشروع | Project Structure

```
Al-Marjaa-Language/
├── src/
│   ├── main.rs              # نقطة الدخول
│   ├── lib.rs               # المكتبة الرئيسية
│   ├── lexer/               # المحلل المعجمي
│   ├── parser/              # المحلل النحوي
│   ├── interpreter/         # المفسر
│   ├── bytecode/            # الآلة الافتراضية
│   ├── ai_engine/           # محرك الذكاء الاصطناعي
│   ├── onnx/                # دعم ONNX
│   ├── ui/                  # نظام واجهات المستخدم (جديد! v3.2.0)
│   ├── formatter/           # منسق الكود
│   ├── linter/              # محلل الجودة
│   └── package_manager/     # مدير الحزم
├── editors/
│   ├── vscode/              # إضافة VS Code
│   └── lsp-server/          # خادم LSP
├── examples/                # أمثلة وتطبيقات
├── tests/                   # اختبارات شاملة
├── docs/                    # التوثيق
└── fine_tuning/             # أدوات التدريب
```

---

## 🛠️ أدوات التطوير | Development Tools

### الوضع التفاعلي (REPL)

```bash
almarjaa -r

>>> متغير س = 10؛
>>> اطبع(س * 2)؛
20
>>> ذكاء              # تفعيل Vibe Coding
[Vibe Coding] اكتب ما تريد بالعربية...
```

### أوامر سطر الأوامر

```bash
almarjaa program.mrj       # تشغيل برنامج
almarjaa -c program.mrj    # تحليل فقط
almarjaa -f program.mrj    # تنسيق الكود
almarjaa -l program.mrj    # تحليل الجودة
almarjaa --debug file.mrj  # وضع التصحيح
almarjaa pm check          # فحص التبعيات
```

### VS Code Integration

```bash
# تثبيت الإضافة
cd editors/vscode && code --install-extension .
```

---

## 🤝 المساهمة | Contributing

نرحب بمساهماتكم! يرجى مراجعة:

- [دليل المساهمة](CONTRIBUTING.md)
- [قواعد السلوك](CODE_OF_CONDUCT.md)

### خطوات المساهمة

1. Fork المستودع
2. إنشاء فرع جديد (`git checkout -b feature/amazing-feature`)
3. إجراء التغييرات
4. تشغيل الاختبارات (`cargo test --all-features`)
5. Commit التغييرات (`git commit -m 'إضافة ميزة رائعة'`)
6. Push إلى الفرع (`git push origin feature/amazing-feature`)
7. فتح Pull Request

---

## 📜 الرخصة والملكية الفكرية | License & IP

### © 2026 رضوان دالي حمدوني | All Rights Reserved

هذا المشروع محمي بموجب حقوق الملكية الفكرية. يُسمح بالاستخدام للأغراض **غير التجارية** فقط مع نسب المصدر.

| النشاط | الإذن المطلوب |
|--------|--------------|
| الاستخدام الشخصي/التعليمي | ✅ مسموح مع النسب |
| البحث الأكاديمي | ✅ مسموح مع النسب |
| الاستخدام التجاري | ❌ يتطلب إذن كتابي |
| إنشاء لغة مشتقة | ❌ يتطلب إذن كتابي |

📧 **للاستفسارات والتراخيص التجارية:** almarjaa.project@hotmail.com

📄 [اقرأ الرخصة الكاملة](LICENSE)

---

## 👨‍💻 المؤلف | Author

<div align="center">

**رضوان دالي حمدوني | RADHWEN DALY HAMDOUNI**

[![GitHub](https://img.shields.io/badge/GitHub-radhwendalyhamdouni-blue?logo=github)](https://github.com/radhwendalyhamdouni)
[![Email](https://img.shields.io/badge/Email-almarjaa.project@hotmail.com-red?logo=mail)](mailto:almarjaa.project@hotmail.com)

*المؤلف والمبتكر الحصري للغة المرجع*

</div>

---

## 🙏 شكر وتقدير | Acknowledgments

- مجتمع Rust على الدعم التقني
- المساهمون في تطوير اللغة
- المجتمع العربي للبرمجة

---

## 📊 إحصائيات المشروع | Project Stats

![GitHub Stats](https://github-readme-stats.vercel.app/api?username=radhwendalyhamdouni&repo=Al-Marjaa-Language&show_icons=true&theme=radical)

---

<div align="center">

**صُنع بـ ❤️ للعالم العربي**

**Made with ❤️ for the Arab World**

---

© 2026 رضوان دالي حمدوني | RADHWEN DALY HAMDOUNI

**جميع الحقوق محفوظة | All Rights Reserved**

</div>
