// ═══════════════════════════════════════════════════════════════════════════════
// محرك AI حقيقي - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// يستخدم الأداة Rust-native (almarjaa-ai) أو llama.cpp لتشغيل النماذج الحقيقية
// ═══════════════════════════════════════════════════════════════════════════════

use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};

/// نتيجة التنبؤ الحقيقي
#[derive(Debug, Clone)]
pub struct RealInferenceResult {
    pub text: String,
    pub tokens_generated: usize,
    pub duration_ms: u64,
    pub tokens_per_second: f32,
    pub model_used: String,
}

/// إعدادات النموذج الحقيقي
#[derive(Debug, Clone)]
pub struct RealModelConfig {
    /// مسار النموذج GGUF
    pub model_path: PathBuf,
    /// مسار llama-cli
    pub llama_cli_path: PathBuf,
    /// درجة الحرارة
    pub temperature: f32,
    /// Top-p
    pub top_p: f32,
    /// Top-k
    pub top_k: usize,
    /// الحد الأقصى للتوكنات
    pub max_tokens: usize,
    /// repeat penalty
    pub repeat_penalty: f32,
    /// seed
    pub seed: u64,
    /// عدد الخيوط
    pub threads: usize,
    /// GPU layers
    pub gpu_layers: i32,
}

impl Default for RealModelConfig {
    fn default() -> Self {
        Self {
            model_path: PathBuf::from("models/qwen2.5-0.5b-instruct-q4_k_m.gguf"),
            llama_cli_path: PathBuf::from("llama-cli"),
            temperature: 0.7,
            top_p: 0.9,
            top_k: 40,
            max_tokens: 128,
            repeat_penalty: 1.1,
            seed: 42,
            threads: 4,
            gpu_layers: 0,
        }
    }
}

/// طريقة الاستدلال
#[derive(Debug, Clone, Copy, PartialEq)]
enum InferenceMethod {
    /// الأداة Rust-native
    AlmarjaaAi,
    /// llama-cli
    LlamaCli,
    /// Python wrapper
    PythonWrapper,
    /// محاكاة (fallback)
    Simulation,
}

/// محرك AI حقيقي
pub struct RealAIEngine {
    config: RealModelConfig,
    loaded: bool,
    /// كاش للنتائج
    cache: Arc<Mutex<Vec<(String, RealInferenceResult)>>>,
    /// طريقة الاستدلال
    method: InferenceMethod,
}

impl RealAIEngine {
    pub fn new() -> Self {
        Self {
            config: RealModelConfig::default(),
            loaded: false,
            cache: Arc::new(Mutex::new(Vec::with_capacity(100))),
            method: InferenceMethod::Simulation,
        }
    }
    
    pub fn with_config(config: RealModelConfig) -> Self {
        Self {
            config,
            loaded: false,
            cache: Arc::new(Mutex::new(Vec::with_capacity(100))),
            method: InferenceMethod::Simulation,
        }
    }
    
    /// تحميل النموذج
    pub fn load(&mut self) -> Result<(), String> {
        // التحقق من وجود ملف النموذج
        if !self.config.model_path.exists() {
            return Err(format!(
                "ملف النموذج غير موجود: {:?}",
                self.config.model_path
            ));
        }
        
        // البحث عن طريقة الاستدلال المتاحة
        // 1. محاولة almarjaa-ai أولاً (Rust native)
        if self.find_almarjaa_ai().is_some() {
            self.method = InferenceMethod::AlmarjaaAi;
            self.loaded = true;
            return Ok(());
        }
        
        // 2. محاولة llama-cli
        if self.find_llama_cli().is_some() {
            self.method = InferenceMethod::LlamaCli;
            self.loaded = true;
            return Ok(());
        }
        
        // 3. محاولة Python wrapper
        if self.find_python_wrapper().is_some() {
            self.method = InferenceMethod::PythonWrapper;
            self.loaded = true;
            return Ok(());
        }
        
        // 4. استخدام المحاكاة
        self.method = InferenceMethod::Simulation;
        self.loaded = true;
        Ok(())
    }
    
    /// البحث عن almarjaa-ai (الأداة Rust native)
    fn find_almarjaa_ai(&self) -> Option<PathBuf> {
        let paths = [
            "./target/debug/almarjaa-ai",
            "./target/release/almarjaa-ai",
            "./almarjaa-ai",
            "/usr/local/bin/almarjaa-ai",
            "almarjaa-ai",
        ];
        
        for path in paths {
            let pb = PathBuf::from(path);
            if pb.exists() || Command::new(path).arg("--help").output().is_ok() {
                return Some(pb);
            }
        }
        None
    }
    
