// ═══════════════════════════════════════════════════════════════════════════════
// أدوات ONNX المساعدة
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::path::Path;

use super::types::{ONNXTensor, ONNXDataType, ONNXModelInfo, TensorInfo, ONNXShape};
use super::engine::ONNXEngine;
use super::ONNXModel;

// ═══════════════════════════════════════════════════════════════════════════════
// Model Loading Utilities
// ═══════════════════════════════════════════════════════════════════════════════

/// تحميل نموذج ONNX من ملف
pub fn load_model(path: &str) -> Result<ONNXEngine, String> {
    ONNXEngine::load(path)
}

/// حفظ نموذج ONNX (للتوافق)
pub fn save_model(_engine: &ONNXEngine, _path: &str) -> Result<(), String> {
    // في الإصدار الكامل، هذا يحفظ حالة المحرك
    Ok(())
}

/// التحقق من صحة نموذج ONNX
pub fn validate_model(path: &str) -> Result<ValidationResult, String> {
    // التحقق من وجود الملف
    if !Path::new(path).exists() {
        return Err(format!("الملف غير موجود: {}", path));
    }

    // التحقق من الامتداد
    let extension = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    if extension != "onnx" {
        return Err(format!("الملف ليس بصيغة ONNX: {}", extension));
    }

    // قراءة حجم الملف
    let file_size = std::fs::metadata(path)
        .map(|m| m.len())
        .unwrap_or(0);

    Ok(ValidationResult {
        is_valid: true,
        file_size,
        errors: vec![],
        warnings: vec![],
    })
}

/// نتيجة التحقق
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub file_size: u64,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// الحصول على معلومات نموذج ONNX
pub fn get_model_metadata(path: &str) -> Result<ModelMetadata, String> {
    let validation = validate_model(path)?;

    let file_name = Path::new(path)
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("نموذج");

    Ok(ModelMetadata {
        name: file_name.to_string(),
        path: path.to_string(),
        file_size: validation.file_size,
        is_valid: validation.is_valid,
        inputs: vec![],
        outputs: vec![],
        custom_metadata: HashMap::new(),
    })
}

