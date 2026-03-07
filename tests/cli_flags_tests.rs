use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_program_path(name: &str, source: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("تعذر حساب الوقت")
        .as_nanos();
    path.push(format!("almarjaa_{name}_{stamp}.mrj"));
    fs::write(&path, source).expect("تعذر كتابة ملف البرنامج المؤقت");
    path
}

#[test]
fn test_compile_flag_succeeds_without_execution() {
    let program = temp_program_path("compile", r#"متغير س = ١٠؛"#);

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("--compile")
        .arg(&program)
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_file(&program).expect("تعذر حذف الملف المؤقت");

    assert!(output.status.success(), "يجب أن ينجح --compile");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("نجح التحليل"),
        "المخرجات يجب أن تؤكد نجاح التحليل: {stdout}"
    );
}

#[test]
fn test_debug_flag_prints_debug_lines() {
    let program = temp_program_path(
        "debug",
        "متغير س = ١٠؛
اطبع(س)؛",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("--debug")
        .arg(&program)
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_file(&program).expect("تعذر حذف الملف المؤقت");

    assert!(output.status.success(), "يجب أن ينجح --debug");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("[debug]"),
        "المخرجات يجب أن تتضمن أسطر debug: {stdout}"
    );
    assert!(
        stdout.contains("زمن التنفيذ"),
        "المخرجات يجب أن تتضمن زمن التنفيذ: {stdout}"
    );
}

#[test]
fn test_compile_flag_reports_expected_token_suggestion() {
    let program = temp_program_path("compile_error", "اطبع(١ + ٢؛");

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("--compile")
        .arg(&program)
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_file(&program).expect("تعذر حذف الملف المؤقت");

    assert!(
        !output.status.success(),
        "يجب أن يفشل --compile عند وجود خطأ نحوي"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("E201"), "يجب عرض كود الخطأ E201: {stderr}");
    assert!(
        stderr.contains("هل تقصد"),
        "يجب عرض اقتراح التصحيح: {stderr}"
    );
}

#[test]
fn test_format_flag_formats_source() {
    let program = temp_program_path("format", "إذا   صحيح   {\nاطبع(\"نعم\")   ؛\n}\n");

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("--format")
        .arg(&program)
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_file(&program).expect("تعذر حذف الملف المؤقت");

    assert!(output.status.success(), "يجب أن ينجح --format");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("إذا صحيح {\n    اطبع(\"نعم\")؛\n}\n"),
        "المخرجات يجب أن تكون منسقة: {stdout}"
    );
}

#[test]
fn test_format_flag_wraps_and_formats_comments() {
    let program = temp_program_path(
        "format_wrap_comment",
        "متغير نتيجة = قيمة_اولى + قيمة_ثانية + قيمة_ثالثة + قيمة_رابعة + قيمة_خامسة + قيمة_سادسة؛    //   تعليق   طويل\n",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("--format")
        .arg(&program)
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_file(&program).expect("تعذر حذف الملف المؤقت");

    assert!(output.status.success(), "يجب أن ينجح --format");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("متغير نتيجة = قيمة_اولى + قيمة_ثانية + قيمة_ثالثة + قيمة_رابعة + قيمة_خامسة +\n    قيمة_سادسة؛ // تعليق طويل\n"),
        "المخرجات يجب أن تطبّق التفاف السطر وتنسيق التعليق: {stdout}"
    );
}

#[test]
fn test_help_mentions_ai_repl_commands() {
    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("--help")
        .output()
        .expect("تعذر تشغيل almarjaa");

    assert!(output.status.success(), "يجب أن ينجح --help");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("أوامر ذكاء REPL"),
        "يجب عرض تلميح أوامر الذكاء: {stdout}"
    );
}

#[test]
fn test_lint_flag_reports_warnings() {
    let program = temp_program_path(
        "lint_warn",
        "متغير س = ١٠؛
",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("--lint")
        .arg(&program)
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_file(&program).expect("تعذر حذف الملف المؤقت");

    assert!(output.status.success(), "يجب أن ينجح --lint");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("L001"),
        "يجب إظهار تحذير المتغير غير المستخدم: {stdout}"
    );
}

