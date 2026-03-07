// ═══════════════════════════════════════════════════════════════════════════════
// Async/Await JIT Support - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// دعم التنفيذ غير المتزامن في JIT:
// - تحويل الدوال إلى state machines
// - تحسين الـ await points
// - إدارة الـ futures بكفاءة
// - تجميع الـ async code
// - تنفيذ متوازي للـ tasks
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::{HashMap, VecDeque};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::pin::Pin;
use std::future::Future;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};
use std::time::{Duration, Instant};

use super::opcodes::OpCode;
use super::type_inference::Type;
use crate::interpreter::value::{Value, SharedValue, Environment};

// ═══════════════════════════════════════════════════════════════════════════════
// تعريف الـ Async Types
// ═══════════════════════════════════════════════════════════════════════════════

/// حالة الـ async function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncState {
    /// لم يبدأ بعد
    Pending,
    /// في انتظار نتيجة
    Waiting,
    /// جاهز للاستمرار
    Ready,
    /// انتهى بنجاح
    Completed,
    /// انتهى بخطأ
    Failed,
}

/// نتيجة async
#[derive(Debug, Clone)]
pub struct AsyncResult {
    /// الحالة
    pub state: AsyncState,
    /// القيمة (إن وجدت)
    pub value: Option<Value>,
    /// الخطأ (إن وجد)
    pub error: Option<String>,
    /// وقت البداية
    pub start_time: Instant,
    /// وقت الانتهاء
    pub end_time: Option<Instant>,
}

impl AsyncResult {
    pub fn pending() -> Self {
        AsyncResult {
            state: AsyncState::Pending,
            value: None,
            error: None,
            start_time: Instant::now(),
            end_time: None,
        }
    }
    
    pub fn completed(value: Value) -> Self {
        AsyncResult {
            state: AsyncState::Completed,
            value: Some(value),
            error: None,
            start_time: Instant::now(),
            end_time: Some(Instant::now()),
        }
    }
    
    pub fn failed(error: String) -> Self {
        AsyncResult {
            state: AsyncState::Failed,
            value: None,
            error: Some(error),
            start_time: Instant::now(),
            end_time: Some(Instant::now()),
        }
    }
}

/// نقطة انتظار في الـ async function
#[derive(Debug, Clone)]
pub struct AwaitPoint {
    /// موقع التعليمة
    pub ip: usize,
    /// نوع الانتظار
    pub await_type: AwaitType,
    /// حفظ حالة المكدس
    pub saved_stack: Vec<Value>,
    /// حفظ المتغيرات المحلية
    pub saved_locals: HashMap<u32, Value>,
    /// الحالة التالية
    pub next_state: u32,
}

/// نوع الانتظار
#[derive(Debug, Clone)]
pub enum AwaitType {
    /// انتظار مستقبل
    Future,
    /// انتظار تأخير زمني
    Delay { duration_ms: u64 },
    /// انتظار I/O
    IO { resource_id: u64 },
    /// انتظار رسالة
    Message { channel_id: u64 },
    /// انتظار حدث
    Event { event_name: String },
}

// ═══════════════════════════════════════════════════════════════════════════════
// State Machine للـ Async Functions
// ═══════════════════════════════════════════════════════════════════════════════

/// State machine للدالة غير المتزامنة
#[derive(Debug)]
pub struct AsyncStateMachine {
    /// اسم الدالة
    pub name: String,
    /// الحالة الحالية
    pub current_state: u32,
    /// الحالات
    pub states: Vec<AsyncStateInfo>,
    /// نقاط الانتظار
    pub await_points: HashMap<u32, AwaitPoint>,
    /// المكدس المحفوظ
    pub saved_stack: Vec<Value>,
    /// المتغيرات المحلية المحفوظة
    pub saved_locals: HashMap<u32, Value>,
    /// النتيجة
    pub result: AsyncResult,
    /// الـ Waker
    pub waker: Option<Waker>,
    /// هل تم إلغاء التنفيذ
    pub cancelled: bool,
}

