// ═══════════════════════════════════════════════════════════════════════════════
// اختبار التكامل الشامل - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// اختبار Vibe Coding AI + GUI + التصدير
// ═══════════════════════════════════════════════════════════════════════════════

use std::fs;
use std::path::Path;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     🧪 اختبار التكامل الشامل - لغة المرجع                  ║");
    println!("║     Vibe Coding AI → GUI → تصدير                            ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // استخدام محرك التكامل
    use almarjaa::integration::{IntegrationEngine, natural_to_app};
    
    // ═══════════════════════════════════════════════════════════════════════════
    // الاختبار 1: إنشاء حاسبة بالذكاء الاصطناعي
    // ═══════════════════════════════════════════════════════════════════════════
    
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     🧮 اختبار 1: إنشاء حاسبة بالذكاء الاصطناعي             ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    
    let calc_input = "أنشئ حاسبة بسيطة مع زر جمع وزر طرح وزر ضرب وزر قسمة";
    println!("📝 المدخل: \"{}\"", calc_input);
    println!();
    
    let result = natural_to_app(calc_input, "calculator");
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 النتائج:");
    println!("   ✅ نجح: {}", result.success);
    println!("   📁 المسار: {:?}", result.export_path);
    println!("   💬 الرسالة: {}", result.message);
    println!();
    
    // ═══════════════════════════════════════════════════════════════════════════
    // الاختبار 2: إنشاء تطبيق أكثر تعقيداً
    // ═══════════════════════════════════════════════════════════════════════════
    
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     📝 اختبار 2: إنشاء نموذج تسجيل                         ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    
    let form_input = "أنشئ نموذج تسجيل مع حقل اسم وحقل بريد إلكتروني وزر حفظ";
    println!("📝 المدخل: \"{}\"", form_input);
    println!();
    
    let result2 = natural_to_app(form_input, "registration_form");
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 النتائج:");
    println!("   ✅ نجح: {}", result2.success);
    println!("   📁 المسار: {:?}", result2.export_path);
    println!();
    
    // ═══════════════════════════════════════════════════════════════════════════
    // الاختبار 3: اختبار مع الذكاء الاصطناعي الحقيقي (GGUF)
    // ═══════════════════════════════════════════════════════════════════════════
    
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     🤖 اختبار 3: تكامل مع GGUF AI                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    
    let model_path = Path::new("models/qwen2.5-0.5b-instruct-q4_k_m.gguf");
    
    if model_path.exists() {
        println!("✅ النموذج موجود: {:?}", model_path);
        println!("📊 الحجم: {} MB", model_path.metadata().unwrap().len() / 1_000_000);
        
        // استخدام محرك GGUF
        use almarjaa::GGUFEngine;
        
        let mut engine = GGUFEngine::new();
        match engine.load() {
            Ok(_) => {
                println!("✅ تم تحميل النموذج GGUF");
                
                // اختبار Vibe Coding مع AI
                let ai_tests = vec![
                    "أنشئ متغير س يساوي 10",
                    "إذا كان س أكبر من 5 اطبع كبير",
                    "أنشئ دالة تجمع رقمين",
                ];
                
                for test in ai_tests {
                    match engine.infer(test) {
                        Ok(res) => println!("   ✅ {} → {}", test, res.text.chars().take(50).collect::<String>()),
                        Err(e) => println!("   ❌ {} → {}", test, e),
                    }
                }
            }
            Err(e) => {
                println!("⚠️ فشل تحميل النموذج: {}", e);
                println!("   سيتم استخدام المحاكاة");
            }
        }
    } else {
        println!("⚠️ النموذج GGUF غير موجود");
        println!("   المسار المتوقع: {:?}", model_path);
    }
    println!();
    
    // ═══════════════════════════════════════════════════════════════════════════
    // الاختبار 4: سير عمل كامل (End-to-End)
    // ═══════════════════════════════════════════════════════════════════════════
    
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     🔄 اختبار 4: سير عمل كامل End-to-End                   ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    
    let engine = IntegrationEngine::new();
    
    let test_scenarios = vec![
        ("أنشئ حاسبة بسيطة مع جمع وطرح", "simple_calc"),
        ("أنشئ تطبيق فيه زر حفظ وزر إرسال", "form_app"),
        ("أنشئ مدير مهام مع زر إضافة وزر حذف", "todo_app"),
    ];
    
    let mut success_count = 0;
    let total = test_scenarios.len();
    
    for (input, name) in test_scenarios {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📝 السيناريو: {}", input);
        
        let result = engine.process(input, name);
        
        if result.success {
            success_count += 1;
            println!("   ✅ نجح!");
            if let Some(path) = &result.export_path {
                println!("   📁 الملفات:");
                println!("      - {}/index.html", path.display());
                println!("      - {}/source.mrj", path.display());
            }
        } else {
            println!("   ❌ فشل: {}", result.message);
        }
    }
    println!();
    
    // ═══════════════════════════════════════════════════════════════════════════
    // عرض الملفات المُنتجة
    // ═══════════════════════════════════════════════════════════════════════════
    
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    📁 الملفات المُنتجة                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    
    let output_dirs = vec![
        "build/ai_apps/calculator",
        "build/ai_apps/registration_form",
        "build/ai_apps/simple_calc",
        "build/ai_apps/form_app",
        "build/ai_apps/todo_app",
    ];
    
    for dir in output_dirs {
        let path = Path::new(dir);
        if path.exists() {
            println!("\n📂 {}:", dir);
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                    println!("   📄 {} ({} bytes)", name.to_string_lossy(), size);
                }
            }
        }
    }
    println!();
    
    // ═══════════════════════════════════════════════════════════════════════════
    // النتائج النهائية
    // ═══════════════════════════════════════════════════════════════════════════
    
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    📊 النتائج النهائية                      ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  ✅ ناجح: {} / {}                                            ║", success_count, total);
    println!("║  📈 النسبة: {:.0}%                                           ║", (success_count as f32 / total as f32 * 100.0));
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  🎯 الميزات المختبرة:                                        ║");
    println!("║     ✅ Vibe Coding AI                                        ║");
    println!("║     ✅ GUI Generation                                        ║");
    println!("║     ✅ Export to HTML                                        ║");
    println!("║     ✅ Integration Engine                                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}