#[test]
fn test_lint_flag_passes_without_warnings() {
    let program = temp_program_path(
        "lint_ok",
        "متغير س = ١؛
اطبع(س)؛
",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("--lint")
        .arg(&program)
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_file(&program).expect("تعذر حذف الملف المؤقت");

    assert!(output.status.success(), "يجب أن ينجح --lint");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("لا توجد تحذيرات"),
        "يجب تأكيد نجاح lint: {stdout}"
    );
}

#[test]
fn test_lint_flag_supports_disabling_rule() {
    let program = temp_program_path(
        "lint_disable",
        "متغير س = ١٠؛
",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("--lint")
        .arg("--lint-disable")
        .arg("L001")
        .arg(&program)
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_file(&program).expect("تعذر حذف الملف المؤقت");

    assert!(output.status.success(), "يجب أن ينجح --lint");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("لا توجد تحذيرات"),
        "يجب تعطيل التحذير L001 بنجاح: {stdout}"
    );
}

#[test]
fn test_lint_flag_supports_max_diagnostics() {
    let program = temp_program_path(
        "lint_max",
        "متغير أ = ١؛
متغير ب = ٢؛
",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("--lint")
        .arg("--lint-max")
        .arg("1")
        .arg(&program)
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_file(&program).expect("تعذر حذف الملف المؤقت");

    assert!(output.status.success(), "يجب أن ينجح --lint");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lint_code_count = stdout.matches("[L001]").count()
        + stdout.matches("[L002]").count()
        + stdout.matches("[L003]").count()
        + stdout.matches("[L004]").count()
        + stdout.matches("[L005]").count()
        + stdout.matches("[L006]").count()
        + stdout.matches("[L007]").count()
        + stdout.matches("[L008]").count();
    assert_eq!(lint_code_count, 1, "يجب عرض تحذير واحد فقط: {stdout}");
}

#[test]
fn test_pm_check_validates_manifest() {
    let mut dir = std::env::temp_dir();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("تعذر حساب الوقت")
        .as_nanos();
    dir.push(format!("almarjaa_pm_check_{stamp}"));
    fs::create_dir_all(&dir).expect("تعذر إنشاء مجلد مؤقت");

    let manifest = r#"[package]
name = "demo"
version = "0.1.0"
entry = "src/main.mrj"

[dependencies]
core = "^1.0"
"#;
    fs::write(dir.join("mrj.toml"), manifest).expect("تعذر كتابة mrj.toml");

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("--pm-check")
        .current_dir(&dir)
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_dir_all(&dir).expect("تعذر حذف المجلد المؤقت");

    assert!(output.status.success(), "يجب أن ينجح --pm-check");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("mrj.toml صالح"),
        "يجب تأكيد صلاحية manifest: {stdout}"
    );
}

#[test]
fn test_lsp_diag_outputs_json() {
    let program = temp_program_path("lsp_diag", "متغير س = ١٠؛\n");

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("--lsp-diag")
        .arg(&program)
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_file(&program).expect("تعذر حذف الملف المؤقت");

    assert!(output.status.success(), "يجب أن ينجح --lsp-diag");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"source\": \"linter\""),
        "يجب أن يتضمن تشخيص linter بصيغة JSON: {stdout}"
    );
    assert!(
        stdout.contains("\"code\": \"L001\""),
        "يجب أن يتضمن كود التحذير L001: {stdout}"
    );
}

#[test]
fn test_pm_check_generates_lockfile() {
    let mut dir = std::env::temp_dir();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("تعذر حساب الوقت")
        .as_nanos();
    dir.push(format!("almarjaa_pm_lock_{stamp}"));
    fs::create_dir_all(&dir).expect("تعذر إنشاء مجلد مؤقت");

    let manifest = r#"[package]
name = "demo"
version = "0.1.0"
entry = "src/main.mrj"

[dependencies]
core = "^1.0"
json = "^2.0"
"#;
    fs::write(dir.join("mrj.toml"), manifest).expect("تعذر كتابة mrj.toml");

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("pm")
        .arg("check")
        .current_dir(&dir)
        .output()
        .expect("تعذر تشغيل almarjaa");

    assert!(output.status.success(), "يجب أن ينجح pm check");
    let lock_content = fs::read_to_string(dir.join("mrj.lock")).expect("يجب إنشاء lockfile");
    assert!(lock_content.contains("manifest_checksum"));

    fs::remove_dir_all(&dir).expect("تعذر حذف المجلد المؤقت");
}