/// معلومات حالة
#[derive(Debug, Clone)]
pub struct AsyncStateInfo {
    /// رقم الحالة
    pub state_id: u32,
    /// عنوان البداية
    pub start_ip: usize,
    /// عنوان النهاية
    pub end_ip: usize,
    /// نوع الحالة
    pub state_type: StateType,
    /// التعليمات
    pub instructions: Vec<OpCode>,
    /// الحالة التالية
    pub next_state: Option<u32>,
}

/// نوع الحالة
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateType {
    /// بداية الدالة
    Entry,
    /// كود عادي
    Normal,
    /// نقطة انتظار
    Await,
    /// بعد الانتظار
    PostAwait,
    /// نهاية الدالة
    Exit,
    /// معالجة خطأ
    Error,
}

impl AsyncStateMachine {
    /// إنشاء state machine جديدة
    pub fn new(name: String) -> Self {
        AsyncStateMachine {
            name,
            current_state: 0,
            states: Vec::new(),
            await_points: HashMap::new(),
            saved_stack: Vec::new(),
            saved_locals: HashMap::new(),
            result: AsyncResult::pending(),
            waker: None,
            cancelled: false,
        }
    }
    
    /// إضافة حالة
    pub fn add_state(&mut self, state: AsyncStateInfo) {
        self.states.push(state);
    }
    
    /// إضافة نقطة انتظار
    pub fn add_await_point(&mut self, point: AwaitPoint) {
        let state_id = point.next_state;
        self.await_points.insert(state_id, point);
    }
    
    /// حفظ الحالة الحالية
    pub fn save_state(&mut self, stack: &[Value], locals: &HashMap<u32, Value>) {
        self.saved_stack = stack.to_vec();
        self.saved_locals = locals.clone();
    }
    
    /// استعادة الحالة
    pub fn restore_state(&self) -> (Vec<Value>, HashMap<u32, Value>) {
        (self.saved_stack.clone(), self.saved_locals.clone())
    }
    
    /// التقدم خطوة واحدة
    pub fn advance(&mut self) -> Poll<AsyncResult> {
        if self.cancelled {
            return Poll::Ready(AsyncResult::failed("تم إلغاء التنفيذ".into()));
        }
        
        // محاكاة التقدم
        if self.current_state < self.states.len() as u32 {
            let state = &self.states[self.current_state as usize];
            self.current_state = state.next_state.unwrap_or(self.current_state + 1);
            
            // التحقق من نقاط الانتظار
            if let Some(await_point) = self.await_points.get(&self.current_state) {
                self.result.state = AsyncState::Waiting;
                return Poll::Pending;
            }
            
            Poll::Pending
        } else {
            self.result.state = AsyncState::Completed;
            self.result.end_time = Some(Instant::now());
            Poll::Ready(self.result.clone())
        }
    }
    
    /// إلغاء التنفيذ
    pub fn cancel(&mut self) {
        self.cancelled = true;
    }
}

// تنفيذ Future لـ AsyncStateMachine
impl Future for AsyncStateMachine {
    type Output = AsyncResult;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.waker = Some(cx.waker().clone());
        self.advance()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Async JIT Compiler
// ═══════════════════════════════════════════════════════════════════════════════

/// مترجم الـ async code
pub struct AsyncJitCompiler {
    /// أسماء الـ state machines المترجمة
    compiled_state_machine_names: Vec<String>,
    /// إحصائيات
    stats: AsyncJitStats,
    /// التحسينات المطبقة
    optimizations: AsyncOptimizations,
}

/// إحصائيات الـ async JIT
#[derive(Debug, Clone, Default)]
pub struct AsyncJitStats {
    /// عدد الدوال المترجمة
    pub compiled_functions: u64,
    /// عدد نقاط الانتظار
    pub await_points: u64,
    /// عدد الـ state machines
    pub state_machines: u64,
    /// وقت الترجمة
    pub compile_time_us: u64,
    /// وقت التنفيذ
    pub execution_time_us: u64,
    /// عدد الـ polls
    pub total_polls: u64,
    /// عدد الـ completions
    pub completions: u64,
}

/// تحسينات الـ async
#[derive(Debug, Clone, Default)]
pub struct AsyncOptimizations {
    /// دمج نقاط الانتظار المتتالية
    pub await_coalescing: bool,
    /// تحسين الـ state machine
    pub state_machine_optimization: bool,
    /// تخزين مؤقت للـ wakers
    pub waker_caching: bool,
    /// إزالة الحالات غير المستخدمة
    pub dead_state_elimination: bool,
}

impl AsyncJitCompiler {
    /// إنشاء مترجم جديد
    pub fn new() -> Self {
        AsyncJitCompiler {
            compiled_state_machine_names: Vec::new(),
            stats: AsyncJitStats::default(),
            optimizations: AsyncOptimizations {
                await_coalescing: true,
                state_machine_optimization: true,
                waker_caching: true,
                dead_state_elimination: true,
            },
        }
    }
    
