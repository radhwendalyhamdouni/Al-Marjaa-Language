// ═══════════════════════════════════════════════════════════════════════════════
// أداة الاستدلال بالذكاء الاصطناعي - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// واجهة سطر أوامر لتشغيل نموذج GGUF والاستدلال
// يدعم: llama.cpp (Rust native) + llama-cpp-python (Python) + المحاكاة
// ═══════════════════════════════════════════════════════════════════════════════

use clap::Parser;
use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

/// أداة الاستدلال بالذكاء الاصطناعي - لغة المرجع
#[derive(Parser, Debug)]
#[command(name = "almarjaa-ai")]
#[command(author = "رضوان دالي حمدوني")]
#[command(version = "3.0.0")]
#[command(about = "أداة الاستدلال بالذكاء الاصطناعي لنموذج GGUF")]
struct Args {
    /// النص المراد تحويله إلى كود
    #[arg(short, long)]
    prompt: String,
    
    /// مسار نموذج GGUF
    #[arg(short = 'm', long, default_value = "models/qwen2.5-0.5b-instruct-q4_k_m.gguf")]
    model: PathBuf,
    
    /// الحد الأقصى للتوكنات
    #[arg(short = 'n', long, default_value = "128")]
    max_tokens: usize,
    
    /// درجة الحرارة
    #[arg(short, long, default_value = "0.7")]
    temperature: f32,
    
    /// إخراج JSON
    #[arg(short, long, default_value = "false")]
    json: bool,
    
    /// عدد الخيوط
    #[arg(long, default_value = "4")]
    threads: usize,
    
    /// حجم السياق
    #[arg(short = 'c', long, default_value = "512")]
    context_size: u32,
    
    /// استخدام المحاكاة فقط (بدون AI حقيقي)
    #[arg(long, default_value = "false")]
    simulation: bool,
    
    /// المهلة بالثواني
    #[arg(long, default_value = "120")]
    timeout_secs: u64,
}

fn main() {
    let args = Args::parse();
    
    // التحقق من وجود النموذج (إذا لم يكن محاكاة فقط)
    if !args.simulation && !args.model.exists() {
        eprintln!("❌ ملف النموذج غير موجود: {:?}", args.model);
        std::process::exit(1);
    }
    
    let start = Instant::now();
    
    // محاولة الاستدلال الحقيقي
    if !args.simulation {
        // 1. محاولة llama.cpp (إذا كان مثبتاً)
        if let Ok(result) = try_llama_cpp(&args) {
            print_result(&result, args.json, start, "llama.cpp");
            return;
        }
        
        // 2. محاولة llama-cpp-python (من البيئة الافتراضية)
        if let Ok(result) = try_llama_python(&args) {
            print_result(&result, args.json, start, "llama-cpp-python");
            return;
        }
    }
    
    // 3. استخدام المحاكاة (fallback)
    let result = simulate_inference(&args.prompt);
    print_result(&result, args.json, start, "simulation");
}

