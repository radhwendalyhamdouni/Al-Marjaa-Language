// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات حقيقية مع نموذج Qwen - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

/// نموذج Qwen
struct QwenModel {
    model_path: PathBuf,
    cli_path: PathBuf,
}

impl QwenModel {
    fn new() -> Self {
        Self {
            model_path: PathBuf::from("models/qwen2.5-0.5b-instruct-q8_0.gguf"),
            cli_path: PathBuf::from("llama_cpp/build/bin/llama-cli"),
        }
    }

    /// تشغيل الاستدلال
    fn infer(&self, prompt: &str, max_tokens: usize) -> Result<String, String> {
        let formatted_prompt = format!(
            "<|im_start|>system\nأنت مساعد برمجي عربي متخصص في تحويل النص العربي إلى كود بلغة المرجع. \
            قواعد التحويل:\n\
            - المتغيرات: 'أنشئ متغير [اسم] يساوي [قيمة]' → متغير [اسم] = [قيمة]؛\n\
            - الطباعة: 'اطبع [نص]' → اطبع(\"[نص]\")؛\n\
            - الشرط: 'إذا كان [شرط] [إجراء]' → إذا [شرط] {{ [إجراء] }}\n\
            - الدوال: 'أنشئ دالة [اسم]' → دالة [اسم]() {{ }}\n\
            - الحلقات: 'كرر [عدد] مرات [إجراء]' → طالما ع < [عدد] {{ [إجراء] }}\n\
            أعد فقط الكود بدون شرح إضافي.<|im_end|>\n\
            <|im_start|>user\n{}<|im_end|>\n\
            <|im_start|>assistant\n",
            prompt
        );

        let output = Command::new(&self.cli_path)
            .args([
                "-m", self.model_path.to_str().unwrap(),
                "-p", &formatted_prompt,
                "-n", &max_tokens.to_string(),
                "--temp", "0.3",
                "--top-p", "0.9",
                "--no-display-prompt",
                "-c", "2048",
                "--no-warmup",
                "-ngl", "0",
            ])
            .output()
            .map_err(|e| format!("فشل تشغيل النموذج: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }

    /// اختبار تحويل النص العربي إلى كود
    fn test_text_to_code(&self, input: &str) -> (String, u64) {
        let start = Instant::now();
        let result = self.infer(input, 100);
        let duration = start.elapsed().as_millis() as u64;
        
        match result {
            Ok(output) => (output, duration),
            Err(e) => (format!("خطأ: {}", e), duration),
        }
    }
}

fn main() {
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("        🧪 اختبارات حقيقية مع نموذج Qwen 2.5 - لغة المرجع");
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!();

    let model = QwenModel::new();

    // التحقق من وجود النموذج
    if !model.model_path.exists() {
        println!("❌ النموذج غير موجود: {:?}", model.model_path);
        println!("📥 قم بتنزيل النموذج من:");
        println!("   https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF");
        return;
    }

    if !model.cli_path.exists() {
        println!("❌ llama-cli غير موجود: {:?}", model.cli_path);
        return;
    }

    println!("✅ النموذج موجود: {:?}", model.model_path);
    println!("✅ llama-cli موجود: {:?}", model.cli_path);
    println!();

    // قائمة الاختبارات
    let tests = vec![
        // اختبارات المتغيرات
        ("أنشئ متغير س يساوي 10", "متغير س = 10؛"),
        ("أنشئ متغير اسم يساوي \"أحمد\"", "متغير اسم = \"أحمد\"؛"),
        ("أنشئ متغير رقم يساوي 100", "متغير رقم = 100؛"),
        
        // اختبارات الطباعة
        ("اطبع مرحبا بالعالم", "اطبع(\"مرحبا بالعالم\")؛"),
        ("اطبع رسالة ترحيب", "اطبع(\"رسالة ترحيب\")؛"),
        
        // اختبارات الشرط
        ("إذا كان س أكبر من 10 اطبع كبير", "إذا س > 10 {"),
        ("إذا كان الرقم يساوي 5 اطبع صغير", "إذا الرقم == 5 {"),
        
        // اختبارات الدوال
        ("أنشئ دالة تجمع رقمين", "دالة"),
        ("أنشئ دالة تضرب رقمين", "دالة"),
        
        // اختبارات الحلقات
        ("كرر طباعة مرحبا 3 مرات", "طالما"),
        ("كرر 5 مرات اطبع رقم", "طالما"),
    ];

    println!("───────────────────────────────────────────────────────────────────────────");
    println!("                    🔄 بدء الاختبارات");
    println!("───────────────────────────────────────────────────────────────────────────");
    println!();

    let mut passed = 0;
    let mut failed = 0;
    let mut total_time = 0u64;

    for (input, expected_contains) in tests {
        println!("📝 الاختبار: {}", input);
        println!("   المتوقع أن يحتوي على: {}", expected_contains);
        
        let (output, duration) = model.test_text_to_code(input);
        total_time += duration;

        // تحقق بسيط - البحث عن الكلمات المفتاحية
        let output_lower = output.to_lowercase();
        let expected_lower = expected_contains.to_lowercase();
        
        if output.contains(expected_contains) || 
           output_lower.contains(&expected_lower) ||
           // تحقق مرن للمتغيرات
           (expected_contains.contains("متغير") && output.contains("متغير")) ||
           (expected_contains.contains("اطبع") && output.contains("اطبع")) ||
           (expected_contains.contains("إذا") && (output.contains("إذا") || output.contains("if"))) ||
           (expected_contains.contains("دالة") && output.contains("دالة")) ||
           (expected_contains.contains("طالما") && (output.contains("طالما") || output.contains("while") || output.contains("for")))
        {
            println!("   ✅ نجح! ({} ms)", duration);
            passed += 1;
        } else {
            println!("   ❌ فشل! ({} ms)", duration);
            failed += 1;
        }
        
        // عرض الناتج (أول 200 حرف)
        let preview: String = output.chars().take(200).collect();
        println!("   الناتج: {}", preview.replace('\n', " "));
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("                         📊 نتائج الاختبارات");
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("   ✅ نجح: {}", passed);
    println!("   ❌ فشل: {}", failed);
    println!("   ⏱️  الوقت الإجمالي: {} ms", total_time);
    println!("   📈 معدل النجاح: {:.1}%", (passed as f64 / (passed + failed) as f64) * 100.0);
    println!();

    // اختبارات Vibe Coding
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("                    🎯 اختبارات Vibe Coding");
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!();

    let vibe_tests = vec![
        "اكتب برنامج يطبع الأرقام من 1 إلى 10",
        "أنشئ برنامج يحسب مجموع رقمين",
        "اكتب دالة ترجع أكبر رقم في قائمة",
    ];

    for input in vibe_tests {
        println!("🎨 المطلوب: {}", input);
        let (output, duration) = model.test_text_to_code(input);
        println!("   ⏱️  الوقت: {} ms", duration);
        let preview: String = output.chars().take(300).collect();
        println!("   📝 الكود:\n{}", preview);
        println!();
    }

    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("                    ✨ انتهت الاختبارات ✨");
    println!("═══════════════════════════════════════════════════════════════════════════");
}
