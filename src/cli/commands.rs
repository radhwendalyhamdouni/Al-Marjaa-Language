use std::fs;
use std::path::Path;
use std::process;
use std::time::Instant;

use colored::Colorize;

use crate::cli::args::RunOptions;
use almarjaa::formatter::format_source;
use almarjaa::interpreter::Interpreter;
use almarjaa::linter::{lint_source_with_config, LintConfig};
use almarjaa::lsp_bridge::{
    build_completions, build_diagnostics_json, find_definition, find_hover, find_references,
};
use almarjaa::package_manager::{
    PackageManager, resolve_dependencies, write_lockfile,
};
use almarjaa::mobile::{
    MobileExporter, MobileExportConfig, MobilePlatform, MobileFramework,
};

pub fn print_help() {
    println!("{}", crate::rtl("استخدام:").bright_yellow());
    println!("  almarjaa [خيارات] [ملف]");
    println!();
    println!("{}", crate::rtl("خيارات:").bright_yellow());
    println!("  -h, --help         عرض هذه الرسالة");
    println!("  -v, --version      عرض الإصدار");
    println!("  -r, --repl         تشغيل الوضع التفاعلي (افتراضي)");
    println!("  -c, --compile      تحليل الملف فقط (بدون تنفيذ)");
    println!("  -f, --format       تنسيق الملف وطباعته");
    println!("  -t, --tokens       عرض الرموز المميزة");
    println!("  -l, --lint         تحليل الشيفرة بقواعد linter (تحذيرات فقط)");
    println!("      --lint-disable  تعطيل قاعدة lint (يمكن تكرارها)");
    println!("      --lint-max      الحد الأقصى لعدد التحذيرات المعروضة");
    println!("  -a, --ast          عرض شجرة الصياغة");
    println!("  -d, --debug        عرض تفاصيل التحليل والتنفيذ");
    println!("      --pm-init      إنشاء ملف mrj.toml مبدئي");
    println!("      --pm-check     فحص mrj.toml + إنشاء mrj.lock");
    println!("      --pm-tree      عرض شجرة التبعيات من mrj.toml");
    println!("      --lsp-diag     إخراج diagnostics بصيغة JSON");
    println!("  pm <cmd>           واجهة جديدة: init/check/tree");
    println!("  lsp <cmd>          واجهة جديدة: diag/complete/hover/definition/references");
    println!("  mobile <cmd>       تصدير للهواتف: export/list");
    println!("  أوامر ذكاء REPL    اكتب: ذكاء");
    println!();
    println!("{}", crate::rtl("أمثلة:").bright_yellow());
    println!("  almarjaa program.mrj          تنفيذ ملف برنامج");
    println!("  almarjaa -r                   تشغيل الوضع التفاعلي");
    println!("  almarjaa -t program.mrj       عرض الرموز المميزة");
    println!("  almarjaa pm check             فحص manifest وتحديث lockfile");
    println!("  almarjaa lsp hover app.mrj 3 8  معلومات hover");
    println!("  almarjaa mobile export app.mrj -p android -f flutter");
}

pub fn print_version(version: &str) {
    println!(
        "{}",
        crate::rtl(&format!("لغة المرجع - الإصدار {}", version))
    );
    println!("{}", crate::rtl("لغة برمجة عربية متكاملة"));
    println!("2024 - فريق المرجع");
}

pub fn handle_package_commands(options: &RunOptions) -> bool {
    if let Some(project_name) = options.pm_init.as_deref() {
        let manifest_path = Path::new(".");
        let mut pm = PackageManager::new(manifest_path);
        let result = pm.init(project_name);
        if result.success {
            println!(
                "{}",
                crate::rtl("تم إنشاء المشروع بنجاح").bright_green()
            );
        } else {
            eprintln!(
                "{}",
                crate::rtl(&format!("خطأ: {}", result.message)).bright_red()
            );
            process::exit(1);
        }
        return true;
    }

    if options.pm_check || options.pm_tree {
        let manifest_path = Path::new(".");
        
        let resolved = match resolve_dependencies(manifest_path) {
            Ok(items) => items,
            Err(err) => {
                eprintln!(
                    "{}",
                    crate::rtl(&format!("خطأ مدير الحزم: {}", err)).bright_red()
                );
                process::exit(1);
            }
        };

        if let Err(err) = write_lockfile(manifest_path) {
            eprintln!(
                "{}",
                crate::rtl(&format!("خطأ مدير الحزم: {}", err)).bright_red()
            );
            process::exit(1);
        }

        if options.pm_tree {
            println!("{}", crate::rtl("شجرة التبعيات:").bright_cyan());
            if resolved.packages.is_empty() {
                println!("- (لا توجد تبعيات)");
            } else {
                for (name, pkg) in &resolved.packages {
                    println!("- {} @ {}", name, pkg.version);
                }
            }
        } else {
            println!(
                "{}",
                crate::rtl("✅ مشروع.toml صالح وتم إنشاء/تحديث قفل.toml بشكل حتمي").bright_green()
            );
        }
        return true;
    }

    false
}

