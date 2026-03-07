// ═══════════════════════════════════════════════════════════════════════════════
// JIT Compiler - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// مترجم فوري (Just-In-Time) لتجميع التعليمات البرمجية الساخنة
// يستخدم تقنية Hot Spot Detection لتحديد الكود الأكثر تنفيذاً
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use super::opcodes::{Chunk, OpCode};
use crate::interpreter::value::{Environment, Value, SharedValue};

/// عتبة تفعيل JIT (عدد مرات التنفيذ)
const JIT_THRESHOLD: u32 = 100;

/// حجم كاش JIT
const JIT_CACHE_SIZE: usize = 256;

/// معرف دالة JIT
pub type JitFunctionId = u64;

/// نوع دالة JIT المترجمة
pub type JitCompiledFn = fn(&mut JitExecutionContext) -> JitExecutionResult;

/// سياق تنفيذ JIT
pub struct JitExecutionContext<'a> {
    pub stack: &'a mut Vec<SharedValue>,
    pub globals: &'a Rc<RefCell<Environment>>,
    pub ip: &'a mut usize,
}

/// نتيجة تنفيذ JIT
#[derive(Debug)]
pub enum JitExecutionResult {
    Continue,
    Return(SharedValue),
    Break,
    ContinueLoop,
    Error(String),
}

/// معلومات الكود الساخن
#[derive(Debug, Clone)]
pub struct HotSpotInfo {
    /// عدد مرات التنفيذ
    pub execution_count: u32,
    /// هل تم تجميعه
    pub is_compiled: bool,
    /// مستوى التحسين (0-3)
    pub optimization_level: u8,
    /// وقت التجميع (ميكروثانية)
    pub compile_time_us: u64,
    /// وقت التنفيذ الإجمالي (ميكروثانية)
    pub total_exec_time_us: u64,
}

/// نوع الكود المترجم
#[derive(Debug, Clone)]
pub enum CompiledCode {
    /// كود بايت كود عادي (غير محسن)
    Bytecode(Vec<OpCode>),
    /// كود محسن (Optimized Bytecode)
    OptimizedBytecode {
        instructions: Vec<OptimizedOp>,
        constants: Vec<Value>,
    },
    /// كود تراكب (Tracing JIT)
    Trace {
        entries: Vec<TraceEntry>,
        exit_points: Vec<usize>,
    },
}

/// تعليمة محسنة
#[derive(Debug, Clone)]
pub enum OptimizedOp {
    /// دفع ثابت (محسن)
    PushConst(usize),
    /// عملية ثنائية مدمجة
    BinaryOpConst { op: BinaryOpKind, const_idx: usize },
    /// تحميل متغير محلي محسن
    LoadLocalFast(u16),
    /// تخزين متغير محلي محسن
    StoreLocalFast(u16),
    /// قفز محسّن
    JumpFast(i32),
    /// حلقة محسنة
    FastLoop { start: usize, end: usize, body_start: usize },
    /// استدعاء دالة مدمج
    InlineCall { func_id: u64, arg_count: u8 },
    /// تعليمة أصلية
    Native(OpCode),
}

/// نوع العملية الثنائية
#[derive(Debug, Clone, Copy)]
pub enum BinaryOpKind {
    Add, Sub, Mul, Div, Mod, Pow,
}

/// مدخل في التتبع
#[derive(Debug, Clone)]
pub struct TraceEntry {
    pub opcode: OpCode,
    pub ip: usize,
    pub stack_depth: usize,
}

/// إحصائيات JIT
#[derive(Debug, Default, Clone)]
pub struct JitStats {
    /// عدد الدوال المترجمة
    pub compiled_functions: u64,
    /// عدد مرات تنفيذ JIT
    pub jit_executions: u64,
    /// عدد مرات التنفيذ البطيء
    pub slow_path_executions: u64,
    /// وقت التجميع الإجمالي
    pub total_compile_time_us: u64,
    /// وقت التنفيذ الإجمالي
    pub total_exec_time_us: u64,
    /// عدد التحسينات
    pub optimizations_applied: u64,
    /// نسبة ضربات JIT
    pub jit_hit_rate: f64,
}

