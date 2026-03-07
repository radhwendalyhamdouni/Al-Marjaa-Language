// ═══════════════════════════════════════════════════════════════════════════════
// الآلة الافتراضية (Virtual Machine) - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// آلة افتراضية سريعة مبنية على المكدس
// مصممة لأداء عالي مع استهلاك ذاكرة منخفض
// 
// التحسينات المطبقة:
// - Inline Caching للمتغيرات العامة
// - تحسين إدارة المكدس
// - تقليل الاستنساخ (cloning)
// - تحسين العمليات الحسابية
// ═══════════════════════════════════════════════════════════════════════════════

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::opcodes::{Chunk, OpCode};
use crate::interpreter::value::{Environment, Value, SharedValue};

/// نتيجة التنفيذ
#[derive(Debug)]
pub enum ExecutionResult {
    /// تم بنجاح مع قيمة
    Ok(SharedValue),
    /// خطأ
    Error(String),
    /// توقف (break)
    Break,
    /// متابعة (continue)
    Continue,
    /// إرجاع من دالة
    Return(SharedValue),
}

/// إطار استدعاء (Call Frame)
#[derive(Clone)]
pub struct CallFrame {
    /// مؤشر التعليمة الحالية
    pub ip: usize,
    /// فهرس بداية المتغيرات المحلية في المكدس
    pub stack_base: usize,
    /// اسم الدالة (للتصحيح)
    pub name: String,
}

impl CallFrame {
    pub fn new(stack_base: usize, name: &str) -> Self {
        CallFrame {
            ip: 0,
            stack_base,
            name: name.to_string(),
        }
    }
}

/// الحد الأقصى لعمق الاستدعاء (منع stack overflow)
const MAX_RECURSION_DEPTH: usize = 1000;

/// حجم المكدس الأولي
const INITIAL_STACK_SIZE: usize = 512;

/// حجم إطارات الاستدعاء الأولي
const INITIAL_CALL_FRAMES_SIZE: usize = 64;

/// الآلة الافتراضية
pub struct VM {
    /// المكدس
    stack: Vec<SharedValue>,
    /// إطارات الاستدعاء
    call_frames: Vec<CallFrame>,
    /// المتغيرات العامة
    globals: Rc<RefCell<Environment>>,
    /// الكود قيد التنفيذ
    chunk: Option<Chunk>,
    /// مؤشر التعليمة
    ip: usize,
    /// هل متوقف
    halted: bool,
    /// إحصائيات
    stats: VMStats,
    /// عمق الاستدعاء الحالي
    recursion_depth: usize,
    /// كاشش للمتغيرات العامة
    global_cache: HashMap<u32, SharedValue>,
}

/// إحصائيات الأداء
#[derive(Debug, Default, Clone)]
pub struct VMStats {
    /// عدد التعليمات المنفذة
    pub instructions_executed: u64,
    /// عدد استدعاءات الدوال
    pub function_calls: u64,
    /// أقصى حجم للمكدس
    pub max_stack_size: usize,
    /// وقت التنفيذ (ميكروثانية)
    pub execution_time_us: u64,
    /// عدد ضربات الكاش
    pub cache_hits: u64,
    /// عدد أخطاء الكاش
    pub cache_misses: u64,
}

impl VM {
    /// إنشاء آلة افتراضية جديدة
    pub fn new(globals: Rc<RefCell<Environment>>) -> Self {
        VM {
            stack: Vec::with_capacity(INITIAL_STACK_SIZE),
            call_frames: Vec::with_capacity(INITIAL_CALL_FRAMES_SIZE),
            globals,
            chunk: None,
            ip: 0,
            halted: false,
            stats: VMStats::default(),
            recursion_depth: 0,
            global_cache: HashMap::with_capacity(64),
        }
    }
    
    /// إنشاء VM مع بيئة افتراضية
    pub fn with_fresh_env() -> Self {
        Self::new(Rc::new(RefCell::new(Environment::new())))
    }
    
    /// تحميل chunk للتنفيذ
    pub fn load(&mut self, chunk: Chunk) {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.halted = false;
        self.stack.clear();
        self.call_frames.clear();
        self.call_frames.push(CallFrame::new(0, "main"));
        self.global_cache.clear();
    }
    