    /// تجميع دالة async
    pub fn compile_async_function(
        &mut self,
        name: &str,
        instructions: &[OpCode],
    ) -> Result<AsyncStateMachine, String> {
        let start = Instant::now();
        
        let mut state_machine = AsyncStateMachine::new(name.to_string());
        
        // تحليل التعليمات وتقسيمها إلى حالات
        let states = self.analyze_and_split_states(instructions);
        
        for state in states {
            if state.state_type == StateType::Await {
                // إنشاء نقطة انتظار
                let await_point = AwaitPoint {
                    ip: state.start_ip,
                    await_type: AwaitType::Future,
                    saved_stack: Vec::new(),
                    saved_locals: HashMap::new(),
                    next_state: state.state_id + 1,
                };
                state_machine.add_await_point(await_point);
                self.stats.await_points += 1;
            }
            state_machine.add_state(state);
        }
        
        // تطبيق التحسينات
        self.optimize_state_machine(&mut state_machine);
        
        // حفظ اسم الـ state machine
        self.compiled_state_machine_names.push(name.to_string());
        
        self.stats.compiled_functions += 1;
        self.stats.state_machines += 1;
        self.stats.compile_time_us += start.elapsed().as_micros() as u64;
        
        Ok(state_machine)
    }
    
    /// تحليل وتقسيم الحالات
    fn analyze_and_split_states(&self, instructions: &[OpCode]) -> Vec<AsyncStateInfo> {
        let mut states = Vec::new();
        let mut current_state_id = 0u32;
        let mut current_state_start = 0usize;
        let mut current_instructions = Vec::new();
        
        for (ip, opcode) in instructions.iter().enumerate() {
            match opcode {
                OpCode::Await => {
                    // إنهاء الحالة الحالية
                    if !current_instructions.is_empty() {
                        states.push(AsyncStateInfo {
                            state_id: current_state_id,
                            start_ip: current_state_start,
                            end_ip: ip,
                            state_type: StateType::Normal,
                            instructions: current_instructions.clone(),
                            next_state: Some(current_state_id + 1),
                        });
                        current_state_id += 1;
                    }
                    
                    // حالة الانتظار
                    states.push(AsyncStateInfo {
                        state_id: current_state_id,
                        start_ip: ip,
                        end_ip: ip + 1,
                        state_type: StateType::Await,
                        instructions: vec![opcode.clone()],
                        next_state: Some(current_state_id + 1),
                    });
                    current_state_id += 1;
                    
                    current_state_start = ip + 1;
                    current_instructions.clear();
                }
                OpCode::Return | OpCode::ReturnValue => {
                    current_instructions.push(opcode.clone());
                    
                    // حالة النهاية
                    states.push(AsyncStateInfo {
                        state_id: current_state_id,
                        start_ip: current_state_start,
                        end_ip: ip,
                        state_type: StateType::Normal,
                        instructions: current_instructions.clone(),
                        next_state: Some(current_state_id + 1),
                    });
                    current_state_id += 1;
                    
                    states.push(AsyncStateInfo {
                        state_id: current_state_id,
                        start_ip: ip,
                        end_ip: ip + 1,
                        state_type: StateType::Exit,
                        instructions: vec![opcode.clone()],
                        next_state: None,
                    });
                    
                    current_state_start = ip + 1;
                    current_instructions.clear();
                }
                OpCode::Halt => {
                    // نهاية
                    states.push(AsyncStateInfo {
                        state_id: current_state_id,
                        start_ip: current_state_start,
                        end_ip: ip,
                        state_type: StateType::Exit,
                        instructions: current_instructions.clone(),
                        next_state: None,
                    });
                }
                _ => {
                    current_instructions.push(opcode.clone());
                }
            }
        }
        
        // إضافة الحالة الأخيرة إن وجدت
        if !current_instructions.is_empty() {
            states.push(AsyncStateInfo {
                state_id: current_state_id,
                start_ip: current_state_start,
                end_ip: instructions.len(),
                state_type: StateType::Normal,
                instructions: current_instructions,
                next_state: None,
            });
        }
        
        states
    }
    
