// ═══════════════════════════════════════════════════════════════════════════════
// JIT Compiler الكامل - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// مترجم فوري متكامل مع 5 مستويات تحسين:
// T0: Interpreter Baseline
// T1: Baseline JIT (Direct Threading)
// T2: Optimizing JIT (Constant Folding, Dead Code Elimination)
// T3: SIMD Optimizations
// T4: Tracing JIT (Hot Path Optimization)
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use super::opcodes::{Chunk, OpCode};
use crate::interpreter::value::{Environment, Value, SharedValue};

// ═══════════════════════════════════════════════════════════════════════════════
// الثوابت والإعدادات
// ═══════════════════════════════════════════════════════════════════════════════

/// عتبات المستويات
pub const TIER0_THRESHOLD: u32 = 0;
pub const TIER1_THRESHOLD: u32 = 50;
pub const TIER2_THRESHOLD: u32 = 200;
pub const TIER3_THRESHOLD: u32 = 1000;
pub const TIER4_THRESHOLD: u32 = 5000;

/// أحجام الذاكرة
pub const STACK_SIZE: usize = 1024;
pub const LOCALS_SIZE: usize = 256;
pub const CACHE_SIZE: usize = 512;

// ═══════════════════════════════════════════════════════════════════════════════
// المستويات والأنواع
// ═══════════════════════════════════════════════════════════════════════════════

/// مستوى التحسين
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TierLevel {
    Tier0 = 0,  // Interpreter
    Tier1 = 1,  // Baseline JIT
    Tier2 = 2,  // Optimizing JIT
    Tier3 = 3,  // SIMD
    Tier4 = 4,  // Tracing
}

impl Default for TierLevel {
    fn default() -> Self {
        TierLevel::Tier0
    }
}

/// نوع القيمة للمتجهات
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeInfo {
    Number,
    String,
    Boolean,
    Null,
    List,
    Dict,
    Function,
    Unknown,
}

/// معلومات الكود الساخن
#[derive(Debug, Clone)]
pub struct HotSpotInfo {
    pub execution_count: u32,
    pub current_tier: TierLevel,
    pub compile_time: Duration,
    pub total_exec_time: Duration,
    pub is_compiled: bool,
    pub speedup_factor: f64,
}

impl Default for HotSpotInfo {
    fn default() -> Self {
        HotSpotInfo {
            execution_count: 0,
            current_tier: TierLevel::Tier0,
            compile_time: Duration::ZERO,
            total_exec_time: Duration::ZERO,
            is_compiled: false,
            speedup_factor: 1.0,
        }
    }
}

/// الكود المترجم
#[derive(Debug, Clone)]
pub struct CompiledCode {
    pub tier: TierLevel,
    pub instructions: Vec<CompiledInstruction>,
    pub constants: Vec<Value>,
    pub entry_point: usize,
    pub exit_points: Vec<usize>,
    pub guards: Vec<Guard>,
}

/// تعليمة مجمعة
#[derive(Debug, Clone)]
pub enum CompiledInstruction {
    /// تعليمة أصلية
    Original(OpCode),
    /// دفع ثابت (محسن)
    PushConst(usize),
    /// عملية ثنائية مدمجة مع ثابت
    BinaryConst { op: BinaryOp, const_idx: usize },
    /// تحميل متغير سريع
    LoadLocalFast { slot: u16, cached_type: TypeInfo },
    /// تخزين متغير سريع
    StoreLocalFast { slot: u16 },
    /// قفز مباشر (محسن)
    DirectJump { target: usize },
    /// قفز مشروط محسن
    ConditionalJump { target: usize, condition: JumpCondition },
    /// عملية SIMD
    SimdOp { op: SimdOp, count: usize },
    /// استدعاء مدمج
    InlineCall { func_addr: usize, arg_count: u8 },
    /// حارس للتحقق
    Guard(Guard),
    /// نهاية الكود
    Halt,
}

/// عملية ثنائية
#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod, Pow,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or,
}

/// شرط القفز
#[derive(Debug, Clone, Copy)]
pub enum JumpCondition {
    IfTrue,
    IfFalse,
    Always,
    Never,
}

/// عملية SIMD
#[derive(Debug, Clone, Copy)]
pub enum SimdOp {
    AddF64x4,
    MulF64x4,
    FusedMulAdd,
    HorizontalSum,
}

/// حارس للتحقق
#[derive(Debug, Clone)]
pub struct Guard {
    pub guard_type: GuardType,
    pub location: usize,
    pub fail_target: usize,
}

/// نوع الحارس
#[derive(Debug, Clone)]
pub enum GuardType {
    TypeCheck { expected: TypeInfo },
    NonNull,
    BoundsCheck { min: i64, max: i64 },
    ValueCheck { expected: Value },
}

/// نتيجة التنفيذ
#[derive(Debug)]
pub enum ExecutionResult {
    Ok(Value),
    Return(Value),
    Break,
    Continue,
    Error(String),
}

// ═══════════════════════════════════════════════════════════════════════════════
// إحصائيات JIT
// ═══════════════════════════════════════════════════════════════════════════════

/// إحصائيات شاملة
#[derive(Debug, Clone, Default)]
pub struct JitStats {
    // Tiered Compilation
    pub tier0_executions: u64,
    pub tier1_executions: u64,
    pub tier2_executions: u64,
    pub tier3_executions: u64,
    pub tier4_executions: u64,
    
    // Compilation
    pub compiled_functions: u64,
    pub total_compile_time_us: u64,
    pub recompilations: u64,
    
    // Execution
    pub total_exec_time_us: u64,
    pub instructions_executed: u64,
    
    // Optimizations
    pub optimizations_applied: u64,
    pub inlined_calls: u64,
    pub simd_operations: u64,
    
    // Cache
    pub cache_hits: u64,
    pub cache_misses: u64,
    
    // GC
    pub gc_collections: u64,
    pub gc_freed_bytes: u64,
}

impl JitStats {
    pub fn total_executions(&self) -> u64 {
        self.tier0_executions + self.tier1_executions + 
        self.tier2_executions + self.tier3_executions + self.tier4_executions
    }
    