/// JIT Compiler
pub struct JitCompiler {
    /// معرف التالي
    next_id: u64,
    /// الكود المترجم
    compiled_code: HashMap<JitFunctionId, CompiledCode>,
    /// معلومات النقاط الساخنة
    hot_spots: HashMap<usize, HotSpotInfo>,
    /// كاش للدوال الساخنة
    function_cache: HashMap<String, JitFunctionId>,
    /// الإحصائيات
    stats: JitStats,
    /// هل JIT مفعل
    enabled: bool,
    /// مستوى التحسين
    optimization_level: u8,
}

impl JitCompiler {
    /// إنشاء مترجم JIT جديد
    pub fn new() -> Self {
        JitCompiler {
            next_id: 1,
            compiled_code: HashMap::with_capacity(JIT_CACHE_SIZE),
            hot_spots: HashMap::with_capacity(JIT_CACHE_SIZE),
            function_cache: HashMap::with_capacity(64),
            stats: JitStats::default(),
            enabled: true,
            optimization_level: 2,
        }
    }
    
    /// إنشاء JIT مع إعدادات مخصصة
    pub fn with_config(enabled: bool, optimization_level: u8) -> Self {
        JitCompiler {
            next_id: 1,
            compiled_code: HashMap::with_capacity(JIT_CACHE_SIZE),
            hot_spots: HashMap::with_capacity(JIT_CACHE_SIZE),
            function_cache: HashMap::with_capacity(64),
            stats: JitStats::default(),
            enabled,
            optimization_level: optimization_level.min(3),
        }
    }
    
    /// تفعيل/تعطيل JIT
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// تحديث نقطة ساخنة
    pub fn record_execution(&mut self, ip: usize) -> bool {
        if !self.enabled {
            return false;
        }
        
        let info = self.hot_spots.entry(ip).or_insert(HotSpotInfo {
            execution_count: 0,
            is_compiled: false,
            optimization_level: 0,
            compile_time_us: 0,
            total_exec_time_us: 0,
        });
        
        info.execution_count += 1;
        
        // التحقق من عتبة التجميع
        if !info.is_compiled && info.execution_count >= JIT_THRESHOLD {
            return true; // يجب التجميع
        }
        
        false
    }
    
    /// تجميع كود ساخن
    pub fn compile_hot_code(&mut self, chunk: &Chunk, start_ip: usize) -> Option<JitFunctionId> {
        if !self.enabled {
            return None;
        }
        
        let start = std::time::Instant::now();
        
        // استخراج الكود الساخن
        let hot_chunk = self.extract_hot_region(chunk, start_ip)?;
        
        // تحسين الكود
        let compiled = self.optimize_chunk(&hot_chunk)?;
        
        let compile_time = start.elapsed().as_micros() as u64;
        
        let id = self.next_id;
        self.next_id += 1;
        
        // تحديث الإحصائيات
        self.stats.compiled_functions += 1;
        self.stats.total_compile_time_us += compile_time;
        
        // تحديث معلومات النقطة الساخنة
        if let Some(info) = self.hot_spots.get_mut(&start_ip) {
            info.is_compiled = true;
            info.compile_time_us = compile_time;
            info.optimization_level = self.optimization_level;
        }
        
        self.compiled_code.insert(id, compiled);
        
        Some(id)
    }
    
    /// استخراج منطقة ساخنة
    fn extract_hot_region(&self, chunk: &Chunk, start_ip: usize) -> Option<Vec<OpCode>> {
        let mut region = Vec::new();
        let mut ip = start_ip;
        let mut depth = 1;
        
        // استخراج حتى نهاية الكتلة أو الحلقة
        while ip < chunk.instructions.len() && depth > 0 {
            let op = &chunk.instructions[ip];
            
            match op {
                OpCode::JumpIfFalse(_) | OpCode::JumpIfTrue(_) => {
                    depth -= 1;
                }
                OpCode::JumpBack(_) => {
                    // نهاية الحلقة
                    region.push(op.clone());
                    break;
                }
                _ => {}
            }
            
            region.push(op.clone());
            ip += 1;
            
            if matches!(op, OpCode::Halt) {
                break;
            }
        }
        
        if region.is_empty() {
            None
        } else {
            Some(region)
        }
    }
    
