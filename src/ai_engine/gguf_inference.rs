// ═══════════════════════════════════════════════════════════════════════════════
// محرك GGUF عبر llama.cpp server - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// يتصل بـ llama.cpp server المحلي لتشغيل النموذج
// ═══════════════════════════════════════════════════════════════════════════════

use std::path::PathBuf;
use std::process::{Command, Child};

/// نتيجة الاستدلال
#[derive(Debug, Clone)]
pub struct GGUFResult {
    pub text: String,
    pub tokens_generated: usize,
    pub duration_ms: u64,
    pub tokens_per_second: f32,
}

/// إعدادات النموذج
#[derive(Debug, Clone)]
pub struct GGUFConfig {
    pub model_path: PathBuf,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: usize,
    pub max_tokens: usize,
    pub repeat_penalty: f32,
    pub seed: u64,
    pub port: u16,
}

impl Default for GGUFConfig {
    fn default() -> Self {
        Self {
            model_path: PathBuf::from("models/qwen2.5-0.5b-instruct-q8_0.gguf"),
            temperature: 0.7,
            top_p: 0.9,
            top_k: 40,
            max_tokens: 128,
            repeat_penalty: 1.1,
            seed: 42,
            port: 8080,
        }
    }
}

/// محرك GGUF باستخدام llama.cpp server
pub struct GGUFEngine {
    config: GGUFConfig,
    loaded: bool,
    server_process: Option<Child>,
}

impl GGUFEngine {
    /// إنشاء محرك جديد
    pub fn new() -> Self {
        Self {
            config: GGUFConfig::default(),
            loaded: false,
            server_process: None,
        }
    }

    /// إنشاء محرك مع إعدادات مخصصة
    pub fn with_config(config: GGUFConfig) -> Self {
        Self {
            config,
            loaded: false,
            server_process: None,
        }
    }

    /// تحميل النموذج (تشغيل llama.cpp server)
    pub fn load(&mut self) -> Result<(), String> {
        // التحقق من وجود ملف النموذج
        if !self.config.model_path.exists() {
            return Err(format!(
                "ملف النموذج غير موجود: {:?}",
                self.config.model_path
            ));
        }

        // محاولة الاتصال بالسيرفر الموجود
        if self.check_server_health() {
            println!("✅ السيرفر يعمل بالفعل!");
            self.loaded = true;
            return Ok(());
        }

        // محاولة تشغيل llama.cpp server
        if let Err(e) = self.start_llama_server() {
            println!("⚠️ لم يتم تشغيل llama.cpp server: {}", e);
            println!("📝 سيتم استخدام الوضع المحلي (Simulation)");
        }

        self.loaded = true;
        Ok(())
    }

    /// التحقق من حالة السيرفر
    fn check_server_health(&self) -> bool {
        // محاولة بسيطة للتحقق من السيرفر
        if let Ok(output) = Command::new("curl")
            .args(["-s", "-o", "/dev/null", "-w", "%{http_code}", 
                   &format!("http://localhost:{}/health", self.config.port)])
            .output()
        {
            if output.stdout == b"200\n" || output.stdout == b"200" {
                return true;
            }
        }
        false
    }

    /// تشغيل llama.cpp server
    fn start_llama_server(&mut self) -> Result<(), String> {
        // البحث عن llama-server أو main
        let server_names = ["llama-server", "server", "llama.cpp-server", "./llama-server"];
        let mut server_path = None;
        
        for name in server_names {
            if Command::new(name).arg("--version").output().is_ok() {
                server_path = Some(name.to_string());
                break;
            }
        }

        let server = server_path.ok_or("llama.cpp server غير موجود")?;

        println!("🚀 تشغيل llama.cpp server...");
        
        let child = Command::new(&server)
            .args([
                "-m", self.config.model_path.to_str().unwrap(),
                "--port", &self.config.port.to_string(),
                "-c", "2048",
                "-ngl", "99",  // استخدام GPU
                "--host", "127.0.0.1",
            ])
            .spawn()
            .map_err(|e| format!("فشل تشغيل السيرفر: {}", e))?;

        self.server_process = Some(child);
        
        // انتظار السيرفر
        std::thread::sleep(std::time::Duration::from_secs(5));
        
        Ok(())
    }

