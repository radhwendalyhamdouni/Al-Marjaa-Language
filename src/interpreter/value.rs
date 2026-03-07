use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::parser::ast::Stmt;

pub type SharedValue = Rc<RefCell<Value>>;
pub type NativeFunc = fn(&[SharedValue]) -> Result<SharedValue, String>;

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    List(Vec<SharedValue>),
    Dictionary(HashMap<String, SharedValue>),
    Function {
        name: String,
        params: Vec<(String, Option<crate::parser::ast::Expr>)>,
        body: Box<Stmt>,
        closure: Option<Rc<RefCell<Environment>>>,
        is_async: bool,
    },
    NativeFunction {
        name: String,
        func: NativeFunc,
    },
    Lambda {
        params: Vec<String>,
        body: Box<crate::parser::ast::Expr>,
        closure: Rc<RefCell<Environment>>,
    },
    Class {
        name: String,
        parent: Option<String>,
        methods: HashMap<String, Value>,
        fields: Vec<(String, Option<crate::parser::ast::Expr>)>,
    },
    Instance {
        class_name: String,
        fields: Rc<RefCell<HashMap<String, SharedValue>>>,
        methods: HashMap<String, Value>,
    },
    Return(Box<Value>),
    Break,
    Continue,
    Error(String),
    Bytes(Vec<u8>),
    
    // قيم جديدة للميزات المتقدمة
    
    /// قيمة المولد (Yield)
    Yield(Box<Value>),
    
    /// مولد - دالة ترجع عدة قيم
    Generator {
        name: String,
        params: Vec<(String, Option<crate::parser::ast::Expr>)>,
        body: Box<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },
    
    /// تعداد
    Enum {
        name: String,
        variants: HashMap<String, f64>,
    },
    
    /// قيمة من تعداد
    EnumVariant {
        enum_name: String,
        variant_name: String,
        value: f64,
    },
    
    /// فئة بيانات (Data Class)
    DataClass {
        name: String,
        fields: Vec<(String, Option<crate::parser::ast::Expr>)>,
    },
    
    /// مثيل من فئة بيانات
    DataInstance {
        class_name: String,
        fields: Rc<RefCell<HashMap<String, SharedValue>>>,
    },
    
    // ========== ميزات الذكاء الاصطناعي ==========
    
    /// متجه (Tensor) - مصفوفة متعددة الأبعاد للذكاء الاصطناعي
    Tensor {
        /// البيانات كقائمة مسطحة
        data: Vec<f64>,
        /// أبعاد المتجه (مثال: [2, 3] لمصفوفة 2x3)
        shape: Vec<usize>,
    },
    
    /// متجه مع تدرجات (AutoGrad Tensor)
    AutoTensor {
        /// معرف فريد للمتجه
        id: usize,
        /// البيانات
        data: Vec<f64>,
        /// التدرجات (Gradients)
        grad: Vec<f64>,
        /// أبعاد المتجه
        shape: Vec<usize>,
        /// هل يتطلب حساب التدرجات
        requires_grad: bool,
        /// العملية التي أنتجت هذا المتجه
        op: Option<String>,
        /// المدخلات للعملية (معرفات)
        parents: Vec<usize>,
        /// البيانات المحفوظة للانتشار العكسي (cached data for backward)
        cached_data: Vec<f64>,
    },
    
    /// عقدة في الرسم البياني الحسابي
    ComputeNode {
        /// المعرف
        id: usize,
        /// نوع العملية
        op_type: String,
        /// المدخلات
        inputs: Vec<SharedValue>,
        /// المخرجات
        output: Option<SharedValue>,
        /// دالة الاشتقاق العكسي
        backward_fn: Option<String>,
    },
    
    /// رسم بياني حسابي كامل
    ComputeGraph {
        /// اسم الرسم البياني
        name: String,
        /// جميع العقد
        nodes: Vec<SharedValue>,
        /// العقد الورقية (المتغيرات)
        leaves: Vec<usize>,
        /// العقد الجذرية (الخسائر)
        roots: Vec<usize>,
        /// العداد التالي للمعرفات
        next_id: usize,
    },
    
    /// نموذج شبكة عصبية
    NeuralNetwork {
        name: String,
        layers: Vec<LayerInfo>,
    },
    
    /// طبقة في الشبكة العصبية
    Layer {
        layer_type: String,
        input_size: usize,
        output_size: usize,
        weights: Option<Rc<RefCell<Vec<f64>>>>,
        biases: Option<Rc<RefCell<Vec<f64>>>>,
    },
}