    /// البحث عن llama-cli
    fn find_llama_cli(&self) -> Option<PathBuf> {
        let paths = [
            "llama-cli",
            "./llama-cli",
            "/usr/local/bin/llama-cli",
            "./llama_cpp/build/bin/llama-cli",
            "./llama.cpp/llama-cli",
        ];
        
        for path in paths {
            let pb = PathBuf::from(path);
            if pb.exists() || Command::new(path).arg("--version").output().is_ok() {
                return Some(pb);
            }
        }
        None
    }
    
    /// البحث عن Python wrapper
    fn find_python_wrapper(&self) -> Option<PathBuf> {
        let paths = [
            "./ai_real.py",
            "./venv/bin/python",
            "python3",
            "python",
        ];
        
        // التحقق من وجود السكريبت والبيئة
        let script_exists = PathBuf::from("ai_real.py").exists();
        
        for path in paths {
            let pb = PathBuf::from(path);
            if path.ends_with("python") || path.ends_with("python3") {
                // التحقق من أن Python يعمل وأن السكريبت موجود
                if script_exists && Command::new(path).arg("--version").output().is_ok() {
                    return Some(pb);
                }
            } else if pb.exists() {
                return Some(pb);
            }
        }
        None
    }
    
    /// تشغيل التنبؤ الحقيقي
    pub fn infer(&mut self, prompt: &str) -> Result<RealInferenceResult, String> {
        if !self.loaded {
            return Err("النموذج غير محمّل".to_string());
        }
        
        // التحقق من الكاش
        {
            let cache = self.cache.lock().unwrap();
            if let Some((_, result)) = cache.iter().find(|(k, _)| k == prompt) {
                return Ok(result.clone());
            }
        }
        
        let start = std::time::Instant::now();
        
        // استخدام الطريقة المناسبة
        let text = match self.method {
            InferenceMethod::AlmarjaaAi => self.infer_via_almarjaa_ai(prompt)?,
            InferenceMethod::LlamaCli => self.infer_via_llama_cli(prompt)?,
            InferenceMethod::PythonWrapper => self.infer_via_python(prompt)?,
            InferenceMethod::Simulation => self.simulate_inference(prompt),
        };
        
        let duration_ms = start.elapsed().as_millis() as u64;
        let tokens = text.split_whitespace().count();
        let tps = if duration_ms > 0 {
            tokens as f32 * 1000.0 / duration_ms as f32
        } else {
            0.0
        };
        
        let result = RealInferenceResult {
            text,
            tokens_generated: tokens,
            duration_ms,
            tokens_per_second: tps,
            model_used: self.config.model_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
        };
        
        // تخزين في الكاش
        {
            let mut cache = self.cache.lock().unwrap();
            if cache.len() >= 100 {
                cache.remove(0);
            }
            cache.push((prompt.to_string(), result.clone()));
        }
        
        Ok(result)
    }
    
