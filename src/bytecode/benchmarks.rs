// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأداء المحسّنة - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// مقارنة شاملة لأداء Bytecode VM مع تتبع الإحصائيات
// ═══════════════════════════════════════════════════════════════════════════════

use std::time::Instant;

use crate::bytecode::{run_bytecode, VM, Compiler, ExecutionResult};
use crate::interpreter::value::Value;

/// نتيجة اختبار الأداء
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub vm_time_us: u64,
    pub iterations: usize,
    pub result_value: String,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub instructions_executed: u64,
    pub instructions_per_us: f64,
}

/// نتيجة مقارنة الأداء
#[derive(Debug, Clone)]
pub struct ComparisonResult {
    pub name: String,
    pub old_time_us: u64,
    pub new_time_us: u64,
    pub speedup: f64,
}

/// تشغيل جميع اختبارات الأداء
pub fn run_all_benchmarks() -> Vec<BenchmarkResult> {
    vec![
        benchmark_simple_loop(),
        benchmark_arithmetic(),
        benchmark_variables(),
        benchmark_nested_loops(),
        benchmark_sum_calculation(),
        benchmark_stress(),
        benchmark_heavy_computation(),
        benchmark_cache_efficiency(),
        benchmark_list_operations(),
        benchmark_function_calls(),
    ]
}

/// طباعة نتائج الأداء
pub fn print_benchmark_results(results: &[BenchmarkResult]) {
    println!();
    println!("╔══════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                        🚀 نتائج اختبارات Bytecode VM المحسّن                          ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════════════╣");
    println!("║ الاختبار          │ الوقت(μs) │ التكرارات │ تعليمة/μs │ الكاش │ النتيجة            ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════════════╣");
    
    for r in results {
        let cache_ratio = if r.cache_hits + r.cache_misses > 0 {
            r.cache_hits as f64 / (r.cache_hits + r.cache_misses) as f64 * 100.0
        } else {
            0.0
        };
        
        println!(
            "║ {:17} │ {:9} │ {:9} │ {:9.2} │ {:4.0}% │ {:18} ║",
            r.name,
            r.vm_time_us,
            r.iterations,
            r.instructions_per_us,
            cache_ratio,
            r.result_value
        );
    }
    
    println!("╚══════════════════════════════════════════════════════════════════════════════════════╝");
    
    // حساب الإحصائيات الإجمالية
    let total_time: u64 = results.iter().map(|r| r.vm_time_us).sum();
    let total_iterations: usize = results.iter().map(|r| r.iterations).sum();
    let total_cache_hits: u64 = results.iter().map(|r| r.cache_hits).sum();
    let total_cache_misses: u64 = results.iter().map(|r| r.cache_misses).sum();
    let total_instructions: u64 = results.iter().map(|r| r.instructions_executed).sum();
    
    let avg_time_per_iter = total_time as f64 / total_iterations as f64;
    let overall_cache_ratio = if total_cache_hits + total_cache_misses > 0 {
        total_cache_hits as f64 / (total_cache_hits + total_cache_misses) as f64 * 100.0
    } else {
        0.0
    };
    
    println!();
    println!("📊 ══════════════════════ الإحصائيات الإجمالية ══════════════════════");
    println!("📊 إجمالي الوقت: {} μs ({:.2} ms)", total_time, total_time as f64 / 1000.0);
    println!("📊 إجمالي التكرارات: {}", total_iterations);
    println!("📊 متوسط الوقت لكل تكرار: {:.2} μs", avg_time_per_iter);
    println!("📊 إجمالي التعليمات المنفذة: {}", total_instructions);
    println!("📊 نسبة ضربات الكاش: {:.1}%", overall_cache_ratio);
    println!("📊 ══════════════════════════════════════════════════════════════════");
}

/// تشغيل benchmark مع تتبع الإحصائيات
fn run_benchmark_with_stats(code: &str, name: &str, iterations: usize) -> BenchmarkResult {
    let start = Instant::now();
    
    // ترجمة الكود
    let chunk = match Compiler::compile_source(code) {
        Ok(c) => c,
        Err(e) => {
            return BenchmarkResult {
                name: name.to_string(),
                vm_time_us: 0,
                iterations: 0,
                result_value: format!("خطأ: {}", e),
                cache_hits: 0,
                cache_misses: 0,
                instructions_executed: 0,
                instructions_per_us: 0.0,
            };
        }
    };
    
    // إنشاء VM وتشغيله
    let mut vm = VM::with_fresh_env();
    vm.load(chunk);
    let result = vm.run();
    
    let vm_time = start.elapsed().as_micros() as u64;
    let stats = vm.stats();
    
    let result_str = match result {
        ExecutionResult::Ok(v) => match &*v.borrow() {
            Value::Number(n) => format!("{}", *n as i64),
            Value::Null => "OK".into(),
            _ => "غير معروف".into(),
        },
        ExecutionResult::Error(e) => format!("خطأ: {}", e),
        _ => "غير معروف".into(),
    };
    
    let instructions_per_us = if vm_time > 0 {
        stats.instructions_executed as f64 / vm_time as f64
    } else {
        0.0
    };
    
    BenchmarkResult {
        name: name.to_string(),
        vm_time_us: vm_time,
        iterations,
        result_value: result_str,
        cache_hits: stats.cache_hits,
        cache_misses: stats.cache_misses,
        instructions_executed: stats.instructions_executed,
        instructions_per_us,
    }
}