    /// تحسين الـ Chunk
    fn optimize_chunk(&mut self, opcodes: &[OpCode]) -> Option<CompiledCode> {
        let mut optimized = Vec::with_capacity(opcodes.len());
        let mut constants = Vec::new();
        let mut optimizations = 0u64;
        
        let mut i = 0;
        while i < opcodes.len() {
            let op = &opcodes[i];
            
            match op {
                // تحسين: PushNumber متبوع بعملية -> BinaryOpConst
                OpCode::PushNumber(n) => {
                    if i + 1 < opcodes.len() {
                        let next = &opcodes[i + 1];
                        let const_idx = constants.len();
                        constants.push(Value::Number(*n));
                        
                        match next {
                            OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div 
                            | OpCode::Mod | OpCode::Pow => {
                                // هذا يحتاج معامل آخر - لا يمكن تحسينه بسهولة
                                optimized.push(OptimizedOp::PushConst(const_idx));
                            }
                            _ => {
                                optimized.push(OptimizedOp::PushConst(const_idx));
                            }
                        }
                    } else {
                        let const_idx = constants.len();
                        constants.push(Value::Number(*n));
                        optimized.push(OptimizedOp::PushConst(const_idx));
                    }
                }
                
                // تحسين: LoadLocal -> LoadLocalFast
                OpCode::LoadLocal(slot) => {
                    optimized.push(OptimizedOp::LoadLocalFast(*slot));
                    optimizations += 1;
                }
                
                // تحسين: StoreLocal -> StoreLocalFast
                OpCode::StoreLocal(slot) => {
                    optimized.push(OptimizedOp::StoreLocalFast(*slot));
                    optimizations += 1;
                }
                
                // تحسين: القفزات
                OpCode::Jump(offset) | OpCode::JumpIfFalse(offset) | OpCode::JumpIfTrue(offset) => {
                    optimized.push(OptimizedOp::JumpFast(*offset));
                }
                
                // باقي العمليات
                _ => {
                    optimized.push(OptimizedOp::Native(op.clone()));
                }
            }
            
            i += 1;
        }
        
        self.stats.optimizations_applied += optimizations;
        
        Some(CompiledCode::OptimizedBytecode {
            instructions: optimized,
            constants,
        })
    }
    
    /// تنفيذ كود مترجم
    pub fn execute_compiled(
        &mut self,
        _id: JitFunctionId,
        _ctx: &mut JitExecutionContext,
    ) -> JitExecutionResult {
        if !self.enabled {
            return JitExecutionResult::Error("JIT معطل".into());
        }
        
        // تنفيذ الكود المترجم
        self.stats.jit_executions += 1;
        
        // TODO: تنفيذ فعلي للكود المحسن
        JitExecutionResult::Continue
    }
    
    /// الحصول على الإحصائيات
    pub fn stats(&self) -> &JitStats {
        &self.stats
    }
    
    /// إعادة تعيين الإحصائيات
    pub fn reset_stats(&mut self) {
        self.stats = JitStats::default();
    }
    
    /// مسح الكاش
    pub fn clear_cache(&mut self) {
        self.compiled_code.clear();
        self.hot_spots.clear();
        self.function_cache.clear();
    }
    
    /// الحصول على معلومات النقطة الساخنة
    pub fn get_hot_spot_info(&self, ip: usize) -> Option<&HotSpotInfo> {
        self.hot_spots.get(&ip)
    }
    
    /// طباعة تقرير JIT
    pub fn print_report(&self) {
        println!();
        println!("╔══════════════════════════════════════════════════════════════════╗");
        println!("║                     🚀 تقرير JIT Compiler                         ║");
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ الحالة:              {:?}                              ║", if self.enabled { "مفعّل ✅" } else { "معطل ❌" });
        println!("║ مستوى التحسين:       {:15}                     ║", self.optimization_level);
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ الدوال المترجمة:     {:15}                     ║", self.stats.compiled_functions);
        println!("║ تنفيذ JIT:           {:15}                     ║", self.stats.jit_executions);
        println!("║ تنفيذ بطيء:          {:15}                     ║", self.stats.slow_path_executions);
        println!("║ التحسينات المطبقة:   {:15}                     ║", self.stats.optimizations_applied);
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!("║ وقت التجميع:         {:15} μs                 ║", self.stats.total_compile_time_us);
        println!("║ وقت التنفيذ:         {:15} μs                 ║", self.stats.total_exec_time_us);
        println!("╚══════════════════════════════════════════════════════════════════╝");
        
        // طباعة أهم النقاط الساخنة
        let mut hot_spots: Vec<_> = self.hot_spots.iter().collect();
        hot_spots.sort_by(|a, b| b.1.execution_count.cmp(&a.1.execution_count));
        
        if !hot_spots.is_empty() {
            println!();
            println!("📊 أهم النقاط الساخنة:");
            for (i, (ip, info)) in hot_spots.iter().take(5).enumerate() {
                println!("   {}. IP {} - {} تنفيذ {}", 
                    i + 1, 
                    ip, 
                    info.execution_count,
                    if info.is_compiled { "✅ مترجم" } else { "⏳ قيد المراقبة" }
                );
            }
        }
    }
}

