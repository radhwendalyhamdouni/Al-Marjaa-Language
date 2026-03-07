// ═══════════════════════════════════════════════════════════════════════════════
// اختبار نظام التصدير - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use std::fs;
use std::path::Path;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     📦 اختبار نظام التصدير - لغة المرجع                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    use almarjaa::exporter::{ExportEngine, ExportConfig, ExportPlatform, export_html_only};
    
    // ═══════════════════════════════════════════════════════════════════════════
    // اختبار 1: تصدير HTML بسيط
    // ═══════════════════════════════════════════════════════════════════════════
    
    println!("📝 اختبار 1: تصدير HTML بسيط");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let simple_code = r#"أنشئ نافذة بعنوان "مرحبا"
أضف زر مكتوب عليه "اضغطني"
أضف تسمية نصها "مرحبا بالعالم""#;
    
    let result = export_html_only(simple_code, "hello_app");
    
    if result.success {
        println!("✅ نجح: {}", result.message);
        if let Some(path) = &result.output_path {
            println!("📁 الملف: {:?}", path);
        }
    } else {
        println!("❌ فشل: {}", result.message);
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبار 2: تصدير حاسبة
    // ═══════════════════════════════════════════════════════════════════════════
    
    println!("📝 اختبار 2: تصدير حاسبة");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let calculator_code = r#"أنشئ نافذة بعنوان "حاسبة المرجع"
أضف حقل نص تلميح "الرقم الأول"
أضف حقل نص تلميح "الرقم الثاني"
أضف زر مكتوب عليه "جمع" باللون الأخضر
أضف زر مكتوب عليه "طرح" باللون الأزرق
أضف زر مكتوب عليه "ضرب" باللون البرتقالي
أضف زر مكتوب عليه "قسمة" باللون الأحمر
أضف تسمية نصها "النتيجة:""#;
    
    let result = export_html_only(calculator_code, "calculator");
    
    if result.success {
        println!("✅ نجح: {}", result.message);
    } else {
        println!("❌ فشل: {}", result.message);
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبار 3: تصدير مشروع كامل
    // ═══════════════════════════════════════════════════════════════════════════
    
    println!("📝 اختبار 3: تصدير مشروع كامل");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let config = ExportConfig {
        project_name: "task_manager".to_string(),
        platform: ExportPlatform::Windows,
        output_dir: std::path::PathBuf::from("build"),
        with_gui: true,
        release_mode: true,
        icon_path: None,
    };
    
    let engine = ExportEngine::new(config);
    
    let project_code = r#"أنشئ نافذة بعنوان "مدير المهام" عرض 600 ارتفاع 500
أضف تسمية نصها "قائمة المهام" في س 20 ص 20
أضف حقل نص تلميح "أدخل المهمة..." في س 20 ص 60 عرض 400
أضف زر مكتوب عليه "إضافة" باللون الأخضر في س 440 ص 55
أضف حاوية في س 20 ص 110 عرض 560 ارتفاع 350"#;
    
    let result = engine.export(project_code);
    
    if result.success {
        println!("✅ نجح: {}", result.message);
        if let Some(path) = &result.output_path {
            println!("📁 المسار: {:?}", path);
        }
    } else {
        println!("❌ فشل: {}", result.message);
    }
    
    for warning in &result.warnings {
        println!("⚠️ {}", warning);
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبار 4: قراءة أمثلة وتصديرها
    // ═══════════════════════════════════════════════════════════════════════════
    
    println!("📝 اختبار 4: تصدير أمثلة جاهزة");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    let examples = vec![
        ("examples/export_calculator.mrj", "example_calculator"),
        ("examples/export_todo_app.mrj", "example_todo"),
    ];
    
    for (file_path, output_name) in examples {
        let path = Path::new(file_path);
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(code) => {
                    let result = export_html_only(&code, output_name);
                    if result.success {
                        println!("✅ {} → {}", file_path, result.message);
                    } else {
                        println!("❌ {} → {}", file_path, result.message);
                    }
                }
                Err(e) => {
                    println!("❌ فشل قراءة {}: {}", file_path, e);
                }
            }
        } else {
            println!("⚠️ الملف غير موجود: {}", file_path);
        }
    }
    println!();

    // ═══════════════════════════════════════════════════════════════════════════
    // النتائج النهائية
    // ═══════════════════════════════════════════════════════════════════════════
    
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    📊 النتائج النهائية                      ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  الملفات المُنتجة:                                          ║");
    println!("║     • build/hello_app/index.html                           ║");
    println!("║     • build/calculator/index.html                          ║");
    println!("║     • build/task_manager/ (مشروع كامل)                     ║");
    println!("║     • build/example_calculator/index.html                  ║");
    println!("║     • build/example_todo/index.html                        ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}