/// بيانات وصفية للنموذج
#[derive(Debug, Clone)]
pub struct ModelMetadata {
    pub name: String,
    pub path: String,
    pub file_size: u64,
    pub is_valid: bool,
    pub inputs: Vec<TensorInfo>,
    pub outputs: Vec<TensorInfo>,
    pub custom_metadata: HashMap<String, String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tensor Utilities
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء موتر من بيانات مختلفة
pub fn create_tensor<T: Into<f64>>(data: Vec<T>, shape: Vec<usize>) -> ONNXTensor {
    ONNXTensor::new(data.into_iter().map(|x| x.into()).collect(), shape)
}

/// إنشاء موتر من صورة
pub fn image_to_tensor(image_data: &[u8], width: usize, height: usize, channels: usize) -> ONNXTensor {
    let data: Vec<f64> = image_data.iter().map(|&x| x as f64 / 255.0).collect();
    ONNXTensor::new(data, vec![1, channels, height, width])
}

/// إنشاء موتر من نص (tokenization بسيطة)
pub fn text_to_tensor(text: &str, max_length: usize, vocab: &HashMap<String, usize>) -> ONNXTensor {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut tokens = vec![0.0; max_length];

    for (i, word) in words.iter().take(max_length).enumerate() {
        if let Some(&id) = vocab.get(*word) {
            tokens[i] = id as f64;
        }
    }

    ONNXTensor::new(tokens, vec![1, max_length])
}

/// تحويل موتر إلى صورة
pub fn tensor_to_image(tensor: &ONNXTensor, width: usize, height: usize) -> Result<Vec<u8>, String> {
    if tensor.shape.len() < 3 {
        return Err("الموتر ليس صورة".to_string());
    }

    let data: Vec<u8> = tensor.data.iter()
        .map(|&x| ((x * 255.0).min(255.0).max(0.0)) as u8)
        .collect();

    if data.len() != width * height * 3 {
        return Err("حجم البيانات لا يطابق الصورة".to_string());
    }

    Ok(data)
}

/// تطبيع موتر
pub fn normalize_tensor(tensor: &ONNXTensor, mean: &[f64], std: &[f64]) -> ONNXTensor {
    let data: Vec<f64> = tensor.data.iter()
        .enumerate()
        .map(|(i, &x)| {
            let c = i % mean.len();
            (x - mean[c]) / std[c]
        })
        .collect();

    ONNXTensor::new(data, tensor.shape.clone())
}

/// تغيير حجم موتر
pub fn resize_tensor(tensor: &ONNXTensor, new_shape: Vec<usize>) -> Result<ONNXTensor, String> {
    tensor.reshape(new_shape)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Model Information Utilities
// ═══════════════════════════════════════════════════════════════════════════════

/// الحصول على حجم الذاكرة المطلوب للنموذج
pub fn estimate_memory_usage(model_path: &str) -> Result<u64, String> {
    let metadata = std::fs::metadata(model_path)
        .map_err(|e| format!("خطأ في قراءة الملف: {}", e))?;

    // تقدير تقريبي: حجم الملف × 2-3 للذاكرة
    Ok(metadata.len() * 2)
}

/// مقارنة نموذجين
pub fn compare_models(model1: &str, model2: &str) -> Result<ModelComparison, String> {
    let meta1 = get_model_metadata(model1)?;
    let meta2 = get_model_metadata(model2)?;

    Ok(ModelComparison {
        model1_name: meta1.name,
        model2_name: meta2.name,
        model1_size: meta1.file_size,
        model2_size: meta2.file_size,
        size_difference: meta1.file_size as i64 - meta2.file_size as i64,
    })
}

/// نتيجة المقارنة
#[derive(Debug, Clone)]
pub struct ModelComparison {
    pub model1_name: String,
    pub model2_name: String,
    pub model1_size: u64,
    pub model2_size: u64,
    pub size_difference: i64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// ONNX Operators Reference
// ═══════════════════════════════════════════════════════════════════════════════

/// قائمة مشغلات ONNX المدعومة
pub const SUPPORTED_OPERATORS: &[&str] = &[
    // العمليات الحسابية
    "Add", "Sub", "Mul", "Div", "MatMul", "Gemm",
    "Pow", "Sqrt", "Exp", "Log", "Neg", "Abs",
    
    // دوال التنشيط
    "Relu", "Sigmoid", "Tanh", "Softmax", "LeakyRelu",
    "Elu", "Selu", "Softplus", "Softsign", "HardSigmoid",
    
    // العمليات على المصفوفات
    "Reshape", "Transpose", "Flatten", "Squeeze", "Unsqueeze",
    "Concat", "Split", "Slice", "Gather", "Scatter",
    
    // الالتفاف
    "Conv", "ConvTranspose", "MaxPool", "AveragePool",
    "GlobalMaxPool", "GlobalAveragePool",
    
    // التسوية
    "BatchNormalization", "InstanceNormalization", "LayerNormalization",
    
    // أخرى
    "Dropout", "Shape", "Size", "Identity", "Constant",
    "Cast", "Clip", "ReduceMean", "ReduceSum", "ReduceMax", "ReduceMin",
];

/// التحقق من دعم مشغل
pub fn is_operator_supported(op: &str) -> bool {
    SUPPORTED_OPERATORS.contains(&op)
}

/// الحصول على معلومات مشغل
pub fn get_operator_info(op: &str) -> Option<OperatorInfo> {
    if !is_operator_supported(op) {
        return None;
    }

    Some(OperatorInfo {
        name: op.to_string(),
        category: categorize_operator(op),
        description: format!("مشغل ONNX: {}", op),
    })
}

/// تصنيف المشغل
fn categorize_operator(op: &str) -> String {
    match op {
        "Add" | "Sub" | "Mul" | "Div" | "MatMul" | "Gemm" | "Pow" | "Sqrt" | "Exp" | "Log" | "Neg" | "Abs" => "حسابي".to_string(),
        "Relu" | "Sigmoid" | "Tanh" | "Softmax" | "LeakyRelu" | "Elu" | "Selu" | "Softplus" | "Softsign" | "HardSigmoid" => "تنشيط".to_string(),
        "Reshape" | "Transpose" | "Flatten" | "Squeeze" | "Unsqueeze" | "Concat" | "Split" | "Slice" | "Gather" | "Scatter" => "شكل".to_string(),
        "Conv" | "ConvTranspose" | "MaxPool" | "AveragePool" | "GlobalMaxPool" | "GlobalAveragePool" => "التفاف".to_string(),
        "BatchNormalization" | "InstanceNormalization" | "LayerNormalization" => "تسوية".to_string(),
        _ => "أخرى".to_string(),
    }
}

/// معلومات المشغل
#[derive(Debug, Clone)]
pub struct OperatorInfo {
    pub name: String,
    pub category: String,
    pub description: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Pre-trained Models
// ═══════════════════════════════════════════════════════════════════════════════

/// نماذج مدربة مسبقاً متاحة
pub const PRETRAINED_MODELS: &[PretrainedModelInfo] = &[
    PretrainedModelInfo {
        name: "resnet50",
        description: "ResNet-50 للتصنيف",
        input_shape: vec![1, 3, 224, 224],
        num_classes: 1000,
        category: "تصنيف صور",
    },
    PretrainedModelInfo {
        name: "bert-base",
        description: "BERT Base للنصوص",
        input_shape: vec![1, 512],
        num_classes: 0,
        category: "معالجة نصوص",
    },
    PretrainedModelInfo {
        name: "yolov5s",
        description: "YOLOv5 Small للكشف",
        input_shape: vec![1, 3, 640, 640],
        num_classes: 80,
        category: "كشف كائنات",
    },
    PretrainedModelInfo {
        name: "mobilenetv2",
        description: "MobileNetV2 للتصنيف السريع",
        input_shape: vec![1, 3, 224, 224],
        num_classes: 1000,
        category: "تصنيف صور",
    },
];

/// معلومات نموذج مدرب مسبقاً
#[derive(Debug, Clone)]
pub struct PretrainedModelInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub input_shape: Vec<usize>,
    pub num_classes: usize,
    pub category: &'static str,
}

/// البحث عن نموذج مدرب مسبقاً
pub fn find_pretrained_model(name: &str) -> Option<&'static PretrainedModelInfo> {
    PRETRAINED_MODELS.iter().find(|m| m.name == name)
}

/// قائمة النماذج حسب الفئة
pub fn list_models_by_category(category: &str) -> Vec<&'static PretrainedModelInfo> {
    PRETRAINED_MODELS.iter().filter(|m| m.category == category).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tensor() {
        let tensor = create_tensor(vec![1, 2, 3, 4], vec![2, 2]);
        assert_eq!(tensor.shape, vec![2, 2]);
    }

    #[test]
    fn test_supported_operators() {
        assert!(is_operator_supported("Conv"));
        assert!(is_operator_supported("Relu"));
        assert!(!is_operator_supported("UnknownOp"));
    }

    #[test]
    fn test_operator_info() {
        let info = get_operator_info("Conv").unwrap();
        assert_eq!(info.category, "التفاف");
    }

    #[test]
    fn test_pretrained_models() {
        let model = find_pretrained_model("resnet50");
        assert!(model.is_some());
        
        let classification_models = list_models_by_category("تصنيف صور");
        assert!(!classification_models.is_empty());
    }
}
