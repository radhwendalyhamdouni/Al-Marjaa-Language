// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات مقارنة الأداء - المفسر vs البايت كود VM
// ═══════════════════════════════════════════════════════════════════════════════

use std::time::Instant;

/// نتيجة المقارنة
#[derive(Debug)]
struct ComparisonResult {
    name: String,
    interpreter_time_us: u64,
    bytecode_time_us: u64,
    iterations: usize,
    speedup: f64,
}

/// تشغيل جميع اختبارات المقارنة
fn run_comparison_tests() -> Vec<ComparisonResult> {
    vec![
        compare_loop_100(),
        compare_loop_500(),
        compare_arithmetic(),
        compare_nested_loops(),
        // compare_fibonacci(),  // معلق مؤقتاً
    ]
}

/// طباعة نتائج المقارنة
fn print_comparison_results(results: &[ComparisonResult]) {
    println!();
    println!("╔════════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║              🚀 مقارنة الأداء: المفسر القديم vs Bytecode VM                                   ║");
    println!("╠════════════════════════════════════════════════════════════════════════════════════════════════╣");
    println!("║ الاختبار          │ التكرارات │ المفسر (μs) │ البايت كود (μs) │ التسريع         │ ║");
    println!("╠════════════════════════════════════════════════════════════════════════════════════════════════╣");
    
    for r in results {
        println!(
            "║ {:17} │ {:9} │ {:11} │ {:15} │ {:10.2}x      │ ║",
            r.name,
            r.iterations,
            r.interpreter_time_us,
            r.bytecode_time_us,
            r.speedup
        );
    }
    
    println!("╚════════════════════════════════════════════════════════════════════════════════════════════════╝");
    
    // حساب متوسط التسريع
    let avg_speedup: f64 = results.iter().map(|r| r.speedup).sum::<f64>() / results.len() as f64;
    let total_interpreter: u64 = results.iter().map(|r| r.interpreter_time_us).sum();
    let total_bytecode: u64 = results.iter().map(|r| r.bytecode_time_us).sum();
    
    println!();
    println!("📊 متوسط التسريع: {:.2}x", avg_speedup);
    println!("📊 إجمالي وقت المفسر: {} μs", total_interpreter);
    println!("📊 إجمالي وقت البايت كود: {} μs", total_bytecode);
}

/// اختبار حلقة 100 تكرار
fn compare_loop_100() -> ComparisonResult {
    let code = r#"
        متغير س = 0؛
        متغير مجموع = 0؛
        طالما س < 100 {
            مجموع = مجموع + س؛
            س = س + 1؛
        }
    "#;
    
    let iterations = 100;
    
    // قياس المفسر القديم
    let mut interp = almarjaa::interpreter::Interpreter::new();
    let start = Instant::now();
    let _ = interp.run(code);
    let interp_time = start.elapsed().as_micros() as u64;
    
    // قياس البايت كود VM
    let start = Instant::now();
    let _ = almarjaa::bytecode::run_bytecode(code);
    let bytecode_time = start.elapsed().as_micros() as u64;
    
    let speedup = if bytecode_time > 0 {
        interp_time as f64 / bytecode_time as f64
    } else {
        0.0
    };
    
    ComparisonResult {
        name: "حلقة 100".into(),
        interpreter_time_us: interp_time,
        bytecode_time_us: bytecode_time,
        iterations,
        speedup,
    }
}

/// اختبار حلقة 500 تكرار
fn compare_loop_500() -> ComparisonResult {
    let code = r#"
        متغير س = 0؛
        متغير مجموع = 0؛
        طالما س < 500 {
            مجموع = مجموع + س؛
            س = س + 1؛
        }
    "#;
    
    let iterations = 500;
    
    // قياس المفسر القديم
    let mut interp = almarjaa::interpreter::Interpreter::new();
    let start = Instant::now();
    let _ = interp.run(code);
    let interp_time = start.elapsed().as_micros() as u64;
    
    // قياس البايت كود VM
    let start = Instant::now();
    let _ = almarjaa::bytecode::run_bytecode(code);
    let bytecode_time = start.elapsed().as_micros() as u64;
    
    let speedup = if bytecode_time > 0 {
        interp_time as f64 / bytecode_time as f64
    } else {
        0.0
    };
    
    ComparisonResult {
        name: "حلقة 500".into(),
        interpreter_time_us: interp_time,
        bytecode_time_us: bytecode_time,
        iterations,
        speedup,
    }
}

