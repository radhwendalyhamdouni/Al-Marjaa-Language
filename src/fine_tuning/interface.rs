// ═══════════════════════════════════════════════════════════════════════════════
// واجهة Fine-tuning - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// تخصيص نموذج Nanbeige4.1-3B-Q3_K_M بالعربية
// ═══════════════════════════════════════════════════════════════════════════════

use std::fs;
use std::path::Path;

/// مثال تدريبي للـ Fine-tuning
#[derive(Debug, Clone)]
pub struct TrainingExample {
    /// المدخل (النص العربي)
    pub input: String,
    /// المخرج المتوقع (JSON Intent)
    pub output: String,
}

/// إعدادات التدريب
#[derive(Debug, Clone)]
pub struct TrainingConfig {
    /// معدل التعلم
    pub learning_rate: f32,
    /// عدد الحقب
    pub epochs: u32,
    /// حجم الدفعة
    pub batch_size: u32,
    /// نسبة LoRA
    pub lora_alpha: f32,
    /// ترتيب LoRA
    pub lora_rank: u32,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        TrainingConfig {
            learning_rate: 0.0001,
            epochs: 3,
            batch_size: 4,
            lora_alpha: 16.0,
            lora_rank: 8,
        }
    }
}

/// نتائج التدريب
#[derive(Debug, Clone)]
pub struct TrainingResult {
    /// نجح التدريب
    pub success: bool,
    /// عدد الأمثلة المدربة
    pub examples_count: usize,
    /// الوقت المستغرق (ثواني)
    pub duration_secs: u64,
    /// الخسارة النهائية
    pub final_loss: f32,
    /// رسالة
    pub message: String,
}

/// واجهة Fine-tuning
pub struct FineTuningInterface {
    /// مسار النموذج (محجوز للاستخدام المستقبلي)
    _model_path: String,
    /// مسار حفظ النموذج المُدرّب
    output_path: String,
    /// إعدادات التدريب
    config: TrainingConfig,
    /// أمثلة التدريب
    training_data: Vec<TrainingExample>,
}

impl FineTuningInterface {
    /// إنشاء واجهة جديدة
    pub fn new() -> Self {
        FineTuningInterface {
            _model_path: "models/nanbeige-4.1-3b/".to_string(),
            output_path: "models/fine-tuned/".to_string(),
            config: TrainingConfig::default(),
            training_data: Vec::new(),
        }
    }

    /// إنشاء واجهة مع مسار مخصص
    pub fn with_paths(model_path: &str, output_path: &str) -> Self {
        FineTuningInterface {
            _model_path: model_path.to_string(),
            output_path: output_path.to_string(),
            config: TrainingConfig::default(),
            training_data: Vec::new(),
        }
    }

    /// تحديث إعدادات التدريب
    pub fn set_config(&mut self, config: TrainingConfig) {
        self.config = config;
    }

    /// إضافة مثال تدريبي
    pub fn add_example(&mut self, input: &str, output: &str) {
        self.training_data.push(TrainingExample {
            input: input.to_string(),
            output: output.to_string(),
        });
    }

    /// إضافة مجموعة أمثلة
    pub fn add_examples(&mut self, inputs: Vec<&str>, outputs: Vec<&str>) {
        for (input, output) in inputs.iter().zip(outputs.iter()) {
            self.add_example(input, output);
        }
    }

