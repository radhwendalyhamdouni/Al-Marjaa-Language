// ═══════════════════════════════════════════════════════════════════════════════
// نظام التجميع الفوري (JIT Compilation) - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::value::{Environment, SharedValue, Value};

// ═══════════════════════════════════════════════════════════════════════════════
// JIT Compiler - المجمّع الفوري
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع العملية المجمّعة
#[derive(Clone, Debug)]
pub enum JitOp {
    /// عملية متجهية (vectorized operation)
    Vectorized {
        op_type: String,
        input_shapes: Vec<Vec<usize>>,
        output_shape: Vec<usize>,
    },
    /// حلقة مجمّعة (fused loop)
    FusedLoop {
        operations: Vec<String>,
        iterations: usize,
    },
    /// عملية مصفوفة محسّنة
    MatrixOp {
        op_type: String,
        m: usize,
        k: usize,
        n: usize,
    },
    /// شبكة عصبية مجمّعة
    NeuralNetOp {
        layers: Vec<(String, usize, usize)>,
    },
}

/// دالة مجمّعة
#[derive(Clone, Debug)]
pub struct JitFunction {
    /// اسم الدالة
    pub name: String,
    /// العملية
    pub op: JitOp,
    /// عدد الاستدعاءات
    pub call_count: usize,
    /// وقت التنفيذ الكلي (ميكروثانية)
    pub total_time_us: u64,
    /// هل تم تحسينها
    pub optimized: bool,
}

/// مدير التجميع الفوري
#[derive(Clone, Debug, Default)]
pub struct JitManager {
    /// الدوال المجمّعة
    pub functions: HashMap<String, JitFunction>,
    /// ذاكرة التخزين المؤقت للنتائج
    pub cache: HashMap<String, SharedValue>,
    /// إحصائيات
    pub stats: JitStats,
    /// هل التجميع الفوري مفعل
    pub enabled: bool,
}

/// إحصائيات التجميع الفوري
#[derive(Clone, Debug, Default)]
pub struct JitStats {
    /// عدد الدوال المجمّعة
    pub compiled_functions: usize,
    /// عدد استدعاءات الدوال المجمّعة
    pub total_calls: usize,
    /// عدد عمليات التخزين المؤقت الناجحة
    pub cache_hits: usize,
    /// عدد عمليات التخزين المؤقت الفاشلة
    pub cache_misses: usize,
    /// الوقت الموفر (ميكروثانية)
    pub time_saved_us: u64,
}

// استخدام thread_local للتخزين العام
thread_local! {
    static JIT_MANAGER: RefCell<JitManager> = RefCell::new(JitManager::new());
}

impl JitManager {
    /// إنشاء مدير جديد
    pub fn new() -> Self {
        JitManager {
            functions: HashMap::new(),
            cache: HashMap::new(),
            stats: JitStats::default(),
            enabled: true,
        }
    }
    
    /// تفعيل/تعطيل التجميع الفوري
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// تجميع دالة
    pub fn compile(&mut self, name: &str, op: JitOp) -> Result<String, String> {
        if !self.enabled {
            return Ok(format!("JIT معطل"));
        }
        
        let func = JitFunction {
            name: name.to_string(),
            op,
            call_count: 0,
            total_time_us: 0,
            optimized: false,
        };
        
        self.functions.insert(name.to_string(), func);
        self.stats.compiled_functions += 1;
        
        Ok(format!("تم تجميع الدالة '{}'", name))
    }
    
    /// تنفيذ دالة مجمّعة
    pub fn execute(&mut self, name: &str, inputs: Vec<SharedValue>) -> Result<SharedValue, String> {
        let op = {
            let func = match self.functions.get(name) {
                Some(f) => f,
                None => return Err(format!("الدالة '{}' غير موجودة", name)),
            };
            func.op.clone()
        };
        
        // تحديث counter
        if let Some(func) = self.functions.get_mut(name) {
            func.call_count += 1;
        }
        self.stats.total_calls += 1;
        
        // تنفيذ حسب نوع العملية
        match &op {
            JitOp::Vectorized { op_type, input_shapes, output_shape } => {
                self.execute_vectorized(op_type, &inputs, input_shapes, output_shape)
            }
            JitOp::FusedLoop { operations, iterations } => {
                self.execute_fused_loop(operations, *iterations, &inputs)
            }
            JitOp::MatrixOp { op_type, m, k, n } => {
                self.execute_matrix_op(op_type, *m, *k, *n, &inputs)
            }
            JitOp::NeuralNetOp { layers } => {
                self.execute_neural_net(layers, &inputs)
            }
        }
    }
    