#[test]
fn test_lsp_definition_subcommand_outputs_location() {
    let program = temp_program_path(
        "lsp_definition",
        "متغير س = ١٠؛
اطبع(س)؛
",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("lsp")
        .arg("definition")
        .arg(&program)
        .arg("2")
        .arg("6")
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_file(&program).expect("تعذر حذف الملف المؤقت");

    assert!(output.status.success(), "يجب أن ينجح lsp definition");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"line\": 1"),
        "يجب إرجاع سطر التعريف: {stdout}"
    );
}

#[test]
fn test_lsp_references_subcommand_outputs_all_symbol_locations() {
    let program = temp_program_path(
        "lsp_references",
        "متغير س = ١٠؛
اطبع(س)؛
س = س + ١؛
",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("lsp")
        .arg("references")
        .arg(&program)
        .arg("2")
        .arg("6")
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_file(&program).expect("تعذر حذف الملف المؤقت");

    assert!(output.status.success(), "يجب أن ينجح lsp references");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\"is_definition\": true"),
        "يجب تضمين موقع التعريف: {stdout}"
    );
    assert!(
        stdout.matches("\"symbol\": \"س\"").count() >= 4,
        "يجب إرجاع كل الإشارات إلى الرمز: {stdout}"
    );
}

#[test]
fn test_lsp_complete_subcommand_outputs_keywords() {
    let program = temp_program_path(
        "lsp_complete",
        "متغير اسم = ١؛
",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg("lsp")
        .arg("complete")
        .arg(&program)
        .arg("م")
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_file(&program).expect("تعذر حذف الملف المؤقت");

    assert!(output.status.success(), "يجب أن ينجح lsp complete");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("متغير"),
        "يجب أن يتضمن keyword عربية: {stdout}"
    );
}

#[test]
fn test_e2e_inventory_restock_workflow() {
    let program = temp_program_path(
        "e2e_inventory",
        r#"
متغير المخزون = {تفاح: ٢، موز: ١}؛
دالة حدّث_المخزون(الصنف، الكمية) {
    إذا الكمية < ٠ {
        أرجع "كمية مرفوضة"؛
    }
    المخزون[الصنف] = المخزون[الصنف] + الكمية؛
    أرجع "تم التحديث"؛
}

اطبع(حدّث_المخزون("تفاح"، ٣))؛
اطبع(المخزون["تفاح"])؛
"انتهى";
"#,
    );

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg(&program)
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_file(&program).expect("تعذر حذف الملف المؤقت");

    assert!(output.status.success(), "يجب أن ينجح سيناريو e2e الواقعي");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("تم التحديث"),
        "يجب طباعة رسالة نجاح: {stdout}"
    );
    assert!(
        stdout.contains("5"),
        "يجب طباعة الرصيد النهائي للصنف: {stdout}"
    );
}

#[test]
fn test_e2e_try_catch_recovers_from_runtime_error() {
    let program = temp_program_path(
        "e2e_try_catch",
        r#"
دالة نفّذ_مهمة(رقم) {
    حاول {
        أرجع ١٠ / رقم؛
    } امسك(خ) {
        اطبع("fallback")؛
        أرجع ٠؛
    }
}

اطبع(نفّذ_مهمة(٠))؛
"#,
    );

    let output = Command::new(env!("CARGO_BIN_EXE_almarjaa"))
        .arg(&program)
        .output()
        .expect("تعذر تشغيل almarjaa");

    fs::remove_file(&program).expect("تعذر حذف الملف المؤقت");

    assert!(
        output.status.success(),
        "يجب أن يتعافى البرنامج عبر try/catch"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("fallback"),
        "يجب تنفيذ مسار التعافي: {stdout}"
    );
    assert!(stdout.contains("0"), "يجب طباعة القيمة البديلة: {stdout}");
}
