// ═══════════════════════════════════════════════════════════════════════════════
// نظام استنباط الأنواع المتقدم (Advanced Type Inference) - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// يتضمن:
// - استنباط الأنواع التلقائي (Type Inference)
// - فحص الأنواع في وقت الترجمة (Type Checking)
// - تحسين الحراس بناءً على الأنواع
// - تتبع أنواع المتغيرات عبر البرنامج
// - دعم الأنواع العامة (Generics)
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use std::rc::Rc;

use super::opcodes::OpCode;
use crate::interpreter::value::Value;

// ═══════════════════════════════════════════════════════════════════════════════
// تعريف الأنواع
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع البيانات الأساسي
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// رقم (عدد حقيقي)
    Number,
    /// نص
    String,
    /// منطقي
    Boolean,
    /// لا شيء
    Null,
    /// قائمة بعنصر محدد
    List(Box<Type>),
    /// قاموس بمفتاح وقيمة محددين
    Dict(Box<Type>, Box<Type>),
    /// دالة مع معاملات وقيمة إرجاع
    Function(Vec<Type>, Box<Type>),
    /// فئة
    Class(String),
    /// كائن من فئة
    Instance(String),
    /// نوع عام (معامل نوع)
    Generic(String),
    /// اتحاد أنواع (أحد الأنواع)
    Union(Vec<Type>),
    /// تقاطع أنواع (جميع الأنواع)
    Intersection(Vec<Type>),
    /// نوع اختياري (قد يكون null)
    Optional(Box<Type>),
    /// مصفوفة متعددة الأبعاد (Tensor)
    Tensor(usize),
    /// نوع غير معروف بعد
    Unknown,
    /// أي نوع (للأنواع الديناميكية)
    Any,
    /// لا شيء (للدوال التي لا ترجع قيمة)
    Void,
}

impl Type {
    /// إنشاء نوع قائمة
    pub fn list_of(inner: Type) -> Self {
        Type::List(Box::new(inner))
    }
    
    /// إنشاء نوع قاموس
    pub fn dict_of(key: Type, value: Type) -> Self {
        Type::Dict(Box::new(key), Box::new(value))
    }
    
    /// إنشاء نوع دالة
    pub fn function(params: Vec<Type>, ret: Type) -> Self {
        Type::Function(params, Box::new(ret))
    }
    
    /// إنشاء نوع اختياري
    pub fn optional(inner: Type) -> Self {
        Type::Optional(Box::new(inner))
    }
    
    /// إنشاء نوع اتحاد
    pub fn union(types: Vec<Type>) -> Self {
        let mut unique_types: Vec<Type> = Vec::new();
        for t in types {
            if !unique_types.contains(&t) {
                unique_types.push(t);
            }
        }
        if unique_types.len() == 1 {
            unique_types.pop().unwrap()
        } else {
            Type::Union(unique_types)
        }
    }
    
