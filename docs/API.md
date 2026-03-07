# API Docs (الواجهة البرمجية) - لغة المرجع

## الهدف
هذا المستند يغطّي واجهة مكتبة Rust الخاصة بـ `almarjaa` حتى يمكن دمج اللغة داخل أدوات خارجية.

## الحزمة العامة (`src/lib.rs`)

### الثوابت
- `VERSION: &str` — إصدار اللغة الحالي.

### الدوال
- `info() -> &'static str` — معلومات تعريفية مختصرة عن اللغة.

### الواجهات المصدّرة
- `Parser` — بناء AST من النص.
- `Lexer` — تحويل النص إلى tokens.
- `Interpreter` — تنفيذ برنامج مرجعي.
- `format_source` — تنسيق مصدر `.mrj`.
- `lint_source` / `lint_program` — فحص جودة الشيفرة وإرجاع diagnostics.

### نماذج الأخطاء
- `AlMarjaaError`
- `ErrorCode`
- `Position`
- `Span`
- `Severity`

## أمثلة تكامل سريعة

### 1) Parse فقط
```rust
use almarjaa::Parser;

let program = Parser::parse("متغير س = ١؛").expect("parse failed");
assert!(!program.statements.is_empty());
```

### 2) تفسير مباشر
```rust
use almarjaa::Interpreter;

let mut vm = Interpreter::new();
vm.run("متغير س = ٢؛").unwrap();
let value = vm.run("س + ٣").unwrap();
assert_eq!(value.borrow().to_string(), "5");
```

### 3) lint + format pipeline
```rust
use almarjaa::{format_source, lint_source};

let src = "متغير س = ١؛\nمتغير س = ٢؛";
let formatted = format_source(src);
let diagnostics = lint_source(&formatted).unwrap();
assert!(!diagnostics.is_empty());
```

## سياسة الاستقرار
- أي تغيير breaking في `src/lib.rs` يتطلب تحديث `CHANGELOG.md` ورفع إصدار رئيسي.
- الإضافات غير الكاسرة (non-breaking) مسموحة في الإصدارات الصغرى.