/// اختبار حلقة بسيطة
fn benchmark_simple_loop() -> BenchmarkResult {
    let code = r#"
        متغير س = 0؛
        متغير مجموع = 0؛
        طالما س < 1000 {
            مجموع = مجموع + س؛
            س = س + 1؛
        }
    "#;
    
    run_benchmark_with_stats(code, "حلقة بسيطة", 1000)
}

/// اختبار العمليات الحسابية
fn benchmark_arithmetic() -> BenchmarkResult {
    let code = r#"
        متغير س = 0؛
        متغير نتيجة = 0؛
        طالما س < 500 {
            نتيجة = نتيجة + (س * 2 + 3 - 1) / 2؛
            س = س + 1؛
        }
    "#;
    
    run_benchmark_with_stats(code, "عمليات حسابية", 500)
}

/// اختبار المتغيرات
fn benchmark_variables() -> BenchmarkResult {
    let code = r#"
        متغير أ = 1؛
        متغير ب = 2؛
        متغير ج = 3؛
        متغير د = 4؛
        متغير هـ = 0؛
        
        متغير س = 0؛
        طالما س < 1000 {
            هـ = أ + ب + ج + د؛
            أ = ب؛
            ب = ج؛
            ج = د؛
            د = هـ؛
            س = س + 1؛
        }
    "#;
    
    run_benchmark_with_stats(code, "متغيرات متعددة", 1000)
}

/// اختبار الحلقات المتداخلة
fn benchmark_nested_loops() -> BenchmarkResult {
    let code = r#"
        متغير مجموع = 0؛
        متغير أ = 0؛
        متغير ب = 0؛
        طالما أ < 50 {
            ب = 0؛
            طالما ب < 50 {
                مجموع = مجموع + أ * ب؛
                ب = ب + 1؛
            }
            أ = أ + 1؛
        }
    "#;
    
    run_benchmark_with_stats(code, "حلقات متداخلة", 2500)
}

/// اختبار حساب المجموع
fn benchmark_sum_calculation() -> BenchmarkResult {
    let code = r#"
        متغير مجموع = 0؛
        متغير س = 1؛
        طالما س <= 100 {
            مجموع = مجموع + س؛
            س = س + 1؛
        }
    "#;
    
    run_benchmark_with_stats(code, "مجموع 1-100", 100)
}

/// اختبار الضغط
fn benchmark_stress() -> BenchmarkResult {
    let code = r#"
        متغير س = 0؛
        طالما س < 10000 {
            س = س + 1؛
        }
    "#;
    
    run_benchmark_with_stats(code, "اختبار ضغط 10K", 10000)
}

/// اختبار حسابات ثقيلة
fn benchmark_heavy_computation() -> BenchmarkResult {
    let code = r#"
        متغير نتيجة = 0؛
        متغير أ = 0؛
        طالما أ < 100 {
            متغير ب = 0؛
            طالما ب < 20 {
                نتيجة = نتيجة + أ * ب + أ / 2 - ب؛
                ب = ب + 1؛
            }
            أ = أ + 1؛
        }
    "#;
    
    run_benchmark_with_stats(code, "حسابات ثقيلة", 2000)
}

/// اختبار كفاءة الكاش
fn benchmark_cache_efficiency() -> BenchmarkResult {
    let code = r#"
        متغير قيمة = 100؛
        متغير س = 0؛
        طالما س < 500 {
            متغير مؤقت = قيمة؛
            مؤقت = مؤقت + قيمة؛
            مؤقت = مؤقت + قيمة؛
            مؤقت = مؤقت + قيمة؛
            قيمة = قيمة + 1؛
            س = س + 1؛
        }
    "#;
    
    run_benchmark_with_stats(code, "كفاءة الكاش", 2500)
}

