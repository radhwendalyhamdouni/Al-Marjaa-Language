//! ═══════════════════════════════════════════════════════════════════════════════
//! اختبارات تكامل الذكاء الاصطناعي الشاملة
//! Comprehensive AI Integration Tests for Al-Marjaa Language
//! ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::Interpreter;

#[cfg(test)]
mod ai_integration_tests {
    use super::*;

    fn create_interpreter() -> Interpreter {
        Interpreter::new()
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات Tensor
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_tensor_creation() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير موتر = Tensor([[١، ٢، ٣]، [٤، ٥، ٦]])؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tensor_zeros() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير موتر = zeros([٣، ٣])؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tensor_ones() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير موتر = ones([٢، ٤])؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tensor_random() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير موتر = random([٣، ٣])؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tensor_operations() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير أ = Tensor([[١، ٢]، [٣، ٤]])؛
            متغير ب = Tensor([[٥، ٦]، [٧، ٨]])؛
            متغير مجموع = أ + ب؛
            متغير فرق = أ - ب؛
            متغير ضرب = أ * ب؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tensor_matmul() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير أ = Tensor([[١، ٢]، [٣، ٤]])؛
            متغير ب = Tensor([[٥، ٦]، [٧، ٨]])؛
            متغير نتيجة = matmul(أ، ب)؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tensor_reshape() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير موتر = Tensor([[١، ٢، ٣، ٤]])؛
            متغير معاد_تشكيله = reshape(موتر، [٢، ٢])؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tensor_transpose() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير موتر = Tensor([[١، ٢، ٣]، [٤، ٥، ٦]])؛
            متغير منقول = transpose(موتر)؛
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات NeuralNetwork
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_neural_network_creation() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير شبكة = NeuralNetwork()؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_neural_network_add_layer() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير شبكة = NeuralNetwork()؛
            شبكة.أضف_طبقة(١٢٨، "relu")؛
            شبكة.أضف_طبقة(٦٤، "relu")؛
            شبكة.أضف_طبقة(١٠، "softmax")؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_neural_network_forward() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير شبكة = NeuralNetwork()؛
            شبكة.أضف_طبقة(٣٢، "relu")؛
            شبكة.أضف_طبقة(١٠، "softmax")؛
            
            متغير مدخل = random([١، ٣٢])؛
            متغير مخرج = شبكة.مرر(مدخل)؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_neural_network_training() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير شبكة = NeuralNetwork()؛
            شبكة.أضف_طبقة(١٦، "relu")؛
            شبكة.أضف_طبقة(١، "sigmoid")؛
            
            متغير بيانات_تدريب = [
                [Tensor([[٠، ٠]])، Tensor([[٠]])]،
                [Tensor([[٠، ١]])، Tensor([[١]])]،
                [Tensor([[١، ٠]])، Tensor([[١]])]،
                [Tensor([[١، ١]])، Tensor([[٠]])]
            ]؛
            
            شبكة.درّب(بيانات_تدريب، ١٠٠، ٠.١)؛
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات دوال التنشيط
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_activation_relu() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير موتر = Tensor([[-٢، -١، ٠، ١، ٢]])؛
            متغير مفعّل = relu(موتر)؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_activation_sigmoid() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير موتر = Tensor([[-٥، ٠، ٥]])؛
            متغير مفعّل = sigmoid(موتر)؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_activation_tanh() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير موتر = Tensor([[-٣، ٠، ٣]])؛
            متغير مفعّل = tanh(موتر)؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_activation_softmax() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير موتر = Tensor([[١، ٢، ٣]])؛
            متغير مفعّل = softmax(موتر)؛
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات دوال الخسارة
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_loss_mse() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير تنبؤ = Tensor([[٠.٥، ٠.٨]])؛
            متغير هدف = Tensor([[١، ١]])؛
            متغير خسارة = mse(تنبؤ، هدف)؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_loss_cross_entropy() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير تنبؤ = Tensor([[٠.٩، ٠.٠٥، ٠.٠٥]])؛
            متغير هدف = Tensor([[١، ٠، ٠]])؛
            متغير خسارة = cross_entropy(تنبؤ، هدف)؛
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات المحسنات
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_optimizer_sgd() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير معاملات = Tensor([[١، ٢، ٣]])؛
            متغير تدرجات = Tensor([[٠.١، ٠.٢، ٠.٣]])؛
            متغير معدل_تعلم = ٠.٠١؛
            
            متغير محسن = SGD(معدل_تعلم)؛
            معاملات = محسن.حدّث(معاملات، تدرجات)؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_optimizer_adam() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير معاملات = Tensor([[١، ٢، ٣]])؛
            متغير تدرجات = Tensor([[٠.١، ٠.٢، ٠.٣]])؛
            
            متغير محسن = Adam(٠.٠٠١)؛
            معاملات = محسن.حدّث(معاملات، تدرجات)؛
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات التدرج التلقائي
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_autograd_basic() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير س = Tensor([[٢]])؛
            متغير ص = س * س + ٣ * س + ١؛
            
            متغير تدرج = backward(ص)؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_autograd_chain() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير أ = Tensor([[٣]])؛
            متغير ب = أ * ٢؛
            متغير ج = ب + ٥؛
            متجر د = ج * ج؛
            
            متغير تدرج = backward(د)؛
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات معالجة البيانات
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_dataloader_creation() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير بيانات = [
                [Tensor([[١، ٢]])، Tensor([[٣]])]،
                [Tensor([[٤، ٥]])، Tensor([[٦]])]
            ]؛
            متغير محمّل = DataLoader(بيانات، ٢)؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_data_normalization() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير بيانات = Tensor([[١٠، ٢٠، ٣٠، ٤٠، ٥٠]])؛
            متوسط_القيمة = mean(بيانات)؛
            متغير الانحراف = std(بيانات)؛
            متغير مُطبّع = (بيانات - متوسط_القيمة) / الانحراف؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_train_test_split() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير بيانات = [١، ٢، ٣، ٤، ٥، ٦، ٧، ٨، ٩، ١٠]؛
            متغير [تدريب، اختبار] = split_data(بيانات، ٠.٨)؛
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات نماذج جاهزة
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_mlp_model() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير نموذج = MLP(١٠، [٦٤، ٣٢]، ٣)؛
            متغير مدخل = random([١، ١٠])؛
            متغير مخرج = نموذج.مرر(مدخل)؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cnn_model() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير نموذج = CNN()؛
            نموذج.أضف_طبقة_التلافيف(٣٢، ٣، "relu")؛
            نموذج.أضف_طبقة_التجميع(٢)؛
            نموذج.أضف_طبقة_متصلة(١٠)؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_rnn_model() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير نموذج = RNN(١٠، ٣٢، ١)؛
            متغير تسلسل = random([٥، ١٠])؛
            متغير مخرج = نموذج.مرر(تسلسل)؛
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات حفظ وتحميل النماذج
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_save_load_model() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير شبكة = NeuralNetwork()؛
            شبكة.أضف_طبقة(١٦، "relu")؛
            شبكة.أضف_طبقة(٣، "softmax")؛
            
            # حفظ النموذج
            save_model(شبكة، "model.json")؛
            
            # تحميل النموذج
            متغير نموذج_محمل = load_model("model.json")؛
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات GPU
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_gpu_tensor() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            # إنشاء موتر على GPU
            متغير موتر = Tensor([[١، ٢، ٣]]، "gpu")؛
            
            # عملية على GPU
            متغير نتيجة = موتر * ٢؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_gpu_available() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير متاح = gpu_available()؛
            اطبع("GPU متاح: " + متاح)؛
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات Regularization
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_dropout() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير موتر = Tensor([[١، ٢، ٣، ٤، ٥]])؛
            متغير مُطبّق = dropout(موتر، ٠.٣)؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_batch_norm() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير موتر = Tensor([[١، ٢، ٣]، [٤، ٥، ٦]])؛
            متغير مُطبّق = batch_norm(موتر)؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_l2_regularization() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير معاملات = Tensor([[١، ٢، ٣]])؛
            متغير عقوبة = l2_penalty(معاملات، ٠.٠١)؛
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات شاملة للتدريب
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_complete_training_pipeline() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            # إنشاء بيانات التدريب
            متغير بيانات_تدريب = [
                [Tensor([[٠، ٠]])، Tensor([[٠]])]،
                [Tensor([[٠، ١]])، Tensor([[١]])]،
                [Tensor([[١، ٠]])، Tensor([[١]])]،
                [Tensor([[١، ١]])، Tensor([[٠]])]
            ]؛
            
            # إنشاء النموذج
            متغير شبكة = NeuralNetwork()؛
            شبكة.أضف_طبقة(٨، "relu")؛
            شبكة.أضف_طبقة(٤، "relu")؛
            شبكة.أضف_طبقة(١، "sigmoid")؛
            
            # إنشاء المحسن
            متغير محسن = Adam(٠.٠١)؛
            
            # التدريب
            شبكة.درّب_مع_محسن(بيانات_تدريب، ٥٠٠، محسن)؛
            
            # الاختبار
            متغير تنبؤ = شبكة.مرر(Tensor([[١، ١]]))؛
            اطبع("تنبؤ XOR [١، ١]: " + تنبؤ)؛
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات الأداء
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_tensor_operations_performance() {
        let mut interp = create_interpreter();
        
        let start = std::time::Instant::now();
        let result = interp.run(r#"
            متغير أ = random([١٠٠، ١٠٠])؛
            متغير ب = random([١٠٠، ١٠٠])؛
            
            لكل _ في مدى(١، ١٠) {
                متغير ج = matmul(أ، ب)؛
            }
        "#);
        let duration = start.elapsed();
        
        println!("✓ 10 matrix multiplications (100x100) in {:?}", duration);
        assert!(result.is_ok());
    }

    #[test]
    fn test_neural_network_inference_performance() {
        let mut interp = create_interpreter();
        
        let start = std::time::Instant::now();
        let result = interp.run(r#"
            متغير شبكة = NeuralNetwork()؛
            شبكة.أضف_طبقة(١٢٨، "relu")؛
            شبكة.أضف_طبقة(٦٤، "relu")؛
            شبكة.أضف_طبقة(١٠، "softmax")؛
            
            متغير مدخل = random([١، ١٢٨])؛
            
            لكل _ في مدى(١، ١٠٠) {
                متغير مخرج = شبكة.مرر(مدخل)؛
            }
        "#);
        let duration = start.elapsed();
        
        println!("✓ 100 forward passes in {:?}", duration);
        assert!(result.is_ok());
    }
}