    /// تنفيذ البرنامج كاملاً
    pub fn run(&mut self) -> ExecutionResult {
        let start = std::time::Instant::now();
        
        // حلقة التنفيذ الرئيسية - محسّنة
        loop {
            match self.step() {
                ExecutionResult::Ok(_) => {
                    if self.halted {
                        let result = self.stack.pop().unwrap_or_else(|| {
                            Rc::new(RefCell::new(Value::Null))
                        });
                        self.stats.execution_time_us = start.elapsed().as_micros() as u64;
                        return ExecutionResult::Ok(result);
                    }
                }
                ExecutionResult::Error(e) => return ExecutionResult::Error(e),
                ExecutionResult::Break => return ExecutionResult::Error("break خارج الحلقة".into()),
                ExecutionResult::Continue => return ExecutionResult::Error("continue خارج الحلقة".into()),
                ExecutionResult::Return(v) => return ExecutionResult::Ok(v),
            }
        }
    }
    
    /// تنفيذ تعليمة واحدة
    pub fn step(&mut self) -> ExecutionResult {
        let opcode = {
            let chunk = match &self.chunk {
                Some(c) => c,
                None => return ExecutionResult::Error("لا يوجد كود للتنفيذ".into()),
            };
            
            if self.ip >= chunk.instructions.len() {
                self.halted = true;
                return ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)));
            }
            
            // استخدام clone فقط عند الضرورة
            chunk.instructions[self.ip].clone()
        };
        
        self.stats.instructions_executed += 1;
        
        // تنفيذ التعليمة
        let result = self.execute_opcode(&opcode);
        
        // تحديث مؤشر التعليمة
        if !self.halted {
            self.ip += 1;
        }
        
        // تحديث إحصائيات المكدس
        let stack_len = self.stack.len();
        if stack_len > self.stats.max_stack_size {
            self.stats.max_stack_size = stack_len;
        }
        
        result
    }
    
    /// تنفيذ تعليمة محددة
    fn execute_opcode(&mut self, opcode: &OpCode) -> ExecutionResult {
        match opcode {
            // ═══════════════════════════════════════════════════════════════
            // تعليمات المكدس - محسّنة
            // ═══════════════════════════════════════════════════════════════
            
            OpCode::PushNumber(n) => {
                self.push_value(Value::Number(*n));
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::PushString(idx) => {
                let chunk = self.chunk.as_ref().unwrap();
                if let Some(s) = chunk.get_string(*idx) {
                    self.push_value(Value::String(s.to_string()));
                } else {
                    return ExecutionResult::Error(format!("فهرس نص غير صالح: {}", idx));
                }
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::PushBool(b) => {
                self.push_value(Value::Boolean(*b));
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::PushNull => {
                self.push_value(Value::Null);
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::Pop => {
                self.pop();
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::Dup => {
                if let Some(v) = self.stack.last() {
                    self.stack.push(Rc::clone(v));
                }
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::Swap => {
                let len = self.stack.len();
                if len >= 2 {
                    self.stack.swap(len - 1, len - 2);
                }
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // ═══════════════════════════════════════════════════════════════
            // العمليات الحسابية - محسّنة للسرعة
            // ═══════════════════════════════════════════════════════════════
            
            OpCode::Add => self.binary_op_fast(|a, b| a + b),
            OpCode::Sub => self.binary_op_fast(|a, b| a - b),
            OpCode::Mul => self.binary_op_fast(|a, b| a * b),
            OpCode::Div => self.binary_op_fast(|a, b| {
                if b == 0.0 { f64::INFINITY } else { a / b }
            }),
            OpCode::Mod => self.binary_op_fast(|a, b| a % b),
            OpCode::Pow => self.binary_op_fast(|a, b| a.powf(b)),
            OpCode::Neg => self.unary_op_fast(|a| -a),
            OpCode::Xor => self.binary_op_fast(|a, b| ((a as i64) ^ (b as i64)) as f64),
            OpCode::Shl => self.binary_op_fast(|a, b| ((a as i64) << (b as i64)) as f64),
            OpCode::Shr => self.binary_op_fast(|a, b| ((a as i64) >> (b as i64)) as f64),
            
            // ═══════════════════════════════════════════════════════════════
            // العمليات المقارنة - محسّنة
            // ═══════════════════════════════════════════════════════════════
            
            OpCode::Equal => self.compare_fast(|a, b| a == b),
            OpCode::NotEqual => self.compare_fast(|a, b| a != b),
            OpCode::Less => self.compare_fast(|a, b| a < b),
            OpCode::Greater => self.compare_fast(|a, b| a > b),
            OpCode::LessEqual => self.compare_fast(|a, b| a <= b),
            OpCode::GreaterEqual => self.compare_fast(|a, b| a >= b),
            
            // ═══════════════════════════════════════════════════════════════
            // العمليات المنطقية
            // ═══════════════════════════════════════════════════════════════
            
            OpCode::And => {
                let b = self.pop_value();
                let a = self.pop_value();
                self.push_value(Value::Boolean(a.is_truthy() && b.is_truthy()));
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::Or => {
                let b = self.pop_value();
                let a = self.pop_value();
                self.push_value(Value::Boolean(a.is_truthy() || b.is_truthy()));
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::Not => {
                let a = self.pop_value();
                self.push_value(Value::Boolean(!a.is_truthy()));
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // ═══════════════════════════════════════════════════════════════
            // المتغيرات - مع Inline Caching
            // ═══════════════════════════════════════════════════════════════
            
            OpCode::LoadGlobal(idx) => {
                // محاولة استخدام الكاش أولاً
                if let Some(cached) = self.global_cache.get(idx) {
                    self.stats.cache_hits += 1;
                    self.stack.push(Rc::clone(cached));
                } else {
                    self.stats.cache_misses += 1;
                    let chunk = self.chunk.as_ref().unwrap();
                    if let Some(name) = chunk.get_string(*idx) {
                        match self.globals.borrow().get(name) {
                            Some(v) => {
                                // تخزين في الكاش
                                self.global_cache.insert(*idx, Rc::clone(&v));
                                self.stack.push(v);
                            }
                            None => return ExecutionResult::Error(format!("المتغير '{}' غير معرف", name)),
                        }
                    }
                }
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::StoreGlobal(idx) => {
                let name = self.chunk.as_ref().and_then(|c| c.get_string(*idx)).map(|s| s.to_string());
                if let Some(name) = name {
                    let value = self.pop();
                    // إبطال الكاش لهذا المتغير
                    self.global_cache.remove(idx);
                    self.globals.borrow_mut().define(&name, (*value.borrow()).clone(), false);
                    // تحديث الكاش
                    self.global_cache.insert(*idx, value);
                }
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::LoadLocal(slot) => {
                let frame = self.call_frames.last().unwrap();
                let index = frame.stack_base + *slot as usize;
                if index < self.stack.len() {
                    self.stack.push(Rc::clone(&self.stack[index]));
                } else {
                    return ExecutionResult::Error("متغير محلي غير صالح".into());
                }
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::StoreLocal(slot) => {
                let frame = self.call_frames.last().unwrap();
                let index = frame.stack_base + *slot as usize;
                if index < self.stack.len() {
                    let value = self.pop();
                    self.stack[index] = value;
                } else {
                    return ExecutionResult::Error("متغير محلي غير صالح".into());
                }
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::DefineConst(idx) => {
                let name = self.chunk.as_ref().and_then(|c| c.get_string(*idx)).map(|s| s.to_string());
                if let Some(name) = name {
                    let value = self.pop();
                    self.globals.borrow_mut().define(&name, (*value.borrow()).clone(), true);
                }
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // ═══════════════════════════════════════════════════════════════
            // التحكم في التدفق
            // ═══════════════════════════════════════════════════════════════
            
            OpCode::Jump(offset) => {
                self.ip = (self.ip as i32 + *offset - 1).max(0) as usize;
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::JumpIfFalse(offset) => {
                let condition = self.pop_value();
                if !condition.is_truthy() {
                    self.ip = (self.ip as i32 + *offset - 1).max(0) as usize;
                }
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::JumpIfTrue(offset) => {
                let condition = self.pop_value();
                if condition.is_truthy() {
                    self.ip = (self.ip as i32 + *offset - 1).max(0) as usize;
                }
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::JumpBack(offset) => {
                self.ip = (self.ip as i32 - *offset - 1).max(0) as usize;
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // ═══════════════════════════════════════════════════════════════
            // الدوال
            // ═══════════════════════════════════════════════════════════════
            
            OpCode::Call(arg_count) => {
                self.stats.function_calls += 1;
                
                if self.recursion_depth >= MAX_RECURSION_DEPTH {
                    return ExecutionResult::Error(format!(
                        "تجاوز حد الاستدعاء المتكرر ({}). قد يكون هناك حلقة لا نهائية.",
                        MAX_RECURSION_DEPTH
                    ));
                }
                
                let callee = self.pop();
                let callee_borrowed = callee.borrow();
                
                match &*callee_borrowed {
                    Value::Function { body: _, closure: _, .. } => {
                        let frame = CallFrame::new(self.stack.len() - *arg_count as usize, "دالة");
                        self.call_frames.push(frame);
                        self.recursion_depth += 1;
                        drop(callee_borrowed);
                        ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
                    }
                    Value::NativeFunction { func, .. } => {
                        let args: Vec<SharedValue> = self.stack.drain(self.stack.len() - *arg_count as usize..).collect();
                        match func(&args) {
                            Ok(result) => {
                                self.stack.push(result);
                                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
                            }
                            Err(e) => ExecutionResult::Error(e),
                        }
                    }
                    _ => ExecutionResult::Error("ليست دالة قابلة للاستدعاء".into()),
                }
            }
            
            OpCode::CallNative { func_index, arg_count } => {
                let args: Vec<SharedValue> = self.stack.drain(self.stack.len() - *arg_count as usize..).collect();
                
                let chunk = match &self.chunk {
                    Some(c) => c,
                    None => return ExecutionResult::Error("لا يوجد كود للتنفيذ".into()),
                };
                
                if let Some(func_name) = chunk.get_string(*func_index) {
                    match self.globals.borrow().get(func_name) {
                        Some(v) => {
                            if let Value::NativeFunction { func, .. } = &*v.borrow() {
                                match func(&args) {
                                    Ok(result) => {
                                        self.stack.push(result);
                                        ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
                                    }
                                    Err(e) => ExecutionResult::Error(e),
                                }
                            } else {
                                ExecutionResult::Error(format!("'{}' ليست دالة أصلية", func_name))
                            }
                        }
                        None => ExecutionResult::Error(format!("الدالة '{}' غير معرفة", func_name)),
                    }
                } else {
                    ExecutionResult::Error("فهرس دالة غير صالح".into())
                }
            }
            
            OpCode::Return => {
                if self.call_frames.len() > 1 {
                    let _frame = self.call_frames.pop();
                    self.recursion_depth = self.recursion_depth.saturating_sub(1);
                }
                ExecutionResult::Return(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::ReturnValue => {
                let value = self.pop();
                if self.call_frames.len() > 1 {
                    let _frame = self.call_frames.pop();
                    self.recursion_depth = self.recursion_depth.saturating_sub(1);
                }
                ExecutionResult::Return(value)
            }
            
            // ═══════════════════════════════════════════════════════════════
            // القوائم والقواميس
            // ═══════════════════════════════════════════════════════════════
            
            OpCode::BuildList(count) => {
                let mut elements = Vec::with_capacity(*count as usize);
                for _ in 0..*count {
                    elements.push(self.pop());
                }
                elements.reverse();
                self.push_value(Value::List(elements));
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::BuildDict(count) => {
                let mut dict = HashMap::with_capacity(*count as usize);
                for _ in 0..*count {
                    let value = self.pop();
                    let key = self.pop_value().to_string_value();
                    dict.insert(key, value);
                }
                self.push_value(Value::Dictionary(dict));
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::Index => {
                let index = self.pop_value();
                let obj = self.pop_value();
                
                match (&obj, &index) {
                    (Value::List(list), Value::Number(idx)) => {
                        match self.normalize_index(*idx, list.len()) {
                            Ok(i) => self.stack.push(Rc::clone(&list[i])),
                            Err(e) => return e,
                        }
                    }
                    (Value::Dictionary(dict), Value::String(key)) => {
                        if let Some(v) = dict.get(key) {
                            self.stack.push(Rc::clone(v));
                        } else {
                            self.push_value(Value::Null);
                        }
                    }
                    (Value::String(s), Value::Number(idx)) => {
                        match self.normalize_index(*idx, s.len()) {
                            Ok(i) => self.push_value(Value::String(s.chars().nth(i).unwrap().to_string())),
                            Err(e) => return e,
                        }
                    }
                    _ => return ExecutionResult::Error("لا يمكن الفهرسة".into()),
                }
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::IndexSet => {
                let value = self.pop();
                let index = self.pop_value();
                let mut obj = self.pop_value();
                
                match (&mut obj, &index) {
                    (Value::List(ref mut list), Value::Number(idx)) => {
                        match self.normalize_index(*idx, list.len()) {
                            Ok(i) => list[i] = value,
                            Err(e) => return e,
                        }
                    }
                    (Value::Dictionary(ref mut dict), Value::String(key)) => {
                        dict.insert(key.clone(), value);
                    }
                    _ => return ExecutionResult::Error("لا يمكن تعيين الفهرس".into()),
                }
                self.push_value(obj);
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::GetProperty(idx) => {
                let prop = self.chunk.as_ref().and_then(|c| c.get_string(*idx)).map(|s| s.to_string());
                if let Some(prop) = prop {
                    let obj = self.pop_value();
                    match &obj {
                        Value::Dictionary(dict) => {
                            if let Some(v) = dict.get(&prop) {
                                self.stack.push(Rc::clone(v));
                            } else {
                                self.push_value(Value::Null);
                            }
                        }
                        Value::Instance { fields, .. } => {
                            if let Some(v) = fields.borrow().get(&prop) {
                                self.stack.push(Rc::clone(v));
                            } else {
                                self.push_value(Value::Null);
                            }
                        }
                        _ => return ExecutionResult::Error("لا يمكن الوصول للخاصية".into()),
                    }
                }
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::SetProperty(idx) => {
                let prop = self.chunk.as_ref().and_then(|c| c.get_string(*idx)).map(|s| s.to_string());
                if let Some(prop) = prop {
                    let value = self.pop();
                    let obj = self.pop();
                    
                    let is_dict = matches!(&*obj.borrow(), Value::Dictionary(_));
                    let is_instance = matches!(&*obj.borrow(), Value::Instance { .. });
                    
                    if is_dict {
                        let mut obj_mut = (*obj.borrow()).clone();
                        if let Value::Dictionary(ref mut dict) = obj_mut {
                            dict.insert(prop.clone(), value);
                        }
                        self.stack.push(Rc::new(RefCell::new(obj_mut)));
                    } else if is_instance {
                        if let Value::Instance { ref fields, .. } = &*obj.borrow() {
                            fields.borrow_mut().insert(prop.clone(), value);
                        }
                        self.stack.push(obj);
                    } else {
                        return ExecutionResult::Error("لا يمكن تعيين الخاصية".into());
                    }
                }
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // ═══════════════════════════════════════════════════════════════
            // الحلقات
            // ═══════════════════════════════════════════════════════════════
            
            OpCode::LoopStart(_slot) => {
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::LoopNext(_offset) => {
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::LoopEnd => {
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // ═══════════════════════════════════════════════════════════════
            // تعليمات أخرى
            // ═══════════════════════════════════════════════════════════════
            
            OpCode::Range => {
                let end = self.pop_value();
                let start = self.pop_value();
                
                let s = start.to_number().unwrap_or(0.0) as i64;
                let e = end.to_number().unwrap_or(0.0) as i64;
                
                let list: Vec<SharedValue> = (s..e)
                    .map(|n| Rc::new(RefCell::new(Value::Number(n as f64))))
                    .collect();
                
                self.push_value(Value::List(list));
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::RangeStep => {
                let step = self.pop_value();
                let end = self.pop_value();
                let start = self.pop_value();
                
                let s = start.to_number().unwrap_or(0.0);
                let e = end.to_number().unwrap_or(0.0);
                let st = step.to_number().unwrap_or(1.0);
                
                let mut list = Vec::new();
                let mut i = s;
                while if st > 0.0 { i < e } else { i > e } {
                    list.push(Rc::new(RefCell::new(Value::Number(i))));
                    i += st;
                }
                
                self.push_value(Value::List(list));
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::Print => {
                let value = self.pop_value();
                println!("{}", value.to_string_value());
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::TypeOf => {
                let value = self.pop_value();
                self.push_value(Value::String(value.type_name().to_string()));
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::Length => {
                let value = self.pop_value();
                let len = match &value {
                    Value::List(l) => l.len(),
                    Value::String(s) => s.len(),
                    Value::Dictionary(d) => d.len(),
                    _ => 0,
                };
                self.push_value(Value::Number(len as f64));
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::Break => ExecutionResult::Break,
            
            OpCode::Continue => ExecutionResult::Continue,
            
            OpCode::TryStart(_offset) => {
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::TryEnd => {
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::Throw => {
                let error = self.pop_value();
                ExecutionResult::Error(error.to_string_value())
            }
            
            OpCode::Nop => {
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::Halt => {
                self.halted = true;
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            // Async opcodes - basic implementation
            OpCode::Await => {
                // Await in non-async context returns null
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::AsyncStart { func_id: _ } => {
                // Async start in non-async context
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::AsyncReturn => {
                ExecutionResult::Return(Rc::new(RefCell::new(Value::Null)))
            }
            
            OpCode::AsyncCancel { task_id: _ } => {
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // دوال مساعدة محسّنة
    // ═══════════════════════════════════════════════════════════════
    
    /// دفع قيمة إلى المكدس (محسّنة)
    #[inline(always)]
    fn push_value(&mut self, value: Value) {
        self.stack.push(Rc::new(RefCell::new(value)));
    }
    
    /// إزالة قيمة من المكدس
    #[inline(always)]
    fn pop(&mut self) -> SharedValue {
        self.stack.pop().unwrap_or_else(|| Rc::new(RefCell::new(Value::Null)))
    }
    
    /// إزالة قيمة وفك الـ Rc
    #[inline(always)]
    fn pop_value(&mut self) -> Value {
        (*self.pop().borrow()).clone()
    }
    
    /// عملية ثنائية سريعة على الأرقام
    #[inline(always)]
    fn binary_op_fast<F>(&mut self, op: F) -> ExecutionResult
    where
        F: FnOnce(f64, f64) -> f64,
    {
        let b = self.pop_value();
        let a = self.pop_value();
        
        match (a.to_number(), b.to_number()) {
            (Ok(a), Ok(b)) => {
                self.push_value(Value::Number(op(a, b)));
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            _ => ExecutionResult::Error("العملية تتطلب أرقام".into()),
        }
    }
    
    /// عملية أحادية سريعة على الأرقام
    #[inline(always)]
    fn unary_op_fast<F>(&mut self, op: F) -> ExecutionResult
    where
        F: FnOnce(f64) -> f64,
    {
        let a = self.pop_value();
        
        match a.to_number() {
            Ok(a) => {
                self.push_value(Value::Number(op(a)));
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            _ => ExecutionResult::Error("العملية تتطلب رقم".into()),
        }
    }
    
    /// عملية مقارنة سريعة
    #[inline(always)]
    fn compare_fast<F>(&mut self, op: F) -> ExecutionResult
    where
        F: FnOnce(f64, f64) -> bool,
    {
        let b = self.pop_value();
        let a = self.pop_value();
        
        match (a.to_number(), b.to_number()) {
            (Ok(a), Ok(b)) => {
                self.push_value(Value::Boolean(op(a, b)));
                ExecutionResult::Ok(Rc::new(RefCell::new(Value::Null)))
            }
            _ => ExecutionResult::Error("المقارنة تتطلب أرقام".into()),
        }
    }
    
    /// تطبيع الفهرس (دعم الفهارس السالبة)
    #[inline(always)]
    fn normalize_index(&self, idx: f64, len: usize) -> Result<usize, ExecutionResult> {
        if idx < 0.0 {
            let neg_idx = (-idx) as usize;
            if neg_idx > len {
                return Err(ExecutionResult::Error(format!("فهرس سالب خارج النطاق: {}", idx)));
            }
            Ok(len - neg_idx)
        } else {
            let i = idx as usize;
            if i < len {
                Ok(i)
            } else {
                Err(ExecutionResult::Error(format!("فهرس خارج النطاق: {}", i)))
            }
        }
    }
    
    /// الحصول على إحصائيات
    pub fn stats(&self) -> &VMStats {
        &self.stats
    }
    
    /// طباعة حالة المكدس (للتصحيح)
    pub fn debug_stack(&self) {
        println!("═══ Stack ═══");
        for (i, v) in self.stack.iter().enumerate() {
            println!("  {:3}  {}", i, v.borrow().to_string_value());
        }
        println!("══════════════");
    }
    
    /// مسح كاش المتغيرات العامة
    pub fn clear_cache(&mut self) {
        self.global_cache.clear();
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_arithmetic() {
        let mut chunk = Chunk::new();
        chunk.emit(OpCode::PushNumber(5.0));
        chunk.emit(OpCode::PushNumber(3.0));
        chunk.emit(OpCode::Add);
        chunk.emit(OpCode::Halt);
        
        let mut vm = VM::with_fresh_env();
        vm.load(chunk);
        
        let result = vm.run();
        match result {
            ExecutionResult::Ok(v) => {
                assert_eq!((*v.borrow()).clone(), Value::Number(8.0));
            }
            _ => panic!("Expected Ok"),
        }
    }
    
    #[test]
    fn test_comparison() {
        let mut chunk = Chunk::new();
        chunk.emit(OpCode::PushNumber(5.0));
        chunk.emit(OpCode::PushNumber(3.0));
        chunk.emit(OpCode::Greater);
        chunk.emit(OpCode::Halt);
        
        let mut vm = VM::with_fresh_env();
        vm.load(chunk);
        
        let result = vm.run();
        match result {
            ExecutionResult::Ok(v) => {
                assert_eq!((*v.borrow()).clone(), Value::Boolean(true));
            }
            _ => panic!("Expected Ok"),
        }
    }
    
    #[test]
    fn test_jump() {
        let mut chunk = Chunk::new();
        chunk.emit(OpCode::PushNumber(1.0));
        chunk.emit(OpCode::Jump(2));
        chunk.emit(OpCode::PushNumber(2.0));
        chunk.emit(OpCode::PushNumber(3.0));
        chunk.emit(OpCode::Halt);
        
        let mut vm = VM::with_fresh_env();
        vm.load(chunk);
        
        let result = vm.run();
        match result {
            ExecutionResult::Ok(v) => {
                assert_eq!((*v.borrow()).clone(), Value::Number(3.0));
            }
            _ => panic!("Expected Ok"),
        }
    }
    
    #[test]
    fn test_variables() {
        let mut chunk = Chunk::new();
        let name_idx = chunk.add_string("س");
        
        chunk.emit(OpCode::PushNumber(42.0));
        chunk.emit(OpCode::StoreGlobal(name_idx));
        chunk.emit(OpCode::LoadGlobal(name_idx));
        chunk.emit(OpCode::PushNumber(8.0));
        chunk.emit(OpCode::Add);
        chunk.emit(OpCode::Halt);
        
        let mut vm = VM::with_fresh_env();
        vm.load(chunk);
        
        let result = vm.run();
        match result {
            ExecutionResult::Ok(v) => {
                assert_eq!((*v.borrow()).clone(), Value::Number(50.0));
            }
            _ => panic!("Expected Ok"),
        }
    }
    
    #[test]
    fn test_list() {
        let mut chunk = Chunk::new();
        chunk.emit(OpCode::PushNumber(1.0));
        chunk.emit(OpCode::PushNumber(2.0));
        chunk.emit(OpCode::PushNumber(3.0));
        chunk.emit(OpCode::BuildList(3));
        chunk.emit(OpCode::Halt);
        
        let mut vm = VM::with_fresh_env();
        vm.load(chunk);
        
        let result = vm.run();
        match result {
            ExecutionResult::Ok(v) => {
                if let Value::List(list) = (*v.borrow()).clone() {
                    assert_eq!(list.len(), 3);
                } else {
                    panic!("Expected List");
                }
            }
            _ => panic!("Expected Ok"),
        }
    }
    
    #[test]
    fn test_cache_hit() {
        let mut chunk = Chunk::new();
        let name_idx = chunk.add_string("س");
        
        chunk.emit(OpCode::PushNumber(10.0));
        chunk.emit(OpCode::StoreGlobal(name_idx));
        // تحميل نفس المتغير عدة مرات
        for _ in 0..10 {
            chunk.emit(OpCode::LoadGlobal(name_idx));
            chunk.emit(OpCode::Pop);
        }
        chunk.emit(OpCode::Halt);
        
        let mut vm = VM::with_fresh_env();
        vm.load(chunk);
        
        let result = vm.run();
        assert!(matches!(result, ExecutionResult::Ok(_)));
        
        // التحقق من أن الكاش يعمل
        assert!(vm.stats().cache_hits > 0, "يجب أن يكون هناك ضربات كاش");
    }
}