    /// تحسين الـ state machine
    fn optimize_state_machine(&self, state_machine: &mut AsyncStateMachine) {
        if self.optimizations.dead_state_elimination {
            self.eliminate_dead_states(state_machine);
        }
        
        if self.optimizations.await_coalescing {
            self.coalesce_awaits(state_machine);
        }
        
        if self.optimizations.state_machine_optimization {
            self.optimize_state_transitions(state_machine);
        }
    }
    
    /// إزالة الحالات غير المستخدمة
    fn eliminate_dead_states(&self, state_machine: &mut AsyncStateMachine) {
        let mut reachable: Vec<bool> = vec![false; state_machine.states.len()];
        
        // الحالة الأولى قابلة للوصول
        if !reachable.is_empty() {
            reachable[0] = true;
        }
        
        // تتبع الحالات القابلة للوصول
        for i in 0..state_machine.states.len() {
            if reachable[i] {
                if let Some(next) = state_machine.states[i].next_state {
                    if (next as usize) < reachable.len() {
                        reachable[next as usize] = true;
                    }
                }
            }
        }
        
        // الاحتفاظ بالحالات القابلة للوصول فقط
        let mut new_states = Vec::new();
        for (i, state) in state_machine.states.drain(..).enumerate() {
            if reachable[i] {
                new_states.push(state);
            }
        }
        state_machine.states = new_states;
    }
    
    /// دمج نقاط الانتظار المتتالية
    fn coalesce_awaits(&self, state_machine: &mut AsyncStateMachine) {
        let mut i = 0;
        while i < state_machine.states.len() {
            if state_machine.states[i].state_type == StateType::Await {
                // البحث عن awaits متتالية
                let mut await_count = 1;
                let mut j = i + 1;
                while j < state_machine.states.len() && 
                      state_machine.states[j].state_type == StateType::Await {
                    await_count += 1;
                    j += 1;
                }
                
                if await_count > 1 {
                    // دمج awaits
                    state_machine.states[i].next_state = state_machine.states[j - 1].next_state;
                    // إزالة awaits المكررة
                    for _ in 1..await_count {
                        state_machine.states.remove(i + 1);
                    }
                }
            }
            i += 1;
        }
    }
    
    /// تحسين انتقالات الحالة
    fn optimize_state_transitions(&self, state_machine: &mut AsyncStateMachine) {
        // تحسين التحويلات المباشرة
        for i in 0..state_machine.states.len() {
            let next = state_machine.states[i].next_state;
            if let Some(next_id) = next {
                if (next_id as usize) < state_machine.states.len() {
                    let next_state = &state_machine.states[next_id as usize];
                    // إذا كانت الحالة التالية فارغة، تخطيها
                    if next_state.instructions.is_empty() {
                        state_machine.states[i].next_state = next_state.next_state;
                    }
                }
            }
        }
    }
    
    /// الحصول على أسماء الـ state machines المترجمة
    pub fn get_compiled_names(&self) -> &[String] {
        &self.compiled_state_machine_names
    }
    
    /// التحقق من وجود دالة مترجمة
    pub fn has_compiled(&self, name: &str) -> bool {
        self.compiled_state_machine_names.iter().any(|n| n == name)
    }
    
