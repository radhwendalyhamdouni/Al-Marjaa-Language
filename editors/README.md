# 🛠️ أدوات التطوير - Development Tools

هذا المجلد يحتوي على أدوات تطوير لغة المرجع المتكاملة.

---

## 📁 الهيكل

```
editors/
├── lsp-server/              # LSP Server (Rust)
│   ├── Cargo.toml           # إعدادات البناء
│   ├── README.md            # توثيق LSP Server
│   ├── integrate.sh         # سكربت الدمج
│   └── src/
│       ├── main.rs          # نقطة الدخول
│       ├── server.rs        # الخادم الرئيسي
│       ├── transport.rs     # طبقة JSON-RPC
│       ├── handlers.rs      # معالجات الطلبات
│       └── state.rs         # إدارة الحالة
│
└── vscode/                  # VS Code Extension
    ├── package.json         # إعدادات الإضافة
    ├── tsconfig.json        # إعدادات TypeScript
    ├── language-configuration.json  # إعدادات اللغة
    ├── src/
    │   └── extension.ts     # الكود الرئيسي
    ├── syntaxes/
    │   └── almarjaa.tmLanguage.json  # التلوين النحوي
    └── snippets/
        └── almarjaa.json    # مقاطع الكود
```

---

## 🚀 التثبيت السريع

### الطريقة الأولى: سكربت تلقائي

```bash
# من جذر المشروع
./setup.sh --all
```

### الطريقة الثانية: يدوياً

```bash
# 1. بناء وتثبيت اللغة
cargo build --release
sudo cp target/release/almarjaa /usr/local/bin/

# 2. بناء وتثبيت LSP Server
cd editors/lsp-server
cargo build --release
sudo cp target/release/almarjaa-lsp /usr/local/bin/

# 3. بناء وتثبيت VS Code Extension
cd ../vscode
npm install && npm run compile
npx vsce package
code --install-extension almarjaa-language-*.vsix
```

---

## 📦 المكونات

### 1. LSP Server

خادم Language Server Protocol كامل مكتوب بـ Rust.

**الميزات:**
- ✅ التشخيصات (Diagnostics)
- ✅ الإكمال التلقائي (Completion)
- ✅ معلومات التمرير (Hover)
- ✅ الانتقال للتعريف (Go to Definition)
- ✅ إيجاد المراجع (Find References)
- ✅ رموز المستند (Document Symbols)

**التكامل:**
```rust
// يستخدم المحلل الأصلي
use almarjaa::{Lexer, Parser, lint_source_with_config};
```

### 2. VS Code Extension

إضافة كاملة لـ Visual Studio Code.

**الميزات:**
- 🎨 تلوين نحوي للكود العربي
- ✨ إكمال تلقائي ذكي
- 📋 مقاطع كود جاهزة
- ▶️ زر تشغيل مباشر
- 🔄 إعادة تشغيل الخادم

---

## 🔧 الأوامر المتاحة

### Makefile

```bash
make lsp            # بناء LSP Server
make install-lsp    # تثبيت LSP Server
make vscode         # بناء VS Code Extension
make vscode-package # تعبئة الإضافة
make install-vscode # تثبيت الإضافة
```

### أوامر VS Code

| الأمر | الوصف |
|-------|-------|
| `Ctrl+Space` | الإكمال التلقائي |
| `F12` | الانتقال للتعريف |
| `Shift+F12` | إيجاد المراجع |
| `Ctrl+Shift+O` | رموز المستند |
| `K Ctrl+I` | معلومات التمرير |

---

## 🧪 الاختبارات

```bash
# اختبار LSP Server
cd editors/lsp-server && cargo test

# اختبار يدوي
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | almarjaa-lsp
```

---

## 📊 مقارنة الأداء

| الطريقة | زمن الاستجابة |
|---------|---------------|
| CLI (الحالي) | ~500ms |
| LSP Server (الجديد) | ~5ms |

**تحسين:** 100x أسرع! 🚀

---

## 🔍 استكشاف الأخطاء

### المشكلة: LSP Server لا يعمل

```bash
# التحقق من التثبيت
which almarjaa-lsp

# التحقق من الصلاحيات
chmod +x /usr/local/bin/almarjaa-lsp

# إعادة التثبيت
make install-lsp
```

### المشكلة: VS Code لا يجد الخادم

1. افتح الإعدادات (`Ctrl+,`)
2. ابحث عن "almarjaa"
3. تحقق من `almarjaa.serverPath`

### المشكلة: التلوين لا يعمل

```bash
# إعادة تحميل النافذة
Ctrl+Shift+P → "Reload Window"
```

---

## 📞 الدعم

- **GitHub**: https://github.com/radhwendalyhamdouni/Al-Marjaa-Language
- **Issues**: https://github.com/radhwendalyhamdouni/Al-Marjaa-Language/issues

---

**🌟 المرجع - لغة برمجة عربية متكاملة مع AI**
