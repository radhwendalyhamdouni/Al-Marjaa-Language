# BENCHMARKS.md - ميزانية الأداء والقياسات الثابتة

**الإصدار:** 2.0.0  
**آخر تحديث:** 2026-02

## 1) ميزانية الأداء (Performance Budget)

| المؤشر | الميزانية المستهدفة للإصدار | ملاحظات |
|---|---:|---|
| تشغيل برنامج `hello` | ≤ 120ms | على GitHub Actions Ubuntu runner |
| تعبيرات حسابية بسيطة | ≤ 200ms (قياس نسبي) | عبر benchmark `interp_arithmetic` |
| استدعاء دالة بسيط | ≤ 250ms (قياس نسبي) | عبر benchmark `interp_function_call` |
| حلقة تكرار (range 1000) | ≤ 300ms (قياس نسبي) | عبر benchmark `interp_loop_iteration` |
| تنسيق نصّي مع interpolation | ≤ 300ms (قياس نسبي) | عبر benchmark `interp_string_interpolation` |

> هذه الميزانية تُستخدم كمرجع ثابت بين الإصدارات، وليس لمقارنة عادلة مع لغات مترجمة مباشرة.

## 2) مجموعة Benchmarks الرسمية

- ملف benches الرسمي: `tests/performance_budget_tests.rs`.
- الأدوات: اختبارات Rust مخصصة بوسم `#[ignore]` ضمن `cargo test --release`.
- الحالات الأساسية:
  - `budget_arithmetic_under_200ms`
  - `budget_function_call_under_250ms`
  - `budget_loop_iteration_under_300ms`
  - `budget_string_interpolation_under_300ms`

## 3) سياسة التنفيذ وبوابة القبول في CI

- على كل Push/PR/Release:
  1. تشغيل `cargo test --release --test performance_budget_tests -- --ignored` في CI كـ **بوابة قبول** (Benchmark Gate).
  2. أي تجاوز للميزانية يفشل الـ workflow مباشرة.
  3. عند تجاوز الميزانية: إنشاء issue أداء قبل اعتماد الإصدار التالي.

## 4) أوامر التشغيل

```bash
# محلياً
cargo test --release --test performance_budget_tests -- --ignored

# في CI (benchmark gate على push/PR/release)
cargo test --release --test performance_budget_tests -- --ignored
```