    /// هل النوع رقمي
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Number | Type::Tensor(_))
    }
    
    /// هل النوع قابل للمقارنة
    pub fn is_comparable(&self) -> bool {
        matches!(self, Type::Number | Type::String | Type::Boolean)
    }
    
    /// هل النوع قابل للتكرار
    pub fn is_iterable(&self) -> bool {
        matches!(self, Type::List(_) | Type::Dict(_, _) | Type::String)
    }
    
    /// الحصول على النوع الداخلي للتكرار
    pub fn iterator_item_type(&self) -> Type {
        match self {
            Type::List(inner) => (**inner).clone(),
            Type::Dict(_, value_type) => (**value_type).clone(),
            Type::String => Type::String,
            _ => Type::Unknown,
        }
    }
    
    /// هل يمكن تحويل النوع ضمنياً
    pub fn can_implicitly_cast_to(&self, target: &Type) -> bool {
        if self == target {
            return true;
        }
        
        match (self, target) {
            // Any يمكن تحويله لأي نوع
            (Type::Any, _) | (_, Type::Any) => true,
            
            // Unknown يمكن تحويله لأي نوع
            (Type::Unknown, _) | (_, Type::Unknown) => true,
            
            // Null يمكن تحويله لـ Optional
            (Type::Null, Type::Optional(_)) => true,
            
            // النوع الأساسي يمكن تحويله لـ Optional
            (t, Type::Optional(inner)) => t.can_implicitly_cast_to(inner),
            
            // Union: إذا كان أحد الأنواع متوافق
            (Type::Union(types), target) => {
                types.iter().any(|t| t.can_implicitly_cast_to(target))
            }
            (t, Type::Union(types)) => {
                types.iter().any(|tt| t.can_implicitly_cast_to(tt))
            }
            
            // List: إذا كانت الأنواع الداخلية متوافقة
            (Type::List(a), Type::List(b)) => a.can_implicitly_cast_to(b),
            
            // Function: التحقق من المعاملات والإرجاع
            (Type::Function(params_a, ret_a), Type::Function(params_b, ret_b)) => {
                // Contravariance للمعاملات، Covariance للإرجاع
                if params_a.len() != params_b.len() {
                    return false;
                }
                let params_ok = params_b.iter().zip(params_a.iter())
                    .all(|(pb, pa)| pb.can_implicitly_cast_to(pa));
                let ret_ok = ret_a.can_implicitly_cast_to(ret_b);
                params_ok && ret_ok
            }
            
            _ => false,
        }
    }
    
    /// دمج نوعين (لتحليل التدفق)
    pub fn merge(&self, other: &Type) -> Type {
        if self == other {
            return self.clone();
        }
        
        match (self, other) {
            (Type::Unknown, t) | (t, Type::Unknown) => t.clone(),
            (Type::Any, _) | (_, Type::Any) => Type::Any,
            (Type::Null, Type::Optional(inner)) | (Type::Optional(inner), Type::Null) => {
                Type::optional((**inner).clone())
            }
            (Type::Optional(a), Type::Optional(b)) => {
                Type::optional(a.merge(b))
            }
            (t, Type::Optional(inner)) | (Type::Optional(inner), t) => {
                if t == inner.as_ref() {
                    Type::optional(t.clone())
                } else {
                    Type::union(vec![t.clone(), Type::Null])
                }
            }
            _ => Type::union(vec![self.clone(), other.clone()]),
        }
    }
    
    /// الحصول على تمثيل نصي
    pub fn to_string_repr(&self) -> String {
        match self {
            Type::Number => "رقم".to_string(),
            Type::String => "نص".to_string(),
            Type::Boolean => "منطقي".to_string(),
            Type::Null => "لا_شيء".to_string(),
            Type::List(inner) => format!("قائمة[{}]", inner.to_string_repr()),
            Type::Dict(k, v) => format!("قاموس[{}, {}]", k.to_string_repr(), v.to_string_repr()),
            Type::Function(params, ret) => {
                let params_str: Vec<String> = params.iter()
                    .map(|p| p.to_string_repr())
                    .collect();
                format!("دالة({}) -> {}", params_str.join(", "), ret.to_string_repr())
            }
            Type::Class(name) => format!("فئة({})", name),
            Type::Instance(name) => format!("كائن({})", name),
            Type::Generic(name) => format!("عام({})", name),
            Type::Union(types) => {
                let types_str: Vec<String> = types.iter()
                    .map(|t| t.to_string_repr())
                    .collect();
                format!("اتحاد({})", types_str.join(" | "))
            }
            Type::Intersection(types) => {
                let types_str: Vec<String> = types.iter()
                    .map(|t| t.to_string_repr())
                    .collect();
                format!("تقاطع({})", types_str.join(" & "))
            }
            Type::Optional(inner) => format!("{}؟", inner.to_string_repr()),
            Type::Tensor(dim) => format!("موتر[{}]", dim),
            Type::Unknown => "غير_معروف".to_string(),
            Type::Any => "أي".to_string(),
            Type::Void => "فراغ".to_string(),
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Type::Unknown
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// معلومات النوع للمتغيرات
// ═══════════════════════════════════════════════════════════════════════════════

/// معلومات نوع المتغير
#[derive(Debug, Clone)]
pub struct VariableTypeInfo {
    /// اسم المتغير
    pub name: String,
    /// النوع المستنتج
    pub inferred_type: Type,
    /// النوع المعلن (إن وجد)
    pub declared_type: Option<Type>,
    /// هل النوع ثابت
    pub is_const: bool,
    /// مواقع الاستخدام
    pub use_sites: Vec<usize>,
    /// مواقع التعريف
    pub def_sites: Vec<usize>,
    /// عدد مرات الاستخدام
    pub use_count: u32,
    /// الثبات (هل تم تعيينه مرة واحدة فقط)
    pub is_single_assignment: bool,
}

impl VariableTypeInfo {
    pub fn new(name: String) -> Self {
        VariableTypeInfo {
            name,
            inferred_type: Type::Unknown,
            declared_type: None,
            is_const: false,
            use_sites: Vec::new(),
            def_sites: Vec::new(),
            use_count: 0,
            is_single_assignment: true,
        }
    }
    
    /// تحديث النوع
    pub fn update_type(&mut self, new_type: &Type) {
        self.inferred_type = self.inferred_type.merge(new_type);
        if self.def_sites.len() > 1 {
            self.is_single_assignment = false;
        }
    }
    
    /// تسجيل استخدام
    pub fn register_use(&mut self, ip: usize) {
        self.use_sites.push(ip);
        self.use_count += 1;
    }
    
    /// تسجيل تعريف
    pub fn register_def(&mut self, ip: usize) {
        self.def_sites.push(ip);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// حارس النوع المحسّن
// ═══════════════════════════════════════════════════════════════════════════════

/// حارس نوع متقدم
#[derive(Debug, Clone)]
pub struct TypeGuard {
    /// معرف الحارس
    pub id: u64,
    /// نوع الحارس
    pub guard_type: TypeGuardKind,
    /// الموقع في الكود
    pub location: usize,
    /// النوع المتوقع
    pub expected_type: Type,
    /// النوع الفعلي (من التنفيذ)
    pub actual_type: Option<Type>,
    /// هل الحارس نشط
    pub is_active: bool,
    /// عدد مرات النجاح
    pub success_count: u64,
    /// عدد مرات الفشل
    pub failure_count: u64,
    /// معدل النجاح
    pub success_rate: f64,
}

/// نوع الحارس
#[derive(Debug, Clone)]
pub enum TypeGuardKind {
    /// فحص نوع بسيط
    TypeCheck,
    /// فحص نوع مع narrow
    TypeNarrow { narrowed_type: Type },
    /// فحص عدم الصفر
    NonNullCheck,
    /// فحص الحدود
    BoundsCheck { min: i64, max: i64 },
    /// فحص قيمة ثابتة
    ConstValueCheck { expected: Value },
    /// فحص نوع العناصر في قائمة
    ListElementTypeCheck { element_type: Type },
    /// فحص نوع المفتاح والقيمة في قاموس
    DictTypeCheck { key_type: Type, value_type: Type },
    /// فحص عدد المعاملات
    ArityCheck { expected: usize },
    /// فحص قيمة منطقية
    BooleanCheck,
    /// فحص قابلية الاستدعاء
    CallableCheck,
}

impl TypeGuard {
    /// إنشاء حارس جديد
    pub fn new(id: u64, guard_type: TypeGuardKind, location: usize, expected_type: Type) -> Self {
        TypeGuard {
            id,
            guard_type,
            location,
            expected_type,
            actual_type: None,
            is_active: true,
            success_count: 0,
            failure_count: 0,
            success_rate: 1.0,
        }
    }
    
    /// تسجيل نجاح
    pub fn record_success(&mut self) {
        self.success_count += 1;
        self.update_success_rate();
    }
    
    /// تسجيل فشل
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.update_success_rate();
    }
    
    /// تحديث معدل النجاح
    fn update_success_rate(&mut self) {
        let total = self.success_count + self.failure_count;
        if total > 0 {
            self.success_rate = self.success_count as f64 / total as f64;
        }
    }
    
    /// هل الحارس يستحق الإبقاء
    pub fn should_keep(&self) -> bool {
        // إبقاء الحارس إذا كان معدل النجاح عالي أو لم يتم اختباره كثيراً
        self.success_rate > 0.5 || (self.success_count + self.failure_count) < 100
    }
    
    /// تحسين الحارس بناءً على البيانات
    pub fn optimize(&mut self) {
        if self.success_rate > 0.99 {
            // الحارس ناجح جداً - يمكن تبسيطه
            self.is_active = true;
        } else if self.success_rate < 0.01 {
            // الحارس يفشل غالباً - تعطيله
            self.is_active = false;
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// محلل الأنواع
// ═══════════════════════════════════════════════════════════════════════════════

/// نتيجة تحليل النوع
#[derive(Debug, Clone, Default)]
pub struct TypeAnalysisResult {
    /// أنواع المتغيرات
    pub variable_types: HashMap<String, VariableTypeInfo>,
    /// أنواع المواقع في الكود
    pub location_types: HashMap<usize, Type>,
    /// الحراس المولدة
    pub guards: Vec<TypeGuard>,
    /// الأخطاء
    pub errors: Vec<TypeError>,
    /// التحذيرات
    pub warnings: Vec<TypeWarning>,
    /// الإحصائيات
    pub stats: TypeAnalysisStats,
}

/// خطأ نوعي
#[derive(Debug, Clone)]
pub struct TypeError {
    pub message: String,
    pub location: usize,
    pub expected: Type,
    pub actual: Type,
}

/// تحذير نوعي
#[derive(Debug, Clone)]
pub struct TypeWarning {
    pub message: String,
    pub location: usize,
}

/// إحصائيات تحليل الأنواع
#[derive(Debug, Clone, Default)]
pub struct TypeAnalysisStats {
    pub total_variables: usize,
    pub typed_variables: usize,
    pub type_inference_success_rate: f64,
    pub guards_generated: usize,
    pub type_errors: usize,
    pub type_warnings: usize,
}

/// محلل الأنواع
pub struct TypeInferenceEngine {
    /// جدول الرموز
    symbol_table: HashMap<String, VariableTypeInfo>,
    /// أنواع المواقع
    location_types: HashMap<usize, Type>,
    /// الحراس
    guards: Vec<TypeGuard>,
    /// الأخطاء
    errors: Vec<TypeError>,
    /// التحذيرات
    warnings: Vec<TypeWarning>,
    /// معرف الحارس التالي
    next_guard_id: u64,
    /// مكدس الأنواع للتحليل
    type_stack: Vec<Type>,
    /// بيئة الأنواع العامة
    generic_env: HashMap<String, Type>,
}

impl TypeInferenceEngine {
    /// إنشاء محرك جديد
    pub fn new() -> Self {
        TypeInferenceEngine {
            symbol_table: HashMap::new(),
            location_types: HashMap::new(),
            guards: Vec::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
            next_guard_id: 1,
            type_stack: Vec::new(),
            generic_env: HashMap::new(),
        }
    }
    
    /// تحليل قطعة من التعليمات
    pub fn analyze(&mut self, instructions: &[OpCode]) -> TypeAnalysisResult {
        self.symbol_table.clear();
        self.location_types.clear();
        self.guards.clear();
        self.errors.clear();
        self.warnings.clear();
        self.type_stack.clear();
        
        for (ip, opcode) in instructions.iter().enumerate() {
            self.analyze_opcode(ip, opcode);
        }
        
        // توليد الحراس بناءً على التحليل
        self.generate_guards();
        
        // حساب الإحصائيات
        let stats = TypeAnalysisStats {
            total_variables: self.symbol_table.len(),
            typed_variables: self.symbol_table.values()
                .filter(|v| v.inferred_type != Type::Unknown)
                .count(),
            type_inference_success_rate: if self.symbol_table.is_empty() {
                1.0
            } else {
                self.symbol_table.values()
                    .filter(|v| v.inferred_type != Type::Unknown)
                    .count() as f64 / self.symbol_table.len() as f64
            },
            guards_generated: self.guards.len(),
            type_errors: self.errors.len(),
            type_warnings: self.warnings.len(),
        };
        
        TypeAnalysisResult {
            variable_types: self.symbol_table.clone(),
            location_types: self.location_types.clone(),
            guards: self.guards.clone(),
            errors: self.errors.clone(),
            warnings: self.warnings.clone(),
            stats,
        }
    }
    
    /// تحليل تعليمة واحدة
    fn analyze_opcode(&mut self, ip: usize, opcode: &OpCode) {
        match opcode {
            OpCode::PushNumber(_) => {
                self.type_stack.push(Type::Number);
                self.location_types.insert(ip, Type::Number);
            }
            
            OpCode::PushString(_) => {
                self.type_stack.push(Type::String);
                self.location_types.insert(ip, Type::String);
            }
            
            OpCode::PushBool(_) => {
                self.type_stack.push(Type::Boolean);
                self.location_types.insert(ip, Type::Boolean);
            }
            
            OpCode::PushNull => {
                self.type_stack.push(Type::Null);
                self.location_types.insert(ip, Type::Null);
            }
            
            OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div | OpCode::Mod | OpCode::Pow => {
                self.analyze_binary_numeric_op(ip);
            }
            
            OpCode::Neg => {
                self.analyze_unary_numeric_op(ip);
            }
            
            OpCode::Equal | OpCode::NotEqual | OpCode::Less | OpCode::Greater |
            OpCode::LessEqual | OpCode::GreaterEqual => {
                self.analyze_comparison_op(ip);
            }
            
            OpCode::And | OpCode::Or => {
                self.analyze_logical_op(ip);
            }
            
            OpCode::Not => {
                if let Some(t) = self.type_stack.pop() {
                    if t != Type::Boolean && t != Type::Unknown {
                        self.warnings.push(TypeWarning {
                            message: format!("عملية 'ليس' على نوع غير منطقي: {}", t.to_string_repr()),
                            location: ip,
                        });
                    }
                }
                self.type_stack.push(Type::Boolean);
                self.location_types.insert(ip, Type::Boolean);
            }
            
            OpCode::BuildList(count) => {
                let count = *count as usize;
                let element_type = if count > 0 {
                    // استنباط نوع العناصر
                    let mut types: Vec<Type> = Vec::new();
                    for _ in 0..count.min(self.type_stack.len()) {
                        if let Some(t) = self.type_stack.pop() {
                            types.push(t);
                        }
                    }
                    if types.is_empty() {
                        Type::Unknown
                    } else {
                        let first = types[0].clone();
                        if types.iter().all(|t| t == &first) {
                            first
                        } else {
                            Type::union(types)
                        }
                    }
                } else {
                    Type::Unknown
                };
                self.type_stack.push(Type::list_of(element_type.clone()));
                self.location_types.insert(ip, Type::list_of(element_type));
            }
            
            OpCode::BuildDict(count) => {
                let count = *count as usize;
                let (key_type, value_type) = if count > 0 {
                    let mut key_types: Vec<Type> = Vec::new();
                    let mut value_types: Vec<Type> = Vec::new();
                    for _ in 0..count.min(self.type_stack.len() / 2) {
                        if let Some(v) = self.type_stack.pop() {
                            value_types.push(v);
                        }
                        if let Some(k) = self.type_stack.pop() {
                            key_types.push(k);
                        }
                    }
                    let kt = if key_types.is_empty() {
                        Type::Unknown
                    } else if key_types.iter().all(|t| t == &key_types[0]) {
                        key_types[0].clone()
                    } else {
                        Type::union(key_types)
                    };
                    let vt = if value_types.is_empty() {
                        Type::Unknown
                    } else if value_types.iter().all(|t| t == &value_types[0]) {
                        value_types[0].clone()
                    } else {
                        Type::union(value_types)
                    };
                    (kt, vt)
                } else {
                    (Type::Unknown, Type::Unknown)
                };
                self.type_stack.push(Type::dict_of(key_type.clone(), value_type.clone()));
                self.location_types.insert(ip, Type::dict_of(key_type, value_type));
            }
            
            OpCode::Index => {
                let index_type = self.type_stack.pop();
                let obj_type = self.type_stack.pop();
                
                match (&obj_type, &index_type) {
                    (Some(Type::List(inner)), Some(Type::Number)) => {
                        self.type_stack.push((**inner).clone());
                        self.location_types.insert(ip, (**inner).clone());
                    }
                    (Some(Type::Dict(k, v)), Some(idx)) => {
                        if idx.can_implicitly_cast_to(k) {
                            self.type_stack.push((**v).clone());
                            self.location_types.insert(ip, (**v).clone());
                        } else {
                            self.type_stack.push(Type::Unknown);
                            self.location_types.insert(ip, Type::Unknown);
                        }
                    }
                    (Some(Type::String), Some(Type::Number)) => {
                        self.type_stack.push(Type::String);
                        self.location_types.insert(ip, Type::String);
                    }
                    _ => {
                        self.type_stack.push(Type::Unknown);
                        self.location_types.insert(ip, Type::Unknown);
                    }
                }
            }
            
            OpCode::LoadLocal(slot) => {
                let slot = *slot as usize;
                // استنباط نوع المتغير المحلي
                let type_key = format!("local_{}", slot);
                let var_type = self.symbol_table
                    .get(&type_key)
                    .map(|v| v.inferred_type.clone())
                    .unwrap_or(Type::Unknown);
                self.type_stack.push(var_type.clone());
                self.location_types.insert(ip, var_type);
            }
            
            OpCode::StoreLocal(slot) => {
                let slot = *slot as usize;
                let type_key = format!("local_{}", slot);
                
                if let Some(value_type) = self.type_stack.last() {
                    let var_info = self.symbol_table
                        .entry(type_key)
                        .or_insert_with(|| VariableTypeInfo::new(format!("local_{}", slot)));
                    var_info.update_type(value_type);
                    var_info.register_def(ip);
                }
            }
            
            OpCode::LoadGlobal(idx) => {
                let type_key = format!("global_{}", idx);
                let var_type = self.symbol_table
                    .get(&type_key)
                    .map(|v| v.inferred_type.clone())
                    .unwrap_or(Type::Unknown);
                self.type_stack.push(var_type.clone());
                self.location_types.insert(ip, var_type);
            }
            
            OpCode::StoreGlobal(idx) => {
                let type_key = format!("global_{}", idx);
                
                if let Some(value_type) = self.type_stack.last() {
                    let var_info = self.symbol_table
                        .entry(type_key)
                        .or_insert_with(|| VariableTypeInfo::new(format!("global_{}", idx)));
                    var_info.update_type(value_type);
                    var_info.register_def(ip);
                }
            }
            
            OpCode::Call(arg_count) => {
                let arg_count = *arg_count as usize;
                let mut param_types: Vec<Type> = Vec::new();
                
                for _ in 0..arg_count.min(self.type_stack.len()) {
                    if let Some(t) = self.type_stack.pop() {
                        param_types.push(t);
                    }
                }
                param_types.reverse();
                
                // الحصول على نوع الدالة
                let func_type = self.type_stack.pop();
                
                let return_type = match func_type {
                    Some(Type::Function(params, ret)) => {
                        // التحقق من المعاملات
                        if params.len() != param_types.len() {
                            self.errors.push(TypeError {
                                message: format!(
                                    "عدد المعاملات غير متطابق: متوقع {}، فعلي {}",
                                    params.len(), param_types.len()
                                ),
                                location: ip,
                                expected: Type::Function(params.clone(), ret.clone()),
                                actual: Type::Function(param_types, Box::new(Type::Unknown)),
                            });
                        } else {
                            for (i, (expected, actual)) in params.iter().zip(param_types.iter()).enumerate() {
                                if !actual.can_implicitly_cast_to(expected) {
                                    self.errors.push(TypeError {
                                        message: format!(
                                            "نوع المعامل {} غير متطابق: متوقع {}، فعلي {}",
                                            i + 1,
                                            expected.to_string_repr(),
                                            actual.to_string_repr()
                                        ),
                                        location: ip,
                                        expected: expected.clone(),
                                        actual: actual.clone(),
                                    });
                                }
                            }
                        }
                        (*ret).clone()
                    }
                    Some(Type::Unknown) | None => Type::Unknown,
                    Some(other) => {
                        self.errors.push(TypeError {
                            message: format!("محاولة استدعاء غير دالة: {}", other.to_string_repr()),
                            location: ip,
                            expected: Type::Function(vec![], Box::new(Type::Unknown)),
                            actual: other,
                        });
                        Type::Unknown
                    }
                };
                
                self.type_stack.push(return_type.clone());
                self.location_types.insert(ip, return_type);
            }
            
            OpCode::JumpIfFalse(_) | OpCode::JumpIfTrue(_) => {
                // استهلاك الشرط من المكدس
                if let Some(cond_type) = self.type_stack.pop() {
                    if cond_type != Type::Boolean && cond_type != Type::Unknown {
                        self.warnings.push(TypeWarning {
                            message: format!(
                                "شرط غير منطقي: {}",
                                cond_type.to_string_repr()
                            ),
                            location: ip,
                        });
                    }
                }
            }
            
            OpCode::Return | OpCode::ReturnValue => {
                // معالجة الإرجاع
            }
            
            OpCode::Halt => {
                // نهاية التنفيذ
            }
            
            _ => {
                // تعليمات أخرى
            }
        }
    }
    
    /// تحليل عملية حسابية ثنائية
    fn analyze_binary_numeric_op(&mut self, ip: usize) {
        let b_type = self.type_stack.pop();
        let a_type = self.type_stack.pop();
        
        match (&a_type, &b_type) {
            (Some(Type::Number), Some(Type::Number)) => {
                self.type_stack.push(Type::Number);
                self.location_types.insert(ip, Type::Number);
            }
            (Some(Type::Tensor(d1)), Some(Type::Tensor(d2))) if d1 == d2 => {
                self.type_stack.push(Type::Tensor(*d1));
                self.location_types.insert(ip, Type::Tensor(*d1));
            }
            (Some(Type::Tensor(_)), Some(Type::Number)) |
            (Some(Type::Number), Some(Type::Tensor(_))) => {
                // Broadcasting
                let tensor_dim = match (&a_type, &b_type) {
                    (Some(Type::Tensor(d)), _) | (_, Some(Type::Tensor(d))) => *d,
                    _ => 0,
                };
                self.type_stack.push(Type::Tensor(tensor_dim));
                self.location_types.insert(ip, Type::Tensor(tensor_dim));
            }
            (Some(a), Some(b)) if a.is_numeric() && b.is_numeric() => {
                self.type_stack.push(Type::Number);
                self.location_types.insert(ip, Type::Number);
            }
            (Some(Type::Unknown), _) | (_, Some(Type::Unknown)) | (None, _) | (_, None) => {
                self.type_stack.push(Type::Unknown);
                self.location_types.insert(ip, Type::Unknown);
            }
            (Some(a), Some(b)) => {
                self.errors.push(TypeError {
                    message: format!(
                        "عملية حسابية على أنواع غير رقمية: {} و {}",
                        a.to_string_repr(),
                        b.to_string_repr()
                    ),
                    location: ip,
                    expected: Type::Number,
                    actual: a.merge(b),
                });
                self.type_stack.push(Type::Unknown);
                self.location_types.insert(ip, Type::Unknown);
            }
        }
    }
    
    /// تحليل عملية حسابية أحادية
    fn analyze_unary_numeric_op(&mut self, ip: usize) {
        let a_type = self.type_stack.pop();
        
        match a_type {
            Some(Type::Number) => {
                self.type_stack.push(Type::Number);
                self.location_types.insert(ip, Type::Number);
            }
            Some(Type::Tensor(d)) => {
                self.type_stack.push(Type::Tensor(d));
                self.location_types.insert(ip, Type::Tensor(d));
            }
            Some(Type::Unknown) | None => {
                self.type_stack.push(Type::Unknown);
                self.location_types.insert(ip, Type::Unknown);
            }
            Some(other) => {
                self.errors.push(TypeError {
                    message: format!(
                        "عملية حسابية على نوع غير رقمي: {}",
                        other.to_string_repr()
                    ),
                    location: ip,
                    expected: Type::Number,
                    actual: other,
                });
                self.type_stack.push(Type::Unknown);
                self.location_types.insert(ip, Type::Unknown);
            }
        }
    }
    
    /// تحليل عملية مقارنة
    fn analyze_comparison_op(&mut self, ip: usize) {
        let b_type = self.type_stack.pop();
        let a_type = self.type_stack.pop();
        
        match (&a_type, &b_type) {
            (Some(a), Some(b)) if a.is_comparable() && b.is_comparable() => {
                if !a.can_implicitly_cast_to(b) && !b.can_implicitly_cast_to(a) {
                    self.warnings.push(TypeWarning {
                        message: format!(
                            "مقارنة أنواع مختلفة: {} و {}",
                            a.to_string_repr(),
                            b.to_string_repr()
                        ),
                        location: ip,
                    });
                }
            }
            _ => {}
        }
        
        self.type_stack.push(Type::Boolean);
        self.location_types.insert(ip, Type::Boolean);
    }
    
    /// تحليل عملية منطقية
    fn analyze_logical_op(&mut self, ip: usize) {
        let b_type = self.type_stack.pop();
        let a_type = self.type_stack.pop();
        
        // التحقق من الأنواع
        if let Some(ref t) = a_type {
            if *t != Type::Boolean && *t != Type::Unknown {
                self.warnings.push(TypeWarning {
                    message: format!("عملية منطقية على نوع غير منطقي: {}", t.to_string_repr()),
                    location: ip,
                });
            }
        }
        if let Some(ref t) = b_type {
            if *t != Type::Boolean && *t != Type::Unknown {
                self.warnings.push(TypeWarning {
                    message: format!("عملية منطقية على نوع غير منطقي: {}", t.to_string_repr()),
                    location: ip,
                });
            }
        }
        
        self.type_stack.push(Type::Boolean);
        self.location_types.insert(ip, Type::Boolean);
    }
    
    /// توليد الحراس بناءً على التحليل
    fn generate_guards(&mut self) {
        for (ip, expected_type) in &self.location_types {
            if *expected_type != Type::Unknown && *expected_type != Type::Any {
                let guard = TypeGuard::new(
                    self.next_guard_id,
                    TypeGuardKind::TypeCheck,
                    *ip,
                    expected_type.clone(),
                );
                self.next_guard_id += 1;
                self.guards.push(guard);
            }
        }
        
        // إضافة حراس للمتغيرات ذات الأنواع المستقرة
        for (name, var_info) in &self.symbol_table {
            if var_info.inferred_type != Type::Unknown && var_info.is_single_assignment {
                let guard = TypeGuard::new(
                    self.next_guard_id,
                    TypeGuardKind::TypeNarrow {
                        narrowed_type: var_info.inferred_type.clone(),
                    },
                    var_info.def_sites.first().copied().unwrap_or(0),
                    var_info.inferred_type.clone(),
                );
                self.next_guard_id += 1;
                self.guards.push(guard);
            }
        }
    }
    
    /// الحصول على نوع المتغير
    pub fn get_variable_type(&self, name: &str) -> Option<&Type> {
        self.symbol_table.get(name).map(|v| &v.inferred_type)
    }
    
    /// طباعة تقرير
    pub fn print_report(&self) {
        println!("╔══════════════════════════════════════════════════════════════════════════╗");
        println!("║              🔍 تقرير تحليل الأنواع - لغة المرجع                         ║");
        println!("╠══════════════════════════════════════════════════════════════════════════╣");
        
        println!("║ 📊 المتغيرات:                                                            ║");
        for (name, info) in &self.symbol_table {
            println!("║    {} : {}                                           ║",
                name,
                info.inferred_type.to_string_repr()
            );
        }
        
        println!("╠══════════════════════════════════════════════════════════════════════════╣");
        println!("║ 🛡️ الحراس:                                                               ║");
        for guard in &self.guards {
            println!("║    [{}] {:?} في {}                                     ║",
                guard.id,
                guard.guard_type,
                guard.location
            );
        }
        
        if !self.errors.is_empty() {
            println!("╠══════════════════════════════════════════════════════════════════════════╣");
            println!("║ ❌ الأخطاء:                                                               ║");
            for error in &self.errors {
                println!("║    {} في {}                                              ║",
                    error.message,
                    error.location
                );
            }
        }
        
        println!("╚══════════════════════════════════════════════════════════════════════════╝");
    }
}

impl Default for TypeInferenceEngine {
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
    fn test_type_inference_basic() {
        let mut engine = TypeInferenceEngine::new();
        
        let instructions = vec![
            OpCode::PushNumber(5.0),
            OpCode::PushNumber(3.0),
            OpCode::Add,
            OpCode::Halt,
        ];
        
        let result = engine.analyze(&instructions);
        
        assert!(result.errors.is_empty());
        assert_eq!(result.location_types.get(&2), Some(&Type::Number));
    }
    
    #[test]
    fn test_type_inference_list() {
        let mut engine = TypeInferenceEngine::new();
        
        let instructions = vec![
            OpCode::PushNumber(1.0),
            OpCode::PushNumber(2.0),
            OpCode::PushNumber(3.0),
            OpCode::BuildList(3),
            OpCode::Halt,
        ];
        
        let result = engine.analyze(&instructions);
        
        assert!(result.errors.is_empty());
        assert_eq!(result.location_types.get(&3), Some(&Type::list_of(Type::Number)));
    }
    
    #[test]
    fn test_type_merge() {
        let t1 = Type::Number;
        let t2 = Type::String;
        let merged = t1.merge(&t2);
        
        match merged {
            Type::Union(types) => {
                assert_eq!(types.len(), 2);
            }
            _ => panic!("Expected Union type"),
        }
    }
    
    #[test]
    fn test_type_implicit_cast() {
        assert!(Type::Number.can_implicitly_cast_to(&Type::Number));
        assert!(Type::Number.can_implicitly_cast_to(&Type::Any));
        assert!(Type::Null.can_implicitly_cast_to(&Type::optional(Type::Number)));
        assert!(!Type::Number.can_implicitly_cast_to(&Type::String));
    }
    
    #[test]
    fn test_guard_optimization() {
        let mut guard = TypeGuard::new(
            1,
            TypeGuardKind::TypeCheck,
            0,
            Type::Number,
        );
        
        // محاكاة نجاحات
        for _ in 0..99 {
            guard.record_success();
        }
        
        assert!(guard.success_rate > 0.99);
        assert!(guard.should_keep());
    }
}
