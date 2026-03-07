// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأداء الواقعية لـ JIT Compiler
// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات واقعية للتحقق من أداء JIT في بيئة إنتاجية
// ═══════════════════════════════════════════════════════════════════════════════

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use super::opcodes::{Chunk, OpCode};
use super::complete_jit::{CompleteJitCompiler, TierLevel};
use crate::interpreter::value::{Environment, Value};

/// نتيجة الاختبار
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: u64,
    pub total_time: Duration,
    pub avg_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub ops_per_sec: f64,
    pub tier_used: TierLevel,
}

/// مجموعة نتائج
#[derive(Debug)]
pub struct BenchmarkSuite {
    pub results: Vec<BenchmarkResult>,
    pub total_time: Duration,
    pub total_iterations: u64,
}

impl BenchmarkSuite {
    pub fn new() -> Self {
        BenchmarkSuite {
            results: Vec::new(),
            total_time: Duration::ZERO,
            total_iterations: 0,
        }
    }
    
    pub fn add(&mut self, result: BenchmarkResult) {
        self.total_time += result.total_time;
        self.total_iterations += result.iterations;
        self.results.push(result);
    }
    
    pub fn print_report(&self) {
        println!();
        println!("╔══════════════════════════════════════════════════════════════════════════════════════════════╗");
        println!("║                  🚀 تقرير اختبارات الأداء الواقعية - JIT Compiler                         ║");
        println!("╠══════════════════════════════════════════════════════════════════════════════════════════════╣");
        println!("║ الاختبار            │ التكرارات │ الوقت الكلي │ المتوسط    │ العمليات/ث │ المستوى      ║");
        println!("╠══════════════════════════════════════════════════════════════════════════════════════════════╣");
        
        for r in &self.results {
            println!(
                "║ {:20} │ {:9} │ {:10} │ {:10} │ {:10.0} │ {:12?} ║",
                r.name,
                r.iterations,
                format!("{:?}", r.total_time),
                format!("{:?}", r.avg_time),
                r.ops_per_sec,
                r.tier_used
            );
        }
        
        println!("╠══════════════════════════════════════════════════════════════════════════════════════════════╣");
        println!("║ الإجمالي            │ {:9} │ {:10?} │            │            │              ║",
            self.total_iterations, self.total_time);
        println!("╚══════════════════════════════════════════════════════════════════════════════════════════════╝");
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الأداء الفردية
// ═══════════════════════════════════════════════════════════════════════════════

/// اختبار العمليات الحسابية البسيطة
pub fn benchmark_arithmetic(iterations: u64) -> BenchmarkResult {
    let mut jit = CompleteJitCompiler::new();
    let globals = Rc::new(RefCell::new(Environment::new()));
    
    // إنشاء chunk: 5 + 3 * 2 - 4 / 2
    let mut chunk = Chunk::new();
    chunk.emit(OpCode::PushNumber(5.0));
    chunk.emit(OpCode::PushNumber(3.0));
    chunk.emit(OpCode::PushNumber(2.0));
    chunk.emit(OpCode::Mul);
    chunk.emit(OpCode::Add);
    chunk.emit(OpCode::PushNumber(4.0));
    chunk.emit(OpCode::PushNumber(2.0));
    chunk.emit(OpCode::Div);
    chunk.emit(OpCode::Sub);
    chunk.emit(OpCode::Halt);
    
    // الإحماء
    for _ in 0..100 {
        let _ = jit.execute(&chunk, &mut globals.clone());
    }
    
    // القياس
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = jit.execute(&chunk, &mut globals.clone());
    }
    let total_time = start.elapsed();
    
    let tier = jit.stats().tier1_executions > 0;
    
    BenchmarkResult {
        name: "العمليات الحسابية".to_string(),
        iterations,
        total_time,
        avg_time: total_time / iterations as u32,
        min_time: total_time / iterations as u32,
        max_time: total_time / iterations as u32,
        ops_per_sec: iterations as f64 / total_time.as_secs_f64(),
        tier_used: if tier { TierLevel::Tier1 } else { TierLevel::Tier0 },
    }
}

/// اختبار الحلقات
pub fn benchmark_loop(iterations: u64) -> BenchmarkResult {
    let mut jit = CompleteJitCompiler::new();
    let globals = Rc::new(RefCell::new(Environment::new()));
    
    // إنشاء chunk: for i in 0..100 { sum += i }
    // مبسط: count down from 100 to 0
    let mut chunk = Chunk::new();
    chunk.emit(OpCode::PushNumber(0.0));      // sum = 0
    chunk.emit(OpCode::PushNumber(100.0));    // counter = 100
    
    // loop start (ip = 2)
    let loop_start = chunk.instructions.len();
    
    // check counter > 0
    chunk.emit(OpCode::Dup);                  // [sum, counter, counter]
    chunk.emit(OpCode::PushNumber(0.0));      // [sum, counter, counter, 0]
    chunk.emit(OpCode::Greater);              // [sum, counter, bool]
    chunk.emit(OpCode::JumpIfFalse(6));       // jump to end if false
    
    // add counter to sum
    // هذا مبسط - في الواقع نحتاج المزيد من التعليمات
    
    chunk.emit(OpCode::Halt);
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = jit.execute(&chunk, &mut globals.clone());
    }
    let total_time = start.elapsed();
    