    /// طباعة تقرير
    pub fn print_report(&self) {
        println!("╔══════════════════════════════════════════════════════════════════════════╗");
        println!("║              ⚡ تقرير Async JIT - لغة المرجع                             ║");
        println!("╠══════════════════════════════════════════════════════════════════════════╣");
        
        println!("║ 📊 الإحصائيات:                                                           ║");
        println!("║    الدوال المترجمة: {:15}                                   ║", self.stats.compiled_functions);
        println!("║    نقاط الانتظار: {:15}                                     ║", self.stats.await_points);
        println!("║    State Machines: {:15}                                    ║", self.stats.state_machines);
        println!("║    وقت الترجمة: {} μs                                                ║", self.stats.compile_time_us);
        
        println!("╠══════════════════════════════════════════════════════════════════════════╣");
        println!("║ 🔧 التحسينات:                                                            ║");
        println!("║    دمج نقاط الانتظار: {:15}                                ║",
            if self.optimizations.await_coalescing { "مفعّل" } else { "معطّل" });
        println!("║    تحسين State Machine: {:15}                              ║",
            if self.optimizations.state_machine_optimization { "مفعّل" } else { "معطّل" });
        println!("║    تخزين Wakers: {:15}                                        ║",
            if self.optimizations.waker_caching { "مفعّل" } else { "معطّل" });
        
        println!("╚══════════════════════════════════════════════════════════════════════════╝");
    }
}

impl Default for AsyncJitCompiler {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Async Runtime
// ═══════════════════════════════════════════════════════════════════════════════

/// مشغل الـ async tasks
pub struct AsyncRuntime {
    /// قائمة المهام الجاهزة
    ready_queue: VecDeque<Box<dyn Future<Output = AsyncResult> + Send>>,
    /// قائمة المهام المنتظرة
    waiting_tasks: HashMap<u64, Box<dyn Future<Output = AsyncResult> + Send>>,
    /// معرف المهمة التالي
    next_task_id: u64,
    /// إحصائيات
    stats: RuntimeStats,
}

/// إحصائيات الـ runtime
#[derive(Debug, Clone, Default)]
pub struct RuntimeStats {
    pub tasks_created: u64,
    pub tasks_completed: u64,
    pub tasks_cancelled: u64,
    pub total_polls: u64,
    pub max_concurrent_tasks: u64,
}

impl AsyncRuntime {
    /// إنشاء runtime جديد
    pub fn new() -> Self {
        AsyncRuntime {
            ready_queue: VecDeque::new(),
            waiting_tasks: HashMap::new(),
            next_task_id: 1,
            stats: RuntimeStats::default(),
        }
    }
    
    /// إضافة مهمة
    pub fn spawn(&mut self, task: Box<dyn Future<Output = AsyncResult> + Send>) -> u64 {
        let id = self.next_task_id;
        self.next_task_id += 1;
        
        self.ready_queue.push_back(task);
        self.stats.tasks_created += 1;
        
        let current_tasks = self.ready_queue.len() + self.waiting_tasks.len();
        if current_tasks as u64 > self.stats.max_concurrent_tasks {
            self.stats.max_concurrent_tasks = current_tasks as u64;
        }
        
        id
    }
    
    /// تشغيل دورة واحدة
    pub fn tick(&mut self) -> Option<AsyncResult> {
        if let Some(mut task) = self.ready_queue.pop_front() {
            // إنشاء waker بسيط
            let waker = self.create_simple_waker();
            let mut cx = Context::from_waker(&waker);
            
            self.stats.total_polls += 1;
            
            // Poll الـ task
            // في التنفيذ الحقيقي، سنستخدم Pin::new
            // هنا نستخدم محاكاة بسيطة
            None
        } else {
            None
        }
    }
    
    /// تشغيل جميع المهام
    pub fn run_all(&mut self) -> Vec<AsyncResult> {
        let mut results = Vec::new();
        
        while !self.ready_queue.is_empty() {
            if let Some(result) = self.tick() {
                results.push(result);
                self.stats.tasks_completed += 1;
            }
        }
        
        results
    }
    
