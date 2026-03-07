// ═══════════════════════════════════════════════════════════════════════════════
// تصدير النماذج إلى ONNX
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

use super::types::ONNXDataType;

// ═══════════════════════════════════════════════════════════════════════════════
// ONNX Exporter
// ═══════════════════════════════════════════════════════════════════════════════

/// مصدّر النماذج إلى ONNX
pub struct ONNXExporter {
    /// خيارات التصدير
    options: ExportOptions,
}

/// خيارات التصدير
#[derive(Debug, Clone)]
pub struct ExportOptions {
    /// إصدار ONNX
    pub onnx_version: String,
    /// اسم المنتج
    pub producer_name: String,
    /// وصف النموذج
    pub description: Option<String>,
    /// تحسين الرسم البياني
    pub optimize: bool,
    /// تضمين الأوزان
    pub include_weights: bool,
    /// نوع البيانات الافتراضي
    pub default_data_type: ONNXDataType,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            onnx_version: "1.15.0".to_string(),
            producer_name: "Al-Marjaa-Language".to_string(),
            description: None,
            optimize: true,
            include_weights: true,
            default_data_type: ONNXDataType::Float,
        }
    }
}

impl ExportOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn with_optimization(mut self, optimize: bool) -> Self {
        self.optimize = optimize;
        self
    }
}

/// نتيجة التصدير
#[derive(Debug, Clone)]
pub struct ExportResult {
    /// هل نجح التصدير؟
    pub success: bool,
    /// مسار الملف الناتج
    pub output_path: String,
    /// حجم الملف بالبايت
    pub file_size: u64,
    /// رسالة (نجاح أو خطأ)
    pub message: String,
    /// وقت التصدير (مللي ثانية)
    pub export_time_ms: f64,
}

// ═══════════════════════════════════════════════════════════════════════════════
// Layer Definition for Export
// ═══════════════════════════════════════════════════════════════════════════════

/// تعريف طبقة للتصدير
#[derive(Debug, Clone)]
pub struct LayerDefinition {
    /// اسم الطبقة
    pub name: String,
    /// نوع الطبقة
    pub op_type: String,
    /// المدخلات
    pub inputs: Vec<String>,
    /// المخرجات
    pub outputs: Vec<String>,
    /// الخصائص
    pub attributes: HashMap<String, AttributeValue>,
}

/// قيمة الخاصية
#[derive(Debug, Clone)]
pub enum AttributeValue {
    Int(i64),
    Float(f64),
    String(String),
    Ints(Vec<i64>),
    Floats(Vec<f64>),
    Strings(Vec<String>),
}

// ═══════════════════════════════════════════════════════════════════════════════
// ONNX Graph Builder
// ═══════════════════════════════════════════════════════════════════════════════

/// منشئ الرسم البياني ONNX
pub struct ONNXGraphBuilder {
    /// اسم الرسم البياني
    pub name: String,
    /// الطبقات
    pub nodes: Vec<LayerDefinition>,
    /// المدخلات
    pub inputs: Vec<ValueInfo>,
    /// المخرجات
    pub outputs: Vec<ValueInfo>,
    /// القيم الأولية (الأوزان)
    pub initializers: Vec<Initializer>,
}

/// معلومات قيمة
#[derive(Debug, Clone)]
pub struct ValueInfo {
    pub name: String,
    pub data_type: ONNXDataType,
    pub shape: Vec<i64>,
}

/// القيم الأولية
#[derive(Debug, Clone)]
pub struct Initializer {
    pub name: String,
    pub data_type: ONNXDataType,
    pub shape: Vec<i64>,
    pub data: Vec<f64>,
}

