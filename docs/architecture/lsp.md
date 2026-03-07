# LSP Server - خادم اللغة المتكامل

## نظرة عامة

LSP Server للغة المرجع هو خادم Language Server Protocol متكامل ومتقدم، يوفر تجربة تطوير احترافية في محررات الأكواد المختلفة (VS Code, Neovim, Helix, وغيرها).

**الإصدار:** 3.3.0
**الحالة:** مكتمل ومتقدم

---

## الميزات المدعومة

### 1. القدرات الأساسية

| الميزة | الحالة | الوصف |
|--------|--------|-------|
| `initialize` | ✅ | تهيئة الخادم مع capability negotiation |
| `initialized` | ✅ | إشعار اكتمال التهيئة |
| `shutdown` | ✅ | إيقاف الخادم بشكل نظيف |
| `exit` | ✅ | إنهاء الخادم |

### 2. إدارة المستندات

| الميزة | الحالة | الوصف |
|--------|--------|-------|
| `textDocument/didOpen` | ✅ | فتح مستند جديد |
| `textDocument/didChange` | ✅ | تغييرات المستند (incremental) |
| `textDocument/didClose` | ✅ | إغلاق المستند |
| `textDocument/didSave` | ✅ | حفظ المستند |

### 3. التشخيصات

| الميزة | الحالة | الوصف |
|--------|--------|-------|
| `textDocument/publishDiagnostics` | ✅ | نشر الأخطاء والتحذيرات |
| أخطاء Parser | ✅ | تحليل نحوي في الوقت الحقيقي |
| تحذيرات Linter | ✅ | 9 قواعد للتحقق من الجودة |
| إصلاحات سريعة | ✅ | Code Actions للإصلاح |

### 4. التنقل

| الميزة | الحالة | الوصف |
|--------|--------|-------|
| `textDocument/definition` | ✅ | الانتقال للتعريف |
| `textDocument/typeDefinition` | ✅ | الانتقال لتعريف النوع |
| `textDocument/implementation` | ✅ | إيجاد التنفيذات |
| `textDocument/references` | ✅ | إيجاد جميع المراجع |
| `textDocument/documentSymbol` | ✅ | رموز المستند |
| `workspace/symbol` | ✅ | البحث في المشروع |

### 5. المعلومات

| الميزة | الحالة | الوصف |
|--------|--------|-------|
| `textDocument/hover` | ✅ | معلومات التمرير |
| `textDocument/completion` | ✅ | الإكمال التلقائي الذكي |
| `completionItem/resolve` | ✅ | حل الإكمال |
| `textDocument/signatureHelp` | ✅ | مساعدة التواقيع |
| `textDocument/documentHighlight` | ✅ | تمييز الاستخدامات |

### 6. Semantic Tokens

| الميزة | الحالة | الوصف |
|--------|--------|-------|
| `textDocument/semanticTokens/full` | ✅ | جميع الرموز الدلالية |
| `textDocument/semanticTokens/range` | ✅ | رموز نطاق محدد |
| `textDocument/semanticTokens/delta` | ✅ | تحديثات تدريجية |

**أنواع الرموز المدعومة:**
- `namespace` (0)
- `type` (1)
- `class` (2)
- `enum` (3)
- `interface` (4)
- `struct` (5)
- `typeParameter` (6)
- `parameter` (7)
- `variable` (8)
- `property` (9)
- `enumMember` (10)
- `event` (11)
- `function` (12)
- `method` (13)
- `macro` (14)
- `keyword` (15)
- `modifier` (16)
- `comment` (17)
- `string` (18)
- `number` (19)
- `regexp` (20)
- `operator` (21)
- `builtin` (22) - مخصص
- `decorator` (23) - مخصص
- `label` (24) - مخصص

### 7. Code Actions

| الميزة | الحالة | الوصف |
|--------|--------|-------|
| `textDocument/codeAction` | ✅ | إجراءات الكود |
| `codeAction/resolve` | ✅ | حل الإجراء |

**أنواع الإجراءات:**
- `quickfix` - إصلاح سريع
- `refactor.extract` - استخراج دالة/متغير
- `refactor.inline` - إدراج المتغير
- `refactor.rewrite` - إعادة كتابة
- `source.organizeImports` - تنظيم الاستيرادات
- `source.fixAll` - إصلاح الكل

### 8. Code Lens

| الميزة | الحالة | الوصف |
|--------|--------|-------|
| `textDocument/codeLens` | ✅ | عدسات الكود |
| `codeLens/resolve` | ✅ | حل العدسة |