    /// إنشاء waker بسيط
    fn create_simple_waker(&self) -> Waker {
        // Waker بسيط لا يفعل شيئاً
        fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        fn wake(_: *const ()) {}
        fn wake_by_ref(_: *const ()) {}
        fn drop(_: *const ()) {}
        
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
        unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
    }
    
    /// الحصول على عدد المهام النشطة
    pub fn active_task_count(&self) -> usize {
        self.ready_queue.len() + self.waiting_tasks.len()
    }
}

impl Default for AsyncRuntime {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Async Opcodes
// ═══════════════════════════════════════════════════════════════════════════════

/// تعليمات async إضافية
#[derive(Debug, Clone)]
pub enum AsyncOpCode {
    /// بدء دالة async
    AsyncStart { func_id: u32 },
    /// نقطة انتظار
    Await,
    /// انتظار مع مهلة
    AwaitWithTimeout { timeout_ms: u64 },
    /// إرجاع من async
    AsyncReturn,
    /// إرجاع قيمة من async
    AsyncReturnValue,
    /// إلغاء async
    AsyncCancel { task_id: u64 },
    /// انتظار جميع المهام
    AwaitAll { task_count: u32 },
    /// انتظار أي مهمة
    AwaitAny { task_count: u32 },
    /// إنشاء مهمة جديدة
    Spawn { func_id: u32 },
    /// إرسال رسالة
    Send { channel_id: u64 },
    /// استقبال رسالة
    Receive { channel_id: u64 },
    /// نوم
    Sleep { duration_ms: u64 },
    /// تحقق من اكتمال
    IsCompleted { task_id: u64 },
    /// الحصول على النتيجة
    GetResult { task_id: u64 },
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_async_state_machine() {
        let mut sm = AsyncStateMachine::new("test".to_string());
        
        sm.add_state(AsyncStateInfo {
            state_id: 0,
            start_ip: 0,
            end_ip: 5,
            state_type: StateType::Entry,
            instructions: vec![OpCode::PushNumber(1.0)],
            next_state: Some(1),
        });
        
        sm.add_state(AsyncStateInfo {
            state_id: 1,
            start_ip: 5,
            end_ip: 10,
            state_type: StateType::Exit,
            instructions: vec![OpCode::Halt],
            next_state: None,
        });
        
        assert_eq!(sm.states.len(), 2);
    }
    
    #[test]
    fn test_async_jit_compiler() {
        let mut compiler = AsyncJitCompiler::new();
        
        let instructions = vec![
            OpCode::PushNumber(1.0),
            OpCode::PushNumber(2.0),
            OpCode::Add,
            OpCode::Halt,
        ];
        
        let result = compiler.compile_async_function("test", &instructions);
        assert!(result.is_ok());
        
        let sm = result.unwrap();
        assert!(!sm.states.is_empty());
    }
    
    #[test]
    fn test_async_result() {
        let pending = AsyncResult::pending();
        assert_eq!(pending.state, AsyncState::Pending);
        
        let completed = AsyncResult::completed(Value::Number(42.0));
        assert_eq!(completed.state, AsyncState::Completed);
        assert_eq!(completed.value, Some(Value::Number(42.0)));
        
        let failed = AsyncResult::failed("خطأ".into());
        assert_eq!(failed.state, AsyncState::Failed);
        assert_eq!(failed.error, Some("خطأ".into()));
    }
    
    #[test]
    fn test_async_runtime() {
        let mut runtime = AsyncRuntime::new();
        
        assert_eq!(runtime.active_task_count(), 0);
        assert_eq!(runtime.stats.tasks_created, 0);
    }
    
    #[test]
    fn test_state_splitting() {
        let compiler = AsyncJitCompiler::new();
        
        let instructions = vec![
            OpCode::PushNumber(1.0),
            OpCode::Await,
            OpCode::PushNumber(2.0),
            OpCode::Halt,
        ];
        
        let states = compiler.analyze_and_split_states(&instructions);
        
        // يجب أن يكون لدينا حالات متعددة بسبب Await
        assert!(states.len() >= 2);
        
        // التحقق من وجود حالة Await
        let has_await_state = states.iter().any(|s| s.state_type == StateType::Await);
        assert!(has_await_state);
    }
}
