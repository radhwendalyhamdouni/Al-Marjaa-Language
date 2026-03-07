// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات وحدة ONNX
// ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::{
    ONNXEngine, ONNXConfig, ONNXTensor, ONNXDataType, 
    ONNXShape, ONNXExporter, ExportOptions, LayerSpec,
    tensor_to_onnx, onnx_to_tensor,
};

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات ONNXTensor
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_tensor_creation() {
    let tensor = ONNXTensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
    assert_eq!(tensor.data.len(), 4);
    assert_eq!(tensor.shape, vec![2, 2]);
    assert_eq!(tensor.ndim(), 2);
}

#[test]
fn test_tensor_scalar() {
    let tensor = ONNXTensor::scalar(42.0);
    assert_eq!(tensor.data, vec![42.0]);
    assert!(tensor.shape.is_empty());
}

#[test]
fn test_tensor_vector() {
    let tensor = ONNXTensor::vector(vec![1.0, 2.0, 3.0]);
    assert_eq!(tensor.ndim(), 1);
    assert_eq!(tensor.len(), 3);
}

#[test]
fn test_tensor_matrix() {
    let tensor = ONNXTensor::matrix(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);
    assert_eq!(tensor.shape, vec![2, 3]);
    assert_eq!(tensor.get(&[0, 0]), Some(1.0));
    assert_eq!(tensor.get(&[1, 2]), Some(6.0));
}

#[test]
fn test_tensor_zeros() {
    let tensor = ONNXTensor::zeros(vec![3, 4]);
    assert_eq!(tensor.data.len(), 12);
    assert!(tensor.data.iter().all(|&x| x == 0.0));
}

#[test]
fn test_tensor_ones() {
    let tensor = ONNXTensor::ones(vec![2, 3]);
    assert_eq!(tensor.data.len(), 6);
    assert!(tensor.data.iter().all(|&x| x == 1.0));
}

#[test]
fn test_tensor_reshape() {
    let tensor = ONNXTensor::vector(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let reshaped = tensor.reshape(vec![2, 3]).unwrap();
    assert_eq!(reshaped.shape, vec![2, 3]);
    
    // Test invalid reshape
    let invalid = tensor.reshape(vec![2, 4]);
    assert!(invalid.is_err());
}

#[test]
fn test_tensor_get_set() {
    let mut tensor = ONNXTensor::zeros(vec![3, 3]);
    
    // Set value
    assert!(tensor.set(&[1, 1], 5.0));
    
    // Get value
    assert_eq!(tensor.get(&[1, 1]), Some(5.0));
    
    // Invalid indices
    assert!(!tensor.set(&[3, 0], 1.0));
    assert_eq!(tensor.get(&[3, 0]), None);
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات ONNXShape
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_shape_static() {
    let shape = ONNXShape::static_shape(vec![3, 224, 224]);
    assert!(!shape.is_dynamic);
    assert_eq!(shape.total_size(), Some(3 * 224 * 224));
    assert_eq!(shape.rank(), 3);
}

#[test]
fn test_shape_dynamic() {
    let shape = ONNXShape::dynamic_shape(vec![None, Some(3), Some(224), Some(224)]);
    assert!(shape.is_dynamic);
    assert!(shape.total_size().is_none());
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات ONNXDataType
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_data_type_sizes() {
    assert_eq!(ONNXDataType::Float.size_in_bytes(), 4);
    assert_eq!(ONNXDataType::Double.size_in_bytes(), 8);
    assert_eq!(ONNXDataType::Int32.size_in_bytes(), 4);
    assert_eq!(ONNXDataType::Int64.size_in_bytes(), 8);
}

#[test]
fn test_data_type_names() {
    assert_eq!(ONNXDataType::Float.onnx_name(), "tensor(float)");
    assert_eq!(ONNXDataType::Int64.onnx_name(), "tensor(int64)");
}

#[test]
fn test_data_type_arabic_names() {
    assert_eq!(ONNXDataType::Float.arabic_name(), "عائم_32");
    assert_eq!(ONNXDataType::Int64.arabic_name(), "صحيح_64");
    assert_eq!(ONNXDataType::Bool.arabic_name(), "منطقي");
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات ONNXEngine
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_engine_creation() {
    let engine = ONNXEngine::new();
    assert_eq!(engine.model_count(), 0);
    assert!(engine.active_model.is_none());
}

#[test]
fn test_engine_with_config() {
    let config = ONNXConfig::new()
        .with_threads(8)
        .with_optimization(3);
    
    let engine = ONNXEngine::with_config(config);
    assert_eq!(engine.model_count(), 0);
}

#[test]
fn test_engine_stats() {
    let engine = ONNXEngine::new();
    let stats = engine.get_stats();
    assert_eq!(stats.inference_count, 0);
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات ONNXExporter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_exporter_creation() {
    let exporter = ONNXExporter::new();
    assert_eq!(exporter.options.producer_name, "Al-Marjaa-Language");
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
fn test_layer_spec() {
    let layer = LayerSpec {
        name: "dense1".to_string(),
        layer_type: "dense".to_string(),
        input_size: 784,
        output_size: 128,
        activation: Some("relu".to_string()),
    };
    
    assert_eq!(layer.input_size, 784);
    assert_eq!(layer.output_size, 128);
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات التكامل
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_tensor_to_onnx_conversion() {
    let data = vec![1.0, 2.0, 3.0, 4.0];
    let shape = vec![2, 2];
    
    let onnx_tensor = tensor_to_onnx(&data, &shape);
    assert_eq!(onnx_tensor.data, data);
    assert_eq!(onnx_tensor.shape, shape);
}

#[test]
fn test_onnx_to_tensor_conversion() {
    let tensor = ONNXTensor::new(vec![1.0, 2.0, 3.0], vec![3]);
    let (data, shape) = onnx_to_tensor(&tensor);
    
    assert_eq!(data, vec![1.0, 2.0, 3.0]);
    assert_eq!(shape, vec![3]);
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات Lexer للكلمات المفتاحية ONNX
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn test_onnx_keywords_in_lexer() {
    use almarjaa::Lexer;
    use almarjaa::lexer::tokens::TokenType;
    
    // Test ONNX keyword
    let mut lexer = Lexer::new("أونكس");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0].token_type, TokenType::ONNX));
    
    // Test model keyword
    let mut lexer = Lexer::new("نموذج");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0].token_type, TokenType::Model));
    
    // Test tensor keyword
    let mut lexer = Lexer::new("موتر");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0].token_type, TokenType::Tensor));
    
    // Test layer keyword
    let mut lexer = Lexer::new("طبقة");
    let tokens = lexer.tokenize().unwrap();
    assert!(matches!(tokens[0].token_type, TokenType::Layer));
}

#[test]
fn test_onnx_code_snippet() {
    use almarjaa::Lexer;
    use almarjaa::lexer::tokens::TokenType;
    
    let code = r#"
        نموذج شبكة = أونكس.حمّل("model.onnx")
        موتر مدخل = موتر.أصفار([1, 3, 224, 224])
        موتر مخرج = شبكة.استدل(مدخل)
    "#;
    
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    
    // Verify ONNX keywords are recognized
    let token_types: Vec<_> = tokens.iter()
        .filter_map(|t| match &t.token_type {
            TokenType::ONNX | TokenType::Model | TokenType::Tensor | 
            TokenType::Load | TokenType::Infer => Some(t.token_type.clone()),
            _ => None,
        })
        .collect();
    
    assert!(!token_types.is_empty());
}
