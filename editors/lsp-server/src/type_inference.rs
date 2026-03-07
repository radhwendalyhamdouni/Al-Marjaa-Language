//! Type Inference - استنتاج الأنواع

use std::collections::HashMap;

/// مستنتج الأنواع
pub struct TypeInferenceEngine {
    /// جدول الأنواع
    type_table: HashMap<String, TypeInfo>,
}

/// معلومات النوع
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub kind: TypeKind,
    pub source: TypeSource,
}

/// نوع البيانات
#[derive(Debug, Clone)]
pub enum TypeKind {
    Number,
    String,
    Boolean,
    List(Box<TypeKind>),
    Dict(Box<TypeKind>, Box<TypeKind>),
    Function(Vec<TypeKind>, Box<TypeKind>),
    Class(String),
    Null,
    Unknown,
    Any,
    Union(Vec<TypeKind>),
    Optional(Box<TypeKind>),
}

/// مصدر النوع
#[derive(Debug, Clone)]
pub enum TypeSource {
    Inferred,
    Annotated,
    Default,
}

impl TypeInferenceEngine {
    pub fn new() -> Self {
        Self {
            type_table: HashMap::new(),
        }
    }
    
    /// استنتاج نوع التعبير
    pub fn infer(&mut self, expr: &str) -> TypeKind {
        // رقم
        if expr.parse::<f64>().is_ok() {
            return TypeKind::Number;
        }
        
        // نص
        if expr.starts_with('"') || expr.starts_with('\'') {
            return TypeKind::String;
        }
        
        // منطقي
        if expr == "صح" || expr == "خطأ" {
            return TypeKind::Boolean;
        }
        
        // فارغ
        if expr == "لا_شيء" {
            return TypeKind::Null;
        }
        
        // قائمة
        if expr.starts_with('[') {
            return TypeKind::List(Box::new(TypeKind::Unknown));
        }
        
        // قاموس
        if expr.starts_with('{') && !expr.contains("=>") {
            return TypeKind::Dict(Box::new(TypeKind::Unknown), Box::new(TypeKind::Unknown));
        }
        
        // دالة
        if expr.starts_with("دالة") || expr.starts_with("لامدا") {
            return TypeKind::Function(Vec::new(), Box::new(TypeKind::Unknown));
        }
        
        TypeKind::Unknown
    }
    
    /// التحقق من توافق النوعين
    pub fn is_compatible(&self, expected: &TypeKind, actual: &TypeKind) -> bool {
        match (expected, actual) {
            (TypeKind::Any, _) | (_, TypeKind::Any) => true,
            (TypeKind::Unknown, _) | (_, TypeKind::Unknown) => true,
            (TypeKind::Optional(inner), actual) => {
                self.is_compatible(inner, actual) || matches!(actual, TypeKind::Null)
            }
            (expected, TypeKind::Null) => matches!(expected, TypeKind::Optional(_) | TypeKind::Null),
            (TypeKind::Number, TypeKind::Number) => true,
            (TypeKind::String, TypeKind::String) => true,
            (TypeKind::Boolean, TypeKind::Boolean) => true,
            (TypeKind::List(e1), TypeKind::List(a1)) => self.is_compatible(e1, a1),
            (TypeKind::Dict(ek, ev), TypeKind::Dict(ak, av)) => {
                self.is_compatible(ek, ak) && self.is_compatible(ev, av)
            }
            (TypeKind::Union(types), actual) => types.iter().any(|t| self.is_compatible(t, actual)),
            (expected, TypeKind::Union(types)) => types.iter().any(|t| self.is_compatible(expected, t)),
            _ => false,
        }
    }
    
    /// الحصول على تمثيل النصي للنوع
    pub fn type_to_string(&self, kind: &TypeKind) -> String {
        match kind {
            TypeKind::Number => "رقم".to_string(),
            TypeKind::String => "نص".to_string(),
            TypeKind::Boolean => "منطقي".to_string(),
            TypeKind::List(inner) => format!("قائمة[{}]", self.type_to_string(inner)),
            TypeKind::Dict(key, value) => format!("قاموس[{}، {}]", self.type_to_string(key), self.type_to_string(value)),
            TypeKind::Function(params, ret) => {
                let params_str: Vec<String> = params.iter().map(|p| self.type_to_string(p)).collect();
                format!("({}) -> {}", params_str.join("، "), self.type_to_string(ret))
            }
            TypeKind::Class(name) => name.clone(),
            TypeKind::Null => "لا_شيء".to_string(),
            TypeKind::Unknown => "غير_معروف".to_string(),
            TypeKind::Any => "أي".to_string(),
            TypeKind::Union(types) => {
                let types_str: Vec<String> = types.iter().map(|t| self.type_to_string(t)).collect();
                types_str.join(" | ")
            }
            TypeKind::Optional(inner) => format!("{}؟", self.type_to_string(inner)),
        }
    }
}

impl Default for TypeInferenceEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_infer_number() {
        let mut engine = TypeInferenceEngine::new();
        assert!(matches!(engine.infer("42"), TypeKind::Number));
        assert!(matches!(engine.infer("3.14"), TypeKind::Number));
    }
    
    #[test]
    fn test_infer_string() {
        let mut engine = TypeInferenceEngine::new();
        assert!(matches!(engine.infer("\"مرحبا\""), TypeKind::String));
    }
    
    #[test]
    fn test_infer_boolean() {
        let mut engine = TypeInferenceEngine::new();
        assert!(matches!(engine.infer("صح"), TypeKind::Boolean));
        assert!(matches!(engine.infer("خطأ"), TypeKind::Boolean));
    }
    
    #[test]
    fn test_type_to_string() {
        let engine = TypeInferenceEngine::new();
        assert_eq!(engine.type_to_string(&TypeKind::Number), "رقم");
        assert_eq!(engine.type_to_string(&TypeKind::String), "نص");
    }
}
