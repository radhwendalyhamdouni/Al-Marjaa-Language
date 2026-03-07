// ═══════════════════════════════════════════════════════════════════════════════
// استدلال ONNX
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

use super::types::ONNXTensor;
use super::engine::ONNXEngine;

// ═══════════════════════════════════════════════════════════════════════════════
// Inference Options
// ═══════════════════════════════════════════════════════════════════════════════

/// خيارات الاستدلال
#[derive(Debug, Clone)]
pub struct InferenceOptions {
    /// حجم الدفعة (Batch size)
    pub batch_size: usize,
    /// استخدام GPU
    pub use_gpu: bool,
    /// عدد الخيوط
    pub num_threads: usize,
    /// الحد الأقصى للوقت (مللي ثانية)
    pub timeout_ms: Option<u64>,
    /// تفعيل القياس
    pub enable_profiling: bool,
    /// مستوى الدقة (fp32, fp16, int8)
    pub precision: Precision,
}

/// مستوى الدقة
#[derive(Debug, Clone, PartialEq)]
pub enum Precision {
    /// Float32 (افتراضي)
    Float32,
    /// Float16
    Float16,
    /// Int8 (كمية)
    Int8,
}

impl Default for InferenceOptions {
    fn default() -> Self {
        Self {
            batch_size: 1,
            use_gpu: false,
            num_threads: num_cpus::get(),
            timeout_ms: None,
            enable_profiling: false,
            precision: Precision::Float32,
        }
    }
}

