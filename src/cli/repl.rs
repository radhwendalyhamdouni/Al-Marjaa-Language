use std::thread;
use std::time::Duration;

use colored::Colorize;

use almarjaa::interpreter::value::Value;
use almarjaa::interpreter::Interpreter;

pub fn run_repl() {
    print_banner();
    print_majestic_terminal_art();
    print_legendary_intro();
    println!(
        "{}",
        crate::rtl("مرحباً بك أيها المبرمج! اكتب 'مساعدة' أو 'ذكاء' للانطلاق بسرعة").bright_green()
    );
    println!(
        "{}",
        crate::rtl("تم تفعيل عرض RTL في الطرفية.").bright_green()
    );

    let mut interpreter = Interpreter::new();

    loop {
        print!("{} ", crate::rtl("المرجع>>").bright_blue());

        use std::io::{self, Write};
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                match input {
                    "خروج" | "exit" | "quit" => {
                        print_legendary_outro();
                        println!("{}", crate::rtl("مع السلامة! 👋").bright_green());
                        break;
                    }
                    "مساعدة" | "help" => {
                        print_repl_help();
                        continue;
                    }
                    "ذكاء" | "ai" => {
                        print_ai_quick_help();
                        continue;
                    }
                    "خطة_تدريب_خفيفة" => {
                        print_low_resource_training_plan();
                        continue;
                    }
                    "مسح" | "clear" => {
                        print!("\x1B[2J\x1B[1;1H");
                        continue;
                    }
                    _ => {}
                }

                if let Some(name) = input.strip_prefix("أنشئ_وكيل ") {
                    let agent_name = name.trim();
                    if agent_name.is_empty() {
                        eprintln!(
                            "{}",
                            crate::rtl("الرجاء كتابة اسم للوكيل بعد الأمر").bright_red()
                        );
                    } else {
                        print_agent_scaffold(agent_name);
                    }
                    continue;
                }

                match interpreter.run(input) {
                    Ok(result) => {
                        let value: std::cell::Ref<Value> = result.borrow();
                        if !matches!(*value, Value::Null) {
                            println!("{}", crate::rtl(&format!("=> {}", value)).bright_yellow());
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "{}",
                            crate::rtl(&format!("خطأ: {}", e.message)).bright_red()
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    crate::rtl(&format!("خطأ في القراءة: {}", e)).bright_red()
                );
                break;
            }
        }
    }
}

fn print_banner() {
    println!(
        "{}",
        r#"
    ╔══════════════════════════════════════════════════════════════════╗
    ║                                                                  ║
    ║                  ⚔️  مجدٌ يكتب بلغة المرجع  ⚔️                           ║
    ║                    ✦ Al-Marjaa Language ✦                        ║
    ║                         الإصدار 2.0.0                               ║
    ║                                                                  ║
    ║           المبرمج الأصلي الحقيقي: رضوان دالي حمدوني                           ║
    ║         شعار: (نكتب المستقبل بثقةٍ... وننفّذ بهيبة)                              ║
    ║                                                                  ║
    ╚══════════════════════════════════════════════════════════════════╝
    "#
        .bright_cyan()
    );
}

fn print_majestic_terminal_art() {
    let crest = [
        "                      ░▒▓█  عرش المرجع  █▓▒░                 ",
        "                    /\\                 /\\                ",
        "                   /  \\    ⚔️   ⚔️    /  \\               ",
        "                  /____\\_____________/____\\              ",
        "                  ||  لغة المرجع - Al-Marjaa   ||              ",
        "                  ||  المبرمج: رضوان دالي حمدوني   ||              ",
        "                  ||      هيبة الكود العربي        ||              ",
        "                  ||________________________||              ",
    ];

    for line in crest {
        typewriter(&crate::rtl(line), |s| s.bright_yellow().bold());
    }

    for pulse in ["◥████◤", "◢████◣", "◤████◥"] {
        println!(
            "{}",
            crate::rtl(&format!("(✦) {}", pulse))
                .bright_magenta()
                .bold()
        );
        thread::sleep(Duration::from_millis(130));
    }
}

fn typewriter(text: &str, color: fn(&str) -> colored::ColoredString) {
    for ch in text.chars() {
        print!("{}", color(&ch.to_string()));
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(12));
    }
    println!();
}