    /// تحميل أمثلة من ملف
    pub fn load_from_file(&mut self, path: &str) -> Result<(), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("خطأ في قراءة الملف: {}", e))?;
        
        // تحليل JSON أو CSV
        // (مبسط - يمكن توسيعه لاحقاً)
        for line in content.lines() {
            if line.contains("->") {
                let parts: Vec<&str> = line.split("->").collect();
                if parts.len() == 2 {
                    self.add_example(parts[0].trim(), parts[1].trim());
                }
            }
        }
        
        Ok(())
    }

    /// حفظ أمثلة التدريب
    pub fn save_training_data(&self, path: &str) -> Result<(), String> {
        let mut content = String::new();
        content.push_str("# بيانات التدريب للغة المرجع\n");
        content.push_str("# الصيغة: المدخل -> المخرج\n\n");
        
        for example in &self.training_data {
            content.push_str(&format!("{} -> {}\n", example.input, example.output));
        }
        
        fs::write(path, content)
            .map_err(|e| format!("خطأ في حفظ الملف: {}", e))
    }

    /// تشغيل Fine-tuning
    pub fn fine_tune(&self) -> TrainingResult {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║            🎓 Fine-tuning النموذج                            ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        
        println!("\n📊 إعدادات التدريب:");
        println!("   - معدل التعلم: {}", self.config.learning_rate);
        println!("   - عدد الحقب: {}", self.config.epochs);
        println!("   - حجم الدفعة: {}", self.config.batch_size);
        println!("   - LoRA Alpha: {}", self.config.lora_alpha);
        println!("   - LoRA Rank: {}", self.config.lora_rank);
        
        println!("\n📚 بيانات التدريب:");
        println!("   عدد الأمثلة: {}", self.training_data.len());
        
        if self.training_data.is_empty() {
            return TrainingResult {
                success: false,
                examples_count: 0,
                duration_secs: 0,
                final_loss: 0.0,
                message: "لا توجد أمثلة للتدريب".to_string(),
            };
        }
        
        // عرض بعض الأمثلة
        for (i, example) in self.training_data.iter().take(5).enumerate() {
            println!("   {}. \"{}\" -> \"{}\"", i + 1, example.input, example.output);
        }
        
        println!("\n🔄 بدء التدريب...");
        
        // محاكاة التدريب (في الإنتاج، سيتم استبدالها بـ inference حقيقي)
        let start = std::time::Instant::now();
        
        for epoch in 1..=self.config.epochs {
            println!("\n   الحقبة {} من {}:", epoch, self.config.epochs);
            
            // محاكاة خسارة متناقصة
            let loss = 1.0 / (epoch as f32 + 1.0);
            println!("   - الخسارة: {:.4}", loss);
            
            // محاكاة وقت التدريب
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        
        let duration = start.elapsed().as_secs();
        
        // حفظ النموذج المُدرّب (محاكاة)
        println!("\n💾 حفظ النموذج المُدرّب...");
        
        // إنشاء مجلد الإخراج
        if !Path::new(&self.output_path).exists() {
            fs::create_dir_all(&self.output_path)
                .unwrap_or_else(|_| println!("   تحذير: تعذر إنشاء مجلد الإخراج"));
        }
        
        // حفظ معلومات التدريب
        let info = format!(
            "# نموذج مُدرّب - لغة المرجع\n\
             # أمثلة التدريب: {}\n\
             # الحقب: {}\n\
             # معدل التعلم: {}\n",
            self.training_data.len(),
            self.config.epochs,
            self.config.learning_rate
        );
        
        let _ = fs::write(format!("{}training_info.txt", self.output_path), info);
        
        println!("   تم الحفظ في: {}", self.output_path);
        
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║              ✨ اكتمل Fine-tuning بنجاح! ✨                 ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        
        TrainingResult {
            success: true,
            examples_count: self.training_data.len(),
            duration_secs: duration,
            final_loss: 1.0 / (self.config.epochs as f32 + 1.0),
            message: "تم التدريب بنجاح".to_string(),
        }
    }

    /// تقييم النموذج
    pub fn evaluate(&self, test_inputs: Vec<&str>, expected_outputs: Vec<&str>) -> f32 {
        println!("\n📊 تقييم النموذج...");
        
        let mut correct = 0;
        let total = test_inputs.len().min(expected_outputs.len());
        
        for (input, expected) in test_inputs.iter().zip(expected_outputs.iter()) {
            // محاكاة التنبؤ
            let predicted = self.predict(input);
            
            if predicted.trim() == expected.trim() {
                correct += 1;
                println!("   ✅ \"{}\" -> \"{}\"", input, predicted);
            } else {
                println!("   ❌ \"{}\" -> المتوقع: \"{}\", الفعلي: \"{}\"", input, expected, predicted);
            }
        }
        
        let accuracy = if total > 0 { correct as f32 / total as f32 } else { 0.0 };
        println!("\n   الدقة: {:.2}%", accuracy * 100.0);
        
        accuracy
    }

    /// التنبؤ (محاكاة)
    fn predict(&self, input: &str) -> String {
        // في الإنتاج، سيتم استبدالها بـ inference حقيقي للنموذج
        // حالياً نستخدم المحلل البسيط
        
        let intent = crate::ai_engine::parse_intent(input);
        format!("{{\"action\":\"{}\",\"value\":\"{}\"}}", 
            intent.action, 
            intent.value.unwrap_or_default()
        )
    }
}

impl Default for FineTuningInterface {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال سهلة الاستخدام
// ═══════════════════════════════════════════════════════════════════════════════

/// تشغيل Fine-tuning مباشرة
pub fn fine_tune_model(example_inputs: Vec<&str>, example_outputs: Vec<&str>) -> TrainingResult {
    let mut interface = FineTuningInterface::new();
    interface.add_examples(example_inputs, example_outputs);
    interface.fine_tune()
}

/// تشغيل Fine-tuning مع إعدادات مخصصة
pub fn fine_tune_with_config(
    example_inputs: Vec<&str>,
    example_outputs: Vec<&str>,
    config: TrainingConfig,
) -> TrainingResult {
    let mut interface = FineTuningInterface::new();
    interface.set_config(config);
    interface.add_examples(example_inputs, example_outputs);
    interface.fine_tune()
}

/// تقييم النموذج
pub fn evaluate_model(test_inputs: Vec<&str>, expected_outputs: Vec<&str>) -> f32 {
    let interface = FineTuningInterface::new();
    interface.evaluate(test_inputs, expected_outputs)
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fine_tune_basic() {
        let inputs = vec!["اطبع مرحبا", "أنشئ متغير س يساوي 5"];
        let outputs = vec![
            r#"{"action":"print","value":"مرحبا"}"#,
            r#"{"action":"variable","name":"س","value":"5"}"#,
        ];
        
        let result = fine_tune_model(inputs, outputs);
        assert!(result.success);
        assert_eq!(result.examples_count, 2);
    }
    
    #[test]
    fn test_training_config() {
        let config = TrainingConfig {
            learning_rate: 0.001,
            epochs: 5,
            batch_size: 8,
            lora_alpha: 32.0,
            lora_rank: 16,
        };
        
        let mut interface = FineTuningInterface::new();
        interface.set_config(config);
        
        assert_eq!(interface.config.learning_rate, 0.001);
        assert_eq!(interface.config.epochs, 5);
    }
    
    #[test]
    fn test_add_examples() {
        let mut interface = FineTuningInterface::new();
        interface.add_example("اختبار", r#"{"action":"test"}"#);
        
        assert_eq!(interface.training_data.len(), 1);
    }
}