pub fn handle_lsp_commands(options: &RunOptions, filename: Option<&str>) -> bool {
    if !(options.lsp_diag
        || options.lsp_complete.is_some()
        || options.lsp_hover.is_some()
        || options.lsp_definition.is_some()
        || options.lsp_references.is_some())
    {
        return false;
    }

    let fname = filename.unwrap_or_else(|| {
        eprintln!("{}", crate::rtl("أوامر LSP تحتاج ملفاً").bright_red());
        process::exit(1);
    });

    let content = fs::read_to_string(fname).unwrap_or_else(|e| {
        eprintln!(
            "{}",
            crate::rtl(&format!("خطأ في قراءة الملف '{}': {}", fname, e)).bright_red()
        );
        process::exit(1);
    });

    if options.lsp_diag {
        let diagnostics = build_diagnostics_json(&content);
        println!("{}", serde_json::to_string_pretty(&diagnostics).unwrap());
        return true;
    }

    if let Some(prefix) = options.lsp_complete.as_deref() {
        let prefix = if prefix.is_empty() {
            None
        } else {
            Some(prefix)
        };
        match build_completions(&content, prefix) {
            Ok(items) => println!("{}", serde_json::to_string_pretty(&items).unwrap()),
            Err(err) => {
                eprintln!(
                    "{}",
                    crate::rtl(&format!("فشل completion: {}", err)).bright_red()
                );
                process::exit(1);
            }
        }
        return true;
    }

    if let Some((line, column)) = options.lsp_hover {
        match find_hover(&content, line, column) {
            Ok(data) => println!("{}", serde_json::to_string_pretty(&data).unwrap()),
            Err(err) => {
                eprintln!(
                    "{}",
                    crate::rtl(&format!("فشل hover: {}", err)).bright_red()
                );
                process::exit(1);
            }
        }
        return true;
    }

    if let Some((line, column)) = options.lsp_definition {
        match find_definition(&content, line, column) {
            Ok(data) => println!("{}", serde_json::to_string_pretty(&data).unwrap()),
            Err(err) => {
                eprintln!(
                    "{}",
                    crate::rtl(&format!("فشل definition: {}", err)).bright_red()
                );
                process::exit(1);
            }
        }
        return true;
    }

    if let Some((line, column)) = options.lsp_references {
        match find_references(&content, line, column) {
            Ok(data) => println!("{}", serde_json::to_string_pretty(&data).unwrap()),
            Err(err) => {
                eprintln!(
                    "{}",
                    crate::rtl(&format!("فشل references: {}", err)).bright_red()
                );
                process::exit(1);
            }
        }
        return true;
    }

    true
}

/// معالجة أوامر تصدير الهواتف المحمولة
pub fn handle_mobile_command(options: &RunOptions, filename: Option<&str>) -> bool {
    if !options.mobile_export {
        return false;
    }

    // عرض القائمة فقط
    if options.mobile_platform.is_none() && options.mobile_framework.is_none() {
        return true; // تمت معالجة الأمر list
    }

    // الحصول على الكود المصدري
    let fname = filename.unwrap_or_else(|| {
        eprintln!("{}", crate::rtl("تصدير الهواتف يحتاج ملفاً").bright_red());
        process::exit(1);
    });

    let source_code = fs::read_to_string(fname).unwrap_or_else(|e| {
        eprintln!(
            "{}",
            crate::rtl(&format!("خطأ في قراءة الملف '{}': {}", fname, e)).bright_red()
        );
        process::exit(1);
    });

    // إنشاء التكوين
    let config = MobileExportConfig {
        project_name: options.mobile_project_name.clone().unwrap_or_else(|| {
            Path::new(fname)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("almarjaa_app")
                .to_string()
        }),
        platform: options.mobile_platform.as_deref()
            .and_then(MobilePlatform::from_arabic)
            .unwrap_or(MobilePlatform::Android),
        framework: options.mobile_framework.as_deref()
            .and_then(MobileFramework::from_arabic)
            .unwrap_or(MobileFramework::Flutter),
        ..Default::default()
    };

    // إنشاء المصدّر والتصدير
    let exporter = MobileExporter::new(config);
    let result = exporter.export(&source_code);

    if result.success {
        if let Some(path) = result.output_path {
            println!(
                "{}",
                crate::rtl(&format!("✅ تم إنشاء المشروع في: {}", path.display())).bright_green()
            );
        }
        
        if !result.warnings.is_empty() {
            println!();
            println!("{}", crate::rtl("التحذيرات:").bright_yellow());
            for warning in &result.warnings {
                println!("  ⚠️  {}", warning);
            }
        }
    } else {
        eprintln!(
            "{}",
            crate::rtl(&format!("❌ فشل التصدير: {}", result.message)).bright_red()
        );
        process::exit(1);
    }

    true
}

