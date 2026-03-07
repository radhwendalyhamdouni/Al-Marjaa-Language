// ═══════════════════════════════════════════════════════════════════════════════
// نظام الـ Bytecode - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// هذا المodule يوفر:
// - تعليمات Bytecode (opcodes)
// - مترجم من AST إلى Bytecode
// - آلة افتراضية سريعة مبنية على المكدس
// - JIT Compiler للكود الساخن
// - Tiered Compilation (مستويات متعددة من التحسين)
// - Tracing JIT (تتبع مسارات التنفيذ)
// - SIMD Operations (تعليمات المتجهات)
// - Threaded Code (تنفيذ متوازي)
// - Parallel Garbage Collector (جامع قمامة متوازي)
// - Type Inference (استنباط الأنواع)
// - PGO (Profile-Guided Optimization)
// - Async/Await JIT Support
// - WebAssembly Compilation Target
// - GC-JIT Integration
// - AOT (Ahead-of-Time) Compilation
// ═══════════════════════════════════════════════════════════════════════════════

pub mod opcodes;
pub mod compiler;
pub mod vm;
pub mod benchmarks;
pub mod jit;
pub mod advanced_jit;
pub mod complete_jit;
pub mod jit_benchmarks;
pub mod gc;
pub mod optimizer;
pub mod type_inference;
pub mod pgo;
pub mod async_jit;
pub mod wasm_target;
pub mod gc_jit_integration;
pub mod aot_compiler;

// إعادة تصدير الأنواع الرئيسية
pub use opcodes::{Chunk, OpCode};
pub use compiler::{Compiler, CompileResult};
pub use vm::{VM, ExecutionResult, VMStats};
pub use benchmarks::{run_all_benchmarks, print_benchmark_results, BenchmarkResult};
pub use jit::{JitCompiler, JitStats, OptimizedExecutor, CompiledCode, HotSpotInfo};

// تصدير JIT المتقدم
pub use advanced_jit::{
    AdvancedJitCompiler, AdvancedJitStats,
    TierLevel, TierInfo, TierThresholds,
    TracingRecorder, Trace, TraceEntry, TraceState, CompiledTrace,
    SimdProcessor, SimdStats, SimdOperation,
    ThreadedCodeExecutor, ThreadedStats, ThreadPool,
};

// تصدير جامع القمامة المتوازي
pub use gc::{
    ParallelGc, GcStats, GcObjectId, GcObjectInfo,
    Generation, WriteBarrier, MemoryManager,
};

// تصدير المُحسِّن
pub use optimizer::{
    Optimizer, OptimizationResult, OptimizationDetail,
    OptimizationKind, OptimizationLevel,
};

// تصدير JIT الكامل
pub use complete_jit::{
    CompleteJitCompiler, JitStats as CompleteJitStats,
    TierLevel as CompleteTierLevel, HotSpotInfo as CompleteHotSpotInfo,
    CompiledCode as CompleteCompiledCode,
    ExecutionResult as CompleteExecutionResult,
};

// تصدير اختبارات JIT
pub use jit_benchmarks::{
    run_all_jit_benchmarks, quick_jit_test, compare_tiers,
    BenchmarkResult as JitBenchmarkResult, BenchmarkSuite,
};

// تصدير Type Inference
pub use type_inference::{
    Type, TypeInferenceEngine, TypeAnalysisResult, TypeGuard,
    TypeGuardKind, VariableTypeInfo, TypeError, TypeWarning,
};

// تصدير PGO
pub use pgo::{
    ProfilingManager, PgoOptimizer, PgoOptimizationStats,
    InstructionProfile, FunctionProfile, LoopProfile, BranchProfile,
    OptimizationDecision,
};

// تصدير Async JIT
pub use async_jit::{
    AsyncJitCompiler, AsyncJitStats, AsyncStateMachine, AsyncState,
    AsyncResult, AwaitPoint, AwaitType, AsyncRuntime,
};

// تصدير WASM Target
pub use wasm_target::{
    WasmCompiler, WasmModule, WasmFunction, WasmType, WasmInstruction,
    WasiSupport, WasmCompilerStats,
};

// تصدير GC-JIT Integration
pub use gc_jit_integration::{
    GcJitCoordinator, SafepointManager, Safepoint, WriteBarrierGenerator,
    AllocationManager, AllocationStats, WriteBarrierType,
};