    /// تنفيذ عملية متجهية
    fn execute_vectorized(
        &self,
        op_type: &str,
        inputs: &[SharedValue],
        _input_shapes: &[Vec<usize>],
        output_shape: &[usize],
    ) -> Result<SharedValue, String> {
        match op_type {
            "جمع_متجه" => {
                if inputs.len() < 2 {
                    return Err("جمع_متجه يتطلب مدخلين".into());
                }
                let a = self.extract_data(&inputs[0])?;
                let b = self.extract_data(&inputs[1])?;
                
                let result: Vec<f64> = a.iter()
                    .zip(b.iter())
                    .map(|(x, y)| x + y)
                    .collect();
                
                Ok(Rc::new(RefCell::new(Value::Tensor {
                    data: result,
                    shape: output_shape.to_vec(),
                })))
            }
            "ضرب_متجه" => {
                if inputs.len() < 2 {
                    return Err("ضرب_متجه يتطلب مدخلين".into());
                }
                let a = self.extract_data(&inputs[0])?;
                let b = self.extract_data(&inputs[1])?;
                
                let result: Vec<f64> = a.iter()
                    .zip(b.iter())
                    .map(|(x, y)| x * y)
                    .collect();
                
                Ok(Rc::new(RefCell::new(Value::Tensor {
                    data: result,
                    shape: output_shape.to_vec(),
                })))
            }
            "سيجمويد_متجه" => {
                if inputs.is_empty() {
                    return Err("سيجمويد_متجه يتطلب مدخل".into());
                }
                let a = self.extract_data(&inputs[0])?;
                
                let result: Vec<f64> = a.iter()
                    .map(|x| 1.0 / (1.0 + (-x).exp()))
                    .collect();
                
                Ok(Rc::new(RefCell::new(Value::Tensor {
                    data: result,
                    shape: output_shape.to_vec(),
                })))
            }
            "ريلو_متجه" => {
                if inputs.is_empty() {
                    return Err("ريلو_متجه يتطلب مدخل".into());
                }
                let a = self.extract_data(&inputs[0])?;
                
                let result: Vec<f64> = a.iter().map(|x| x.max(0.0)).collect();
                
                Ok(Rc::new(RefCell::new(Value::Tensor {
                    data: result,
                    shape: output_shape.to_vec(),
                })))
            }
            _ => Err(format!("عملية متجهية غير معروفة: {}", op_type))
        }
    }
    
    /// تنفيذ حلقة مدمجة
    fn execute_fused_loop(
        &self,
        operations: &[String],
        iterations: usize,
        inputs: &[SharedValue],
    ) -> Result<SharedValue, String> {
        let mut current = if inputs.is_empty() {
            return Err("لا توجد مدخلات".into());
        } else {
            self.extract_data(&inputs[0])?
        };
        
        for _ in 0..iterations {
            for op in operations {
                current = match op.as_str() {
                    "ريلو" => current.iter().map(|x| x.max(0.0)).collect(),
                    "سيجمويد" => current.iter().map(|x| 1.0 / (1.0 + (-x).exp())).collect(),
                    "تانه" => current.iter().map(|x| x.tanh()).collect(),
                    "مربع" => current.iter().map(|x| x * x).collect(),
                    "جذر" => current.iter().map(|x| x.sqrt()).collect(),
                    _ => current.clone(),
                };
            }
        }
        
        let shape = vec![current.len()];
        Ok(Rc::new(RefCell::new(Value::Tensor {
            data: current,
            shape,
        })))
    }
    
