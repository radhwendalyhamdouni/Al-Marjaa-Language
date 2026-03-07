// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات Vibe Coding المتقدمة - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// اختبار حدود النموذج وقدرات لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use std::path::PathBuf;
use almarjaa::ai_engine::{GGUFEngine, GGUFConfig};

/// فئة الاختبار
#[derive(Debug, Clone)]
struct TestCase {
    category: String,
    difficulty: String,
    input: String,
    expected_keywords: Vec<&'static str>,
}

impl TestCase {
    fn new(category: &str, difficulty: &str, input: &str, keywords: Vec<&'static str>) -> Self {
        Self {
            category: category.to_string(),
            difficulty: difficulty.to_string(),
            input: input.to_string(),
            expected_keywords: keywords,
        }
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     🧠 اختبارات Vibe Coding المتقدمة                        ║");
    println!("║     لغة المرجع - اختبار حدود النموذج                        ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // إعداد المحرك
    let model_path = PathBuf::from("models/qwen2.5-0.5b-instruct-q4_k_m.gguf");
    let config = GGUFConfig {
        model_path,
        temperature: 0.3, // درجة حرارة منخفضة للدقة
        top_p: 0.95,
        top_k: 50,
        max_tokens: 256,
        repeat_penalty: 1.1,
        seed: 42,
        port: 8080,
    };

    let mut engine = GGUFEngine::with_config(config);
    
    match engine.load() {
        Ok(_) => println!("✅ تم تحميل المحرك"),
        Err(e) => {
            println!("❌ فشل التحميل: {}", e);
            return;
        }
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات المستوى الأول: أساسي
    // ═══════════════════════════════════════════════════════════════════════════
    
    let basic_tests = vec![
        TestCase::new("متغيرات", "سهل", "أنشئ متغير اسمه عمر يساوي 25", vec!["متغير", "عمر", "25"]),
        TestCase::new("متغيرات", "سهل", "عرّف ثابت باي يساوي 3.14", vec!["ثابت", "باي"]),
        TestCase::new("طباعة", "سهل", "اعرض رسالة ترحيب بالعالم", vec!["اطبع"]),
        TestCase::new("دوال", "سهل", "اكتب دالة ترجع الرقم 5", vec!["دالة", "أعطِ"]),
    ];

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات المستوى الثاني: متوسط
    // ═══════════════════════════════════════════════════════════════════════════
    
    let intermediate_tests = vec![
        TestCase::new("شروط", "متوسط", "إذا كان العمر أكبر من 18 اطبع بالغ", vec!["إذا", ">"]),
        TestCase::new("شروط", "متوسط", "إذا كانت الدرجة أصغر من 50 اطبع راسب وإلا اطبع ناجح", vec!["إذا"]),
        TestCase::new("حلقات", "متوسط", "كرر طباعة العداد 10 مرات", vec!["طالما"]),
        TestCase::new("دوال", "متوسط", "أنشئ دالة تحسب مساحة الدائرة بنصف قطر معطى", vec!["دالة"]),
        TestCase::new("دوال", "متوسط", "اكتب دالة تستقبل رقمين وترجع الأكبر بينهما", vec!["دالة"]),
        TestCase::new("قوائم", "متوسط", "أنشئ قائمة أسماء تحتوي أحمد ومحمد وعلي", vec!["قائمة"]),
    ];

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات المستوى الثالث: متقدم
    // ═══════════════════════════════════════════════════════════════════════════
    
    let advanced_tests = vec![
        TestCase::new("خوارزميات", "متقدم", "أنشئ دالة لحساب المضروب لرقبم معطى", vec!["دالة"]),
        TestCase::new("خوارزميات", "متقدم", "اكتب دالة تتحقق إذا كان الرقم أولي", vec!["دالة"]),
        TestCase::new("خوارزميات", "متقدم", "أنشئ دالة ترجع متوسط قيم قائمة", vec!["دالة"]),
        TestCase::new("رياضيات", "متقدم", "احسب مجموع الأرقام من 1 إلى 100", vec!["طالما", "متغير"]),
        TestCase::new("رياضيات", "متقدم", "أنشئ دالة تحسب Fibonacci للرقم ن", vec!["دالة"]),
    ];

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات المستوى الرابع: خبير
    // ═══════════════════════════════════════════════════════════════════════════
    
    let expert_tests = vec![
        TestCase::new("AI/ML", "خبير", "أنشئ شبكة عصبية بثلاث طبقات للتصنيف", vec!["شبكة", "طبقة"]),
        TestCase::new("AI/ML", "خبير", "اكتب دالة للتدريب على بيانات مع الانحدار", vec!["دالة", "تدريب"]),
        TestCase::new("أنظمة", "خبير", "أنشئ نظام لإدارة قاعدة بيانات الطلاب", vec!["نظام", "طلاب"]),
        TestCase::new("أنظمة", "خبير", "اكتب برنامج يقرأ ملف ويحلل محتواه", vec!["ملف"]),
        TestCase::new("أتمتة", "خبير", "أنشئ سكريبت لأتمتة النسخ الاحتياطي", vec!["نسخ"]),
    ];

    // ═══════════════════════════════════════════════════════════════════════════
    // تشغيل الاختبارات
    // ═══════════════════════════════════════════════════════════════════════════

    let mut total_passed = 0;
    let mut total_failed = 0;

    fn run_tests(engine: &GGUFEngine, tests: Vec<TestCase>, level: &str, passed: &mut usize, failed: &mut usize) {
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║     📊 المستوى: {:<43} ║", level);
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!();

        for test in tests {
            print!("┌──────────────────────────────────────────────────────────────┐\n");
            print!("│ 🏷️  الفئة: {:<49}│\n", test.category);
            print!("│ ⚡ الصعوبة: {:<48}│\n", test.difficulty);
            print!("│ 🗣️  المدخل: {:<48}│\n", truncate(&test.input, 48));
            print!("└──────────────────────────────────────────────────────────────┘\n");
            
            match engine.infer(&test.input) {
                Ok(result) => {
                    print!("📤 النتيجة:\n");
                    for line in result.text.lines() {
                        println!("   {}", line);
                    }
                    
                    // التحقق من الكلمات المفتاحية
                    let mut found = 0;
                    for keyword in &test.expected_keywords {
                        if result.text.contains(keyword) {
                            found += 1;
                        }
                    }
                    
                    let success_rate = found as f32 / test.expected_keywords.len() as f32 * 100.0;
                    if success_rate >= 50.0 {
                        println!("✅ نجح ({:.0}% تطابق)", success_rate);
                        *passed += 1;
                    } else {
                        println!("⚠️ جزئي ({:.0}% تطابق)", success_rate);
                        *failed += 1;
                    }
                }
                Err(e) => {
                    println!("❌ فشل: {}", e);
                    *failed += 1;
                }
            }
            println!();
        }
    }

    run_tests(&engine, basic_tests, "أساسي", &mut total_passed, &mut total_failed);
    run_tests(&engine, intermediate_tests, "متوسط", &mut total_passed, &mut total_failed);
    run_tests(&engine, advanced_tests, "متقدم", &mut total_passed, &mut total_failed);
    run_tests(&engine, expert_tests, "خبير", &mut total_passed, &mut total_failed);

    // ═══════════════════════════════════════════════════════════════════════════
    // النتائج النهائية
    // ═══════════════════════════════════════════════════════════════════════════
    
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    📊 النتائج النهائية                      ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  ✅ ناجح: {}                                                ║", total_passed);
    println!("║  ❌ فاشل: {}                                                ║", total_failed);
    println!("║  📈 النسبة: {:.1}%                                          ║", (total_passed as f32 / (total_passed + total_failed) as f32 * 100.0));
    println!("╚══════════════════════════════════════════════════════════════╝");
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    } else {
        s.to_string()
    }
}