/// معلومات الطبقة للشبكة العصبية
#[derive(Clone, Debug)]
pub struct LayerInfo {
    pub layer_type: String,
    pub input_size: usize,
    pub output_size: usize,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::List(a), Value::List(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                a.iter()
                    .zip(b.iter())
                    .all(|(x, y)| *x.borrow() == *y.borrow())
            }
            (Value::Dictionary(a), Value::Dictionary(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                a.iter()
                    .all(|(k, v)| b.get(k).is_some_and(|bv| *v.borrow() == *bv.borrow()))
            }
            (Value::Return(a), Value::Return(b)) => a == b,
            (Value::Break, Value::Break) => true,
            (Value::Continue, Value::Continue) => true,
            (Value::Bytes(a), Value::Bytes(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    /// مقارنة القيم (للنصوص والقوائم والأرقام)
    /// 
    /// أمثلة:
    /// - "أحمد" < "محمد" (ترتيب أبجدي عربي)
    /// - [1, 2] < [1, 3] (مقارنة عنصرية)
    /// - 5 < 10 (مقارنة رقمية)
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // مقارنة الأرقام
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b),
            
            // مقارنة النصوص (باستخدام الترتيب المعجمي)
            (Value::String(a), Value::String(b)) => {
                // استخدام الترتيب المعجمي العربي
                Some(compare_arabic_strings(a, b))
            }
            
            // مقارنة القوائم (عنصرية)
            (Value::List(a), Value::List(b)) => {
                for (x, y) in a.iter().zip(b.iter()) {
                    let cmp = x.borrow().partial_cmp(&*y.borrow())?;
                    if cmp != std::cmp::Ordering::Equal {
                        return Some(cmp);
                    }
                }
                // إذا كانت جميع العناصر متساوية، نقارن الأطوال
                a.len().partial_cmp(&b.len())
            }
            
            // مقارنة المنطقية
            (Value::Boolean(a), Value::Boolean(b)) => a.partial_cmp(b),
            
            // Null أقل من كل شيء
            (Value::Null, Value::Null) => Some(std::cmp::Ordering::Equal),
            (Value::Null, _) => Some(std::cmp::Ordering::Less),
            (_, Value::Null) => Some(std::cmp::Ordering::Greater),
            
            // مقارنة بين أنواع مختلفة - الأرقام أولاً، ثم النصوص، ثم القوائم
            (Value::Number(_), _) => Some(std::cmp::Ordering::Less),
            (_, Value::Number(_)) => Some(std::cmp::Ordering::Greater),
            (Value::String(_), _) => Some(std::cmp::Ordering::Less),
            (_, Value::String(_)) => Some(std::cmp::Ordering::Greater),
            
            _ => None,
        }
    }
}

/// مقارنة النصوص العربية (مع دعم الترتيب الأبجدي)
fn compare_arabic_strings(a: &str, b: &str) -> std::cmp::Ordering {
    // أولاً: محاولة الترتيب المعجمي العادي
    let cmp = a.cmp(b);
    if cmp != std::cmp::Ordering::Equal {
        return cmp;
    }
    std::cmp::Ordering::Equal
}

impl Value {
    /// مقارنة قيمتين مع إرجاع نتيجة قابلة للاستخدام في الـ interpreter
    pub fn compare(&self, other: &Self) -> Result<std::cmp::Ordering, String> {
        self.partial_cmp(other)
            .ok_or_else(|| format!("لا يمكن مقارنة {} مع {}", self.type_name(), other.type_name()))
    }
    
    /// هل هذه القيمة أقل من الأخرى؟
    pub fn less_than(&self, other: &Self) -> Result<bool, String> {
        Ok(self.compare(other)? == std::cmp::Ordering::Less)
    }
    
    /// هل هذه القيمة أكبر من الأخرى؟
    pub fn greater_than(&self, other: &Self) -> Result<bool, String> {
        Ok(self.compare(other)? == std::cmp::Ordering::Greater)
    }
    
    /// هل هذه القيمة أقل أو تساوي الأخرى؟
    pub fn less_or_equal(&self, other: &Self) -> Result<bool, String> {
        Ok(matches!(self.compare(other)?, std::cmp::Ordering::Less | std::cmp::Ordering::Equal))
    }
    