impl ONNXGraphBuilder {
    /// إنشاء منشئ جديد
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            nodes: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            initializers: Vec::new(),
        }
    }

    /// إضافة مدخل
    pub fn add_input(&mut self, name: &str, data_type: ONNXDataType, shape: Vec<i64>) {
        self.inputs.push(ValueInfo {
            name: name.to_string(),
            data_type,
            shape,
        });
    }

    /// إضافة مخرج
    pub fn add_output(&mut self, name: &str, data_type: ONNXDataType, shape: Vec<i64>) {
        self.outputs.push(ValueInfo {
            name: name.to_string(),
            data_type,
            shape,
        });
    }

    /// إضافة طبقة (عقدة)
    pub fn add_node(&mut self, node: LayerDefinition) {
        self.nodes.push(node);
    }

    /// إضافة وزن أولي
    pub fn add_initializer(&mut self, name: &str, data_type: ONNXDataType, shape: Vec<i64>, data: Vec<f64>) {
        self.initializers.push(Initializer {
            name: name.to_string(),
            data_type,
            shape,
            data,
        });
    }

    /// إضافة طبقة كثيفة (Dense/Linear)
    pub fn add_dense(&mut self, name: &str, input: &str, output: &str, 
                     in_features: usize, out_features: usize,
                     weights: Vec<f64>, bias: Vec<f64>) {
        // إضافة عقدة MatMul
        let weight_name = format!("{}.weight", name);
        let matmul_output = format!("{}.matmul_output", name);
        
        self.add_initializer(&weight_name, ONNXDataType::Float, 
                           vec![in_features as i64, out_features as i64], weights);
        
        self.add_node(LayerDefinition {
            name: format!("{}.matmul", name),
            op_type: "MatMul".to_string(),
            inputs: vec![input.to_string(), weight_name],
            outputs: vec![matmul_output.clone()],
            attributes: HashMap::new(),
        });

        // إضافة عقدة Add (bias)
        let bias_name = format!("{}.bias", name);
        self.add_initializer(&bias_name, ONNXDataType::Float, 
                           vec![out_features as i64], bias);
        
        self.add_node(LayerDefinition {
            name: format!("{}.add", name),
            op_type: "Add".to_string(),
            inputs: vec![matmul_output, bias_name],
            outputs: vec![output.to_string()],
            attributes: HashMap::new(),
        });
    }

    /// إضافة طبقة تنشيط ReLU
    pub fn add_relu(&mut self, name: &str, input: &str, output: &str) {
        self.add_node(LayerDefinition {
            name: name.to_string(),
            op_type: "Relu".to_string(),
            inputs: vec![input.to_string()],
            outputs: vec![output.to_string()],
            attributes: HashMap::new(),
        });
    }

    /// إضافة طبقة تنشيط Sigmoid
    pub fn add_sigmoid(&mut self, name: &str, input: &str, output: &str) {
        self.add_node(LayerDefinition {
            name: name.to_string(),
            op_type: "Sigmoid".to_string(),
            inputs: vec![input.to_string()],
            outputs: vec![output.to_string()],
            attributes: HashMap::new(),
        });
    }

    /// إضافة طبقة تنشيط Tanh
    pub fn add_tanh(&mut self, name: &str, input: &str, output: &str) {
        self.add_node(LayerDefinition {
            name: name.to_string(),
            op_type: "Tanh".to_string(),
            inputs: vec![input.to_string()],
            outputs: vec![output.to_string()],
            attributes: HashMap::new(),
        });
    }

    /// إضافة طبقة Softmax
    pub fn add_softmax(&mut self, name: &str, input: &str, output: &str, axis: i64) {
        let mut attrs = HashMap::new();
        attrs.insert("axis".to_string(), AttributeValue::Int(axis));
        
        self.add_node(LayerDefinition {
            name: name.to_string(),
            op_type: "Softmax".to_string(),
            inputs: vec![input.to_string()],
            outputs: vec![output.to_string()],
            attributes: attrs,
        });
    }

    /// إضافة طبقة Conv2D
    pub fn add_conv2d(&mut self, name: &str, input: &str, output: &str,
                      in_channels: usize, out_channels: usize, 
                      kernel_size: usize, weights: Vec<f64>, bias: Option<Vec<f64>>) {
        let weight_name = format!("{}.weight", name);
        let conv_output = format!("{}.conv_output", name);
        
        self.add_initializer(&weight_name, ONNXDataType::Float,
                           vec![out_channels as i64, in_channels as i64, 
                                kernel_size as i64, kernel_size as i64], weights);
        
        let mut inputs = vec![input.to_string(), weight_name];
        
        if let Some(b) = bias {
            let bias_name = format!("{}.bias", name);
            self.add_initializer(&bias_name, ONNXDataType::Float,
                               vec![out_channels as i64], b);
            inputs.push(bias_name);
        }
        
        self.add_node(LayerDefinition {
            name: name.to_string(),
            op_type: "Conv".to_string(),
            inputs,
            outputs: vec![conv_output.clone()],
            attributes: HashMap::new(),
        });
        
        // إعادة تسمية المخرج
        self.add_node(LayerDefinition {
            name: format!("{}.identity", name),
            op_type: "Identity".to_string(),
            inputs: vec![conv_output],
            outputs: vec![output.to_string()],
            attributes: HashMap::new(),
        });
    }

    /// إضافة طبقة MaxPool2D
    pub fn add_maxpool2d(&mut self, name: &str, input: &str, output: &str,
                         kernel_size: usize, stride: usize) {
        let mut attrs = HashMap::new();
        attrs.insert("kernel_shape".to_string(), 
                    AttributeValue::Ints(vec![kernel_size as i64, kernel_size as i64]));
        attrs.insert("strides".to_string(), 
                    AttributeValue::Ints(vec![stride as i64, stride as i64]));
        
        self.add_node(LayerDefinition {
            name: name.to_string(),
            op_type: "MaxPool".to_string(),
            inputs: vec![input.to_string()],
            outputs: vec![output.to_string()],
            attributes: attrs,
        });
    }

    /// إضافة طبقة Dropout
    pub fn add_dropout(&mut self, name: &str, input: &str, output: &str, ratio: f64) {
        let mut attrs = HashMap::new();
        attrs.insert("ratio".to_string(), AttributeValue::Float(ratio));
        
        self.add_node(LayerDefinition {
            name: name.to_string(),
            op_type: "Dropout".to_string(),
            inputs: vec![input.to_string()],
            outputs: vec![output.to_string()],
            attributes: attrs,
        });
    }

    /// إضافة طبقة BatchNorm
    pub fn add_batchnorm(&mut self, name: &str, input: &str, output: &str,
                        num_features: usize, scale: Vec<f64>, bias: Vec<f64>,
                        mean: Vec<f64>, var: Vec<f64>) {
        let scale_name = format!("{}.scale", name);
        let bias_name = format!("{}.bias", name);
        let mean_name = format!("{}.mean", name);
        let var_name = format!("{}.var", name);
        
        self.add_initializer(&scale_name, ONNXDataType::Float, 
                           vec![num_features as i64], scale);
        self.add_initializer(&bias_name, ONNXDataType::Float, 
                           vec![num_features as i64], bias);
        self.add_initializer(&mean_name, ONNXDataType::Float, 
                           vec![num_features as i64], mean);
        self.add_initializer(&var_name, ONNXDataType::Float, 
                           vec![num_features as i64], var);
        
        self.add_node(LayerDefinition {
            name: name.to_string(),
            op_type: "BatchNormalization".to_string(),
            inputs: vec![input.to_string(), scale_name, bias_name, mean_name, var_name],
            outputs: vec![output.to_string()],
            attributes: HashMap::new(),
        });
    }

    /// إضافة طبقة Flatten
    pub fn add_flatten(&mut self, name: &str, input: &str, output: &str, axis: i64) {
        let mut attrs = HashMap::new();
        attrs.insert("axis".to_string(), AttributeValue::Int(axis));
        
        self.add_node(LayerDefinition {
            name: name.to_string(),
            op_type: "Flatten".to_string(),
            inputs: vec![input.to_string()],
            outputs: vec![output.to_string()],
            attributes: attrs,
        });
    }

    /// إضافة Reshape
    pub fn add_reshape(&mut self, name: &str, input: &str, output: &str, shape: Vec<i64>) {
        let shape_name = format!("{}.shape", name);
        self.add_initializer(&shape_name, ONNXDataType::Int64, 
                           vec![shape.len() as i64], shape.iter().map(|&x| x as f64).collect());
        
        self.add_node(LayerDefinition {
            name: name.to_string(),
            op_type: "Reshape".to_string(),
            inputs: vec![input.to_string(), shape_name],
            outputs: vec![output.to_string()],
            attributes: HashMap::new(),
        });
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Exporter Implementation
// ═══════════════════════════════════════════════════════════════════════════════

impl ONNXExporter {
    /// إنشاء مصدّر جديد
    pub fn new() -> Self {
        Self {
            options: ExportOptions::default(),
        }
    }

    /// إنشاء مصدّر مع خيارات
    pub fn with_options(options: ExportOptions) -> Self {
        Self { options }
    }

    /// تصدير رسم بياني إلى ملف ONNX
    pub fn export_graph(&self, graph: &ONNXGraphBuilder, output_path: &str) -> Result<ExportResult, String> {
        use std::time::Instant;
        let start = Instant::now();

        // إنشاء محتوى ONNX (تنفيذ مبسط)
        let content = self.serialize_graph(graph)?;

        // كتابة الملف
        let mut file = std::fs::File::create(output_path)
            .map_err(|e| format!("فشل إنشاء الملف: {}", e))?;

        file.write_all(content.as_bytes())
            .map_err(|e| format!("فشل كتابة الملف: {}", e))?;

        let file_size = std::fs::metadata(output_path)
            .map(|m| m.len())
            .unwrap_or(0);

        Ok(ExportResult {
            success: true,
            output_path: output_path.to_string(),
            file_size,
            message: "تم التصدير بنجاح".to_string(),
            export_time_ms: start.elapsed().as_millis() as f64,
        })
    }

    /// تصدير شبكة عصبية بسيطة
    pub fn export(&self, name: &str, layers: &[super::LayerSpec], output_path: &str) -> Result<(), String> {
        let mut graph = ONNXGraphBuilder::new(name);

        // إضافة المدخل
        if let Some(first) = layers.first() {
            graph.add_input("input", ONNXDataType::Float, 
                          vec![-1, first.input_size as i64]);
        }

        let mut current_input = "input".to_string();

        for (i, layer) in layers.iter().enumerate() {
            let output_name = format!("layer_{}_output", i);

            match layer.layer_type.as_str() {
                "dense" | "linear" => {
                    // إنشاء أوزان عشوائية للتصدير
                    let weight_size = layer.input_size * layer.output_size;
                    let weights: Vec<f64> = (0..weight_size).map(|i| (i as f64 % 10.0) / 10.0).collect();
                    let bias: Vec<f64> = (0..layer.output_size).map(|i| (i as f64 % 5.0) / 10.0).collect();

                    graph.add_dense(&format!("dense_{}", i), &current_input, &output_name,
                                  layer.input_size, layer.output_size, weights, bias);
                }
                "relu" => {
                    graph.add_relu(&format!("relu_{}", i), &current_input, &output_name);
                }
                "sigmoid" => {
                    graph.add_sigmoid(&format!("sigmoid_{}", i), &current_input, &output_name);
                }
                "tanh" => {
                    graph.add_tanh(&format!("tanh_{}", i), &current_input, &output_name);
                }
                "softmax" => {
                    graph.add_softmax(&format!("softmax_{}", i), &current_input, &output_name, -1);
                }
                "flatten" => {
                    graph.add_flatten(&format!("flatten_{}", i), &current_input, &output_name, 1);
                }
                _ => {}
            }

            current_input = output_name;
        }

        // إضافة المخرج
        if let Some(last) = layers.last() {
            graph.add_output("output", ONNXDataType::Float, 
                           vec![-1, last.output_size as i64]);
        }

        let _result = self.export_graph(&graph, output_path)?;
        Ok(())
    }

    /// تسلسل الرسم البياني إلى نص ONNX (تنفيذ مبسط)
    fn serialize_graph(&self, graph: &ONNXGraphBuilder) -> Result<String, String> {
        let mut output = String::new();

        output.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
        output.push_str("<!-- ONNX Model exported from Al-Marjaa Language -->\n");
        output.push_str(&format!("<!-- Producer: {} -->\n", self.options.producer_name));
        output.push_str(&format!("<!-- ONNX Version: {} -->\n", self.options.onnx_version));
        output.push_str(&format!("<!-- Model: {} -->\n", graph.name));
        output.push_str("\n");

        // معلومات النموذج
        output.push_str(&format!("Model: {}\n", graph.name));
        output.push_str(&format!("Inputs: {}\n", graph.inputs.len()));
        output.push_str(&format!("Outputs: {}\n", graph.outputs.len()));
        output.push_str(&format!("Nodes: {}\n", graph.nodes.len()));
        output.push_str(&format!("Initializers: {}\n", graph.initializers.len()));
        output.push_str("\n");

        // المدخلات
        output.push_str("Inputs:\n");
        for input in &graph.inputs {
            output.push_str(&format!("  - {}: {:?} {:?}\n", 
                          input.name, input.data_type, input.shape));
        }
        output.push_str("\n");

        // المخرجات
        output.push_str("Outputs:\n");
        for output_info in &graph.outputs {
            output.push_str(&format!("  - {}: {:?} {:?}\n", 
                          output_info.name, output_info.data_type, output_info.shape));
        }
        output.push_str("\n");

        // الطبقات
        output.push_str("Nodes:\n");
        for node in &graph.nodes {
            output.push_str(&format!("  - {} ({})\n", node.name, node.op_type));
            output.push_str(&format!("    Inputs: {:?}\n", node.inputs));
            output.push_str(&format!("    Outputs: {:?}\n", node.outputs));
        }

        Ok(output)
    }
}

impl Default for ONNXExporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_builder() {
        let mut graph = ONNXGraphBuilder::new("test_model");
        graph.add_input("input", ONNXDataType::Float, vec![-1, 784]);
        graph.add_output("output", ONNXDataType::Float, vec![-1, 10]);
        
        assert_eq!(graph.inputs.len(), 1);
        assert_eq!(graph.outputs.len(), 1);
    }

    #[test]
    fn test_export_options() {
        let options = ExportOptions::new()
            .with_description("Test model")
            .with_optimization(true);
        
        assert!(options.description.is_some());
        assert!(options.optimize);
    }

    #[test]
    fn test_exporter() {
        let exporter = ONNXExporter::new();
        assert_eq!(exporter.options.producer_name, "Al-Marjaa-Language");
    }
}