/// طباعة النتيجة
fn print_result(text: &str, json: bool, start: Instant, mode: &str) {
    let duration_ms = start.elapsed().as_millis() as u64;
    let tokens = text.split_whitespace().count();
    
    if json {
        let output = serde_json::json!({
            "success": true,
            "text": text,
            "tokens": tokens,
            "duration_ms": duration_ms,
            "tokens_per_second": if duration_ms > 0 { tokens as f64 * 1000.0 / duration_ms as f64 } else { 0.0 },
            "mode": mode
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        println!("{}", text);
    }
}

/// محاولة استخدام llama.cpp (Rust native)
fn try_llama_cpp(args: &Args) -> Result<String, String> {
    // البحث عن llama-cli
    let llama_paths = [
        "llama-cli",
        "./llama-cli",
        "/usr/local/bin/llama-cli",
        "/home/z/my-project/llama.cpp/build/bin/llama-cli",
    ];
    
    let mut llama_path = None;
    for path in llama_paths {
        if PathBuf::from(path).exists() || Command::new(path).arg("--version").output().is_ok() {
            llama_path = Some(path);
            break;
        }
    }
    
    let llama = llama_path.ok_or("llama-cli غير موجود")?;
    
    let formatted_prompt = format_prompt(&args.prompt);
    
    let output = Command::new(llama)
        .args([
            "-m", args.model.to_str().unwrap(),
            "-p", &formatted_prompt,
            "-n", &args.max_tokens.to_string(),
            "--temp", &args.temperature.to_string(),
            "--no-display-prompt",
            "-c", &args.context_size.to_string(),
            "-t", &args.threads.to_string(),
            "--log-disable",
        ])
        .output()
        .map_err(|e| format!("فشل تشغيل llama-cli: {}", e))?;
    
    if !output.status.success() {
        return Err("فشل الاستدلال".to_string());
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(extract_response(&stdout))
}

/// محاولة استخدام llama-cpp-python
fn try_llama_python(args: &Args) -> Result<String, String> {
    // مسارات Python مع llama-cpp-python
    let python_paths = [
        "/home/z/my-project/llama_env/bin/python3",
        "/home/z/my-project/llama_env/bin/python",
    ];
    
    let mut python_path = None;
    for path in python_paths {
        if PathBuf::from(path).exists() {
            // التحقق من وجود llama_cpp
            let check = Command::new(path)
                .args(["-c", "import llama_cpp; print('ok')"])
                .output();
            if check.is_ok() && check.unwrap().status.success() {
                python_path = Some(path);
                break;
            }
        }
    }
    
    let python = python_path.ok_or("llama-cpp-python غير متوفر")?;
    
    // إنشاء سكربت Python مؤقت
    let script = format!(r#"
import sys
sys.stdout.reconfigure(encoding='utf-8')
from llama_cpp import Llama

llm = Llama(
    model_path="{model}",
    n_ctx={ctx},
    n_threads={threads},
    verbose=False
)

prompt = """<|im_start|>system
أنت مساعد برمجي عربي متخصص في تحويل النص العربي الطبيعي إلى كود بلغة المرجع.

قواعد التحويل:
- المتغيرات: "أنشئ متغير [اسم] يساوي [قيمة]" → متغير [اسم] = [قيمة]؛
- الطباعة: "اطبع [نص]" → اطبع("[نص]")؛
- الشرط: "إذا كان [شرط] [إجراء]" → إذا [شرط] {{ [إجراء] }}
- الدوال: "أنشئ دالة [اسم]" → دالة [اسم]() {{ }}
- الحلقات: "كرر [عدد] مرات [إجراء]" → طالما ع < [عدد] {{ [إجراء] }}

أعد فقط الكود بدون شرح إضافي.<|im_end|>
<|im_start|>user
{prompt}<|im_end|>
<|im_start|>assistant
"""

output = llm(
    prompt,
    max_tokens={max_tokens},
    temperature={temp},
    stop=["<|im_end|>", "<|im_start|>"],
)

print(output['choices'][0]['text'].strip())
"#,
        model = args.model.display(),
        ctx = args.context_size,
        threads = args.threads,
        prompt = args.prompt.replace("\"", "\\\"").replace("\n", "\\n"),
        max_tokens = args.max_tokens,
        temp = args.temperature,
    );
    
    let output = Command::new(python)
        .args(["-c", &script])
        .output()
        .map_err(|e| format!("فشل تشغيل Python: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("خطأ Python: {}", stderr));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.trim().to_string())
}

/// تنسيق الـ prompt
fn format_prompt(user_input: &str) -> String {
    format!(
        r#"<|im_start|>system
أنت مساعد برمجي عربي متخصص في تحويل النص العربي الطبيعي إلى كود بلغة المرجع.

قواعد التحويل:
- المتغيرات: "أنشئ متغير [اسم] يساوي [قيمة]" → متغير [اسم] = [قيمة]؛
- الطباعة: "اطبع [نص]" → اطبع("[نص]")؛
- الشرط: "إذا كان [شرط] [إجراء]" → إذا [شرط] {{ [إجراء] }}
- الدوال: "أنشئ دالة [اسم]" → دالة [اسم]() {{ }}
- الحلقات: "كرر [عدد] مرات [إجراء]" → طالما ع < [عدد] {{ [إجراء] }}
- التصدير: "صدر البرنامج [اسم] على [منصة]" → صدر البرنامج "[اسم]" على [منصة]؛

أعد فقط الكود بدون شرح إضافي.<|im_end|>
<|im_start|>user
{}<|im_end|>
<|im_start|>assistant
"#,
        user_input
    )
}

/// استخراج الاستجابة
fn extract_response(output: &str) -> String {
    let marker = "<|im_start|>assistant";
    
    let text = if let Some(pos) = output.rfind(marker) {
        let after = &output[pos + marker.len()..];
        after.trim()
    } else {
        output.trim()
    };
    
    text.replace("<|im_end|>", "")
        .replace("<|im_start|>", "")
        .trim()
        .to_string()
}

/// محاكاة ذكية للاستدلال
fn simulate_inference(input: &str) -> String {
    let input_lower = input.to_lowercase();
    
    if input_lower.contains("صدر") && (input_lower.contains("البرنامج") || input_lower.contains("برنامج")) {
        extract_export(input)
    } else if input_lower.contains("متغير") || input_lower.contains("أنشئ متغير") {
        extract_variable(input)
    } else if input_lower.contains("إذا") || input_lower.contains("شرط") {
        extract_condition(input)
    } else if input_lower.contains("دالة") || input_lower.contains("وظيفة") {
        extract_function(input)
    } else if input_lower.contains("كرر") || input_lower.contains("حلقة") || input_lower.contains("طالما") {
        extract_loop(input)
    } else if input_lower.contains("اطبع") || input_lower.contains("اعرض") || input_lower.contains("اكتب") {
        extract_print(input)
    } else {
        format!("// لم أفهم المطلوب: {}", input)
    }
}

fn extract_variable(input: &str) -> String {
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

fn extract_print(input: &str) -> String {
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

fn extract_condition(input: &str) -> String {
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

fn extract_function(input: &str) -> String {
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

fn extract_loop(input: &str) -> String {
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

fn extract_export(input: &str) -> String {
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
        "// 📦 تصدير البرنامج\nصدر البرنامج \"{}\" على {}؛",
        program_name, platform
    )
}