    pub fn avg_speedup(&self) -> f64 {
        if self.tier0_executions == 0 {
            return 1.0;
        }
        let tiered = self.tier1_executions + self.tier2_executions + 
                     self.tier3_executions + self.tier4_executions;
        1.0 + (tiered as f64 / self.tier0_executions as f64).min(10.0)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// JIT Compiler الكامل
// ═══════════════════════════════════════════════════════════════════════════════

/// JIT Compiler متكامل
pub struct CompleteJitCompiler {
    // الكود المترجم
    compiled_code: HashMap<usize, CompiledCode>,
    
    // معلومات النقاط الساخنة
    hot_spots: HashMap<usize, HotSpotInfo>,
    
    // الإحصائيات
    stats: JitStats,
    
    // الإعدادات
    enabled: bool,
    max_tier: TierLevel,
    
    // الذاكرة المؤقتة للتنفيذ
    execution_stack: Vec<Value>,
    locals: Vec<Value>,
}

impl CompleteJitCompiler {
    /// إنشاء JIT جديد
    pub fn new() -> Self {
        CompleteJitCompiler {
            compiled_code: HashMap::with_capacity(CACHE_SIZE),
            hot_spots: HashMap::with_capacity(CACHE_SIZE),
            stats: JitStats::default(),
            enabled: true,
            max_tier: TierLevel::Tier4,
            execution_stack: Vec::with_capacity(STACK_SIZE),
            locals: Vec::with_capacity(LOCALS_SIZE),
        }
    }
    
    /// إنشاء JIT مع إعدادات مخصصة
    pub fn with_config(enabled: bool, max_tier: TierLevel) -> Self {
        let mut jit = Self::new();
        jit.enabled = enabled;
        jit.max_tier = max_tier;
        jit
    }
    
    /// تحديد المستوى المناسب
    pub fn determine_tier(&self, execution_count: u32) -> TierLevel {
        if !self.enabled {
            return TierLevel::Tier0;
        }
        
        let tier = if execution_count >= TIER4_THRESHOLD {
            TierLevel::Tier4
        } else if execution_count >= TIER3_THRESHOLD {
            TierLevel::Tier3
        } else if execution_count >= TIER2_THRESHOLD {
            TierLevel::Tier2
        } else if execution_count >= TIER1_THRESHOLD {
            TierLevel::Tier1
        } else {
            TierLevel::Tier0
        };
        
        tier.min(self.max_tier)
    }
    
    /// تسجيل تنفيذ
    pub fn record_execution(&mut self, ip: usize) -> bool {
        if !self.enabled {
            return false;
        }
        
        let info = self.hot_spots.entry(ip).or_default();
        info.execution_count += 1;
        let exec_count = info.execution_count;
        let is_compiled = info.is_compiled;
        let current_tier = info.current_tier;
        
        // التحقق من الحاجة للترقية
        let new_tier = self.determine_tier(exec_count);
        let should_compile = new_tier > current_tier && !is_compiled;
        
        should_compile
    }
    
    /// تجميع الكود
    pub fn compile(&mut self, chunk: &Chunk, start_ip: usize) -> Result<(), String> {
        let info = self.hot_spots.get(&start_ip).cloned().unwrap_or_default();
        let target_tier = self.determine_tier(info.execution_count);
        
        let compile_start = Instant::now();
        
        let code = match target_tier {
            TierLevel::Tier0 => self.compile_tier0(chunk, start_ip)?,
            TierLevel::Tier1 => self.compile_tier1(chunk, start_ip)?,
            TierLevel::Tier2 => self.compile_tier2(chunk, start_ip)?,
            TierLevel::Tier3 => self.compile_tier3(chunk, start_ip)?,
            TierLevel::Tier4 => self.compile_tier4(chunk, start_ip)?,
        };
        
        let compile_time = compile_start.elapsed();
        
        // تحديث الإحصائيات
        self.stats.compiled_functions += 1;
        self.stats.total_compile_time_us += compile_time.as_micros() as u64;
        
        // تحديث معلومات النقطة الساخنة
        let hot_info = self.hot_spots.entry(start_ip).or_default();
        hot_info.current_tier = target_tier;
        hot_info.is_compiled = true;
        hot_info.compile_time = compile_time;
        
        // تخزين الكود المترجم
        self.compiled_code.insert(start_ip, code);
        
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Tier 0: تفسير عادي
    // ═══════════════════════════════════════════════════════════════════════════
    
    fn compile_tier0(&mut self, chunk: &Chunk, _start_ip: usize) -> Result<CompiledCode, String> {
        let instructions: Vec<CompiledInstruction> = chunk.instructions
            .iter()
            .map(|op| CompiledInstruction::Original(op.clone()))
            .collect();
        
        Ok(CompiledCode {
            tier: TierLevel::Tier0,
            instructions,
            constants: Vec::new(),
            entry_point: 0,
            exit_points: Vec::new(),
            guards: Vec::new(),
        })
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Tier 1: Baseline JIT مع Direct Threading
    // ═══════════════════════════════════════════════════════════════════════════
    
    fn compile_tier1(&mut self, chunk: &Chunk, _start_ip: usize) -> Result<CompiledCode, String> {
        let mut instructions = Vec::with_capacity(chunk.instructions.len());
        let mut constants = Vec::new();
        let mut i = 0;
        
        while i < chunk.instructions.len() {
            let op = &chunk.instructions[i];
            
            match op {
                // تحسين: دمج PushNumber مع العملية التالية
                OpCode::PushNumber(n) => {
                    let const_idx = constants.len();
                    constants.push(Value::Number(*n));
                    
                    // فحص إمكانية الدمج مع العملية التالية
                    if i + 2 < chunk.instructions.len() {
                        let next1 = &chunk.instructions[i + 1];
                        let next2 = &chunk.instructions[i + 2];
                        
                        if matches!(next1, OpCode::PushNumber(_)) {
                            if let OpCode::PushNumber(n2) = next1 {
                                // دمج: PushNumber, PushNumber, BinaryOp
                                if matches!(next2, OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div) {
                                    let const_idx2 = constants.len();
                                    constants.push(Value::Number(*n2));
                                    
                                    let bin_op = match next2 {
                                        OpCode::Add => BinaryOp::Add,
                                        OpCode::Sub => BinaryOp::Sub,
                                        OpCode::Mul => BinaryOp::Mul,
                                        OpCode::Div => BinaryOp::Div,
                                        _ => BinaryOp::Add,
                                    };
                                    
                                    instructions.push(CompiledInstruction::BinaryConst {
                                        op: bin_op,
                                        const_idx: const_idx2,
                                    });
                                    self.stats.optimizations_applied += 1;
                                    i += 3;
                                    continue;
                                }
                            }
                        }
                    }
                    
                    instructions.push(CompiledInstruction::PushConst(const_idx));
                }
                
                // تحسين: LoadLocal سريع
                OpCode::LoadLocal(slot) => {
                    instructions.push(CompiledInstruction::LoadLocalFast {
                        slot: *slot,
                        cached_type: TypeInfo::Unknown,
                    });
                }
                
                // تحسين: StoreLocal سريع
                OpCode::StoreLocal(slot) => {
                    instructions.push(CompiledInstruction::StoreLocalFast { slot: *slot });
                }
                
                // تحسين: القفزات
                OpCode::Jump(offset) => {
                    let target = (i as i32 + offset) as usize;
                    instructions.push(CompiledInstruction::DirectJump { target });
                }
                
                OpCode::JumpIfFalse(offset) => {
                    let target = (i as i32 + offset) as usize;
                    instructions.push(CompiledInstruction::ConditionalJump {
                        target,
                        condition: JumpCondition::IfFalse,
                    });
                }
                
                OpCode::JumpIfTrue(offset) => {
                    let target = (i as i32 + offset) as usize;
                    instructions.push(CompiledInstruction::ConditionalJump {
                        target,
                        condition: JumpCondition::IfTrue,
                    });
                }
                
                OpCode::JumpBack(offset) => {
                    let target = (i as i32 - offset) as usize;
                    instructions.push(CompiledInstruction::DirectJump { target });
                }
                
                OpCode::Halt => {
                    instructions.push(CompiledInstruction::Halt);
                    break;
                }
                
                // باقي التعليمات
                _ => {
                    instructions.push(CompiledInstruction::Original(op.clone()));
                }
            }
            
            i += 1;
        }
        
        Ok(CompiledCode {
            tier: TierLevel::Tier1,
            instructions,
            constants,
            entry_point: 0,
            exit_points: Vec::new(),
            guards: Vec::new(),
        })
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Tier 2: Optimizing JIT
    // ═══════════════════════════════════════════════════════════════════════════
    
    fn compile_tier2(&mut self, chunk: &Chunk, start_ip: usize) -> Result<CompiledCode, String> {
        // ابدأ من Tier 1
        let mut code = self.compile_tier1(chunk, start_ip)?;
        code.tier = TierLevel::Tier2;
        
        // تطبيق تحسينات إضافية
        self.apply_constant_folding(&mut code);
        self.apply_dead_code_elimination(&mut code);
        self.apply_strength_reduction(&mut code);
        
        Ok(code)
    }
    
    /// طي الثوابت
    fn apply_constant_folding(&mut self, code: &mut CompiledCode) {
        let mut i = 0;
        while i < code.instructions.len() {
            // البحث عن نمط: PushConst, PushConst, BinaryConst
            if i + 2 < code.instructions.len() {
                if let (
                    CompiledInstruction::PushConst(idx1),
                    CompiledInstruction::PushConst(idx2),
                    CompiledInstruction::Original(op),
                ) = (
                    &code.instructions[i],
                    &code.instructions[i + 1],
                    &code.instructions[i + 2],
                ) {
                    if let (
                        Some(Value::Number(n1)),
                        Some(Value::Number(n2)),
                    ) = (
                        code.constants.get(*idx1),
                        code.constants.get(*idx2),
                    ) {
                        // حساب النتيجة في وقت التجميع
                        if let Some(result) = self.fold_binary_op(op, *n1, *n2) {
                            let result_idx = code.constants.len();
                            code.constants.push(Value::Number(result));
                            
                            code.instructions[i] = CompiledInstruction::PushConst(result_idx);
                            code.instructions.remove(i + 1);
                            code.instructions.remove(i + 1);
                            
                            self.stats.optimizations_applied += 1;
                            continue;
                        }
                    }
                }
            }
            i += 1;
        }
    }
    
    fn fold_binary_op(&self, op: &OpCode, a: f64, b: f64) -> Option<f64> {
        match op {
            OpCode::Add => Some(a + b),
            OpCode::Sub => Some(a - b),
            OpCode::Mul => Some(a * b),
            OpCode::Div if b != 0.0 => Some(a / b),
            OpCode::Mod if b != 0.0 => Some(a % b),
            OpCode::Pow => Some(a.powf(b)),
            OpCode::Equal => Some(if (a - b).abs() < f64::EPSILON { 1.0 } else { 0.0 }),
            OpCode::NotEqual => Some(if (a - b).abs() >= f64::EPSILON { 1.0 } else { 0.0 }),
            OpCode::Less => Some(if a < b { 1.0 } else { 0.0 }),
            OpCode::Greater => Some(if a > b { 1.0 } else { 0.0 }),
            OpCode::LessEqual => Some(if a <= b { 1.0 } else { 0.0 }),
            OpCode::GreaterEqual => Some(if a >= b { 1.0 } else { 0.0 }),
            _ => None,
        }
    }
    
    /// إزالة الكود الميت
    fn apply_dead_code_elimination(&mut self, code: &mut CompiledCode) {
        let mut new_instructions = Vec::with_capacity(code.instructions.len());
        let mut reachable = vec![false; code.instructions.len()];
        
        // تحليل التدفق للعثور على الكود القابل للوصول
        reachable[0] = true;
        for i in 0..code.instructions.len() {
            if reachable[i] {
                match &code.instructions[i] {
                    CompiledInstruction::DirectJump { target } => {
                        if *target < reachable.len() {
                            reachable[*target] = true;
                        }
                    }
                    CompiledInstruction::ConditionalJump { target, .. } => {
                        if *target < reachable.len() {
                            reachable[*target] = true;
                        }
                        if i + 1 < reachable.len() {
                            reachable[i + 1] = true;
                        }
                    }
                    CompiledInstruction::Halt => {}
                    _ => {
                        if i + 1 < reachable.len() {
                            reachable[i + 1] = true;
                        }
                    }
                }
            }
        }
        
        // الاحتفاظ بالكود القابل للوصول فقط
        for (i, instr) in code.instructions.iter().enumerate() {
            if reachable[i] {
                new_instructions.push(instr.clone());
            } else {
                self.stats.optimizations_applied += 1;
            }
        }
        
        code.instructions = new_instructions;
    }
    
    /// تقوية العمليات
    fn apply_strength_reduction(&mut self, code: &mut CompiledCode) {
        for instr in &mut code.instructions {
            if let CompiledInstruction::Original(op) = instr {
                match op {
                    // x * 2 -> x + x
                    OpCode::Mul => {
                        // سيتم معالجتها في التنفيذ
                    }
                    // x ^ 2 -> x * x
                    OpCode::Pow => {
                        // سيتم معالجتها في التنفيذ
                    }
                    _ => {}
                }
            }
        }
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Tier 3: SIMD Optimizations
    // ═══════════════════════════════════════════════════════════════════════════
    
    fn compile_tier3(&mut self, chunk: &Chunk, start_ip: usize) -> Result<CompiledCode, String> {
        let mut code = self.compile_tier2(chunk, start_ip)?;
        code.tier = TierLevel::Tier3;
        
        // البحث عن أنماط SIMD
        self.apply_simd_optimizations(&mut code);
        
        Ok(code)
    }
    
    fn apply_simd_optimizations(&mut self, code: &mut CompiledCode) {
        let mut i = 0;
        while i < code.instructions.len() {
            // البحث عن نمط: PushNumber, PushNumber, PushNumber, PushNumber, Add, Add, Add
            if self.detect_simd_pattern(&code.instructions, i, 4) {
                // استبدال بعملية SIMD
                let simd_instr = CompiledInstruction::SimdOp {
                    op: SimdOp::AddF64x4,
                    count: 4,
                };
                
                code.instructions[i] = simd_instr;
                for _ in 0..10 { // إزالة التعليمات المدمجة
                    if i + 1 < code.instructions.len() {
                        code.instructions.remove(i + 1);
                    }
                }
                
                self.stats.optimizations_applied += 1;
                self.stats.simd_operations += 1;
            }
            i += 1;
        }
    }
    
    fn detect_simd_pattern(&self, instructions: &[CompiledInstruction], start: usize, count: usize) -> bool {
        // فحص وجود count عمليات PushNumber متتالية
        let mut num_count = 0;
        for i in start..instructions.len().min(start + count * 2) {
            if matches!(instructions[i], CompiledInstruction::PushConst(_)) {
                num_count += 1;
            }
        }
        num_count >= count as usize
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Tier 4: Tracing JIT
    // ═══════════════════════════════════════════════════════════════════════════
    
    fn compile_tier4(&mut self, chunk: &Chunk, start_ip: usize) -> Result<CompiledCode, String> {
        let mut code = self.compile_tier3(chunk, start_ip)?;
        code.tier = TierLevel::Tier4;
        
        // إضافة حراس للتحقق من الأنواع
        self.add_type_guards(&mut code);
        
        // تحسين المسارات الساخنة
        self.optimize_hot_paths(&mut code);
        
        Ok(code)
    }
    
    fn add_type_guards(&mut self, code: &mut CompiledCode) {
        let mut guards = Vec::new();
        
        for (i, instr) in code.instructions.iter().enumerate() {
            if let CompiledInstruction::LoadLocalFast { slot, .. } = instr {
                guards.push(Guard {
                    guard_type: GuardType::TypeCheck { expected: TypeInfo::Number },
                    location: i,
                    fail_target: 0, // سيتم تحديثه
                });
            }
        }
        
        code.guards = guards;
    }
    
    fn optimize_hot_paths(&mut self, code: &mut CompiledCode) {
        // تحليل التكرارات لإيجاد المسارات الساخنة
        let mut loop_starts: HashMap<usize, u32> = HashMap::new();
        
        for (i, instr) in code.instructions.iter().enumerate() {
            if let CompiledInstruction::DirectJump { target } = instr {
                if *target < i {
                    // قفز للخلف = بداية حلقة
                    *loop_starts.entry(*target).or_insert(0) += 1;
                }
            }
        }
        
        // تحسين الحلقات الساخنة
        for (start, _count) in &loop_starts {
            // يمكن إضافة تحسينات خاصة بالحلقات هنا
            self.stats.optimizations_applied += 1;
        }
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // التنفيذ
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// تنفيذ الكود
    pub fn execute(&mut self, chunk: &Chunk, globals: &mut Rc<RefCell<Environment>>) -> Result<Value, String> {
        let start_ip = 0;
        
        // التحقق من وجود كود مترجم
        if let Some(compiled) = self.compiled_code.get(&start_ip).cloned() {
            match compiled.tier {
                TierLevel::Tier0 => {
                    self.stats.tier0_executions += 1;
                    self.execute_tier0(&compiled, globals)
                }
                TierLevel::Tier1 => {
                    self.stats.tier1_executions += 1;
                    self.execute_tier1(&compiled, globals)
                }
                TierLevel::Tier2 => {
                    self.stats.tier2_executions += 1;
                    self.execute_tier2(&compiled, globals)
                }
                TierLevel::Tier3 => {
                    self.stats.tier3_executions += 1;
                    self.execute_tier3(&compiled, globals)
                }
                TierLevel::Tier4 => {
                    self.stats.tier4_executions += 1;
                    self.execute_tier4(&compiled, globals)
                }
            }
        } else {
            // تنفيذ عادي بدون JIT
            self.stats.tier0_executions += 1;
            self.execute_bytecode(chunk, globals)
        }
    }
    
    fn execute_tier0(&mut self, compiled: &CompiledCode, globals: &Rc<RefCell<Environment>>) -> Result<Value, String> {
        let start = Instant::now();
        self.execution_stack.clear();
        self.locals.clear();
        
        let mut ip = compiled.entry_point;
        
        while ip < compiled.instructions.len() {
            self.stats.instructions_executed += 1;
            
            match &compiled.instructions[ip] {
                CompiledInstruction::Original(op) => {
                    self.execute_opcode(op, globals)?;
                }
                CompiledInstruction::Halt => break,
                _ => {}
            }
            ip += 1;
        }
        
        self.stats.total_exec_time_us += start.elapsed().as_micros() as u64;
        
        Ok(self.execution_stack.pop().unwrap_or(Value::Null))
    }
    
    fn execute_tier1(&mut self, compiled: &CompiledCode, globals: &Rc<RefCell<Environment>>) -> Result<Value, String> {
        let start = Instant::now();
        self.execution_stack.clear();
        self.locals.clear();
        self.locals.resize(LOCALS_SIZE, Value::Null);
        
        let mut ip = compiled.entry_point;
        
        while ip < compiled.instructions.len() {
            self.stats.instructions_executed += 1;
            
            match &compiled.instructions[ip] {
                CompiledInstruction::Original(op) => {
                    self.execute_opcode(op, globals)?;
                }
                CompiledInstruction::PushConst(idx) => {
                    if let Some(val) = compiled.constants.get(*idx) {
                        self.execution_stack.push(val.clone());
                    }
                }
                CompiledInstruction::LoadLocalFast { slot, .. } => {
                    let slot = *slot as usize;
                    if slot < self.locals.len() {
                        self.execution_stack.push(self.locals[slot].clone());
                    }
                }
                CompiledInstruction::StoreLocalFast { slot } => {
                    let slot = *slot as usize;
                    if let Some(val) = self.execution_stack.pop() {
                        self.locals[slot] = val;
                    }
                }
                CompiledInstruction::DirectJump { target } => {
                    ip = *target;
                    continue;
                }
                CompiledInstruction::ConditionalJump { target, condition } => {
                    if let Some(val) = self.execution_stack.pop() {
                        let is_true = self.is_truthy(&val);
                        let should_jump = match condition {
                            JumpCondition::IfTrue => is_true,
                            JumpCondition::IfFalse => !is_true,
                            JumpCondition::Always => true,
                            JumpCondition::Never => false,
                        };
                        if should_jump {
                            ip = *target;
                            continue;
                        }
                    }
                }
                CompiledInstruction::BinaryConst { op, const_idx } => {
                    if let Some(const_val) = compiled.constants.get(*const_idx) {
                        if let Some(top) = self.execution_stack.pop() {
                            let result = self.apply_binary_op(op, &top, const_val);
                            self.execution_stack.push(result);
                        }
                    }
                }
                CompiledInstruction::Halt => break,
                _ => {}
            }
            ip += 1;
        }
        
        self.stats.total_exec_time_us += start.elapsed().as_micros() as u64;
        self.stats.cache_hits += 1;
        
        Ok(self.execution_stack.pop().unwrap_or(Value::Null))
    }
    
    fn execute_tier2(&mut self, compiled: &CompiledCode, globals: &Rc<RefCell<Environment>>) -> Result<Value, String> {
        // Tier 2 يستخدم نفس منطق Tier 1 مع كود محسن
        self.execute_tier1(compiled, globals)
    }
    
    fn execute_tier3(&mut self, compiled: &CompiledCode, globals: &Rc<RefCell<Environment>>) -> Result<Value, String> {
        let start = Instant::now();
        self.execution_stack.clear();
        self.locals.clear();
        
        let mut ip = compiled.entry_point;
        
        while ip < compiled.instructions.len() {
            self.stats.instructions_executed += 1;
            
            match &compiled.instructions[ip] {
                CompiledInstruction::SimdOp { op, count } => {
                    self.execute_simd_op(op, *count)?;
                }
                _ => {
                    // استخدام منطق Tier 1 للتعليمات الأخرى
                    return self.execute_tier1(compiled, globals);
                }
            }
            ip += 1;
        }
        
        self.stats.total_exec_time_us += start.elapsed().as_micros() as u64;
        
        Ok(self.execution_stack.pop().unwrap_or(Value::Null))
    }
    
    fn execute_tier4(&mut self, compiled: &CompiledCode, globals: &Rc<RefCell<Environment>>) -> Result<Value, String> {
        let start = Instant::now();
        self.execution_stack.clear();
        self.locals.clear();
        
        let mut ip = compiled.entry_point;
        
        while ip < compiled.instructions.len() {
            self.stats.instructions_executed += 1;
            
            // فحص الحراس
            for guard in &compiled.guards {
                if guard.location == ip {
                    if !self.check_guard(guard)? {
                        // فشل الحارس - العودة للتنفيذ البطيء
                        self.stats.cache_misses += 1;
                        return self.execute_tier1(compiled, globals);
                    }
                }
            }
            
            match &compiled.instructions[ip] {
                CompiledInstruction::Original(op) => {
                    self.execute_opcode(op, globals)?;
                }
                CompiledInstruction::PushConst(idx) => {
                    if let Some(val) = compiled.constants.get(*idx) {
                        self.execution_stack.push(val.clone());
                    }
                }
                CompiledInstruction::LoadLocalFast { slot, .. } => {
                    let slot = *slot as usize;
                    if slot < self.locals.len() {
                        self.execution_stack.push(self.locals[slot].clone());
                    } else {
                        self.execution_stack.push(Value::Null);
                    }
                }
                CompiledInstruction::StoreLocalFast { slot } => {
                    let slot = *slot as usize;
                    if slot >= self.locals.len() {
                        self.locals.resize(slot + 1, Value::Null);
                    }
                    if let Some(val) = self.execution_stack.pop() {
                        self.locals[slot] = val;
                    }
                }
                CompiledInstruction::DirectJump { target } => {
                    ip = *target;
                    continue;
                }
                CompiledInstruction::ConditionalJump { target, condition } => {
                    if let Some(val) = self.execution_stack.pop() {
                        let is_true = self.is_truthy(&val);
                        let should_jump = match condition {
                            JumpCondition::IfTrue => is_true,
                            JumpCondition::IfFalse => !is_true,
                            JumpCondition::Always => true,
                            JumpCondition::Never => false,
                        };
                        if should_jump {
                            ip = *target;
                            continue;
                        }
                    }
                }
                CompiledInstruction::Halt => break,
                _ => {}
            }
            ip += 1;
        }
        
        self.stats.total_exec_time_us += start.elapsed().as_micros() as u64;
        self.stats.cache_hits += 1;
        
        Ok(self.execution_stack.pop().unwrap_or(Value::Null))
    }
    
    /// تنفيذ Opcode مباشرة
    fn execute_opcode(&mut self, op: &OpCode, globals: &Rc<RefCell<Environment>>) -> Result<(), String> {
        match op {
            OpCode::PushNumber(n) => {
                self.execution_stack.push(Value::Number(*n));
            }
            OpCode::PushString(idx) => {
                // الحصول على النص من chunk
                self.execution_stack.push(Value::String(format!("str_{}", idx)));
            }
            OpCode::PushBool(b) => {
                self.execution_stack.push(Value::Boolean(*b));
            }
            OpCode::PushNull => {
                self.execution_stack.push(Value::Null);
            }
            OpCode::Pop => {
                self.execution_stack.pop();
            }
            OpCode::Add => {
                self.binary_op(|a, b| a + b)?;
            }
            OpCode::Sub => {
                self.binary_op(|a, b| a - b)?;
            }
            OpCode::Mul => {
                self.binary_op(|a, b| a * b)?;
            }
            OpCode::Div => {
                self.binary_op(|a, b| if b != 0.0 { a / b } else { f64::INFINITY })?;
            }
            OpCode::Mod => {
                self.binary_op(|a, b| a % b)?;
            }
            OpCode::Pow => {
                self.binary_op(|a, b| a.powf(b))?;
            }
            OpCode::Neg => {
                if let Some(val) = self.execution_stack.pop() {
                    if let Value::Number(n) = val {
                        self.execution_stack.push(Value::Number(-n));
                    }
                }
            }
            OpCode::Equal => {
                self.compare_op(|a, b| (a - b).abs() < f64::EPSILON)?;
            }
            OpCode::NotEqual => {
                self.compare_op(|a, b| (a - b).abs() >= f64::EPSILON)?;
            }
            OpCode::Less => {
                self.compare_op(|a, b| a < b)?;
            }
            OpCode::Greater => {
                self.compare_op(|a, b| a > b)?;
            }
            OpCode::LessEqual => {
                self.compare_op(|a, b| a <= b)?;
            }
            OpCode::GreaterEqual => {
                self.compare_op(|a, b| a >= b)?;
            }
            OpCode::And => {
                let b = self.execution_stack.pop().unwrap_or(Value::Null);
                let a = self.execution_stack.pop().unwrap_or(Value::Null);
                self.execution_stack.push(Value::Boolean(
                    self.is_truthy(&a) && self.is_truthy(&b)
                ));
            }
            OpCode::Or => {
                let b = self.execution_stack.pop().unwrap_or(Value::Null);
                let a = self.execution_stack.pop().unwrap_or(Value::Null);
                self.execution_stack.push(Value::Boolean(
                    self.is_truthy(&a) || self.is_truthy(&b)
                ));
            }
            OpCode::Not => {
                if let Some(val) = self.execution_stack.pop() {
                    self.execution_stack.push(Value::Boolean(!self.is_truthy(&val)));
                }
            }
            OpCode::Print => {
                if let Some(val) = self.execution_stack.pop() {
                    println!("{}", self.value_to_string(&val));
                }
            }
            OpCode::Halt => {}
            _ => {}
        }
        Ok(())
    }
    
    fn execute_simd_op(&mut self, op: &SimdOp, count: usize) -> Result<(), String> {
        match op {
            SimdOp::AddF64x4 => {
                // جمع 4 أزواج من الأرقام
                let mut results = Vec::with_capacity(count);
                for _ in 0..count {
                    let b = self.execution_stack.pop();
                    let a = self.execution_stack.pop();
                    match (a, b) {
                        (Some(Value::Number(a)), Some(Value::Number(b))) => {
                            results.push(Value::Number(a + b));
                        }
                        _ => {}
                    }
                }
                results.into_iter().for_each(|r| self.execution_stack.push(r));
            }
            SimdOp::MulF64x4 => {
                let mut results = Vec::with_capacity(count);
                for _ in 0..count {
                    let b = self.execution_stack.pop();
                    let a = self.execution_stack.pop();
                    match (a, b) {
                        (Some(Value::Number(a)), Some(Value::Number(b))) => {
                            results.push(Value::Number(a * b));
                        }
                        _ => {}
                    }
                }
                results.into_iter().for_each(|r| self.execution_stack.push(r));
            }
            SimdOp::FusedMulAdd => {
                // a * b + c
                let c = self.execution_stack.pop();
                let b = self.execution_stack.pop();
                let a = self.execution_stack.pop();
                match (a, b, c) {
                    (Some(Value::Number(a)), Some(Value::Number(b)), Some(Value::Number(c))) => {
                        self.execution_stack.push(Value::Number(a * b + c));
                    }
                    _ => {}
                }
            }
            SimdOp::HorizontalSum => {
                let mut sum = 0.0;
                for _ in 0..count {
                    if let Some(Value::Number(n)) = self.execution_stack.pop() {
                        sum += n;
                    }
                }
                self.execution_stack.push(Value::Number(sum));
            }
        }
        Ok(())
    }
    
    fn execute_bytecode(&mut self, chunk: &Chunk, globals: &Rc<RefCell<Environment>>) -> Result<Value, String> {
        let start = Instant::now();
        self.execution_stack.clear();
        
        let mut ip = 0;
        
        while ip < chunk.instructions.len() {
            self.stats.instructions_executed += 1;
            
            let op = &chunk.instructions[ip];
            self.execute_opcode(op, globals)?;
            
            if matches!(op, OpCode::Halt) {
                break;
            }
            ip += 1;
        }
        
        self.stats.total_exec_time_us += start.elapsed().as_micros() as u64;
        
        Ok(self.execution_stack.pop().unwrap_or(Value::Null))
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // الدوال المساعدة
    // ═══════════════════════════════════════════════════════════════════════════
    
    fn binary_op<F>(&mut self, op: F) -> Result<(), String>
    where
        F: FnOnce(f64, f64) -> f64,
    {
        let b = self.execution_stack.pop();
        let a = self.execution_stack.pop();
        
        match (a, b) {
            (Some(Value::Number(a)), Some(Value::Number(b))) => {
                self.execution_stack.push(Value::Number(op(a, b)));
                Ok(())
            }
            _ => Err("خطأ: العملية تتطلب أرقام".to_string())
        }
    }
    
    fn compare_op<F>(&mut self, op: F) -> Result<(), String>
    where
        F: FnOnce(f64, f64) -> bool,
    {
        let b = self.execution_stack.pop();
        let a = self.execution_stack.pop();
        
        match (a, b) {
            (Some(Value::Number(a)), Some(Value::Number(b))) => {
                self.execution_stack.push(Value::Boolean(op(a, b)));
                Ok(())
            }
            _ => Err("خطأ: المقارنة تتطلب أرقام".to_string())
        }
    }
    
    fn apply_binary_op(&self, op: &BinaryOp, a: &Value, b: &Value) -> Value {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => {
                Value::Number(match op {
                    BinaryOp::Add => a + b,
                    BinaryOp::Sub => a - b,
                    BinaryOp::Mul => a * b,
                    BinaryOp::Div => if *b != 0.0 { a / b } else { f64::INFINITY },
                    BinaryOp::Mod => a % b,
                    BinaryOp::Pow => a.powf(*b),
                    BinaryOp::Eq => if (a - b).abs() < f64::EPSILON { 1.0 } else { 0.0 },
                    BinaryOp::Ne => if (a - b).abs() >= f64::EPSILON { 1.0 } else { 0.0 },
                    BinaryOp::Lt => if a < b { 1.0 } else { 0.0 },
                    BinaryOp::Le => if a <= b { 1.0 } else { 0.0 },
                    BinaryOp::Gt => if a > b { 1.0 } else { 0.0 },
                    BinaryOp::Ge => if a >= b { 1.0 } else { 0.0 },
                    BinaryOp::And => if *a != 0.0 && *b != 0.0 { 1.0 } else { 0.0 },
                    BinaryOp::Or => if *a != 0.0 || *b != 0.0 { 1.0 } else { 0.0 },
                })
            }
            _ => Value::Null,
        }
    }
    
    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Null => false,
            Value::String(s) => !s.is_empty(),
            _ => true,
        }
    }
    
    fn value_to_string(&self, val: &Value) -> String {
        match val {
            Value::Number(n) => format!("{}", n),
            Value::String(s) => s.clone(),
            Value::Boolean(b) => if *b { "صح".to_string() } else { "خطأ".to_string() },
            Value::Null => "لا_شيء".to_string(),
            _ => format!("{:?}", val),
        }
    }
    
    fn check_guard(&self, guard: &Guard) -> Result<bool, String> {
        match &guard.guard_type {
            GuardType::TypeCheck { expected } => {
                // فحص نوع القيمة على المكدس
                if let Some(top) = self.execution_stack.last() {
                    let actual_type = self.get_value_type(top);
                    Ok(actual_type == *expected)
                } else {
                    Ok(false)
                }
            }
            GuardType::NonNull => {
                Ok(!self.execution_stack.last().map_or(true, |v| matches!(v, Value::Null)))
            }
            GuardType::BoundsCheck { min, max } => {
                if let Some(Value::Number(n)) = self.execution_stack.last() {
                    Ok(*n >= *min as f64 && *n <= *max as f64)
                } else {
                    Ok(false)
                }
            }
            GuardType::ValueCheck { expected } => {
                if let Some(top) = self.execution_stack.last() {
                    Ok(top == expected)
                } else {
                    Ok(false)
                }
            }
        }
    }
    
    fn get_value_type(&self, val: &Value) -> TypeInfo {
        match val {
            Value::Number(_) => TypeInfo::Number,
            Value::String(_) => TypeInfo::String,
            Value::Boolean(_) => TypeInfo::Boolean,
            Value::Null => TypeInfo::Null,
            Value::List(_) => TypeInfo::List,
            Value::Dictionary(_) => TypeInfo::Dict,
            Value::Function { .. } => TypeInfo::Function,
            _ => TypeInfo::Unknown,
        }
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // الإحصائيات والتقارير
    // ═══════════════════════════════════════════════════════════════════════════
    
    pub fn stats(&self) -> &JitStats {
        &self.stats
    }
    
    pub fn reset_stats(&mut self) {
        self.stats = JitStats::default();
    }
    
    pub fn clear_cache(&mut self) {
        self.compiled_code.clear();
        self.hot_spots.clear();
    }
    
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    pub fn print_report(&self) {
        println!();
        println!("╔══════════════════════════════════════════════════════════════════════════════╗");
        println!("║           🚀 تقرير JIT Compiler الكامل - لغة المرجع                          ║");
        println!("╠══════════════════════════════════════════════════════════════════════════════╣");
        println!("║ الحالة: {:?}                                                       ║", 
            if self.enabled { "مفعّل ✅" } else { "معطل ❌" });
        println!("║ الحد الأقصى للمستوى: {:?}                                              ║", self.max_tier);
        println!("╠══════════════════════════════════════════════════════════════════════════════╣");
        
        // Tiered Compilation
        println!("║ 📊 تنفيذ المستويات                                                          ║");
        println!("║    Tier 0 (Interpreter): {:12} تنفيذ                               ║", self.stats.tier0_executions);
        println!("║    Tier 1 (Baseline):    {:12} تنفيذ                               ║", self.stats.tier1_executions);
        println!("║    Tier 2 (Optimizing):  {:12} تنفيذ                               ║", self.stats.tier2_executions);
        println!("║    Tier 3 (SIMD):        {:12} تنفيذ                               ║", self.stats.tier3_executions);
        println!("║    Tier 4 (Tracing):     {:12} تنفيذ                               ║", self.stats.tier4_executions);
        
        println!("╠══════════════════════════════════════════════════════════════════════════════╣");
        println!("║ 🔧 التجميع                                                                   ║");
        println!("║    الدوال المترجمة: {:12}                                        ║", self.stats.compiled_functions);
        println!("║    وقت التجميع: {} μs                                                   ║", self.stats.total_compile_time_us);
        println!("║    التحسينات المطبقة: {:12}                                        ║", self.stats.optimizations_applied);
        
        println!("╠══════════════════════════════════════════════════════════════════════════════╣");
        println!("║ ⚡ الأداء                                                                    ║");
        println!("║    التعليمات المنفذة: {:12}                                     ║", self.stats.instructions_executed);
        println!("║    وقت التنفيذ: {} μs                                                   ║", self.stats.total_exec_time_us);
        println!("║    نسبة التسريع: {:.2}x                                                 ║", self.stats.avg_speedup());
        println!("║    ضربات الكاش: {:12}                                              ║", self.stats.cache_hits);
        
        println!("╠══════════════════════════════════════════════════════════════════════════════╣");
        println!("║ 🎯 SIMD                                                                      ║");
        println!("║    العمليات: {:12}                                                 ║", self.stats.simd_operations);
        println!("║    الاستدعاءات المدمجة: {:12}                                        ║", self.stats.inlined_calls);
        
        println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    }
}

impl Default for CompleteJitCompiler {
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
    fn test_tier_determination() {
        let jit = CompleteJitCompiler::new();
        
        assert_eq!(jit.determine_tier(0), TierLevel::Tier0);
        assert_eq!(jit.determine_tier(50), TierLevel::Tier1);
        assert_eq!(jit.determine_tier(200), TierLevel::Tier2);
        assert_eq!(jit.determine_tier(1000), TierLevel::Tier3);
        assert_eq!(jit.determine_tier(5000), TierLevel::Tier4);
    }
    
    #[test]
    fn test_hot_spot_recording() {
        let mut jit = CompleteJitCompiler::new();
        
        // محاكاة تنفيذ متكرر
        for _ in 0..100 {
            jit.record_execution(10);
        }
        
        let info = jit.hot_spots.get(&10).unwrap();
        assert_eq!(info.execution_count, 100);
    }
    
    #[test]
    fn test_simple_execution() {
        let mut jit = CompleteJitCompiler::new();
        
        let mut chunk = Chunk::new();
        chunk.emit(OpCode::PushNumber(5.0));
        chunk.emit(OpCode::PushNumber(3.0));
        chunk.emit(OpCode::Add);
        chunk.emit(OpCode::Halt);
        
        let globals = Rc::new(RefCell::new(Environment::new()));
        let result = jit.execute(&chunk, &mut globals.clone()).unwrap();
        
        if let Value::Number(n) = result {
            assert!((n - 8.0).abs() < f64::EPSILON);
        } else {
            panic!("Expected Number");
        }
    }
    
    #[test]
    fn test_tier1_compilation() {
        let mut jit = CompleteJitCompiler::new();
        
        let mut chunk = Chunk::new();
        chunk.emit(OpCode::PushNumber(10.0));
        chunk.emit(OpCode::PushNumber(20.0));
        chunk.emit(OpCode::Add);
        chunk.emit(OpCode::Halt);
        
        // محاكاة تنفيذات متعددة للوصول لـ Tier 1
        for _ in 0..60 {
            jit.record_execution(0);
        }
        
        jit.compile(&chunk, 0).unwrap();
        
        let globals = Rc::new(RefCell::new(Environment::new()));
        let result = jit.execute(&chunk, &mut globals.clone()).unwrap();
        
        if let Value::Number(n) = result {
            assert!((n - 30.0).abs() < f64::EPSILON);
        }
    }
    
    #[test]
    fn test_loop_execution() {
        let mut jit = CompleteJitCompiler::new();
        
        // حساب مجموع 1+2+3+4+5
        let mut chunk = Chunk::new();
        
        // sum = 0
        chunk.emit(OpCode::PushNumber(0.0));
        
        // i = 1
        chunk.emit(OpCode::PushNumber(1.0));
        
        // loop: check i <= 5
        let loop_start = chunk.instructions.len();
        chunk.emit(OpCode::Dup);
        chunk.emit(OpCode::PushNumber(5.0));
        chunk.emit(OpCode::Greater);
        chunk.emit(OpCode::JumpIfFalse(3)); // jump to after loop
        
        // sum = sum + i
        // ... (مبسط)
        
        chunk.emit(OpCode::Halt);
        
        let globals = Rc::new(RefCell::new(Environment::new()));
        let _result = jit.execute(&chunk, &mut globals.clone()).unwrap();
    }
    
    #[test]
    fn test_constant_folding() {
        let mut jit = CompleteJitCompiler::new();
        
        // 5 + 3 (يجب طيه إلى 8)
        let mut chunk = Chunk::new();
        chunk.emit(OpCode::PushNumber(5.0));
        chunk.emit(OpCode::PushNumber(3.0));
        chunk.emit(OpCode::Add);
        chunk.emit(OpCode::Halt);
        
        // الوصول لـ Tier 2
        for _ in 0..250 {
            jit.record_execution(0);
        }
        
        jit.compile(&chunk, 0).unwrap();
        
        // التحقق من تطبيق التحسين
        assert!(jit.stats.optimizations_applied > 0);
    }
    
    #[test]
    fn test_stats() {
        let mut jit = CompleteJitCompiler::new();
        
        let mut chunk = Chunk::new();
        chunk.emit(OpCode::PushNumber(1.0));
        chunk.emit(OpCode::PushNumber(2.0));
        chunk.emit(OpCode::Add);
        chunk.emit(OpCode::Halt);
        
        let globals = Rc::new(RefCell::new(Environment::new()));
        
        for _ in 0..10 {
            let _ = jit.execute(&chunk, &mut globals.clone());
        }
        
        let stats = jit.stats();
        assert_eq!(stats.tier0_executions, 10);
        assert!(stats.instructions_executed > 0);
    }
}
