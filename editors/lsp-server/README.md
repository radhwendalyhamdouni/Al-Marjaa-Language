# 🖥️ LSP Server للغة المرجع

خادم Language Server Protocol كامل متكامل مع المحلل الأصلي.

---

## 📋 الميزات

| الميزة | الوصف | الحالة |
|--------|-------|--------|
| **التشخيصات** | أخطاء Parser + Linter | ✅ |
| **الإكمال التلقائي** | كلمات محجوزة + رموز الملف | ✅ |
| **معلومات التمرير** | تعريفات + استخدامات | ✅ |
| **الانتقال للتعريف** | F12 للانتقال | ✅ |
| **إيجاد المراجع** | Shift+F12 | ✅ |
| **رموز المستند** | Ctrl+Shift+O | ✅ |
| **تمييز المراجع** | تمييز جميع الاستخدامات | ✅ |

---

## 🔧 البناء

```bash
# من مجلد LSP Server
cd editors/lsp-server
cargo build --release
```

---

## 📦 التثبيت

```bash
# تثبيت في النظام
sudo cp target/release/almarjaa-lsp /usr/local/bin/

# أو باستخدام Makefile
make install-lsp
```

---

## 🔗 التكامل مع المحلل الأصلي

```rust
// في state.rs
use almarjaa::{
    Lexer,           // المحلل المعجمي
    Parser,          // المحلل النحوي
    lint_source_with_config,  // Linter
    LintConfig,
};
use almarjaa::lexer::tokens::TokenType;

fn analyze(&self, content: &str) -> AnalysisResult {
    // 1. التحليل المعجمي
    let tokens = Lexer::new(content).tokenize()?;

    // 2. التحليل النحوي
    Parser::parse(content)?;

    // 3. التحليل الثابت
    let lints = lint_source_with_config(content, &LintConfig::default())?;

    // ...
}
```

---

## 🧪 الاختبارات

```bash
# تشغيل الاختبارات
cargo test

# اختبار يدوي
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | almarjaa-lsp
```

---

## 📊 الأداء

```
CLI (الحالي):
├── كل طلب = تشغيل process جديد
├── قراءة الملف من القرص
├── تحليل كامل
└── ⏱️ ~500ms لكل طلب

LSP Server (الجديد):
├── اتصال مستمر مع VS Code
├── الملف محفوظ في الذاكرة
├── تحليل تدريجي
└── ⏱️ ~5ms لكل طلب (100x أسرع!)
```

---

## 🔍 استكشاف الأخطاء

### تفعيل التسجيل

```bash
# في VS Code settings.json
{
    "almarjaa.trace.server": "verbose"
}
```

### عرض السجلات

```bash
# في VS Code
Ctrl+Shift+P → "Output" → "Al-Marjaa Language Server"
```

---

## 📁 الملفات

| الملف | الوصف |
|-------|-------|
| `main.rs` | نقطة الدخول ورسائل LSP |
| `server.rs` | الخادم الرئيسي |
| `transport.rs` | طبقة النقل JSON-RPC |
| `handlers.rs` | معالجات الطلبات |
| `state.rs` | إدارة الحالة والتحليل |

---

**🌟 المرجع - لغة برمجة عربية متكاملة مع AI**