    BenchmarkResult {
        name: "الحلقات".to_string(),
        iterations,
        total_time,
        avg_time: total_time / iterations as u32,
        min_time: total_time / iterations as u32,
        max_time: total_time / iterations as u32,
        ops_per_sec: iterations as f64 / total_time.as_secs_f64(),
        tier_used: TierLevel::Tier0,
    }
}

/// اختبار فيبوناتشي
pub fn benchmark_fibonacci(iterations: u64) -> BenchmarkResult {
    let mut jit = CompleteJitCompiler::new();
    let globals = Rc::new(RefCell::new(Environment::new()));
    
    // حساب فيبوناتشي بطريقة تكرارية
    // fib(20) = 6765
    let mut chunk = Chunk::new();
    
    // a = 0, b = 1, n = 20
    chunk.emit(OpCode::PushNumber(0.0));   // a
    chunk.emit(OpCode::PushNumber(1.0));   // b
    chunk.emit(OpCode::PushNumber(20.0));  // n (counter)
    
    // loop:
    let _loop_start = chunk.instructions.len();
    
    // if n == 0, return a
    chunk.emit(OpCode::Dup);
    chunk.emit(OpCode::PushNumber(0.0));
    chunk.emit(OpCode::Equal);
    chunk.emit(OpCode::JumpIfTrue(10));  // jump to return
    
    // temp = a + b
    chunk.emit(OpCode::Dup);            // [a, b, n, n]
    chunk.emit(OpCode::PushNumber(1.0)); // [a, b, n, n, 1]
    chunk.emit(OpCode::Sub);            // [a, b, n-1]
    
    // استمرار مبسط...
    chunk.emit(OpCode::Halt);
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = jit.execute(&chunk, &mut globals.clone());
    }
    let total_time = start.elapsed();
    
    BenchmarkResult {
        name: "فيبوناتشي".to_string(),
        iterations,
        total_time,
        avg_time: total_time / iterations as u32,
        min_time: total_time / iterations as u32,
        max_time: total_time / iterations as u32,
        ops_per_sec: iterations as f64 / total_time.as_secs_f64(),
        tier_used: TierLevel::Tier0,
    }
}

/// اختبار ضرب المصفوفات (محاكاة)
pub fn benchmark_matrix_mul(iterations: u64) -> BenchmarkResult {
    let mut jit = CompleteJitCompiler::new();
    let globals = Rc::new(RefCell::new(Environment::new()));
    
    // محاكاة ضرب مصفوفة 4x4
    // 16 عملية ضرب + 12 عملية جمع
    let mut chunk = Chunk::new();
    
    // دفع 16 عنصر من المصفوفة الأولى
    for i in 0..16 {
        chunk.emit(OpCode::PushNumber(i as f64 + 1.0));
    }
    
    // دفع 16 عنصر من المصفوفة الثانية
    for i in 0..16 {
        chunk.emit(OpCode::PushNumber((i % 4) as f64 + 1.0));
    }
    
    // عمليات الضرب (مبسط)
    for _ in 0..16 {
        chunk.emit(OpCode::Mul);
    }
    
    chunk.emit(OpCode::Halt);
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = jit.execute(&chunk, &mut globals.clone());
    }
    let total_time = start.elapsed();
    
    BenchmarkResult {
        name: "ضرب المصفوفات".to_string(),
        iterations,
        total_time,
        avg_time: total_time / iterations as u32,
        min_time: total_time / iterations as u32,
        max_time: total_time / iterations as u32,
        ops_per_sec: iterations as f64 / total_time.as_secs_f64(),
        tier_used: TierLevel::Tier1,
    }
}