pub fn run_file(filename: &str, options: &RunOptions) {
    let content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "{}",
                crate::rtl(&format!("خطأ في قراءة الملف '{}': {}", filename, e)).bright_red()
            );
            process::exit(1);
        }
    };

    if options.debug {
        println!(
            "{}",
            crate::rtl(&format!("[debug] الملف: {}", filename)).bright_blue()
        );
        println!(
            "{}",
            crate::rtl(&format!("[debug] الحجم: {} بايت", content.len())).bright_blue()
        );
    }

    if options.format_only {
        let formatted = format_source(&content);
        print!("{}", formatted);
        return;
    }

    if options.lint_only {
        let lint_start = Instant::now();
        let lint_config = LintConfig {
            disabled_rules: options.lint_disabled_rules.iter().cloned().collect(),
            max_diagnostics: options.lint_max,
        };

        match lint_source_with_config(&content, &lint_config) {
            Ok(diagnostics) => {
                if diagnostics.is_empty() {
                    println!("{}", crate::rtl("✅ لا توجد تحذيرات lint").bright_green());
                } else {
                    println!("{}", crate::rtl("تحذيرات lint:").bright_yellow());
                    for diagnostic in &diagnostics {
                        println!("- [{}] {}", diagnostic.code, diagnostic.message);
                    }
                }

                if options.debug {
                    println!(
                        "{}",
                        crate::rtl(&format!(
                            "[debug] عدد تحذيرات lint: {} | زمن التحليل: {:?}",
                            diagnostics.len(),
                            lint_start.elapsed()
                        ))
                        .bright_blue()
                    );
                }
            }
            Err(err) => {
                eprintln!(
                    "{}",
                    crate::rtl(&format!("خطأ أثناء lint: {}", err)).bright_red()
                );
                process::exit(1);
            }
        }
        return;
    }

    if options.show_tokens {
        use almarjaa::lexer::Lexer;
        let lex_start = Instant::now();
        let mut lexer = Lexer::new(&content);
        match lexer.tokenize() {
            Ok(tokens) => {
                println!("{}", crate::rtl("=== الرموز المميزة ===").bright_cyan());
                for token in &tokens {
                    println!("{:4}:{:4} {:?}", token.line, token.column, token.token_type);
                }
                if options.debug {
                    println!(
                        "{}",
                        crate::rtl(&format!(
                            "[debug] عدد الرموز: {} | زمن التحليل اللغوي: {:?}",
                            tokens.len(),
                            lex_start.elapsed()
                        ))
                        .bright_blue()
                    );
                }
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    crate::rtl(&format!("خطأ في التحليل اللغوي: {}", e)).bright_red()
                );
                process::exit(1);
            }
        }
        return;
    }

    use almarjaa::parser::Parser;
    if options.show_ast || options.compile_only || options.debug {
        let parse_start = Instant::now();
        match Parser::parse(&content) {
            Ok(program) => {
                if options.show_ast {
                    println!("{}", crate::rtl("=== شجرة الصياغة ===").bright_cyan());
                    for (i, stmt) in program.statements.iter().enumerate() {
                        println!("{:<3} {:?}", i + 1, stmt);
                    }
                    if options.debug {
                        println!(
                            "{}",
                            crate::rtl(&format!(
                                "[debug] عدد التعليمات: {} | زمن التحليل النحوي: {:?}",
                                program.statements.len(),
                                parse_start.elapsed()
                            ))
                            .bright_blue()
                        );
                    }
                    return;
                }

                if options.compile_only {
                    println!(
                        "{}",
                        crate::rtl(&format!(
                            "نجح التحليل: {} تعليمة | الزمن: {:?}",
                            program.statements.len(),
                            parse_start.elapsed()
                        ))
                        .bright_green()
                    );
                    return;
                }

                if options.debug {
                    println!(
                        "{}",
                        crate::rtl(&format!(
                            "[debug] عدد التعليمات: {} | زمن التحليل النحوي: {:?}",
                            program.statements.len(),
                            parse_start.elapsed()
                        ))
                        .bright_blue()
                    );
                }
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    crate::rtl(&format!(
                        "خطأ في التحليل: {} (السطر {}، العمود {})",
                        e.message, e.line, e.column
                    ))
                    .bright_red()
                );
                process::exit(1);
            }
        }
    }

    let mut interpreter = Interpreter::new();
    let exec_start = Instant::now();

    match interpreter.run(&content) {
        Ok(_) => {
            if options.debug {
                println!(
                    "{}",
                    crate::rtl(&format!("[debug] زمن التنفيذ: {:?}", exec_start.elapsed()))
                        .bright_blue()
                );
            }
        }
        Err(e) => {
            eprintln!(
                "{}",
                crate::rtl(&format!("خطأ في التنفيذ: {}", e.message)).bright_red()
            );
            process::exit(1);
        }
    }
}