    /// هل هذه القيمة أكبر أو تساوي الأخرى؟
    pub fn greater_or_equal(&self, other: &Self) -> Result<bool, String> {
        Ok(matches!(self.compare(other)?, std::cmp::Ordering::Greater | std::cmp::Ordering::Equal))
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub variables: HashMap<String, SharedValue>,
    pub constants: HashMap<String, bool>,
    pub parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
            constants: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Environment {
            variables: HashMap::new(),
            constants: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn define(&mut self, name: &str, value: Value, is_const: bool) {
        self.variables
            .insert(name.to_string(), Rc::new(RefCell::new(value)));
        if is_const {
            self.constants.insert(name.to_string(), true);
        }
    }

    pub fn get(&self, name: &str) -> Option<SharedValue> {
        if let Some(value) = self.variables.get(name) {
            Some(Rc::clone(value))
        } else if let Some(ref parent) = self.parent {
            parent.borrow().get(name)
        } else {
            None
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.constants.get(name).copied().unwrap_or(false) {
            return Err(format!("لا يمكن تعديل الثابت '{}'", name));
        }

        if self.variables.contains_key(name) {
            self.variables
                .insert(name.to_string(), Rc::new(RefCell::new(value)));
            Ok(())
        } else if let Some(ref parent) = self.parent {
            parent.borrow_mut().assign(name, value)
        } else {
            Err(format!("المتغير '{}' غير معرف", name))
        }
    }

    pub fn assign_or_define(&mut self, name: &str, value: Value) {
        if self.is_defined(name) {
            let _ = self.assign(name, value);
        } else {
            self.define(name, value, false);
        }
    }

    pub fn is_defined(&self, name: &str) -> bool {
        self.variables.contains_key(name)
            || self
                .parent
                .as_ref()
                .map(|p| p.borrow().is_defined(name))
                .unwrap_or(false)
    }

    pub fn delete(&mut self, name: &str) -> bool {
        if self.variables.remove(name).is_some() {
            self.constants.remove(name);
            true
        } else if let Some(ref parent) = self.parent {
            parent.borrow_mut().delete(name)
        } else {
            false
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => {
                if n.is_nan() {
                    write!(f, "ليس_رقم")
                } else if n.is_infinite() {
                    if *n > 0.0 {
                        write!(f, "لانهاية")
                    } else {
                        write!(f, "ناقص_لانهاية")
                    }
                } else if n.fract() == 0.0 && n.abs() < 1e15 {
                    write!(f, "{:.0}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", if *b { "صح" } else { "خطأ" }),
            Value::Null => write!(f, "لا_شيء"),
            Value::List(list) => {
                write!(f, "[")?;
                for (i, v) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, "، ")?;
                    }
                    let val = v.borrow();
                    match &*val {
                        Value::String(s) => write!(f, "\"{}\"", s)?,
                        other => write!(f, "{}", other)?,
                    }
                }
                write!(f, "]")
            }
            Value::Dictionary(dict) => {
                write!(f, "{{")?;
                let mut first = true;
                for (k, v) in dict.iter() {
                    if !first {
                        write!(f, "، ")?;
                    }
                    first = false;
                    write!(f, "\"{}\": {}", k, v.borrow())?;
                }
                write!(f, "}}")
            }
            Value::Function { name, .. } => write!(f, "<دالة {}>", name),
            Value::NativeFunction { name, .. } => write!(f, "<دالة_مدمجة {}>", name),
            Value::Lambda { .. } => write!(f, "<دالة_مجهولة>"),
            Value::Class { name, .. } => write!(f, "<صنف {}>", name),
            Value::Instance { class_name, .. } => write!(f, "<كائن {}>", class_name),
            Value::Return(v) => write!(f, "{}", v),
            Value::Break => write!(f, "توقف"),
            Value::Continue => write!(f, "أكمل"),
            Value::Error(e) => write!(f, "خطأ: {}", e),
            Value::Bytes(b) => write!(f, "<بيانات {} بايت>", b.len()),
            Value::Tensor { data, shape } => {
                write!(f, "متجه({})", shape.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("×"))?;
                if data.len() <= 10 {
                    write!(f, " [{}]", data.iter().map(|n| format!("{:.2}", n)).collect::<Vec<_>>().join("، "))?;
                } else {
                    write!(f, " [...{} عنصر]", data.len())?;
                }
                Ok(())
            }
            Value::AutoTensor { id, data, grad, shape, requires_grad, op, .. } => {
                write!(f, "تدرج_متجه#{}({})", id, shape.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("×"))?;
                if *requires_grad {
                    write!(f, " [تدرج]")?;
                }
                if let Some(op_name) = op {
                    write!(f, " ← {}", op_name)?;
                }
                if data.len() <= 5 {
                    write!(f, " قيم=[{}]", data.iter().map(|n| format!("{:.2}", n)).collect::<Vec<_>>().join("، "))?;
                    let has_grad = grad.iter().any(|g| *g != 0.0);
                    if has_grad && grad.len() <= 5 {
                        write!(f, " تدرج=[{}]", grad.iter().map(|n| format!("{:.4}", n)).collect::<Vec<_>>().join("، "))?;
                    }
                }
                Ok(())
            }
            Value::ComputeNode { id, op_type, .. } => {
                write!(f, "<عقدة#{}: {}>", id, op_type)
            }
            Value::ComputeGraph { name, nodes, leaves, .. } => {
                write!(f, "<رسم_بياني '{}' | {} عقدة | {} متغير>", name, nodes.len(), leaves.len())
            }
            Value::NeuralNetwork { name, layers } => {
                write!(f, "<شبكة '{}' {} طبقة>", name, layers.len())
            }
            Value::Layer { layer_type, input_size, output_size, .. } => {
                write!(f, "<طبقة {} {}→{}>", layer_type, input_size, output_size)
            }
            Value::Yield(v) => write!(f, "إنتاج({})", v),
            Value::Generator { name, .. } => write!(f, "<مولد {}>", name),
            Value::Enum { name, variants } => {
                write!(f, "تعداد {} {{", name)?;
                for (i, (k, v)) in variants.iter().enumerate() {
                    if i > 0 { write!(f, "، ")?; }
                    write!(f, "{} = {}", k, v)?;
                }
                write!(f, "}}")
            }
            Value::EnumVariant { enum_name, variant_name, value } => {
                write!(f, "{}::{} = {}", enum_name, variant_name, value)
            }
            Value::DataClass { name, fields } => {
                write!(f, "فئة_بيانات {}(", name)?;
                for (i, (field, _)) in fields.iter().enumerate() {
                    if i > 0 { write!(f, "، ")?; }
                    write!(f, "{}", field)?;
                }
                write!(f, ")")
            }
            Value::DataInstance { class_name, fields } => {
                write!(f, "<{} ", class_name)?;
                for (i, (k, v)) in fields.borrow().iter().enumerate() {
                    if i > 0 { write!(f, "، ")?; }
                    write!(f, "{}: {}", k, v.borrow())?;
                }
                write!(f, ">")
            }
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            Value::Null => false,
            Value::List(l) => !l.is_empty(),
            Value::Dictionary(d) => !d.is_empty(),
            Value::Error(_) => false,
            _ => true,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "رقم",
            Value::String(_) => "نص",
            Value::Boolean(_) => "منطقي",
            Value::Null => "لا_شيء",
            Value::List(_) => "قائمة",
            Value::Dictionary(_) => "قاموس",
            Value::Function { .. } => "دالة",
            Value::NativeFunction { .. } => "دالة_مدمجة",
            Value::Lambda { .. } => "دالة_مجهولة",
            Value::Class { .. } => "صنف",
            Value::Instance { .. } => "كائن",
            Value::Return(_) => "إرجاع",
            Value::Break => "توقف",
            Value::Continue => "أكمل",
            Value::Error(_) => "خطأ",
            Value::Bytes(_) => "بيانات",
            Value::Tensor { .. } => "متجه",
            Value::AutoTensor { .. } => "متجه_تدرج",
            Value::ComputeNode { .. } => "عقدة_حساب",
            Value::ComputeGraph { .. } => "رسم_بياني",
            Value::NeuralNetwork { .. } => "شبكة_عصبية",
            Value::Layer { .. } => "طبقة",
            Value::Yield(_) => "إنتاج",
            Value::Generator { .. } => "مولد",
            Value::Enum { .. } => "تعداد",
            Value::EnumVariant { .. } => "قيمة_تعداد",
            Value::DataClass { .. } => "فئة_بيانات",
            Value::DataInstance { .. } => "نموذج_بيانات",
        }
    }

    pub fn to_number(&self) -> Result<f64, String> {
        match self {
            Value::Number(n) => Ok(*n),
            Value::String(s) => {
                let s = s.trim();
                let converted: String = s
                    .chars()
                    .map(|c| match c {
                        '٠' => '0',
                        '١' => '1',
                        '٢' => '2',
                        '٣' => '3',
                        '٤' => '4',
                        '٥' => '5',
                        '٦' => '6',
                        '٧' => '7',
                        '٨' => '8',
                        '٩' => '9',
                        _ => c,
                    })
                    .collect();
                converted.parse().map_err(|_| format!("'{}' ليس رقماً", s))
            }
            Value::Boolean(true) => Ok(1.0),
            Value::Boolean(false) => Ok(0.0),
            Value::Null => Ok(0.0),
            _ => Err(format!("لا يمكن تحويل {} إلى رقم", self.type_name())),
        }
    }

    pub fn to_string_value(&self) -> String {
        self.to_string()
    }

    pub fn to_repr(&self) -> String {
        match self {
            Value::String(s) => format!("\"{}\"", s),
            other => other.to_string(),
        }
    }

    pub fn length(&self) -> Result<usize, String> {
        match self {
            Value::String(s) => Ok(s.chars().count()),
            Value::List(l) => Ok(l.len()),
            Value::Dictionary(d) => Ok(d.len()),
            Value::Bytes(b) => Ok(b.len()),
            _ => Err(format!("نوع {} لا يدعم الطول", self.type_name())),
        }
    }

    pub fn make_shared(self) -> SharedValue {
        Rc::new(RefCell::new(self))
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