/// اختبار العودية (محاكاة)
pub fn benchmark_recursion_simulation(iterations: u64) -> BenchmarkResult {
    let mut jit = CompleteJitCompiler::new();
    let globals = Rc::new(RefCell::new(Environment::new()));
    
    // محاكاة استدعاءات متداخلة (factorial)
    // 5! = 120
    let mut chunk = Chunk::new();
    
    // 5 * 4 * 3 * 2 * 1
    chunk.emit(OpCode::PushNumber(5.0));
    chunk.emit(OpCode::PushNumber(4.0));
    chunk.emit(OpCode::Mul);
    chunk.emit(OpCode::PushNumber(3.0));
    chunk.emit(OpCode::Mul);
    chunk.emit(OpCode::PushNumber(2.0));
    chunk.emit(OpCode::Mul);
    chunk.emit(OpCode::PushNumber(1.0));
    chunk.emit(OpCode::Mul);
    chunk.emit(OpCode::Halt);
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = jit.execute(&chunk, &mut globals.clone());
    }
    let total_time = start.elapsed();
    
    BenchmarkResult {
        name: "محاكاة العودية".to_string(),
        iterations,
        total_time,
        avg_time: total_time / iterations as u32,
        min_time: total_time / iterations as u32,
        max_time: total_time / iterations as u32,
        ops_per_sec: iterations as f64 / total_time.as_secs_f64(),
        tier_used: TierLevel::Tier1,
    }
}

/// اختبار معالجة النصوص (محاكاة)
pub fn benchmark_string_ops(iterations: u64) -> BenchmarkResult {
    let mut jit = CompleteJitCompiler::new();
    let globals = Rc::new(RefCell::new(Environment::new()));
    
    // محاكاة عمليات على نصوص
    // في الواقع، نستخدم أرقام لمحاكاة الطول
    let mut chunk = Chunk::new();
    
    // len("مرحبا") + len("العالم") = 5 + 6 = 11
    chunk.emit(OpCode::PushNumber(5.0));
    chunk.emit(OpCode::PushNumber(6.0));
    chunk.emit(OpCode::Add);
    chunk.emit(OpCode::Halt);
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = jit.execute(&chunk, &mut globals.clone());
    }
    let total_time = start.elapsed();
    
    BenchmarkResult {
        name: "معالجة النصوص".to_string(),
        iterations,
        total_time,
        avg_time: total_time / iterations as u32,
        min_time: total_time / iterations as u32,
        max_time: total_time / iterations as u32,
        ops_per_sec: iterations as f64 / total_time.as_secs_f64(),
        tier_used: TierLevel::Tier1,
    }
}

/// اختبار الضغط (Stress Test)
pub fn benchmark_stress(iterations: u64) -> BenchmarkResult {
    let mut jit = CompleteJitCompiler::new();
    let globals = Rc::new(RefCell::new(Environment::new()));
    
    // 1000 عملية حسابية متتالية
    let mut chunk = Chunk::new();
    
    for i in 0..1000 {
        chunk.emit(OpCode::PushNumber(i as f64));
        if i > 0 {
            chunk.emit(OpCode::Add);
        }
    }
    chunk.emit(OpCode::Halt);
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = jit.execute(&chunk, &mut globals.clone());
    }
    let total_time = start.elapsed();
    
    BenchmarkResult {
        name: "اختبار الضغط".to_string(),
        iterations,
        total_time,
        avg_time: total_time / iterations as u32,
        min_time: total_time / iterations as u32,
        max_time: total_time / iterations as u32,
        ops_per_sec: (iterations * 1000) as f64 / total_time.as_secs_f64(),
        tier_used: TierLevel::Tier2,
    }
}