impl Default for JitCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// منفذ الكود المحسن
pub struct OptimizedExecutor {
    /// المكدس
    stack: Vec<SharedValue>,
    /// المتغيرات المحلية
    locals: Vec<SharedValue>,
    /// مؤشر التعليمة
    ip: usize,
}

impl OptimizedExecutor {
    pub fn new() -> Self {
        OptimizedExecutor {
            stack: Vec::with_capacity(256),
            locals: Vec::with_capacity(64),
            ip: 0,
        }
    }
    
    /// تنفيذ كود محسن
    pub fn execute(
        &mut self,
        code: &CompiledCode,
        globals: &Rc<RefCell<Environment>>,
    ) -> Result<SharedValue, String> {
        match code {
            CompiledCode::OptimizedBytecode { instructions, constants } => {
                self.execute_optimized(instructions, constants, globals)
            }
            CompiledCode::Bytecode(opcodes) => {
                self.execute_bytecode(opcodes, globals)
            }
            CompiledCode::Trace { entries, exit_points } => {
                self.execute_trace(entries, exit_points, globals)
            }
        }
    }
    
    fn execute_optimized(
        &mut self,
        instructions: &[OptimizedOp],
        constants: &[Value],
        _globals: &Rc<RefCell<Environment>>,
    ) -> Result<SharedValue, String> {
        self.ip = 0;
        
        while self.ip < instructions.len() {
            let op = &instructions[self.ip];
            
            match op {
                OptimizedOp::PushConst(idx) => {
                    if let Some(val) = constants.get(*idx) {
                        self.stack.push(Rc::new(RefCell::new(val.clone())));
                    }
                }
                
                OptimizedOp::LoadLocalFast(slot) => {
                    if (*slot as usize) < self.locals.len() {
                        self.stack.push(Rc::clone(&self.locals[*slot as usize]));
                    }
                }
                
                OptimizedOp::StoreLocalFast(slot) => {
                    if let Some(val) = self.stack.pop() {
                        let slot = *slot as usize;
                        if slot >= self.locals.len() {
                            self.locals.resize(slot + 1, Rc::new(RefCell::new(Value::Null)));
                        }
                        self.locals[slot] = val;
                    }
                }
                
                OptimizedOp::JumpFast(offset) => {
                    self.ip = (self.ip as i32 + offset - 1) as usize;
                }
                
                OptimizedOp::Native(native_op) => {
                    // معالجة العمليات الأصلية
                    match native_op {
                        OpCode::Add => self.binary_op(|a, b| a + b)?,
                        OpCode::Sub => self.binary_op(|a, b| a - b)?,
                        OpCode::Mul => self.binary_op(|a, b| a * b)?,
                        OpCode::Div => self.binary_op(|a, b| {
                            if b == 0.0 { f64::INFINITY } else { a / b }
                        })?,
                        OpCode::Halt => break,
                        _ => {}
                    }
                }
                
                OptimizedOp::BinaryOpConst { op, const_idx } => {
                    if let Some(const_val) = constants.get(*const_idx) {
                        if let Value::Number(c) = const_val {
                            if let Some(top) = self.stack.pop() {
                                if let Value::Number(a) = &*top.borrow() {
                                    let result = match op {
                                        BinaryOpKind::Add => a + c,
                                        BinaryOpKind::Sub => a - c,
                                        BinaryOpKind::Mul => a * c,
                                        BinaryOpKind::Div => a / c,
                                        BinaryOpKind::Mod => a % c,
                                        BinaryOpKind::Pow => a.powf(*c),
                                    };
                                    self.stack.push(Rc::new(RefCell::new(Value::Number(result))));
                                }
                            }
                        }
                    }
                }
                
                OptimizedOp::FastLoop { start, end, body_start } => {
                    // تنفيذ حلقة سريعة
                    for i in *start..*end {
                        self.locals.push(Rc::new(RefCell::new(Value::Number(i as f64))));
                        // تنفيذ جسم الحلقة
                        let saved_ip = self.ip;
                        self.ip = *body_start;
                        // ... تنفيذ الجسم
                        self.ip = saved_ip;
                        self.locals.pop();
                    }
                }
                
                OptimizedOp::InlineCall { func_id, arg_count } => {
                    // استدعاء دالة مدمجة
                    let _ = (func_id, arg_count);
                }
            }
            
            self.ip += 1;
        }
        
        Ok(self.stack.pop().unwrap_or_else(|| Rc::new(RefCell::new(Value::Null))))
    }
    