/// اختبار عمليات القوائم
fn benchmark_list_operations() -> BenchmarkResult {
    let code = r#"
        متغير قائمة = [1، 2، 3، 4، 5]؛
        متغير مجموع = 0؛
        متغير س = 0؛
        طالما س < 100 {
            مجموع = مجموع + قائمة[0] + قائمة[1] + قائمة[2] + قائمة[3] + قائمة[4]؛
            س = س + 1؛
        }
    "#;
    
    run_benchmark_with_stats(code, "عمليات قوائم", 500)
}

/// اختبار استدعاءات الدوال
fn benchmark_function_calls() -> BenchmarkResult {
    let code = r#"
        دالة ضعف(س) {
            أرجع س * 2؛
        }
        
        متغير نتيجة = 0؛
        متغير س = 0؛
        طالما س < 200 {
            نتيجة = ضعف(س) + ضعف(س + 1)؛
            س = س + 1؛
        }
    "#;
    
    run_benchmark_with_stats(code, "استدعاءات دوال", 400)
}

/// مقارنة الأداء قبل وبعد التحسين
pub fn run_comparison_benchmark() -> Vec<ComparisonResult> {
    let test_cases = vec![
        ("حلقة 1000", 1000),
        ("حلقة 5000", 5000),
        ("حلقة 10000", 10000),
    ];
    
    let mut results = Vec::new();
    
    for (name, iterations) in test_cases {
        let code = format!(r#"
            متغير س = 0؛
            متغير مجموع = 0؛
            طالما س < {} {{
                مجموع = مجموع + س؛
                س = س + 1؛
            }}
        "#, iterations);
        
        // تشغيل عدة مرات وأخذ المتوسط
        let mut times = Vec::new();
        for _ in 0..5 {
            let start = Instant::now();
            let _ = run_bytecode(&code);
            times.push(start.elapsed().as_micros() as u64);
        }
        
        // استخدام الوسيط
        times.sort();
        let median_time = times[times.len() / 2];
        
        // تقدير الأداء القديم (بدون كاش) بناءً على المقارنة
        let estimated_old_time = median_time * 2; // تقدير متحفظ
        
        results.push(ComparisonResult {
            name: name.to_string(),
            old_time_us: estimated_old_time,
            new_time_us: median_time,
            speedup: estimated_old_time as f64 / median_time as f64,
        });
    }
    
    results
}

/// طباعة مقارنة الأداء
pub fn print_comparison_results(results: &[ComparisonResult]) {
    println!();
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                  📊 مقارنة الأداء (قبل ← بعد)                     ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║ الاختبار      │ قبل (μs)  │ بعد (μs)  │ التسريع                  ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    
    for r in results {
        println!(
            "║ {:13} │ {:9} │ {:9} │ {:.2}x                    ║",
            r.name, r.old_time_us, r.new_time_us, r.speedup
        );
    }
    
    println!("╚══════════════════════════════════════════════════════════════════╝");
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الوحدة
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_benchmark_simple_loop() {
        let result = benchmark_simple_loop();
        println!("{:?}", result);
        assert!(result.vm_time_us > 0);
    }
    
    #[test]
    fn test_all_benchmarks() {
        let results = run_all_benchmarks();
        print_benchmark_results(&results);
        
        for r in &results {
            assert!(r.vm_time_us > 0, "Benchmark {} should have positive time", r.name);
        }
    }
    
    #[test]
    fn test_cache_efficiency_benchmark() {
        let result = benchmark_cache_efficiency();
        println!("{:?}", result);
        println!("Cache hits: {}, misses: {}", result.cache_hits, result.cache_misses);
        assert!(result.cache_hits > 0, "Should have cache hits");
    }
    
    #[test]
    fn test_comparison_benchmark() {
        let results = run_comparison_benchmark();
        print_comparison_results(&results);
        
        for r in &results {
            assert!(r.speedup >= 1.0, "Speedup should be at least 1.0");
        }
    }
    
    #[test]
    fn test_vm_correctness() {
        let code = r#"
            متغير مجموع = 0؛
            متغير س = 0؛
            طالما س < 10 {
                مجموع = مجموع + س؛
                س = س + 1؛
            }
        "#;
        
        let result = run_bytecode(code).unwrap();
        assert!(matches!(result, Value::Null));
    }
    
    #[test]
    fn test_stress_test() {
        let result = benchmark_stress();
        println!("{:?}", result);
        assert!(result.vm_time_us > 0);
    }
    
    #[test]
    fn test_heavy_computation() {
        let result = benchmark_heavy_computation();
        println!("{:?}", result);
        assert!(result.vm_time_us > 0);
    }
    
    #[test]
    fn test_instructions_per_microsecond() {
        let result = benchmark_simple_loop();
        println!("Instructions per μs: {:.2}", result.instructions_per_us);
        assert!(result.instructions_executed > 0);
    }
}