// تصدير AOT Compiler
pub use aot_compiler::{
    AotCompiler, AotSettings, AotCompilerStats, CompilationUnit,
    CompiledModule, CompiledFunction, OptimizationLevel as AotOptimizationLevel,
};

// ═══════════════════════════════════════════════════════════════════════════════
// دوال سهلة الاستخدام
// ═══════════════════════════════════════════════════════════════════════════════

use std::cell::RefCell;
use std::rc::Rc;

use crate::interpreter::value::{Environment, Value};

/// تشغيل كود المرجع باستخدام الـ VM
pub fn run_bytecode(source: &str) -> Result<Value, String> {
    // ترجمة
    let chunk = Compiler::compile_source(source)?;
    
    // إنشاء VM مع البيئة الافتراضية
    let globals = Rc::new(RefCell::new(Environment::new()));
    
    // تعريف الدوال الأصلية
    define_native_functions(&globals);
    
    // تشغيل
    let mut vm = VM::new(globals);
    vm.load(chunk);
    
    match vm.run() {
        ExecutionResult::Ok(v) => Ok((*v.borrow()).clone()),
        ExecutionResult::Error(e) => Err(e),
        ExecutionResult::Return(v) => Ok((*v.borrow()).clone()),
        _ => Ok(Value::Null),
    }
}

/// تشغيل كود مع قياس الأداء
pub fn run_bytecode_benchmark(source: &str) -> Result<(Value, VMStats), String> {
    let chunk = Compiler::compile_source(source)?;
    let globals = Rc::new(RefCell::new(Environment::new()));
    define_native_functions(&globals);
    
    let mut vm = VM::new(globals);
    vm.load(chunk);
    
    let result = vm.run();
    let stats = vm.stats().clone();
    
    match result {
        ExecutionResult::Ok(v) => Ok(((*v.borrow()).clone(), stats)),
        ExecutionResult::Error(e) => Err(e),
        ExecutionResult::Return(v) => Ok(((*v.borrow()).clone(), stats)),
        _ => Ok((Value::Null, stats)),
    }
}