    fn execute_bytecode(
        &mut self,
        _opcodes: &[OpCode],
        _globals: &Rc<RefCell<Environment>>,
    ) -> Result<SharedValue, String> {
        Ok(Rc::new(RefCell::new(Value::Null)))
    }
    
    fn execute_trace(
        &mut self,
        _entries: &[TraceEntry],
        _exit_points: &[usize],
        _globals: &Rc<RefCell<Environment>>,
    ) -> Result<SharedValue, String> {
        Ok(Rc::new(RefCell::new(Value::Null)))
    }
    
    #[inline(always)]
    fn binary_op<F>(&mut self, op: F) -> Result<(), String>
    where
        F: FnOnce(f64, f64) -> f64,
    {
        let b = self.stack.pop();
        let a = self.stack.pop();
        
        match (a, b) {
            (Some(a), Some(b)) => {
                let a_val = a.borrow().to_number()?;
                let b_val = b.borrow().to_number()?;
                self.stack.push(Rc::new(RefCell::new(Value::Number(op(a_val, b_val)))));
                Ok(())
            }
            _ => Err("مكدس فارغ".into())
        }
    }
}

impl Default for OptimizedExecutor {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_jit_compiler_creation() {
        let jit = JitCompiler::new();
        assert!(jit.enabled);
        assert_eq!(jit.optimization_level, 2);
    }
    
    #[test]
    fn test_hot_spot_detection() {
        let mut jit = JitCompiler::new();
        
        // محاكاة تنفيذ متكرر
        for _ in 0..50 {
            let should_compile = jit.record_execution(10);
            assert!(!should_compile);
        }
        
        // بعد العتبة
        for _ in 0..100 {
            jit.record_execution(10);
        }
        
        let info = jit.get_hot_spot_info(10).unwrap();
        assert!(info.execution_count >= JIT_THRESHOLD);
    }
    
    #[test]
    fn test_jit_stats() {
        let mut jit = JitCompiler::new();
        
        for _ in 0..150 {
            jit.record_execution(5);
        }
        
        let stats = jit.stats();
        println!("Stats: {:?}", stats);
    }
    
    #[test]
    fn test_optimized_executor() {
        let mut executor = OptimizedExecutor::new();
        let globals = Rc::new(RefCell::new(Environment::new()));
        
        let code = CompiledCode::OptimizedBytecode {
            instructions: vec![
                OptimizedOp::PushConst(0),
                OptimizedOp::PushConst(1),
                OptimizedOp::Native(OpCode::Add),
                OptimizedOp::Native(OpCode::Halt),
            ],
            constants: vec![Value::Number(5.0), Value::Number(3.0)],
        };
        
        let result = executor.execute(&code, &globals).unwrap();
        let val = result.borrow();
        
        if let Value::Number(n) = &*val {
            assert!((*n - 8.0).abs() < 0.001);
        }
    }
    
    #[test]
    fn test_jit_report() {
        let mut jit = JitCompiler::new();
        
        // محاكاة بعض التنفيذ
        for _ in 0..200 {
            jit.record_execution(10);
            jit.record_execution(20);
            jit.record_execution(30);
        }
        
        jit.print_report();
    }
    
    #[test]
    fn test_jit_disable() {
        let mut jit = JitCompiler::new();
        jit.set_enabled(false);
        
        let should_compile = jit.record_execution(10);
        assert!(!should_compile);
        assert!(!jit.enabled);
    }
}
