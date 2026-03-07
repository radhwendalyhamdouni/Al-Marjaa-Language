// ═══════════════════════════════════════════════════════════════════════════════
// وحدة ONNX - دعم Open Neural Network Exchange
// ═══════════════════════════════════════════════════════════════════════════════
// © 2026 رضوان دالي حمدوني | RADHWEN DALY HAMDOUNI
// جميع الحقوق محفوظة | All Rights Reserved
// ═══════════════════════════════════════════════════════════════════════════════

//! # وحدة ONNX - دعم نماذج الذكاء الاصطناعي الموحدة
//!
//! توفر هذه الوحدة دعمًا كاملاً لـ ONNX (Open Neural Network Exchange)،
//! مما يتيح:
//! - تحميل نماذج من PyTorch و TensorFlow وغيرها
//! - تشغيل الاستدلال بكفاءة عالية
//! - تصدير النماذج المدربة بصيغة ONNX
//! - التكامل مع الأنواع الأصلية للغة المرجع

pub mod engine;
pub mod types;
pub mod export;
pub mod inference;
pub mod utils;

pub use engine::{ONNXEngine, ONNXSession, ONNXConfig};
pub use types::{ONNXTensor, ONNXShape, ONNXDataType, ONNXModelInfo};
pub use export::{ONNXExporter, ExportOptions, ExportResult};
pub use inference::{ONNXInference, InferenceOptions, InferenceResult};
pub use utils::{load_model, save_model, validate_model, get_model_metadata};

// ═══════════════════════════════════════════════════════════════════════════════
// الأنواع الأساسية
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// معلومات نموذج ONNX الكاملة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ONNXModel {
    /// اسم النموذج
    pub name: String,
    /// إصدار النموذج
    pub version: i64,
    /// اسم المنتج (مثل PyTorch, TensorFlow)
    pub producer: String,
    /// وصف النموذج
    pub description: Option<String>,
    /// المدخلات
    pub inputs: Vec<ONNXInput>,
    /// المخرجات
    pub outputs: Vec<ONNXOutput>,
    /// البيانات الوصفية
    pub metadata: HashMap<String, String>,
    /// حجم الملف بالبايت
    pub file_size: Option<u64>,
}

/// مدخل نموذج ONNX
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ONNXInput {
    /// اسم المدخل
    pub name: String,
    /// نوع البيانات
    pub data_type: ONNXDataType,
    /// الشكل
    pub shape: Vec<i64>,
    /// وصف اختياري
    pub description: Option<String>,
}

/// مخرج نموذج ONNX
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ONNXOutput {
    /// اسم المخرج
    pub name: String,
    /// نوع البيانات
    pub data_type: ONNXDataType,
    /// الشكل
    pub shape: Vec<i64>,
    /// وصف اختياري
    pub description: Option<String>,
}

/// حالة نموذج ONNX
#[derive(Debug, Clone, PartialEq)]
pub enum ONNXModelStatus {
    /// غير محمّل
    Unloaded,
    /// قيد التحميل
    Loading,
    /// محمّل وجاهز
    Ready,
    /// قيد الاستدلال
    Inferring,
    /// خطأ
    Error(String),
}

/// إحصائيات نموذج ONNX
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ONNXStats {
    /// عدد مرات الاستدلال
    pub inference_count: u64,
    /// إجمالي وقت الاستدلال (مللي ثانية)
    pub total_inference_time_ms: f64,
    /// متوسط وقت الاستدلال (مللي ثانية)
    pub avg_inference_time_ms: f64,
    /// حجم الذاكرة المستخدمة (ميجابايت)
    pub memory_used_mb: f64,
    /// وقت التحميل (مللي ثانية)
    pub load_time_ms: f64,
}

impl ONNXStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_inference(&mut self, time_ms: f64) {
        self.inference_count += 1;
        self.total_inference_time_ms += time_ms;
        self.avg_inference_time_ms = self.total_inference_time_ms / self.inference_count as f64;
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال مساعدة سهلة الاستخدام
// ═══════════════════════════════════════════════════════════════════════════════

/// تحميل نموذج ONNX من ملف
pub fn onnx_load(path: &str) -> Result<ONNXEngine, String> {
    ONNXEngine::load(path)
}

/// إنشاء محرك ONNX جديد
pub fn onnx_engine() -> ONNXEngine {
    ONNXEngine::new()
}

/// تشغيل استدلال سريع على نموذج ONNX
pub fn onnx_infer(model_path: &str, inputs: &[(String, Vec<f64>, Vec<usize>)]) -> Result<HashMap<String, Vec<f64>>, String> {
    let mut engine = ONNXEngine::load(model_path)?;
    engine.infer_simple(inputs)
}

/// تصدير شبكة عصبية إلى ONNX
pub fn onnx_export(network_name: &str, layers: &[LayerSpec], output_path: &str) -> Result<(), String> {
    let exporter = ONNXExporter::new();
    exporter.export(network_name, layers, output_path)
}

/// مواصفات طبقة للتصدير
#[derive(Debug, Clone)]
pub struct LayerSpec {
    pub name: String,
    pub layer_type: String,
    pub input_size: usize,
    pub output_size: usize,
    pub activation: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// التكامل مع لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء تمثيل ONNX من Tensor المرجع
pub fn tensor_to_onnx(data: &[f64], shape: &[usize]) -> ONNXTensor {
    ONNXTensor::new(data.to_vec(), shape.to_vec())
}

/// تحويل ONNX Tensor إلى بيانات المرجع
pub fn onnx_to_tensor(tensor: &ONNXTensor) -> (Vec<f64>, Vec<usize>) {
    (tensor.data.clone(), tensor.shape.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onnx_tensor_creation() {
        let tensor = ONNXTensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        assert_eq!(tensor.data.len(), 4);
        assert_eq!(tensor.shape, vec![2, 2]);
    }

    #[test]
    fn test_onnx_stats() {
        let mut stats = ONNXStats::new();
        stats.add_inference(10.0);
        stats.add_inference(20.0);
        assert_eq!(stats.inference_count, 2);
        assert_eq!(stats.avg_inference_time_ms, 15.0);
    }

    #[test]
    fn test_layer_spec() {
        let layer = LayerSpec {
            name: "dense1".to_string(),
            layer_type: "dense".to_string(),
            input_size: 784,
            output_size: 128,
            activation: Some("relu".to_string()),
        };
        assert_eq!(layer.input_size, 784);
    }
}