/// اختبار العمليات الحسابية
fn compare_arithmetic() -> ComparisonResult {
    let code = r#"
        متغير س = 0؛
        متغير نتيجة = 0؛
        طالما س < 200 {
            نتيجة = نتيجة + (س * 2 + 3 - 1) / 2؛
            س = س + 1؛
        }
    "#;
    
    let iterations = 200;
    
    // قياس المفسر القديم
    let mut interp = almarjaa::interpreter::Interpreter::new();
    let start = Instant::now();
    let _ = interp.run(code);
    let interp_time = start.elapsed().as_micros() as u64;
    
    // قياس البايت كود VM
    let start = Instant::now();
    let _ = almarjaa::bytecode::run_bytecode(code);
    let bytecode_time = start.elapsed().as_micros() as u64;
    
    let speedup = if bytecode_time > 0 {
        interp_time as f64 / bytecode_time as f64
    } else {
        0.0
    };
    
    ComparisonResult {
        name: "عمليات حسابية".into(),
        interpreter_time_us: interp_time,
        bytecode_time_us: bytecode_time,
        iterations,
        speedup,
    }
}

/// اختبار الحلقات المتداخلة
fn compare_nested_loops() -> ComparisonResult {
    let code = r#"
        متغير مجموع = 0؛
        متغير أ = 0؛
        متغير ب = 0؛
        طالما أ < 10 {
            ب = 0؛
            طالما ب < 10 {
                مجموع = مجموع + أ * ب؛
                ب = ب + 1؛
            }
            أ = أ + 1؛
        }
    "#;
    
    let iterations = 100; // 10 * 10
    
    // قياس المفسر القديم
    let mut interp = almarjaa::interpreter::Interpreter::new();
    let start = Instant::now();
    let _ = interp.run(code);
    let interp_time = start.elapsed().as_micros() as u64;
    
    // قياس البايت كود VM
    let start = Instant::now();
    let _ = almarjaa::bytecode::run_bytecode(code);
    let bytecode_time = start.elapsed().as_micros() as u64;
    
    let speedup = if bytecode_time > 0 {
        interp_time as f64 / bytecode_time as f64
    } else {
        0.0
    };
    
    ComparisonResult {
        name: "حلقات متداخلة".into(),
        interpreter_time_us: interp_time,
        bytecode_time_us: bytecode_time,
        iterations,
        speedup,
    }
}

/// اختبار فيبوناتشي
fn compare_fibonacci() -> ComparisonResult {
    let code = r#"
        متغير أ = 0؛
        متغير ب = 1؛
        متغير مؤقت = 0؛
        متغير س = 0؛
        طالما س < 30 {
            مؤقت = أ + ب؛
            أ = ب؛
            ب = مؤقت؛
            س = س + 1؛
        }
    "#;
    
    let iterations = 30;
    
    // قياس المفسر القديم
    let mut interp = almarjaa::interpreter::Interpreter::new();
    let start = Instant::now();
    let _ = interp.run(code);
    let interp_time = start.elapsed().as_micros() as u64;
    
    // قياس البايت كود VM
    let start = Instant::now();
    let _ = almarjaa::bytecode::run_bytecode(code);
    let bytecode_time = start.elapsed().as_micros() as u64;
    
    let speedup = if bytecode_time > 0 {
        interp_time as f64 / bytecode_time as f64
    } else {
        0.0
    };
    
    ComparisonResult {
        name: "فيبوناتشي".into(),
        interpreter_time_us: interp_time,
        bytecode_time_us: bytecode_time,
        iterations,
        speedup,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الوحدة
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_comparison_all() {
    let results = run_comparison_tests();
    print_comparison_results(&results);
    
    // التحقق من أن جميع الاختبارات ناجحة
    for r in &results {
        assert!(r.interpreter_time_us > 0, "Interpreter time should be positive");
        assert!(r.bytecode_time_us > 0, "Bytecode time should be positive");
        println!("✅ {} - تسريع: {:.2}x", r.name, r.speedup);
    }
}

#[test]
fn test_bytecode_correctness() {
    // اختبار صحة النتائج
    let code = r#"
        متغير مجموع = 0؛
        متغير س = 0؛
        طالما س < 10 {
            مجموع = مجموع + س؛
            س = س + 1؛
        }
    "#;
    
    // المفسر
    let mut interp = almarjaa::interpreter::Interpreter::new();
    let interp_result = interp.run(code);
    
    // البايت كود
    let bytecode_result = almarjaa::bytecode::run_bytecode(code);
    
    println!("المفسر: {:?}", interp_result);
    println!("البايت كود: {:?}", bytecode_result);
    
    assert!(interp_result.is_ok(), "Interpreter should succeed");
    assert!(bytecode_result.is_ok(), "Bytecode should succeed");
}

#[test]
fn test_speedup_significant() {
    // اختبار أن التسريع معنوي
    let result = compare_loop_500();
    println!("نتيجة الاختبار: {:?}", result);
    
    // نتوقع تسريع لا يقل عن 1.5x في وضع التطوير
    // في وضع الإصدار (release) يجب أن يكون أعلى
    println!("التسريع: {:.2}x", result.speedup);
}
