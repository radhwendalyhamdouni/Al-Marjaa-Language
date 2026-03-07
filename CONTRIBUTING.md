# 🤝 دليل المساهمة - Contributing Guide

شكراً لاهتمامك بالمساهمة في لغة المرجع!

---

## 📋 طرق المساهمة

| النوع | الوصف |
|-------|-------|
| 🐛 الإبلاغ عن الأخطاء | فتح Issue في GitHub |
| 💡 اقتراح ميزات | فتح Issue مع عنوان [Feature] |
| 📝 تحسين التوثيق | فتح Pull Request |
| 🔧 إصلاح الأخطاء | فتح Pull Request |
| ✨ أمثلة جديدة | إضافة ملفات في `examples/` |

---

## 🔧 إعداد بيئة التطوير

```bash
# 1. استنساخ المشروع
git clone https://github.com/radhwendalyhamdouni/Al-Marjaa-Language.git
cd Al-Marjaa-Language

# 2. تثبيت Rust (إذا لم يكن مثبتاً)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. البناء
cargo build

# 4. تشغيل الاختبارات
cargo test

# 5. التحقق من الكود
cargo clippy
cargo fmt --check
```

---

## 📝 معايير الكود

### أسلوب الكتابة
```rust
// ✅ صحيح - تعليقات بالعربية للمنطق
// تحويل الأرقام العربية إلى أرقام لاتينية
fn convert_arabic_number(s: &str) -> String { ... }

// ❌ خطأ - بدون تعليقات
fn convert_arabic_number(s: &str) -> String { ... }
```

### تسمية المتغيرات
```rust
// ✅ صحيح - أسماء واضحة
let variable_name = 10;
let arabic_number = "١٢٣";

// ❌ خطأ - أسماء غامضة
let x = 10;
let n = "١٢٣";
```

---

## 🧪 الاختبارات

### إضافة اختبار جديد
```rust
#[test]
fn test_new_feature() {
    // ترتيب
    let input = "متغير س = ١٠؛";

    // تنفيذ
    let result = parse(input);

    // التحقق
    assert!(result.is_ok());
}
```

### تشغيل الاختبارات
```bash
# جميع الاختبارات
cargo test

# اختبار محدد
cargo test test_name

# مع الإخراج
cargo test -- --nocapture
```

---

## 📂 هيكل المساهمة

```
contributions/
├── 📂 bug_fixes/        # إصلاحات الأخطاء
├── 📂 features/         # ميزات جديدة
├── 📂 documentation/    # تحسينات التوثيق
└── 📂 examples/         # أمثلة جديدة
```

---

## ✅ قائمة التحقق قبل الإرسال

- [ ] الكود يبني بدون أخطاء (`cargo build`)
- [ ] الاختبارات تمر (`cargo test`)
- [ ] التنسيق صحيح (`cargo fmt`)
- [ ] لا تحذيرات من Clippy (`cargo clippy`)
- [ ] التوثيق محدث
- [ ] رسالة Commit واضحة

---

## 📧 التواصل

- **البريد**: almarjaa.project@hotmail.com
- **GitHub**: [@radhwendalyhamdouni](https://github.com/radhwendalyhamdouni)

---

شكراً لمساهمتك! 🙏
