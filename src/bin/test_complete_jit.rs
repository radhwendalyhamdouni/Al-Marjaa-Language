// ═══════════════════════════════════════════════════════════════════════════════
// اختبار JIT Compiler الكامل
// ═══════════════════════════════════════════════════════════════════════════════

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use almarjaa::bytecode::{
    Chunk, OpCode, 
    CompleteJitCompiler, CompleteTierLevel,
    run_all_jit_benchmarks, quick_jit_test, compare_tiers,
};
use almarjaa::interpreter::value::{Environment, Value};

fn main() {
    println!();
    println!("╔════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║             🚀 اختبار JIT Compiler الكامل - لغة المرجع                                   ║");
    println!("╚════════════════════════════════════════════════════════════════════════════════════════════╝");
    println!();
    
    // اختبار سريع
    println!("━━━ الاختبار السريع ━━━");
    if quick_jit_test() {
        println!("✅ الاختبار السريع ناجح!");
    } else {
        println!("❌ فشل الاختبار السريع!");
        return;
    }
    println!();
    
    // اختبار المستويات
    test_tier_levels();
    println!();
    
    // اختبار الأداء
    test_performance();
    println!();
    
    // مقارنة المستويات
    compare_tiers();
    println!();
    
    // تشغيل جميع الاختبارات
    let suite = run_all_jit_benchmarks();
    suite.print_report();
    println!();
    
    // اختبار نهائي شامل
    comprehensive_test();
}

fn test_tier_levels() {
    println!("━━━ اختبار المستويات ━━━");
    
    let mut jit = CompleteJitCompiler::new();
    
    // اختبار تحديد المستويات
    let test_cases = [
        (0, CompleteTierLevel::Tier0),
        (50, CompleteTierLevel::Tier1),
        (200, CompleteTierLevel::Tier2),
        (1000, CompleteTierLevel::Tier3),
        (5000, CompleteTierLevel::Tier4),
        (10000, CompleteTierLevel::Tier4),
    ];
    
    for (count, expected) in test_cases {
        let tier = jit.determine_tier(count);
        let status = if tier == expected { "✅" } else { "❌" };
        println!("  {} تنفيذ {} → {:?} (متوقع {:?})", status, count, tier, expected);
    }
}

fn test_performance() {
    println!("━━━ اختبار الأداء ━━━");
    
    let mut jit = CompleteJitCompiler::new();
    let globals = Rc::new(RefCell::new(Environment::new()));
    
    // إنشاء chunk للاختبار: حساب مجموع 1+2+3+...+100
    let mut chunk = Chunk::new();
    
    // دفع الأرقام وجمعها
    for i in 1..=100 {
        chunk.emit(OpCode::PushNumber(i as f64));
        if i > 1 {
            chunk.emit(OpCode::Add);
        }
    }
    chunk.emit(OpCode::Halt);
    
    // الإحماء
    for _ in 0..100 {
        let _ = jit.execute(&chunk, &mut globals.clone());
    }
    
    // قياس الأداء
    let iterations = 10000u64;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = jit.execute(&chunk, &mut globals.clone());
    }
    
    let elapsed = start.elapsed();
    let ops_per_sec = (iterations * 100) as f64 / elapsed.as_secs_f64();
    
    println!("  التكرارات: {}", iterations);
    println!("  الوقت الكلي: {:?}", elapsed);
    println!("  المتوسط: {:?}", elapsed / iterations as u32);
    println!("  العمليات/ثانية: {:.0}", ops_per_sec);
    
    // عرض الإحصائيات
    jit.print_report();
}

fn comprehensive_test() {
    println!("╔════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                       🎯 الاختبار الشامل النهائي                                         ║");
    println!("╠════════════════════════════════════════════════════════════════════════════════════════════╣");
    
    let mut jit = CompleteJitCompiler::new();
    let globals = Rc::new(RefCell::new(Environment::new()));
    
    // اختبارات متنوعة
    let tests = [
        ("الجمع", vec![OpCode::PushNumber(5.0), OpCode::PushNumber(3.0), OpCode::Add, OpCode::Halt], 8.0),
        ("الطرح", vec![OpCode::PushNumber(10.0), OpCode::PushNumber(4.0), OpCode::Sub, OpCode::Halt], 6.0),
        ("الضرب", vec![OpCode::PushNumber(7.0), OpCode::PushNumber(6.0), OpCode::Mul, OpCode::Halt], 42.0),
        ("القسمة", vec![OpCode::PushNumber(20.0), OpCode::PushNumber(4.0), OpCode::Div, OpCode::Halt], 5.0),
        ("الأس", vec![OpCode::PushNumber(2.0), OpCode::PushNumber(10.0), OpCode::Pow, OpCode::Halt], 1024.0),
    ];
    
    let mut passed = 0;
    let mut failed = 0;
    
    for (name, ops, expected) in tests {
        let mut chunk = Chunk::new();
        for op in ops {
            chunk.emit(op);
        }
        
        // تنفيذ عدة مرات لتفعيل JIT
        for _ in 0..60 {
            jit.record_execution(0);
            let _ = jit.execute(&chunk, &mut globals.clone());
        }
        
        // تجميع
        let _ = jit.compile(&chunk, 0);
        
        // التنفيذ النهائي
        match jit.execute(&chunk, &mut globals.clone()) {
            Ok(Value::Number(n)) if (n - expected).abs() < f64::EPSILON => {
                println!("║ ✅ {:15} = {} (صحيح)                                            ║", name, n);
                passed += 1;
            }
            Ok(Value::Number(n)) => {
                println!("║ ❌ {:15} = {} (متوقع {})                                    ║", name, n, expected);
                failed += 1;
            }
            Ok(other) => {
                println!("║ ❌ {:15} = {:?} (نوع غير متوقع)                              ║", name, other);
                failed += 1;
            }
            Err(e) => {
                println!("║ ❌ {:15} خطأ: {}                                        ║", name, e);
                failed += 1;
            }
        }
    }
    
    println!("╠════════════════════════════════════════════════════════════════════════════════════════════╣");
    println!("║ النتيجة: {} نجاح، {} فشل                                                            ║", passed, failed);
    println!("╚════════════════════════════════════════════════════════════════════════════════════════════╝");
    
    // طباعة التقرير النهائي
    println!();
    jit.print_report();
}
