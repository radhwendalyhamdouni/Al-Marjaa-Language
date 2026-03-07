# Cookbook - وصفات عملية للمرجع

## 1) تشغيل برنامج
```bash
cargo run -- examples/hello.mrj
```

## 2) تحليل بدون تنفيذ
```bash
cargo run -- --compile examples/functions.mrj
```

## 3) تنسيق ملف
```bash
cargo run -- --format examples/loops.mrj
```

## 4) Lint سريع
```bash
cargo run -- --lint examples/conditions.mrj
```

## 5) استخدام المرجع كمكتبة Rust
```rust
use almarjaa::Interpreter;

let mut i = Interpreter::new();
i.run("دالة جمع(أ، ب) { أرجع أ + ب؛ }").unwrap();
let out = i.run("جمع(٣، ٤)").unwrap();
assert_eq!(out.borrow().to_string(), "7");
```

## 6) Golden tests
```bash
cargo test --test golden_tests
```
تُستخدم للتأكد أن:
- مخرجات formatter ثابتة.
- رسائل linter لا تتغير بدون قصد.

## 7) Coverage gate محلياً
```bash
cargo llvm-cov --workspace --all-features --fail-under-lines 80
```

## 8) Benchmark baseline
```bash
cargo test --release --test performance_budget_tests -- --ignored
```


## 9) مثال واجهة احترافية مع انيميشن (HMI)
```bash
cargo run -- examples/hmi_professional_animation.mrj
```
هذا المثال يعرض:
- انتقالات شاشة (Fade steps)
- عدّاد سرعة متحرك
- بطاقات مؤشرات بأسلوب dashboard
- تحديثات حية مع Modbus وسجل أحداث