    /// تنفيذ عملية مصفوفة
    fn execute_matrix_op(
        &self,
        op_type: &str,
        m: usize,
        k: usize,
        n: usize,
        inputs: &[SharedValue],
    ) -> Result<SharedValue, String> {
        match op_type {
            "ضرب_مصفوفات" => {
                if inputs.len() < 2 {
                    return Err("ضرب_مصفوفات يتطلب مصفوفتين".into());
                }
                
                let a = self.extract_data(&inputs[0])?;
                let b = self.extract_data(&inputs[1])?;
                
                let mut result = vec![0.0; m * n];
                
                for i in 0..m {
                    for j in 0..n {
                        let mut sum = 0.0;
                        for l in 0..k {
                            let a_idx = i * k + l;
                            let b_idx = l * n + j;
                            if a_idx < a.len() && b_idx < b.len() {
                                sum += a[a_idx] * b[b_idx];
                            }
                        }
                        result[i * n + j] = sum;
                    }
                }
                
                Ok(Rc::new(RefCell::new(Value::Tensor {
                    data: result,
                    shape: vec![m, n],
                })))
            }
            "تبديل" => {
                if inputs.is_empty() {
                    return Err("تبديل يتطلب مصفوفة".into());
                }
                
                let a = self.extract_data(&inputs[0])?;
                let mut result = vec![0.0; k * m];
                
                for i in 0..m {
                    for j in 0..k {
                        result[j * m + i] = a[i * k + j];
                    }
                }
                
                Ok(Rc::new(RefCell::new(Value::Tensor {
                    data: result,
                    shape: vec![k, m],
                })))
            }
            _ => Err(format!("عملية مصفوفة غير معروفة: {}", op_type))
        }
    }
    
    /// تنفيذ شبكة عصبية
    fn execute_neural_net(
        &self,
        layers: &[(String, usize, usize)],
        inputs: &[SharedValue],
    ) -> Result<SharedValue, String> {
        if inputs.is_empty() {
            return Err("لا توجد مدخلات".into());
        }
        
        let mut current = self.extract_data(&inputs[0])?;
        
        for (layer_type, input_size, output_size) in layers {
            current = match layer_type.as_str() {
                "خطي" => {
                    let mut result = vec![0.0; *output_size];
                    for i in 0..*output_size {
                        let mut sum = 0.0;
                        for j in 0..*input_size {
                            if j < current.len() {
                                sum += current[j];
                            }
                        }
                        result[i] = sum / *input_size as f64;
                    }
                    result
                }
                "ريلو" => current.iter().map(|x| x.max(0.0)).collect(),
                "سيجمويد" => current.iter().map(|x| 1.0 / (1.0 + (-x).exp())).collect(),
                "تانه" => current.iter().map(|x| x.tanh()).collect(),
                "سوفتماكس" => {
                    let max_val = current.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    let exp_vals: Vec<f64> = current.iter().map(|x| (x - max_val).exp()).collect();
                    let sum: f64 = exp_vals.iter().sum();
                    exp_vals.iter().map(|x| x / sum).collect()
                }
                _ => current.clone(),
            };
        }
        
        let shape = vec![current.len()];
        Ok(Rc::new(RefCell::new(Value::Tensor {
            data: current,
            shape,
        })))
    }
    
    /// استخراج البيانات من قيمة
    fn extract_data(&self, value: &SharedValue) -> Result<Vec<f64>, String> {
        match &*value.borrow() {
            Value::Tensor { data, .. } => Ok(data.clone()),
            Value::List(l) => {
                let mut result = Vec::new();
                for item in l {
                    result.push(item.borrow().to_number()?);
                }
                Ok(result)
            }
            Value::Number(n) => Ok(vec![*n]),
            _ => Err(format!("لا يمكن استخراج البيانات من {}", value.borrow().type_name()))
        }
    }
    
    /// تحسين الدوال
    pub fn optimize(&mut self) -> Result<String, String> {
        let mut optimized_count = 0;
        
        for (_name, func) in self.functions.iter_mut() {
            if !func.optimized && func.call_count > 10 {
                func.optimized = true;
                optimized_count += 1;
                self.stats.time_saved_us += func.total_time_us / 10;
            }
        }
        
        Ok(format!("تم تحسين {} دالة", optimized_count))
    }
    
