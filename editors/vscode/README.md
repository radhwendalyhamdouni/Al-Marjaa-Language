# 🌙 Al-Marjaa Language Support - لغة المرجع

<div align="center">

![Al-Marjaa Logo](icons/icon.png)

**دعم شامل ومتقدم للغة المرجع العربية في Visual Studio Code**

[![Version](https://img.shields.io/badge/version-2.0.0-blue.svg)](https://marketplace.visualstudio.com/items?itemName=almarjaa.almarjaa-language)
[![Rating](https://img.shields.io/badge/rating-★★★★★-brightgreen.svg)](https://marketplace.visualstudio.com/items?itemName=almarjaa.almarjaa-language)
[![Downloads](https://img.shields.io/badge/downloads-10K+-green.svg)](https://marketplace.visualstudio.com/items?itemName=almarjaa.almarjaa-language)
[![License](https://img.shields.io/badge/license-MIT-orange.svg)](LICENSE)

[التثبيت](#-التثبيت) • [الميزات](#-الميزات) • [الاستخدام](#-الاستخدام) • [التوثيق](https://docs.almarjaa.io)

</div>

---

## 📖 عن لغة المرجع

**لغة المرجع** هي أول لغة برمجة عربية متكاملة، مصممة لتكون سهلة التعلم والاستخدام للمتحدثين بالعربية. توفر بنية لغوية عربية طبيعية مع ميزات حديثة.

```mrj
// مثال بلغة المرجع
دالة حساب_المجموع(الأرقام: قائمة<رقم>) -> رقم {
    متغير المجموع = 0
    لكل رقم من الأرقام {
        المجموع = المجموع + رقم
    }
    أرجع المجموع
}

اطبع("المجموع: " + حساب_المجموع([1, 2, 3, 4, 5]))
```

---

## 🚀 التثبيت

### الطريقة الأولى: من Marketplace
1. افتح VS Code
2. اضغط `Ctrl+Shift+X` لفتح Extensions
3. ابحث عن `Al-Marjaa`
4. اضغط **Install**

### الطريقة الثانية: من سطر الأوامر
```bash
code --install-extension almarjaa.almarjaa-language
```

### الطريقة الثالثة: من الملف
1. حمّل ملف `.vsix` من [Releases](https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/releases)
2. افتح VS Code
3. اضغط `Ctrl+Shift+P`
4. اكتب `Extensions: Install from VSIX`
5. اختر الملف المحمّل

---

## ✨ الميزات

### 🎨 تلوين الصيغة (Syntax Highlighting)
تلوين ذكي للكود العربي مع تمييز:
- الكلمات المفتاحية (`دالة`، `متغير`، `إذا`، ...)
- الدوال والأسماء
- النصوص والأرقام
- التعليقات

### 💡 الإكمال التلقائي (IntelliSense)
- اقتراحات ذكية أثناء الكتابة
- إكمال الكلمات المفتاحية
- إكمال أسماء الدوال والمتغيرات
- **Vibe Coding**: اقتراحات AI ذكية

### 📝 Snippets (قوالب جاهزة)
اكتب البادئة واضغط `Tab`:

| البادئة | النتيجة |
|---------|---------|
| `دالة` | دالة كاملة |
| `إذا` | جملة شرطية |
| `لكل` | حلقة تكرار |
| `فئة` | فئة/كلاس |
| `اختبار` | اختبار وحدة |
| `vibe` | طلب AI |

### 🔍 التنقل في الكود
- **Go to Definition**: `F12` - الانتقال للتعريف
- **Find References**: `Shift+F12` - إيجاد المراجع
- **Rename Symbol**: `F2` - إعادة تسمية ذكية
- **Document Symbols**: `Ctrl+Shift+O` - قائمة الرموز

### 🎯 Code Lens
أزرار تفاعلية فوق كل دالة:
- `▶ تشغيل` - تشغيل الدالة
- `🧪 اختبار` - توليد اختبار
- `ℹ️ شرح` - شرح الدالة

### ⚡ Code Actions
اضغط على المصباح 💡 أو `Ctrl+.`:
- ✨ شرح الكود المحدد
- ⚡ تحسين الكود
- 🧪 توليد اختبارات تلقائية
- 🔄 تحويل إلى TypeScript
- 🐍 تحويل إلى Python

### 🎨 الثيمات
ثيمات عربية مخصصة:
- **Al-Marjaa Dark**: ثيم داكن احترافي
- **Al-Marjaa Arabic Night**: ثيم ليلي عربي

### 📊 إحصائيات الكود
- عدد الأسطر
- عدد الدوال
- عدد المتغيرات

### 🔧 التنسيق التلقائي
- تنسيق عند الحفظ
- تنسيق عند اللصق
- مسافات بادئة ذكية

---

## ⌨️ اختصارات لوحة المفاتيح

| الاختصار | الأمر |
|----------|-------|
| `F5` | تشغيل الملف الحالي |
| `Ctrl+Shift+E` | شرح الكود المحدد |
| `Ctrl+Shift+O` | تحسين الكود |
| `Ctrl+Shift+T` | توليد اختبارات |
| `F12` | الانتقال للتعريف |
| `Shift+F12` | إيجاد المراجع |
| `F2` | إعادة تسمية |

---

## 🤖 Vibe Coding مع AI

ميزة Vibe Coding توفر اقتراحات ذكية:

### شرح الكود
```
1. حدد الكود
2. اضغط Ctrl+Shift+E
3. سيظهر شرح مفصل
```

### تحسين الكود
```
1. حدد الكود
2. اضغط Ctrl+Shift+O
3. راجع التحسينات المقترحة
4. اضغط "تطبيق"
```

### توليد اختبارات
```
1. افتح الملف
2. اضغط Ctrl+Shift+T
3. سيُنشأ ملف اختبار تلقائياً
```

---

## 📁 هيكل المشروع

```
مشروعي/
├── مصدر/
│   ├── رئيسي.mrj       # الملف الرئيسي
│   └── مساعد.mrj       # ملفات إضافية
├── حزم/                # الحزم المثبتة
└── مشروع.toml          # ملف التكوين
```

---

## ⚙️ الإعدادات

افتح `File > Preferences > Settings` وابحث عن `almarjaa`:

```json
{
  "almarjaa.serverPath": "almarjaa-lsp",
  "almarjaa.trace.server": "off",
  "almarjaa.vibeCoding.enabled": true,
  "almarjaa.vibeCoding.inlineSuggestions": true,
  "almarjaa.formatting.enabled": true,
  "almarjaa.formatting.indentSize": 4,
  "almarjaa.diagnostics.enabled": true,
  "almarjaa.codeLens.enabled": true
}
```

---

## 🔧 المتطلبات

- **VS Code**: الإصدار 1.80.0 أو أحدث
- **لغة المرجع**: يجب تثبيت المترجم

### تثبيت مترجم المرجع

#### Linux/macOS
```bash
curl -fsSL https://get.almarjaa.io | sh
```

#### Windows
```powershell
iwr -useb https://get.almarjaa.io/windows | iex
```

---

## 📚 التوثيق

- [دليل البدء السريع](https://docs.almarjaa.io/quickstart)
- [مرجع اللغة](https://docs.almarjaa.io/reference)
- [أمثلة](https://docs.almarjaa.io/examples)
- [الأسئلة الشائعة](https://docs.almarjaa.io/faq)

---

## 🤝 المساهمة

نرحب بمساهماتكم!

1. Fork المستودع
2. إنشاء فرع جديد (`git checkout -b feature/amazing-feature`)
3. Commit التغييرات (`git commit -m '✨ إضافة ميزة رائعة'`)
4. Push للفرع (`git push origin feature/amazing-feature`)
5. فتح Pull Request

---

## 📝 الترخيص

هذا المشروع مرخص تحت [MIT License](LICENSE).

---

## 💬 الدعم والمجتمع

- 📧 البريد: almarjaa.project@hotmail.com
- 💬 Discord: [discord.gg/almarjaa](https://discord.gg/almarjaa)
- 🐦 Twitter: [@AlMarjaaLang](https://twitter.com/AlMarjaaLang)
- 📖 التوثيق: [docs.almarjaa.io](https://docs.almarjaa.io)

---

## 🙏 شكر وتقدير

شكراً لكل من ساهم في تطوير هذه الإضافة والمشروع ككل.

---

<div align="center">

**صنع بـ ❤️ للمجتمع العربي**

[⬆ العودة للأعلى](#-al-marjaa-language-support---لغة-المرجع)

</div>