fn print_legendary_intro() {
    typewriter(
        &crate::rtl("(⊳…⊲) مرحباً بك في لغة المرجع"),
        |s| s.bright_magenta().bold(),
    );
    typewriter(
        &crate::rtl("(⊳…⊲) اسم اللغة: Al-Marjaa | المؤسس: رضوان دالي حمدوني"),
        |s| s.bright_cyan(),
    );
    typewriter(
        &crate::rtl("(⊳…⊲) هيبة عربية... ودقة تنفيذية"),
        |s| s.bright_green(),
    );
}

fn print_legendary_outro() {
    typewriter(
        &crate::rtl("(⊳…⊲) تم الإغلاق بهيبة لغة المرجع"),
        |s| s.bright_magenta().bold(),
    );
    typewriter(
        &crate::rtl("(⊳…⊲) إلى لقاءٍ جديد أيها المبرمج العظيم"),
        |s| s.bright_green(),
    );
}

fn print_ai_quick_help() {
    println!(
        "{}",
        crate::rtl("\nأوامر الذكاء الاصطناعي السريعة:").bright_magenta()
    );
    println!("  ذكاء                      عرض لوحة الأوامر الذكية باللغة العربية");
    println!("  خطة_تدريب_خفيفة          خطة حديثة لتدريب/تكييف نموذج بأقل رام وهاردوير");
    println!("  أنشئ_وكيل <اسم>          توليد قالب وكيل ذكي عربي قابل للتعديل");
}

fn print_low_resource_training_plan() {
    println!(
        "{}",
        crate::rtl("\nخارطة تدريب نموذج ذكاء اصطناعي بكفاءة عالية:").bright_cyan()
    );
    println!("  ١) اختر نموذجاً أساسياً صغيراً (3B-8B) مع دعم quantization.");
    println!("  ٢) استخدم QLoRA (4-bit NF4) لتقليل استهلاك RAM/VRAM.");
    println!("  ٣) فعّل gradient checkpointing و mixed precision (bf16/fp16). ");
    println!("  ٤) اضبط micro-batch صغير مع gradient accumulation.");
    println!("  ٥) ابدأ بـ SFT على بيانات عربية نظيفة ثم DPO/ORPO لتحسين الجودة.");
    println!("  ٦) قيّم على benchmarks عربية + حالات استخدام واقعية قبل الإطلاق.");
    println!(
        "{}",
        crate::rtl(
            "نصيحة: استهدف fine-tuning بدلاً من التدريب من الصفر للحصول على نتيجة رائعة بأقل تكلفة."
        )
        .bright_green()
    );
}

fn print_agent_scaffold(agent_name: &str) {
    println!(
        "{}",
        crate::rtl(&format!("\nقالب وكيل ذكي جاهز: {}", agent_name)).bright_cyan()
    );
    println!("  المتطلبات: هدف واضح + أدوات + ذاكرة سياقية");
    println!("  الدور: مساعد عربي خبير");
    println!("  الهدف: تنفيذ المهام بأوامر عربية بسيطة");
    println!("  الأدوات: [بحث، قراءة_ملف، تنفيذ_أمر]");
    println!("  النمط: دقيق، مختصر، مهني");
    println!("  الحماية: منع الأوامر الخطرة + توثيق كل خطوة");
    println!("  ----------------------------------------------");
    println!(
        "{}",
        crate::rtl("يمكنك نسخ القالب وتخصيصه فوراً لسيناريوك.").bright_green()
    );
}

fn print_repl_help() {
    println!("{}", crate::rtl("\nأوامر REPL:").bright_yellow());
    println!("  خروج, exit, quit    الخروج من البرنامج");
    println!("  مساعدة, help        عرض هذه المساعدة");
    println!("  ذكاء, ai            لوحة الأوامر الذكية");
    println!("  خطة_تدريب_خفيفة    وصفة تدريب حديثة قليلة الموارد");
    println!("  أنشئ_وكيل <اسم>    إنشاء قالب وكيل ذكي عربي");
    println!("  مسح, clear          مسح الشاشة");
    println!();
    println!("{}", crate::rtl("أمثلة على الكود:").bright_yellow());
    println!("  متغير س = ١٠؛");
    println!("  اطبع(س + ٥)؛");
    println!("  دالة جمع(أ، ب) {{ أرجع أ + ب؛ }}");
    println!("  إذا س > ٥ {{ اطبع(\"كبير\")؛ }}");
    println!();
}