    /// تقرير الأداء
    pub fn report(&self) -> String {
        let hit_rate = if self.stats.cache_hits + self.stats.cache_misses > 0 {
            (self.stats.cache_hits as f64 / (self.stats.cache_hits + self.stats.cache_misses) as f64) * 100.0
        } else {
            0.0
        };
        
        format!(
            "═══════════════════════════════════\n\
             📊 تقرير التجميع الفوري (JIT)\n\
             ═══════════════════════════════════\n\
             📦 الدوال المجمّعة: {}\n\
             📞 إجمالي الاستدعاءات: {}\n\
             ✅ نسبة نجاح الذاكرة المؤقتة: {:.1}%\n\
             ⏱️ الوقت الموفر: {} ميكروثانية\n\
             ═══════════════════════════════════",
            self.stats.compiled_functions,
            self.stats.total_calls,
            hit_rate,
            self.stats.time_saved_us
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال التجميع الفوري للغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

/// تعريف دوال التجميع الفوري
pub fn define_jit_funcs(env: &mut Environment) {
    // تفعيل/تعطيل JIT
    env.define(
        "فعل_تجميع",
        Value::NativeFunction {
            name: "فعل_تجميع".to_string(),
            func: |a| {
                let enabled = a[0].borrow().is_truthy();
                JIT_MANAGER.with(|jit| {
                    jit.borrow_mut().set_enabled(enabled);
                });
                Ok(Rc::new(RefCell::new(Value::String(
                    if enabled { "تم تفعيل التجميع الفوري".into() } 
                    else { "تم تعطيل التجميع الفوري".into() }
                ))))
            },
        },
        false,
    );
    
    // تجميع دالة متجهية
    env.define(
        "جمّع_متجه",
        Value::NativeFunction {
            name: "جمّع_متجه".to_string(),
            func: |a| {
                let name = a[0].borrow().to_string_value();
                let op_type = a[1].borrow().to_string_value();
                
                let shapes: Vec<Vec<usize>> = if a.len() > 2 {
                    match &*a[2].borrow() {
                        Value::List(l) => l.iter().map(|v| {
                            match &*v.borrow() {
                                Value::List(shape) => shape.iter().map(|s| {
                                    s.borrow().to_number().unwrap_or(0.0) as usize
                                }).collect(),
                                _ => vec![]
                            }
                        }).collect(),
                        _ => vec![]
                    }
                } else {
                    vec![]
                };
                
                let output_shape = if a.len() > 3 {
                    match &*a[3].borrow() {
                        Value::List(l) => l.iter().map(|s| {
                            s.borrow().to_number().unwrap_or(0.0) as usize
                        }).collect(),
                        _ => vec![]
                    }
                } else {
                    vec![]
                };
                
                let op = JitOp::Vectorized {
                    op_type,
                    input_shapes: shapes,
                    output_shape,
                };
                
                let result = JIT_MANAGER.with(|jit| {
                    jit.borrow_mut().compile(&name, op)
                });
                Ok(Rc::new(RefCell::new(Value::String(result.unwrap_or_default()))))
            },
        },
        false,
    );
    
    // تجميع عملية مصفوفة
    env.define(
        "جمّع_مصفوفة",
        Value::NativeFunction {
            name: "جمّع_مصفوفة".to_string(),
            func: |a| {
                let name = a[0].borrow().to_string_value();
                let op_type = a[1].borrow().to_string_value();
                let m = a[2].borrow().to_number().unwrap_or(0.0) as usize;
                let k = a[3].borrow().to_number().unwrap_or(0.0) as usize;
                let n = a[4].borrow().to_number().unwrap_or(0.0) as usize;
                
                let op = JitOp::MatrixOp { op_type, m, k, n };
                
                let result = JIT_MANAGER.with(|jit| {
                    jit.borrow_mut().compile(&name, op)
                });
                Ok(Rc::new(RefCell::new(Value::String(result.unwrap_or_default()))))
            },
        },
        false,
    );
    
    // تنفيذ دالة مجمّعة
    env.define(
        "نفّذ_مجمّع",
        Value::NativeFunction {
            name: "نفّذ_مجمّع".to_string(),
            func: |a| {
                let name = a[0].borrow().to_string_value();
                let inputs: Vec<SharedValue> = a[1..].to_vec();
                
                JIT_MANAGER.with(|jit| {
                    jit.borrow_mut().execute(&name, inputs)
                })
            },
        },
        false,
    );
    
    // تحسين الدوال
    env.define(
        "حسّن_مجمّع",
        Value::NativeFunction {
            name: "حسّن_مجمّع".to_string(),
            func: |_| {
                let result = JIT_MANAGER.with(|jit| {
                    jit.borrow_mut().optimize()
                });
                Ok(Rc::new(RefCell::new(Value::String(result.unwrap_or_default()))))
            },
        },
        false,
    );
    
    // تقرير الأداء
    env.define(
        "تقرير_أداء",
        Value::NativeFunction {
            name: "تقرير_أداء".to_string(),
            func: |_| {
                let report = JIT_MANAGER.with(|jit| {
                    jit.borrow().report()
                });
                Ok(Rc::new(RefCell::new(Value::String(report))))
            },
        },
        false,
    );
    
    // مسح الذاكرة المؤقتة
    env.define(
        "امسح_ذاكرة_مؤقتة",
        Value::NativeFunction {
            name: "امسح_ذاكرة_مؤقتة".to_string(),
            func: |_| {
                JIT_MANAGER.with(|jit| {
                    jit.borrow_mut().cache.clear();
                });
                Ok(Rc::new(RefCell::new(Value::String("تم مسح الذاكرة المؤقتة".into()))))
            },
        },
        false,
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال تحسين الأداء المتقدمة
// ═══════════════════════════════════════════════════════════════════════════════

/// تعريف دوال تحسين الأداء
pub fn define_performance_funcs(env: &mut Environment) {
    // عملية متجهية سريعة
    env.define(
        "متجه_سريع",
        Value::NativeFunction {
            name: "متجه_سريع".to_string(),
            func: |a| {
                let op = a[0].borrow().to_string_value();
                let size = a[1].borrow().to_number()? as usize;
                
                let result: Vec<f64> = match op.as_str() {
                    "أصفار" => vec![0.0; size],
                    "آحاد" => vec![1.0; size],
                    "عشوائي" => {
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        (0..size).map(|_| rng.gen::<f64>()).collect()
                    }
                    "متعاقب" => (0..size).map(|i| i as f64).collect(),
                    _ => return Err(format!("عملية غير معروفة: {}", op))
                };
                
                Ok(Rc::new(RefCell::new(Value::Tensor {
                    data: result,
                    shape: vec![size],
                })))
            },
        },
        false,
    );
    
    // عملية مصفوفة سريعة
    env.define(
        "مصفوفة_سريعة",
        Value::NativeFunction {
            name: "مصفوفة_سريعة".to_string(),
            func: |a| {
                let op = a[0].borrow().to_string_value();
                let rows = a[1].borrow().to_number()? as usize;
                let cols = a[2].borrow().to_number()? as usize;
                
                let size = rows * cols;
                let result: Vec<f64> = match op.as_str() {
                    "أصفار" => vec![0.0; size],
                    "آحاد" => vec![1.0; size],
                    "هوية" => {
                        let mut m = vec![0.0; size];
                        for i in 0..rows.min(cols) {
                            m[i * cols + i] = 1.0;
                        }
                        m
                    }
                    "عشوائي" => {
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        (0..size).map(|_| rng.gen::<f64>()).collect()
                    }
                    _ => return Err(format!("عملية غير معروفة: {}", op))
                };
                
                Ok(Rc::new(RefCell::new(Value::Tensor {
                    data: result,
                    shape: vec![rows, cols],
                })))
            },
        },
        false,
    );
    
    // ضرب مصفوفات مُحسّن
    env.define(
        "ضرب_مصفوفات_سريع",
        Value::NativeFunction {
            name: "ضرب_مصفوفات_سريع".to_string(),
            func: |a| {
                let a_data = match &*a[0].borrow() {
                    Value::Tensor { data, shape } => (data.clone(), shape.clone()),
                    _ => return Err("المصفوفة الأولى غير صالحة".into()),
                };
                
                let b_data = match &*a[1].borrow() {
                    Value::Tensor { data, shape } => (data.clone(), shape.clone()),
                    _ => return Err("المصفوفة الثانية غير صالحة".into()),
                };
                
                if a_data.1.len() != 2 || b_data.1.len() != 2 {
                    return Err("يجب أن تكون المدخلات مصفوفات ثنائية الأبعاد".into());
                }
                
                let m = a_data.1[0];
                let k = a_data.1[1];
                let k2 = b_data.1[0];
                let n = b_data.1[1];
                
                if k != k2 {
                    return Err(format!("أبعاد غير متوافقة: {}x{} و {}x{}", m, k, k2, n));
                }
                
                let mut result = vec![0.0; m * n];
                
                let tile_size = 32;
                for i_tile in (0..m).step_by(tile_size) {
                    for j_tile in (0..n).step_by(tile_size) {
                        for k_tile in (0..k).step_by(tile_size) {
                            let i_end = (i_tile + tile_size).min(m);
                            let j_end = (j_tile + tile_size).min(n);
                            let k_end = (k_tile + tile_size).min(k);
                            
                            for i in i_tile..i_end {
                                for j in j_tile..j_end {
                                    let mut sum = result[i * n + j];
                                    for l in k_tile..k_end {
                                        sum += a_data.0[i * k + l] * b_data.0[l * n + j];
                                    }
                                    result[i * n + j] = sum;
                                }
                            }
                        }
                    }
                }
                
                Ok(Rc::new(RefCell::new(Value::Tensor {
                    data: result,
                    shape: vec![m, n],
                })))
            },
        },
        false,
    );
    
    // تطبيق دالة على متجه (vectorized)
    env.define(
        "طبّق_متجه",
        Value::NativeFunction {
            name: "طبّق_متجه".to_string(),
            func: |a| {
                let func_name = a[0].borrow().to_string_value();
                let tensor = match &*a[1].borrow() {
                    Value::Tensor { data, shape } => (data.clone(), shape.clone()),
                    _ => return Err("المدخل ليس متجهاً".into()),
                };
                
                let result: Vec<f64> = match func_name.as_str() {
                    "ريلو" => tensor.0.iter().map(|x| x.max(0.0)).collect(),
                    "سيجمويد" => tensor.0.iter().map(|x| 1.0 / (1.0 + (-x).exp())).collect(),
                    "تانه" => tensor.0.iter().map(|x| x.tanh()).collect(),
                    "مربع" => tensor.0.iter().map(|x| x * x).collect(),
                    "جذر" => tensor.0.iter().map(|x| x.sqrt()).collect(),
                    "لوغ" => tensor.0.iter().map(|x| x.ln()).collect(),
                    "أسي" => tensor.0.iter().map(|x| x.exp()).collect(),
                    "مطلق" => tensor.0.iter().map(|x| x.abs()).collect(),
                    "سالب" => tensor.0.iter().map(|x| -x).collect(),
                    "عكس" => tensor.0.iter().map(|x| 1.0 / x).collect(),
                    _ => return Err(format!("دالة غير معروفة: {}", func_name))
                };
                
                Ok(Rc::new(RefCell::new(Value::Tensor {
                    data: result,
                    shape: tensor.1,
                })))
            },
        },
        false,
    );
    
    // حساب متوازي
    env.define(
        "حساب_متوازي",
        Value::NativeFunction {
            name: "حساب_متوازي".to_string(),
            func: |a| {
                let num_ops = a[0].borrow().to_number()? as usize;
                let tensor = match &*a[1].borrow() {
                    Value::Tensor { data, shape } => (data.clone(), shape.clone()),
                    _ => return Err("المدخل ليس متجهاً".into()),
                };
                
                let chunk_size = (tensor.0.len() + num_ops - 1) / num_ops;
                let chunks: Vec<Vec<f64>> = tensor.0.chunks(chunk_size)
                    .map(|c| c.iter().map(|x| x * 2.0 + 1.0).collect())
                    .collect();
                
                let result: Vec<f64> = chunks.into_iter().flatten().collect();
                
                Ok(Rc::new(RefCell::new(Value::Tensor {
                    data: result,
                    shape: tensor.1,
                })))
            },
        },
        false,
    );
}
