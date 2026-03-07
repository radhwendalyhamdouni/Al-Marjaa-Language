use std::time::{Duration, Instant};

use almarjaa::Interpreter;

fn run(code: &str) -> Duration {
    let start = Instant::now();
    let mut interpreter = Interpreter::new();
    interpreter
        .run(code)
        .expect("برنامج benchmark يجب أن يعمل بدون أخطاء");
    start.elapsed()
}

fn assert_budget(elapsed: Duration, budget_ms: u64, benchmark: &str) {
    assert!(
        elapsed <= Duration::from_millis(budget_ms),
        "تجاوز benchmark '{benchmark}' الميزانية: {:?} > {}ms",
        elapsed,
        budget_ms
    );
}

#[test]
#[ignore = "تشغيله في release/CI release gate فقط"]
fn budget_arithmetic_under_200ms() {
    let elapsed = run("متغير س = ١؛ متغير ص = ٢؛ (س + ص) * ١٠؛");
    assert_budget(elapsed, 200, "arithmetics");
}

#[test]
#[ignore = "تشغيله في release/CI release gate فقط"]
fn budget_function_call_under_250ms() {
    let elapsed = run("دالة جمع(أ، ب) { أرجع أ + ب؛ }\nجمع(١٠٠، ٢٣)؛");
    assert_budget(elapsed, 250, "function_call");
}

#[test]
#[ignore = "تشغيله في release/CI release gate فقط"]
fn budget_loop_iteration_under_300ms() {
    let elapsed = run(r#"
        متغير مجموع = ٠؛
        لكل i في مدى(٠، ١٠٠٠) {
            مجموع += i؛
        }
        مجموع؛
    "#);
    assert_budget(elapsed, 300, "loop_iteration");
}

#[test]
#[ignore = "تشغيله في release/CI release gate فقط"]
fn budget_string_interpolation_under_300ms() {
    let elapsed = run(r#"
        متغير اسم = "المصنع"؛
        متغير رقم = ١٢٣؛
        "طلب #{رقم} من {اسم}"؛
    "#);
    assert_budget(elapsed, 300, "string_interpolation");
}