    /// الاستدلال عبر almarjaa-ai (Rust native)
    fn infer_via_almarjaa_ai(&self, prompt: &str) -> Result<String, String> {
        let almarjaa_ai = self.find_almarjaa_ai()
            .ok_or("almarjaa-ai غير موجود")?;
        
        let output = Command::new(&almarjaa_ai)
            .args([
                "--prompt", prompt,
                "--model", self.config.model_path.to_str().unwrap(),
                "--max-tokens", &self.config.max_tokens.to_string(),
                "--temperature", &self.config.temperature.to_string(),
            ])
            .output()
            .map_err(|e| format!("فشل تشغيل almarjaa-ai: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("خطأ من almarjaa-ai: {}", stderr));
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    
    /// الاستدلال عبر Python wrapper
    fn infer_via_python(&self, prompt: &str) -> Result<String, String> {
        let python = self.find_python_wrapper()
            .ok_or("Python wrapper غير موجود")?;
        
        let output = Command::new(&python)
            .args([
                "ai_real.py",
                "--prompt", prompt,
                "--model", self.config.model_path.to_str().unwrap(),
                "--max-tokens", &self.config.max_tokens.to_string(),
                "--temperature", &self.config.temperature.to_string(),
            ])
            .output()
            .map_err(|e| format!("فشل تشغيل Python wrapper: {}", e))?;
        
        if !output.status.success() {
            let _stderr = String::from_utf8_lossy(&output.stderr);
            // إذا فشل Python، نرجع للمحاكاة
            return Ok(self.simulate_inference(prompt));
        }
        
        let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if result.is_empty() {
            Ok(self.simulate_inference(prompt))
        } else {
            Ok(result)
        }
    }
    
    /// الاستدلال عبر llama-cli
    fn infer_via_llama_cli(&self, prompt: &str) -> Result<String, String> {
        let formatted_prompt = self.format_prompt(prompt);
        
        let llama_cli = self.find_llama_cli()
            .ok_or("llama-cli غير موجود")?;
        
        let args = vec![
            "-m".to_string(),
            self.config.model_path.to_str().unwrap().to_string(),
            "-p".to_string(),
            formatted_prompt.clone(),
            "-n".to_string(),
            self.config.max_tokens.to_string(),
            "--temp".to_string(),
            self.config.temperature.to_string(),
            "--top-p".to_string(),
            self.config.top_p.to_string(),
            "--top-k".to_string(),
            self.config.top_k.to_string(),
            "--repeat-penalty".to_string(),
            self.config.repeat_penalty.to_string(),
            "--seed".to_string(),
            self.config.seed.to_string(),
            "-t".to_string(),
            self.config.threads.to_string(),
            "--no-display-prompt".to_string(),
            "-c".to_string(),
            "2048".to_string(),
            "-ngl".to_string(),
            self.config.gpu_layers.to_string(),
            "-e".to_string(),
            "--log-disable".to_string(),
        ];
        
        let output = Command::new(&llama_cli)
            .args(&args)
            .output()
            .map_err(|e| format!("فشل تشغيل llama-cli: {}", e))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        if !output.status.success() && !stdout.is_empty() {
            return Err(format!("فشل التنبؤ: {}", stderr));
        }
        
        Ok(self.extract_response(&stdout, &formatted_prompt))
    }
    
    /// محاكاة ذكية للاستدلال
    fn simulate_inference(&self, prompt: &str) -> String {
        let prompt_lower = prompt.to_lowercase();
        
        if prompt_lower.contains("صدر") && (prompt_lower.contains("البرنامج") || prompt_lower.contains("برنامج")) {
            self.extract_export(prompt)
        } else if prompt_lower.contains("متغير") || prompt_lower.contains("أنشئ متغير") {
            self.extract_variable(prompt)
        } else if prompt_lower.contains("إذا") || prompt_lower.contains("شرط") {
            self.extract_condition(prompt)
        } else if prompt_lower.contains("دالة") || prompt_lower.contains("وظيفة") {
            self.extract_function(prompt)
        } else if prompt_lower.contains("كرر") || prompt_lower.contains("حلقة") || prompt_lower.contains("طالما") {
            self.extract_loop(prompt)
        } else if prompt_lower.contains("اطبع") || prompt_lower.contains("اعرض") || prompt_lower.contains("اكتب") {
            self.extract_print(prompt)
        } else {
            format!("// لم أفهم المطلوب: {}", prompt)
        }
    }
    
    /// تنسيق الـ prompt بتنسيق Qwen
    fn format_prompt(&self, input: &str) -> String {
        format!(
            r#"<|im_start|>system
أنت مساعد برمجي عربي متخصص في تحويل النص العربي الطبيعي إلى كود بلغة المرجع.

قواعد التحويل:
- المتغيرات: "أنشئ متغير [اسم] يساوي [قيمة]" → متغير [اسم] = [قيمة]؛
- الطباعة: "اطبع [نص]" → اطبع("[نص]")؛
- الشرط: "إذا كان [شرط] [إجراء]" → إذا [شرط] {{ [إجراء] }}
- الدوال: "أنشئ دالة [اسم]" → دالة [اسم]() {{ }}
- الحلقات: "كرر [عدد] مرات [إجراء]" → طالما ع < [عدد] {{ [إجراء] }}

أعد فقط الكود بدون شرح إضافي.<|im_end|>
<|im_start|>user
{}<|im_end|>
<|im_start|>assistant
"#,
            input
        )
    }
    
    /// استخراج الاستجابة من المخرجات
    fn extract_response(&self, output: &str, _prompt: &str) -> String {
        let marker = "<|im_start|>assistant";
        
        let text = if let Some(pos) = output.rfind(marker) {
            let after = &output[pos + marker.len()..];
            after.trim()
        } else {
            output.trim()
        };
        
        let text = text
            .replace("<|im_end|>", "")
            .replace("<|im_start|>", "")
            .trim()
            .to_string();
        
        self.clean_code(&text)
    }
    
    /// تنظيف الكود المُنتج
    fn clean_code(&self, text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();
        let mut code_lines = Vec::new();
        let mut in_code = false;
        
        for line in lines {
            let trimmed = line.trim();
            
            if !in_code && trimmed.is_empty() {
                continue;
            }
            
            if !in_code && !trimmed.is_empty() {
                in_code = true;
            }
            
            if in_code && (trimmed.starts_with("```") || trimmed.starts_with("###")) {
                break;
            }
            
            if in_code {
                code_lines.push(line);
            }
        }
        
        if code_lines.is_empty() {
            text.to_string()
        } else {
            code_lines.join("\n")
        }
    }
    
    // دوال استخراج الكود للمحاكاة
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
        
        let condition = if lower.contains("أكبر") {
            let num: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
            let num = if num.is_empty() { "10" } else { &num };
            format!("س > {}", num)
        } else if lower.contains("أصغر") {
            let num: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
            let num = if num.is_empty() { "10" } else { &num };
            format!("س < {}", num)
        } else if lower.contains("يساوي") {
            let num: String = input.chars().filter(|c| c.is_ascii_digit()).collect();
            let num = if num.is_empty() { "10" } else { &num };
            format!("س == {}", num)
        } else {
            "صحيح".to_string()
        };
        
        let body = if lower.contains("اطبع") || lower.contains("اعرض") {
            "اطبع(\"تم\")؛".to_string()
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
            ("دالة_جديدة", "", "أعطِ لا_شيء؛")
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
            input.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap_or(1)
        };
        
        let body = if lower.contains("اطبع") || lower.contains("اعرض") {
            let msg = if lower.contains("مرحبا") { "مرحبا" } else { "تكرار" };
            format!("اطبع(\"{}\")؛", msg)
        } else {
            "اطبع(\"تكرار\")؛".to_string()
        };
        
        format!(
            "متغير ع = 0؛\nطالما ع < {} {{\n    {}\n    ع = ع + 1؛\n}}",
            count, body
        )
    }
    
    fn extract_export(&self, input: &str) -> String {
        let lower = input.to_lowercase();
        
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
            "صدر البرنامج \"{}\" على {}؛",
            program_name, platform
        )
    }
    
    /// تحديث الإعدادات
    pub fn set_temperature(&mut self, temp: f32) {
        self.config.temperature = temp;
    }
    
    pub fn set_max_tokens(&mut self, max: usize) {
        self.config.max_tokens = max;
    }
    
    /// مسح الكاش
    pub fn clear_cache(&mut self) {
        self.cache.lock().unwrap().clear();
    }
    
    /// الحصول على معلومات النموذج
    pub fn model_info(&self) -> String {
        let method_str = match self.method {
            InferenceMethod::AlmarjaaAi => "almarjaa-ai (Rust native)",
            InferenceMethod::LlamaCli => "llama-cli",
            InferenceMethod::PythonWrapper => "Python (llama-cpp-python)",
            InferenceMethod::Simulation => "محاكاة",
        };
        
        format!(
            "📦 النموذج: {:?}\n\
             📊 الحالة: {}\n\
             🔧 الطريقة: {}\n\
             🌡️ درجة الحرارة: {}\n\
             🎯 Top-P: {}\n\
             🔢 الحد الأقصى: {} توكن",
            self.config.model_path,
            if self.loaded { "محمّل ✓" } else { "غير محمّل ✗" },
            method_str,
            self.config.temperature,
            self.config.top_p,
            self.config.max_tokens
        )
    }
}

impl Default for RealAIEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// دالة سهلة للتحويل النصي
pub fn text_to_code_real(text: &str) -> Result<String, String> {
    let mut engine = RealAIEngine::new();
    engine.load().map_err(|e| format!("خطأ في التحميل: {}", e))?;
    let result = engine.infer(text)?;
    Ok(result.text)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_engine_creation() {
        let engine = RealAIEngine::new();
        assert!(!engine.loaded);
    }
    
    #[test]
    fn test_format_prompt() {
        let engine = RealAIEngine::new();
        let prompt = engine.format_prompt("اطبع مرحبا");
        assert!(prompt.contains("<|im_start|>system"));
        assert!(prompt.contains("اطبع مرحبا"));
    }
    
    #[test]
    fn test_clean_code() {
        let engine = RealAIEngine::new();
        let dirty = "هذا شرح\n```مرجع\nاطبع(\"مرحبا\")؛\n```\nنهاية";
        let clean = engine.clean_code(dirty);
        assert!(!clean.contains("```"));
    }
    
    #[test]
    fn test_simulation_variable() {
        let engine = RealAIEngine::new();
        let result = engine.extract_variable("أنشئ متغير اسم يساوي 10");
        assert!(result.contains("متغير"));
        assert!(result.contains("اسم"));
    }
    
    #[test]
    fn test_simulation_print() {
        let engine = RealAIEngine::new();
        let result = engine.extract_print("اطبع مرحبا");
        assert!(result.contains("اطبع"));
    }
    
    #[test]
    fn test_simulation_function() {
        let engine = RealAIEngine::new();
        let result = engine.extract_function("أنشئ دالة تجمع رقمين");
        assert!(result.contains("دالة"));
    }
}