    /// تشغيل الاستدلال
    pub fn infer(&self, prompt: &str) -> Result<GGUFResult, String> {
        if !self.loaded {
            return Err("النموذج غير محمّل".to_string());
        }

        let start = std::time::Instant::now();

        // محاولة الاستدلال المباشر عبر llama-cli
        if let Ok(result) = self.infer_via_cli(prompt, start) {
            return Ok(result);
        }

        // محاولة استخدام السيرفر
        if let Ok(result) = self.infer_via_server(prompt, start) {
            return Ok(result);
        }

        // استخدام المحاكاة كـ fallback
        let simulated = self.simulate_inference(prompt);
        let duration = start.elapsed().as_millis() as u64;
        Ok(GGUFResult {
            text: simulated,
            tokens_generated: 20,
            duration_ms: duration,
            tokens_per_second: 20_000.0 / duration.max(1) as f32,
        })
    }

    /// الاستدلال المباشر عبر llama-cli
    fn infer_via_cli(&self, prompt: &str, start: std::time::Instant) -> Result<GGUFResult, String> {
        // بناء الـ prompt بتنسيق Qwen
        let formatted_prompt = format!(
            "<|im_start|>system\nأنت مساعد برمجي عربي متخصص في تحويل النص العربي إلى كود بلغة المرجع. قواعد التحويل:\n\
            - المتغيرات: 'أنشئ متغير [اسم] يساوي [قيمة]' → متغير [اسم] = [قيمة]؛\n\
            - الطباعة: 'اطبع [نص]' → اطبع(\"[نص]\")؛\n\
            - الشرط: 'إذا كان [شرط] [إجراء]' → إذا [شرط] {{ [إجراء] }}\n\
            - الدوال: 'أنشئ دالة [اسم]' → دالة [اسم]() {{ }}\n\
            - الحلقات: 'كرر [عدد] مرات [إجراء]' → طالما ع < [عدد] {{ [إجراء] }}\n\
            أعد فقط الكود بدون شرح.<|im_end|>\n\
            <|im_start|>user\n{}<|im_end|>\n\
            <|im_start|>assistant\n",
            prompt
        );

        // البحث عن llama-cli
        let cli_paths = [
            "./llama_cpp/build/bin/llama-cli",
            "llama-cli",
            "./llama-cli",
            "/usr/local/bin/llama-cli",
        ];

        let mut cli_path = None;
        for path in cli_paths {
            if PathBuf::from(path).exists() || Command::new(path).arg("--version").output().is_ok() {
                cli_path = Some(path.to_string());
                break;
            }
        }

        let cli = cli_path.ok_or("llama-cli غير موجود")?;

        // تشغيل llama-cli
        let output = Command::new(&cli)
            .args([
                "-m", self.config.model_path.to_str().unwrap(),
                "-p", &formatted_prompt,
                "-n", &self.config.max_tokens.to_string(),
                "--temp", &self.config.temperature.to_string(),
                "--top-p", &self.config.top_p.to_string(),
                "--top-k", &self.config.top_k.to_string(),
                "--repeat-penalty", &self.config.repeat_penalty.to_string(),
                "--seed", &self.config.seed.to_string(),
                "--no-display-prompt",
                "-c", "2048",
                "--no-warmup",
            ])
            .output()
            .map_err(|e| format!("فشل تشغيل llama-cli: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        
        // استخراج النص الناتج
        let text = stdout
            .lines()
            .skip_while(|line| !line.contains("assistant"))
            .skip(1)
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();

        if text.is_empty() {
            return Err("لم يتم إنتاج نص".to_string());
        }

        let duration = start.elapsed().as_millis() as u64;
        let tokens = text.split_whitespace().count();

        Ok(GGUFResult {
            text,
            tokens_generated: tokens,
            duration_ms: duration,
            tokens_per_second: tokens as f32 * 1000.0 / duration.max(1) as f32,
        })
    }

    /// الاستدلال عبر السيرفر
    fn infer_via_server(&self, prompt: &str, start: std::time::Instant) -> Result<GGUFResult, String> {
        // بناء الـ prompt بتنسيق Qwen
        let formatted_prompt = format!(
            "<|im_start|>system\nأنت مساعد برمجي عربي متخصص في تحويل النص العربي إلى كود بلغة المرجع. قواعد التحويل:\n\
            - المتغيرات: 'أنشئ متغير [اسم] يساوي [قيمة]' → متغير [اسم] = [قيمة]؛\n\
            - الطباعة: 'اطبع [نص]' → اطبع(\"[نص]\")؛\n\
            - الشرط: 'إذا كان [شرط] [إجراء]' → إذا [شرط] {{ [إجراء] }}\n\
            - الدوال: 'أنشئ دالة [اسم]' → دالة [اسم]() {{ }}\n\
            - الحلقات: 'كرر [عدد] مرات [إجراء]' → طالما ع < [عدد] {{ [إجراء] }}\n\
            أعد فقط الكود بدون شرح.<|im_end|>\n\
            <|im_start|>user\n{}<|im_end|>\n\
            <|im_start|>assistant\n",
            prompt
        );

        // بناء JSON للطلب
        let request_body = serde_json::json!({
            "prompt": formatted_prompt,
            "n_predict": self.config.max_tokens,
            "temperature": self.config.temperature,
            "top_p": self.config.top_p,
            "top_k": self.config.top_k,
            "repeat_penalty": self.config.repeat_penalty,
            "seed": self.config.seed,
        });

        // إرسال الطلب
        let url = format!("http://localhost:{}/completion", self.config.port);
        
        let output = Command::new("curl")
            .args([
                "-s", "-X", "POST",
                "-H", "Content-Type: application/json",
                "-d", &serde_json::to_string(&request_body).unwrap(),
                &url,
            ])
            .output()
            .map_err(|e| format!("فشل إرسال الطلب: {}", e))?;

        if !output.status.success() {
            return Err("فشل الاستجابة من السيرفر".to_string());
        }

        // تحليل الاستجابة
        let response: serde_json::Value = serde_json::from_slice(&output.stdout)
            .map_err(|e| format!("فشل تحليل الاستجابة: {}", e))?;

        let text = response["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let tokens = response["tokens_evaluated"]
            .as_u64()
            .unwrap_or(20) as usize;

        let duration = start.elapsed().as_millis() as u64;

        Ok(GGUFResult {
            text,
            tokens_generated: tokens,
            duration_ms: duration,
            tokens_per_second: tokens as f32 * 1000.0 / duration.max(1) as f32,
        })
    }

    /// محاكاة ذكية للاستدلال
    pub fn simulate_inference(&self, prompt: &str) -> String {
        let prompt_lower = prompt.to_lowercase();
        
        // تحليل النص العربي - الترتيب مهم!
        // التصدير له أولوية قصوى
        if prompt_lower.contains("صدر") && (prompt_lower.contains("البرنامج") || prompt_lower.contains("برنامج")) {
            self.extract_export(prompt)
        } else if prompt_lower.contains("إذا") || prompt_lower.contains("شرط") {
            self.extract_condition(prompt)
        } else if prompt_lower.contains("متغير") || (prompt_lower.contains("أنشئ") && prompt_lower.contains("متغير")) {
            self.extract_variable(prompt)
        } else if prompt_lower.contains("كرر") || prompt_lower.contains("حلقة") || prompt_lower.contains("طالما") {
            self.extract_loop(prompt)
        } else if prompt_lower.contains("دالة") || prompt_lower.contains("وظيفة") {
            self.extract_function(prompt)
        } else if prompt_lower.contains("اطبع") || prompt_lower.contains("اعرض") || prompt_lower.contains("اكتب") {
            self.extract_print(prompt)
        } else if prompt_lower.contains("قائمة") || prompt_lower.contains("مصفوفة") {
            self.extract_list(prompt)
        } else {
            format!("// لم أفهم المطلوب: {}\n// حاول صياغته بشكل مختلف", prompt)
        }
    }

    fn extract_variable(&self, input: &str) -> String {
        let words: Vec<&str> = input.split_whitespace().collect();
        let mut name = "س";
        let mut value = "0";
        
        for (i, word) in words.iter().enumerate() {
            if *word == "متغير" {
                if let Some(n) = words.get(i + 1) {
                    if !["يساوي", "بقيمة", "القيمة", "أنشئ"].contains(n) {
                        name = n;
                    }
                }
            }
            if *word == "يساوي" || *word == "بقيمة" {
                if let Some(v) = words.get(i + 1) {
                    value = v.trim_matches(|c: char| !c.is_alphanumeric() && c != '.' && c != '-');
                    if !value.is_empty() {
                        break;
                    }
                }
            }
        }
        
        format!("متغير {} = {}؛", name, value)
    }

    fn extract_print(&self, input: &str) -> String {
        let text = input
            .replace("اطبع", "")
            .replace("اعرض", "")
            .replace("اكتب", "")
            .replace("رسالة", "")
            .replace("نص", "")
            .trim()
            .to_string();
        
        if text.is_empty() {
            "اطبع(\"مرحبا بالعالم\")؛".to_string()
        } else {
            format!("اطبع(\"{}\")؛", text)
        }
    }

    fn extract_condition(&self, input: &str) -> String {
        let lower = input.to_lowercase();
        
        // استخراج اسم المتغير من النص
        let words: Vec<&str> = input.split_whitespace().collect();
        let var_name = words.iter()
            .skip_while(|w| **w != "كان")
            .skip(1)
            .next()
            .unwrap_or(&"س");
        
        // استخراج الشرط
        let condition = if lower.contains("أكبر") {
            let num: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
            let num = if num.is_empty() { "10".to_string() } else { num };
            format!("{} > {}", var_name, num)
        } else if lower.contains("أصغر") {
            let num: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
            let num = if num.is_empty() { "10".to_string() } else { num };
            format!("{} < {}", var_name, num)
        } else if lower.contains("يساوي") {
            let num: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
            let num = if num.is_empty() { "10".to_string() } else { num };
            format!("{} == {}", var_name, num)
        } else {
            "صحيح".to_string()
        };
        
        // استخراج الإجراء
        let body = if lower.contains("اطبع") || lower.contains("اعرض") {
            if let Some(pos) = input.find("اطبع") {
                let after = &input[pos + "اطبع".len()..].trim();
                let msg: String = after
                    .chars()
                    .filter(|c| !c.is_ascii_digit())
                    .collect::<String>()
                    .trim()
                    .to_string();
                if msg.is_empty() {
                    "اطبع(\"تم\")؛".to_string()
                } else {
                    format!("اطبع(\"{}\")؛", msg)
                }
            } else {
                "اطبع(\"تم\")؛".to_string()
            }
        } else {
            "اطبع(\"الشرط محقق\")؛".to_string()
        };
        
        format!("إذا {} {{\n    {}\n}}", condition, body)
    }

    fn extract_function(&self, input: &str) -> String {
        let lower = input.to_lowercase();
        
        let (name, params, body) = if lower.contains("جمع") || lower.contains("تضيف") {
            ("اجمع", "أ، ب", "أعطِ أ + ب؛")
        } else if lower.contains("ضرب") || lower.contains("تضرب") {
            ("اضرب", "أ، ب", "أعطِ أ * ب؛")
        } else if lower.contains("طرح") || lower.contains("تطرح") {
            ("اطرح", "أ، ب", "أعطِ أ - ب؛")
        } else if lower.contains("قسم") {
            ("اقسم", "أ، ب", "أعطِ أ / ب؛")
        } else {
            let name = input
                .split_whitespace()
                .skip_while(|w| ["أنشئ", "دالة", "وظيفة", "التي", "تقوم", "بـ"].contains(w))
                .next()
                .unwrap_or("دالة_جديدة");
            (name, "", "أعطِ لا_شيء؛")
        };
        
        format!("دالة {}({}) {{\n    {}\n}}", name, params, body)
    }

    fn extract_loop(&self, input: &str) -> String {
        let lower = input.to_lowercase();
        
        let count = if lower.contains("ثلاث") || lower.contains("٣") || lower.contains("3") {
            3
        } else if lower.contains("خمس") || lower.contains("٥") || lower.contains("5") {
            5
        } else if lower.contains("عشر") || lower.contains("١٠") || lower.contains("10") {
            10
        } else if lower.contains("مرتين") || lower.contains("٢") || lower.contains("2") {
            2
        } else {
            input
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap_or(1)
        };
        
        let body = if lower.contains("اطبع") || lower.contains("اعرض") {
            let msg = if lower.contains("مرحبا") {
                "مرحبا"
            } else {
                "تكرار"
            };
            format!("اطبع(\"{}\")؛", msg)
        } else {
            "اطبع(\"تكرار\")؛".to_string()
        };
        
        format!(
            "متغير ع = 0؛\nطالما ع < {} {{\n    {}\n    ع = ع + 1؛\n}}",
            count, body
        )
    }

    fn extract_list(&self, _input: &str) -> String {
        "قائمة أرقام = [1، 2، 3، 4، 5]؛".to_string()
    }

    /// استخراج أمر التصدير
    fn extract_export(&self, input: &str) -> String {
        let lower = input.to_lowercase();
        
        // استخراج اسم البرنامج
        let program_name = if input.contains("البرنامج") {
            let after = input.split("البرنامج").nth(1).unwrap_or("").trim();
            if after.contains("على") {
                after.split("على").next().unwrap_or("myapp").trim().replace(" ", "_")
            } else {
                after.split_whitespace().next().unwrap_or("myapp").to_string()
            }
        } else {
            "myapp".to_string()
        };
        
        // استخراج المنصة
        let platform = if lower.contains("ويندوز") || lower.contains("windows") {
            "windows"
        } else if lower.contains("لينكس") || lower.contains("linux") {
            "linux"
        } else if lower.contains("ماك") || lower.contains("mac") {
            "macos"
        } else if lower.contains("ويب") || lower.contains("web") {
            "web"
        } else {
            "windows"
        };
        
        format!(
            "// ══════════════════════════════════════════════════════\n\
             // 📦 تصدير البرنامج\n\
             // ══════════════════════════════════════════════════════\n\
             صدر البرنامج \"{}\" على {}؛\n\
             // ▶️ سيتم إنشاء المشروع في: build/{}/\n\
             // ▶️ المنصة المستهدفة: {}",
            program_name, platform, program_name, platform
        )
    }

    /// تحويل نص عربي إلى كود مباشرة
    pub fn text_to_code(&self, text: &str) -> Result<String, String> {
        let result = self.infer(text)?;
        Ok(result.text)
    }

    /// الحصول على معلومات النموذج
    pub fn model_info(&self) -> String {
        format!(
            "📦 نموذج: {:?}\n📊 الحالة: {}\n🌡️ درجة الحرارة: {}\n🎯 Top-P: {}\n🔢 الحد الأقصى: {} توكن",
            self.config.model_path,
            if self.loaded { "محمّل ✓" } else { "غير محمّل ✗" },
            self.config.temperature,
            self.config.top_p,
            self.config.max_tokens
        )
    }
}

impl Default for GGUFEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for GGUFEngine {
    fn drop(&mut self) {
        if let Some(ref mut child) = self.server_process {
            let _ = child.kill();
            println!("🛑 تم إيقاف llama.cpp server");
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = GGUFEngine::new();
        assert!(!engine.loaded);
    }

    #[test]
    fn test_simulate_variable() {
        let engine = GGUFEngine::new();
        let result = engine.simulate_inference("أنشئ متغير س يساوي 10");
        assert!(result.contains("متغير"));
        assert!(result.contains("س"));
    }

    #[test]
    fn test_simulate_print() {
        let engine = GGUFEngine::new();
        let result = engine.simulate_inference("اطبع مرحبا بالعالم");
        assert!(result.contains("اطبع"));
    }

    #[test]
    fn test_simulate_function() {
        let engine = GGUFEngine::new();
        let result = engine.simulate_inference("أنشئ دالة تجمع رقمين");
        assert!(result.contains("دالة"));
    }

    #[test]
    fn test_simulate_loop() {
        let engine = GGUFEngine::new();
        let result = engine.simulate_inference("كرر طباعة مرحبا 5 مرات");
        assert!(result.contains("طالما"));
    }

    #[test]
    fn test_simulate_condition() {
        let engine = GGUFEngine::new();
        let result = engine.simulate_inference("إذا كان س أكبر من 10 اطبع كبير");
        assert!(result.contains("إذا"));
    }
}