impl InferenceOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = size;
        self
    }

    pub fn with_gpu(mut self) -> Self {
        self.use_gpu = true;
        self
    }

    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }

    pub fn with_precision(mut self, precision: Precision) -> Self {
        self.precision = precision;
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Inference Result
// ═══════════════════════════════════════════════════════════════════════════════

/// نتيجة الاستدلال
#[derive(Debug, Clone)]
pub struct InferenceResult {
    /// المخرجات
    pub outputs: HashMap<String, ONNXTensor>,
    /// وقت الاستدلال (مللي ثانية)
    pub inference_time_ms: f64,
    /// وقت المعالجة المسبقة (مللي ثانية)
    pub preprocess_time_ms: f64,
    /// وقت المعالجة اللاحقة (مللي ثانية)
    pub postprocess_time_ms: f64,
    /// استهلاك الذاكرة (ميجابايت)
    pub memory_used_mb: f64,
    /// معلومات إضافية
    pub metadata: HashMap<String, String>,
}

impl InferenceResult {
    pub fn new(outputs: HashMap<String, ONNXTensor>) -> Self {
        Self {
            outputs,
            inference_time_ms: 0.0,
            preprocess_time_ms: 0.0,
            postprocess_time_ms: 0.0,
            memory_used_mb: 0.0,
            metadata: HashMap::new(),
        }
    }

    /// الحصول على مخرج بالاسم
    pub fn get(&self, name: &str) -> Option<&ONNXTensor> {
        self.outputs.get(name)
    }

    /// الحصول على أول مخرج
    pub fn first_output(&self) -> Option<&ONNXTensor> {
        self.outputs.values().next()
    }

    /// الحصول على البيانات كقائمة
    pub fn to_vec(&self, name: &str) -> Option<Vec<f64>> {
        self.outputs.get(name).map(|t| t.data.clone())
    }

    /// إضافة معلومات
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ONNX Inference
// ═══════════════════════════════════════════════════════════════════════════════

/// مستدل ONNX
pub struct ONNXInference {
    /// المحرك
    engine: ONNXEngine,
    /// خيارات الاستدلال
    options: InferenceOptions,
}

impl ONNXInference {
    /// إنشاء مستدل جديد
    pub fn new(engine: ONNXEngine) -> Self {
        Self {
            engine,
            options: InferenceOptions::default(),
        }
    }

    /// تحميل نموذج وإنشاء مستدل
    pub fn load(model_path: &str) -> Result<Self, String> {
        let engine = ONNXEngine::load(model_path)?;
        Ok(Self::new(engine))
    }

    /// تعيين خيارات الاستدلال
    pub fn with_options(mut self, options: InferenceOptions) -> Self {
        self.options = options;
        self
    }

    /// تشغيل الاستدلال
    pub fn run(&mut self, inputs: HashMap<String, ONNXTensor>) -> Result<InferenceResult, String> {
        use std::time::Instant;

        let start = Instant::now();

        // المعالجة المسبقة
        let preprocess_start = Instant::now();
        let processed_inputs = self.preprocess(inputs)?;
        let preprocess_time = preprocess_start.elapsed().as_millis() as f64;

        // الاستدلال
        let outputs = self.engine.infer(processed_inputs)?;

        // المعالجة اللاحقة
        let postprocess_start = Instant::now();
        let result = self.postprocess(outputs)?;
        let postprocess_time = postprocess_start.elapsed().as_millis() as f64;

        let mut inference_result = InferenceResult::new(result);
        inference_result.inference_time_ms = start.elapsed().as_millis() as f64;
        inference_result.preprocess_time_ms = preprocess_time;
        inference_result.postprocess_time_ms = postprocess_time;

        Ok(inference_result)
    }

    /// استدلال مبسط مع قائمة
    pub fn run_simple(&mut self, inputs: &[(String, Vec<f64>, Vec<usize>)]) -> Result<HashMap<String, Vec<f64>>, String> {
        self.engine.infer_simple(inputs)
    }

    /// المعالجة المسبقة
    fn preprocess(&self, inputs: HashMap<String, ONNXTensor>) -> Result<HashMap<String, ONNXTensor>, String> {
        // تطبيق التحويلات إذا لزم الأمر
        // مثل: تطبيع، تغيير الحجم، إلخ
        Ok(inputs)
    }

    /// المعالجة اللاحقة
    fn postprocess(&self, outputs: HashMap<String, ONNXTensor>) -> Result<HashMap<String, ONNXTensor>, String> {
        // تطبيق التحويلات على المخرجات
        // مثل: Softmax، تطبيق العتبة، إلخ
        Ok(outputs)
    }

    /// الحصول على المحرك
    pub fn engine(&self) -> &ONNXEngine {
        &self.engine
    }

    /// الحصول على المحرك كقابل للتعديل
    pub fn engine_mut(&mut self) -> &mut ONNXEngine {
        &mut self.engine
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Batch Inference
// ═══════════════════════════════════════════════════════════════════════════════

/// استدلال بالجملة
pub struct BatchInference {
    /// المستدل
    inference: ONNXInference,
    /// حجم الدفعة
    batch_size: usize,
}

impl BatchInference {
    pub fn new(inference: ONNXInference, batch_size: usize) -> Self {
        Self { inference, batch_size }
    }

    /// تشغيل استدلال على دفعات
    pub fn run_batch(&mut self, all_inputs: Vec<HashMap<String, ONNXTensor>>) -> Vec<Result<InferenceResult, String>> {
        all_inputs
            .into_iter()
            .map(|inputs| self.inference.run(inputs))
            .collect()
    }

    /// تشغيل استدلال مجمّع (دمج المدخلات)
    pub fn run_batched(&mut self, inputs_list: &[HashMap<String, ONNXTensor>]) -> Result<InferenceResult, String> {
        // دمج المدخلات في دفعة واحدة
        let mut batched_inputs = HashMap::new();
        
        for inputs in inputs_list {
            for (name, tensor) in inputs {
                if !batched_inputs.contains_key(name) {
                    batched_inputs.insert(name.clone(), tensor.clone());
                }
            }
        }

        self.inference.run(batched_inputs)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Specialized Inference
// ═══════════════════════════════════════════════════════════════════════════════

/// استدلال للتصنيف
pub struct ClassificationInference {
    inference: ONNXInference,
    labels: Vec<String>,
}

impl ClassificationInference {
    pub fn new(inference: ONNXInference, labels: Vec<String>) -> Self {
        Self { inference, labels }
    }

    /// تصنيف صورة أو بيانات
    pub fn classify(&mut self, input_name: &str, data: Vec<f64>, shape: Vec<usize>) -> Result<ClassificationResult, String> {
        let mut inputs = HashMap::new();
        inputs.insert(input_name.to_string(), ONNXTensor::new(data, shape));

        let result = self.inference.run(inputs)?;
        
        // الحصول على الاحتمالات
        let output = result.first_output()
            .ok_or("لا يوجد مخرج")?;

        // البحث عن أعلى احتمال
        let mut max_idx = 0;
        let mut max_val = output.data[0];
        
        for (i, &val) in output.data.iter().enumerate() {
            if val > max_val {
                max_val = val;
                max_idx = i;
            }
        }

        let label = self.labels.get(max_idx)
            .cloned()
            .unwrap_or_else(|| format!("class_{}", max_idx));

        Ok(ClassificationResult {
            class_index: max_idx,
            label,
            confidence: max_val,
            all_probabilities: output.data.clone(),
        })
    }
}

/// نتيجة التصنيف
#[derive(Debug, Clone)]
pub struct ClassificationResult {
    /// فهرس الفئة
    pub class_index: usize,
    /// اسم الفئة
    pub label: String,
    /// الثقة
    pub confidence: f64,
    /// جميع الاحتمالات
    pub all_probabilities: Vec<f64>,
}

/// استدلال للكشف عن الكائنات
pub struct DetectionInference {
    inference: ONNXInference,
    confidence_threshold: f64,
}

impl DetectionInference {
    pub fn new(inference: ONNXInference) -> Self {
        Self {
            inference,
            confidence_threshold: 0.5,
        }
    }

    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.confidence_threshold = threshold;
        self
    }

    /// كشف الكائنات
    pub fn detect(&mut self, input_name: &str, data: Vec<f64>, shape: Vec<usize>) -> Result<DetectionResult, String> {
        let mut inputs = HashMap::new();
        inputs.insert(input_name.to_string(), ONNXTensor::new(data, shape));

        let result = self.inference.run(inputs)?;

        // تحليل المخرجات للكشف
        let detections = self.parse_detections(&result)?;

        Ok(DetectionResult {
            boxes: detections,
            inference_time_ms: result.inference_time_ms,
        })
    }

    fn parse_detections(&self, result: &InferenceResult) -> Result<Vec<BoundingBox>, String> {
        // تنفيذ مبسط لتحليل الكشف
        let output = result.first_output()
            .ok_or("لا يوجد مخرج")?;

        let mut boxes = Vec::new();
        
        // افتراض أن المخرج هو [N, 6] (x, y, w, h, confidence, class)
        let data = &output.data;
        let num_detections = data.len() / 6;

        for i in 0..num_detections {
            let confidence = data[i * 6 + 4];
            if confidence >= self.confidence_threshold {
                boxes.push(BoundingBox {
                    x: data[i * 6],
                    y: data[i * 6 + 1],
                    width: data[i * 6 + 2],
                    height: data[i * 6 + 3],
                    confidence,
                    class_id: data[i * 6 + 5] as usize,
                    label: None,
                });
            }
        }

        Ok(boxes)
    }
}

/// صندوق إحاطة
#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub confidence: f64,
    pub class_id: usize,
    pub label: Option<String>,
}

/// نتيجة الكشف
#[derive(Debug, Clone)]
pub struct DetectionResult {
    pub boxes: Vec<BoundingBox>,
    pub inference_time_ms: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inference_options() {
        let options = InferenceOptions::new()
            .with_batch_size(32)
            .with_gpu()
            .with_timeout(1000);

        assert_eq!(options.batch_size, 32);
        assert!(options.use_gpu);
        assert_eq!(options.timeout_ms, Some(1000));
    }

    #[test]
    fn test_precision() {
        assert_eq!(Precision::Float32, Precision::Float32);
        assert_ne!(Precision::Float32, Precision::Float16);
    }

    #[test]
    fn test_inference_result() {
        let mut outputs = HashMap::new();
        outputs.insert("output".to_string(), ONNXTensor::vector(vec![1.0, 2.0, 3.0]));
        
        let result = InferenceResult::new(outputs);
        assert!(result.get("output").is_some());
        assert_eq!(result.to_vec("output"), Some(vec![1.0, 2.0, 3.0]));
    }
}