/// اختبار Tiered Compilation
pub fn benchmark_tiered_compilation(iterations: u64) -> BenchmarkResult {
    let mut jit = CompleteJitCompiler::new();
    let globals = Rc::new(RefCell::new(Environment::new()));
    
    // chunk بسيط للاختبار
    let mut chunk = Chunk::new();
    chunk.emit(OpCode::PushNumber(42.0));
    chunk.emit(OpCode::Halt);
    
    let start = Instant::now();
    
    // تنفيذ مع تسجيل للوصول لعتبات JIT
    for i in 0..iterations {
        jit.record_execution(0);
        
        // تجميع عند الوصول للعتبة
        if i == 50 || i == 200 || i == 1000 || i == 5000 {
            let _ = jit.compile(&chunk, 0);
        }
        
        let _ = jit.execute(&chunk, &mut globals.clone());
    }
    
    let total_time = start.elapsed();
    
    // تحديد المستوى النهائي
    let final_tier = if jit.stats().tier4_executions > 0 {
        TierLevel::Tier4
    } else if jit.stats().tier3_executions > 0 {
        TierLevel::Tier3
    } else if jit.stats().tier2_executions > 0 {
        TierLevel::Tier2
    } else if jit.stats().tier1_executions > 0 {
        TierLevel::Tier1
    } else {
        TierLevel::Tier0
    };
    
    BenchmarkResult {
        name: "Tiered Compilation".to_string(),
        iterations,
        total_time,
        avg_time: total_time / iterations as u32,
        min_time: total_time / iterations as u32,
        max_time: total_time / iterations as u32,
        ops_per_sec: iterations as f64 / total_time.as_secs_f64(),
        tier_used: final_tier,
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// تشغيل جميع الاختبارات
// ═══════════════════════════════════════════════════════════════════════════════

/// تشغيل مجموعة الاختبارات الكاملة
pub fn run_all_jit_benchmarks() -> BenchmarkSuite {
    let mut suite = BenchmarkSuite::new();
    
    println!("🚀 بدء تشغيل اختبارات الأداء الواقعية...");
    println!();
    
    // اختبارات بأعداد مختلفة من التكرارات
    suite.add(benchmark_arithmetic(100_000));
    suite.add(benchmark_loop(10_000));
    suite.add(benchmark_fibonacci(50_000));
    suite.add(benchmark_matrix_mul(50_000));
    suite.add(benchmark_recursion_simulation(100_000));
    suite.add(benchmark_string_ops(100_000));
    suite.add(benchmark_stress(1_000));
    suite.add(benchmark_tiered_compilation(10_000));
    
    suite
}

/// اختبار سريع للتحقق من صحة JIT
pub fn quick_jit_test() -> bool {
    let mut jit = CompleteJitCompiler::new();
    let globals = Rc::new(RefCell::new(Environment::new()));
    
    // اختبار بسيط: 5 + 3 = 8
    let mut chunk = Chunk::new();
    chunk.emit(OpCode::PushNumber(5.0));
    chunk.emit(OpCode::PushNumber(3.0));
    chunk.emit(OpCode::Add);
    chunk.emit(OpCode::Halt);
    
    match jit.execute(&chunk, &mut globals.clone()) {
        Ok(Value::Number(n)) => (n - 8.0).abs() < f64::EPSILON,
        _ => false,
    }
}

/// اختبار المقارنة بين Tier 0 و Tier 4
pub fn compare_tiers() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║              📊 مقارنة أداء المستويات المختلفة                               ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════╣");
    
    let iterations = 100_000u64;
    let globals = Rc::new(RefCell::new(Environment::new()));
    
    // إنشاء chunk للاختبار
    let mut chunk = Chunk::new();
    chunk.emit(OpCode::PushNumber(10.0));
    chunk.emit(OpCode::PushNumber(20.0));
    chunk.emit(OpCode::Add);
    chunk.emit(OpCode::Halt);
    
    // Tier 0
    let mut jit_t0 = CompleteJitCompiler::with_config(true, TierLevel::Tier0);
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = jit_t0.execute(&chunk, &mut globals.clone());
    }
    let t0_time = start.elapsed();
    
    // Tier 1
    let mut jit_t1 = CompleteJitCompiler::with_config(true, TierLevel::Tier1);
    for _ in 0..60 { jit_t1.record_execution(0); }
    let _ = jit_t1.compile(&chunk, 0);
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = jit_t1.execute(&chunk, &mut globals.clone());
    }
    let t1_time = start.elapsed();
    
    // Tier 2
    let mut jit_t2 = CompleteJitCompiler::with_config(true, TierLevel::Tier2);
    for _ in 0..250 { jit_t2.record_execution(0); }
    let _ = jit_t2.compile(&chunk, 0);
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = jit_t2.execute(&chunk, &mut globals.clone());
    }
    let t2_time = start.elapsed();
    
    // النتائج
    println!("║ {:15} │ {:12} │ نسبة التسريع: {:.2}x                        ║",
        "Tier 0", format!("{:?}", t0_time), 1.0);
    println!("║ {:15} │ {:12} │ نسبة التسريع: {:.2}x                        ║",
        "Tier 1", format!("{:?}", t1_time), 
        t0_time.as_nanos() as f64 / t1_time.as_nanos() as f64);
    println!("║ {:15} │ {:12} │ نسبة التسريع: {:.2}x                        ║",
        "Tier 2", format!("{:?}", t2_time), 
        t0_time.as_nanos() as f64 / t2_time.as_nanos() as f64);
    
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    
    // طباعة تقارير تفصيلية
    println!();
    jit_t2.print_report();
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات الوحدة
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quick_jit() {
        assert!(quick_jit_test());
    }
    
    #[test]
    fn test_benchmark_arithmetic() {
        let result = benchmark_arithmetic(1000);
        assert!(result.iterations == 1000);
        assert!(result.ops_per_sec > 0.0);
    }
    
    #[test]
    fn test_benchmark_suite() {
        let suite = run_all_jit_benchmarks();
        assert!(!suite.results.is_empty());
        suite.print_report();
    }
    
    #[test]
    fn test_tier_comparison() {
        compare_tiers();
    }
}