/// تعريف الدوال الأصلية الأساسية
fn define_native_functions(env: &Rc<RefCell<Environment>>) {
    // دوال أساسية
    env.borrow_mut().define(
        "اطبع",
        Value::NativeFunction {
            name: "اطبع".to_string(),
            func: |args| {
                for arg in args {
                    print!("{} ", arg.borrow().to_string_value());
                }
                println!();
                Ok(Rc::new(RefCell::new(Value::Null)))
            },
        },
        false,
    );
    
    env.borrow_mut().define(
        "نص",
        Value::NativeFunction {
            name: "نص".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Ok(Rc::new(RefCell::new(Value::String(String::new()))));
                }
                Ok(Rc::new(RefCell::new(Value::String(
                    args[0].borrow().to_string_value()
                ))))
            },
        },
        false,
    );
    
    env.borrow_mut().define(
        "رقم",
        Value::NativeFunction {
            name: "رقم".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Ok(Rc::new(RefCell::new(Value::Number(0.0))));
                }
                match args[0].borrow().to_number() {
                    Ok(n) => Ok(Rc::new(RefCell::new(Value::Number(n)))),
                    Err(e) => Err(e),
                }
            },
        },
        false,
    );
    
    env.borrow_mut().define(
        "طول",
        Value::NativeFunction {
            name: "طول".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Ok(Rc::new(RefCell::new(Value::Number(0.0))));
                }
                let len = match &*args[0].borrow() {
                    Value::List(l) => l.len(),
                    Value::String(s) => s.chars().count(),
                    Value::Dictionary(d) => d.len(),
                    _ => 0,
                };
                Ok(Rc::new(RefCell::new(Value::Number(len as f64))))
            },
        },
        false,
    );
    
    env.borrow_mut().define(
        "نوع",
        Value::NativeFunction {
            name: "نوع".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Ok(Rc::new(RefCell::new(Value::String("لا_شيء".into()))));
                }
                Ok(Rc::new(RefCell::new(Value::String(
                    args[0].borrow().type_name().to_string()
                ))))
            },
        },
        false,
    );
    
    env.borrow_mut().define(
        "نطاق",
        Value::NativeFunction {
            name: "نطاق".to_string(),
            func: |args| {
                let (start, end, step) = match args.len() {
                    1 => (0.0, args[0].borrow().to_number().unwrap_or(0.0), 1.0),
                    2 => (
                        args[0].borrow().to_number().unwrap_or(0.0),
                        args[1].borrow().to_number().unwrap_or(0.0),
                        1.0
                    ),
                    _ => (
                        args.get(0).map(|a| a.borrow().to_number().unwrap_or(0.0)).unwrap_or(0.0),
                        args.get(1).map(|a| a.borrow().to_number().unwrap_or(0.0)).unwrap_or(0.0),
                        args.get(2).map(|a| a.borrow().to_number().unwrap_or(1.0)).unwrap_or(1.0)
                    ),
                };
                
                let mut list = Vec::new();
                let mut i = start;
                while if step > 0.0 { i < end } else { i > end } {
                    list.push(Rc::new(RefCell::new(Value::Number(i))));
                    i += step;
                }
                
                Ok(Rc::new(RefCell::new(Value::List(list))))
            },
        },
        false,
    );
    
    // دوال رياضية - كل واحدة على حدة
    env.borrow_mut().define(
        "جذر",
        Value::NativeFunction {
            name: "جذر".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Err("جذر يتطلب معاملاً واحداً".into());
                }
                let n = args[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.sqrt()))))
            },
        },
        false,
    );
    
    env.borrow_mut().define(
        "مطلق",
        Value::NativeFunction {
            name: "مطلق".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Err("مطلق يتطلب معاملاً واحداً".into());
                }
                let n = args[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.abs()))))
            },
        },
        false,
    );
    
    env.borrow_mut().define(
        "تقريب",
        Value::NativeFunction {
            name: "تقريب".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Err("تقريب يتطلب معاملاً واحداً".into());
                }
                let n = args[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.round()))))
            },
        },
        false,
    );
    
    env.borrow_mut().define(
        "طابق",
        Value::NativeFunction {
            name: "طابق".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Err("طابق يتطلب معاملاً واحداً".into());
                }
                let n = args[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.floor()))))
            },
        },
        false,
    );
    
    env.borrow_mut().define(
        "سقف",
        Value::NativeFunction {
            name: "سقف".to_string(),
            func: |args| {
                if args.is_empty() {
                    return Err("سقف يتطلب معاملاً واحداً".into());
                }
                let n = args[0].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(n.ceil()))))
            },
        },
        false,
    );
    
    env.borrow_mut().define(
        "أس",
        Value::NativeFunction {
            name: "أس".to_string(),
            func: |args| {
                if args.len() < 2 {
                    return Err("أس يتطلب معاملين".into());
                }
                let base = args[0].borrow().to_number()?;
                let exp = args[1].borrow().to_number()?;
                Ok(Rc::new(RefCell::new(Value::Number(base.powf(exp)))))
            },
        },
        false,
    );
    
    // ثوابت رياضية
    env.borrow_mut().define("ط", Value::Number(std::f64::consts::PI), true);
    env.borrow_mut().define("هـ", Value::Number(std::f64::consts::E), true);
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات التكامل
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_hello_world() {
        let result = run_bytecode(r#"اطبع("مرحباً بالعالم")؛"#);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_arithmetic() {
        let result = run_bytecode(r#"اطبع(5 + 3 * 2)؛"#);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_variables() {
        let result = run_bytecode(r#"
            متغير س = 10؛
            متجر ص = 20؛
            اطبع(س + ص)؛
        "#);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_performance_benchmark() {
        let code = r#"
            متغير س = 0؛
            طالما س < 1000 {
                س = س + 1؛
            }
        "#;
        
        let (result, stats) = run_bytecode_benchmark(code).unwrap();
        
        println!("═══════════════════════════════════");
        println!("📊 نتيجة اختبار الأداء");
        println!("═══════════════════════════════════");
        println!("📦 التعليمات المنفذة: {}", stats.instructions_executed);
        println!("⏱️ الوقت: {} ميكروثانية", stats.execution_time_us);
        println!("📊 أقصى حجم للمكدس: {}", stats.max_stack_size);
        
        if let Value::Number(n) = result {
            assert_eq!(n, 1000.0);
        }
    }
}