**العدسات المتاحة:**
- `$(play) تشغيل` - تشغيل الدالة
- `$(beaker) اختبار` - اختبار الدالة
- `$(info) شرح` - شرح الدالة
- `$(references) N مرجع` - عدد المراجع
- `$(symbol-misc) النوع` - إظهار النوع

### 9. Inlay Hints

| الميزة | الحالة | الوصف |
|--------|--------|-------|
| `textDocument/inlayHint` | ✅ | التلميحات المضمنة |
| `inlayHint/resolve` | ✅ | حل التلميح |

**أنواع التلميحات:**
- `TYPE` - تلميحات الأنواع
- `PARAMETER` - أسماء المعاملات

### 10. Call Hierarchy

| الميزة | الحالة | الوصف |
|--------|--------|-------|
| `textDocument/prepareCallHierarchy` | ✅ | تجهيز التسلسل |
| `callHierarchy/incomingCalls` | ✅ | الاستدعاءات الواردة |
| `callHierarchy/outgoingCalls` | ✅ | الاستدعاءات الصادرة |

### 11. إعادة التسمية

| الميزة | الحالة | الوصف |
|--------|--------|-------|
| `textDocument/prepareRename` | ✅ | تجهيز إعادة التسمية |
| `textDocument/rename` | ✅ | إعادة التسمية |

### 12. التنسيق

| الميزة | الحالة | الوصف |
|--------|--------|-------|
| `textDocument/formatting` | ✅ | تنسيق المستند |
| `textDocument/rangeFormatting` | ✅ | تنسيق نطاق |
| `textDocument/onTypeFormatting` | ✅ | تنسيق عند الكتابة |

### 13. الطي

| الميزة | الحالة | الوصف |
|--------|--------|-------|
| `textDocument/foldingRange` | ✅ | نطاقات الطي |

**أنواع الطي:**
- الدوال
- الأصناف
- الكتل
- التعليقات
- المناطق المخصصة

### 14. Linked Editing

| الميزة | الحالة | الوصف |
|--------|--------|-------|
| `textDocument/linkedEditingRange` | ✅ | التحرير المرتبط |

---

## الأوامر المخصصة

| الأمر | الوصف |
|-------|-------|
| `almarjaa.runFile` | تشغيل الملف الحالي |
| `almarjaa.format` | تنسيق الكود |
| `almarjaa.fixAll` | إصلاح كل المشاكل |
| `almarjaa.organizeImports` | تنظيم الاستيرادات |
| `almarjaa.extractFunction` | استخراج كدالة |
| `almarjaa.extractVariable` | استخراج كمتغير |
| `almarjaa.inlineVariable` | إدراج المتغير |
| `almarjaa.generateTests` | توليد اختبارات |
| `almarjaa.explainCode` | شرح الكود |
| `almarjaa.optimizeCode` | تحسين الكود |
| `almarjaa.convertToTypeScript` | تحويل لـ TypeScript |
| `almarjaa.convertToPython` | تحويل لـ Python |
| `almarjaa.showType` | إظهار النوع |
| `almarjaa.findImplementations` | إيجاد التنفيذات |

---

## البنية التقنية

```
editors/lsp-server/
├── Cargo.toml          # إعدادات البناء
├── src/
│   ├── main.rs         # نقطة الدخول
│   ├── lib.rs          # المكتبة
│   ├── server.rs       # الخادم الرئيسي
│   ├── handlers.rs     # معالجات الطلبات
│   ├── state.rs        # إدارة الحالة
│   ├── capabilities.rs # القدرات
│   ├── transport.rs    # طبقة النقل
│   ├── semantic_tokens.rs  # الرموز الدلالية
│   ├── code_actions.rs     # إجراءات الكود
│   ├── code_lens.rs        # عدسات الكود
│   ├── inlay_hints.rs      # التلميحات المضمنة
│   ├── call_hierarchy.rs   # تسلسل الاستدعاءات
│   ├── formatting.rs       # التنسيق
│   ├── folding.rs          # الطي
│   ├── rename.rs           # إعادة التسمية
│   ├── signature_help.rs   # مساعدة التواقيع
│   ├── workspace_symbols.rs # رموز المشروع
│   ├── diagnostics.rs      # التشخيصات
│   ├── arabic_support.rs   # دعم العربية
│   ├── type_inference.rs   # استنتاج الأنواع
│   └── cache.rs            # التخزين المؤقت
```

---

## دعم اللغة العربية

### معالجة النص العربي

