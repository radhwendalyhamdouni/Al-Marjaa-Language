// ═══════════════════════════════════════════════════════════════════════════════
// اختبار محرك GGUF - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use std::path::PathBuf;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     🧠 اختبار محرك GGUF - لغة المرجع                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // إعداد المسار
    let model_path = PathBuf::from("models/qwen2.5-0.5b-instruct-q4_k_m.gguf");
    
    // التحقق من وجود النموذج
    if !model_path.exists() {
        println!("❌ النموذج غير موجود في: {:?}", model_path);
        println!("📁 المسار الحالي: {:?}", std::env::current_dir().unwrap());
        println!();
        println!("💡 لتحميل النموذج:");
        println!("   wget -O models/qwen2.5-0.5b-instruct-q4_k_m.gguf \\");
        println!("     'https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf'");
        return;
    }
    
    println!("📦 النموذج: {:?}", model_path);
    println!("📊 الحجم: {} MB", model_path.metadata().unwrap().len() / 1_000_000);
    println!();

    // إنشاء المحرك
    println!("🔧 إنشاء محرك GGUF...");
    
    // نستخدم المحرك من المكتبة
    use almarjaa::ai_engine::{GGUFEngine, GGUFConfig};
    
    let config = GGUFConfig {
        model_path,
        temperature: 0.7,
        top_p: 0.9,
        top_k: 40,
        max_tokens: 128,
        repeat_penalty: 1.1,
        seed: 42,
        port: 8080,
    };

    let mut engine = GGUFEngine::with_config(config);
    
    // تحميل النموذج
    println!("⏳ جاري تحميل النموذج...");
    match engine.load() {
        Ok(_) => println!("✅ تم تحميل النموذج بنجاح!"),
        Err(e) => {
            println!("❌ فشل تحميل النموذج: {}", e);
            return;
        }
    }
    println!();

    // عرض معلومات النموذج
    println!("{}", engine.model_info());
    println!();

    // اختبارات Vibe Coding
    let tests = vec![
        ("أنشئ متغير س يساوي 10", "متغير"),
        ("اطبع مرحبا بالعالم", "طباعة"),
        ("إذا كان س أكبر من 5 اطبع كبير", "شرط"),
        ("أنشئ دالة تجمع رقمين", "دالة"),
        ("كرر طباعة مرحبا 3 مرات", "حلقة"),
    ];

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     🎯 اختبارات Vibe Coding                                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    for (input, category) in tests {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📝 النوع: {}", category);
        println!("🗣️  المدخل: \"{}\"", input);
        
        let _start = std::time::Instant::now();
        match engine.infer(input) {
            Ok(result) => {
                println!("📤 النتيجة:");
                println!("┌──────────────────────────────────────────────────────────────┐");
                for line in result.text.lines() {
                    println!("│ {}", line);
                }
                println!("└──────────────────────────────────────────────────────────────┘");
                println!("⏱️  الوقت: {} ms", result.duration_ms);
                println!("🔢 التوكنات: {}", result.tokens_generated);
                println!("⚡ السرعة: {:.1} توكن/ثانية", result.tokens_per_second);
            }
            Err(e) => {
                println!("❌ خطأ: {}", e);
            }
        }
        println!();
    }

    println!("✨ انتهى الاختبار!");
}