```rust
pub struct ArabicTextProcessor {
    arabic_chars: Vec<char>,
}

impl ArabicTextProcessor {
    /// هل الحرف عربي؟
    pub fn is_arabic(&self, c: char) -> bool;
    
    /// هل الكلمة عربية؟
    pub fn is_arabic_word(&self, word: &str) -> bool;
    
    /// تحويل الأرقام العربية/الهندية
    pub fn convert_digits(&self, text: &str, to_arabic: bool) -> String;
    
    /// تحسين العرض RTL
    pub fn rtl_display(&self, text: &str) -> String;
    
    /// اتجاه الكتابة
    pub fn get_direction(&self, text: &str) -> TextDirection;
    
    /// التحقق من صحة الاسم العربي
    pub fn is_valid_identifier(&self, name: &str) -> bool;
    
    /// اقتراحات التصحيح
    pub fn suggest_corrections(&self, word: &str) -> Vec<String>;
}
```

### الكلمات المفتاحية العربية

```rust
pub const ARABIC_KEYWORDS: &[(&str, &str)] = &[
    ("متغير", "متغير"),
    ("ثابت", "ثابت"),
    ("دالة", "دالة"),
    ("إذا", "إذا"),
    ("وإلا", "وإلا"),
    ("طالما", "طالما"),
    ("لكل", "لكل"),
    ("أرجع", "أرجع"),
    ("اطبع", "اطبع"),
    ("صح", "صح"),
    ("خطأ", "خطأ"),
];
```

---

## استنتاج الأنواع

```rust
pub struct TypeInferenceEngine {
    type_table: HashMap<String, TypeInfo>,
}

impl TypeInferenceEngine {
    /// استنتاج نوع التعبير
    pub fn infer(&mut self, expr: &str) -> TypeKind;
    
    /// التحقق من توافق النوعين
    pub fn is_compatible(&self, expected: &TypeKind, actual: &TypeKind) -> bool;
    
    /// الحصول على تمثيل نصي للنوع
    pub fn type_to_string(&self, kind: &TypeKind) -> String;
}
```

**أنواع البيانات:**
- `Number` - رقم
- `String` - نص
- `Boolean` - منطقي
- `List<T>` - قائمة
- `Dict<K, V>` - قاموس
- `Function` - دالة
- `Class` - صنف
- `Null` - فارغ
- `Unknown` - غير معروف
- `Any` - أي
- `Union<T, U>` - اتحاد
- `Optional<T>` - اختياري

---

## التخزين المؤقت

```rust
pub struct GlobalCache {
    completion_cache: Mutex<LruCache<String, Vec<String>>>,
    analysis_cache: Mutex<LruCache<String, AnalysisCacheEntry>>,
    docs_cache: Mutex<LruCache<String, String>>,
}
```

**المميزات:**
- ذاكرة LRU للإكمال التلقائي
- تخزين نتائج التحليل مع وقت الصلاحية
- تخزين التوثيق
- مسح تلقائي للبيانات القديمة

---

## الأداء

| العملية | الزمن المستهدف | الوصف |
|---------|---------------|-------|
| Hover | < 120ms | P95 للملف المتوسط |
| Definition | < 100ms | الانتقال للتعريف |
| References | < 150ms | إيجاد جميع المراجع |
| Completion | < 150ms | P95 مع سياق |
| Formatting | < 500ms | للملف الكامل |
| Diagnostics | < 200ms | بعد التغيير |

---

## التكامل مع VS Code

```json
{
  "contributes": {
    "languages": [{
      "id": "almarjaa",
      "extensions": [".mrj"],
      "configuration": "./language-configuration.json"
    }],
    "grammars": [{
      "language": "almarjaa",
      "scopeName": "source.almarjaa",
      "path": "./syntaxes/almarjaa.tmLanguage.json"
    }]
  }
}
```

---

## التثبيت

### من المصدر

```bash
cd editors/lsp-server
cargo build --release
```

### مع VS Code

```bash
cd editors/vscode
npm install
npm run compile
```

---

## الإعدادات

```json
{
  "almarjaa.serverPath": "almarjaa-lsp",
  "almarjaa.semanticTokens": true,
  "almarjaa.inlayHints": true,
  "almarjaa.codeLens": true,
  "almarjaa.maxDiagnostics": 100,
  "almarjaa.autoAnalysis": true
}
```

---

© 2026 رضوان دالي حمدوني | RADHWEN DALY HAMDOUNI
جميع الحقوق محفوظة | All Rights Reserved
