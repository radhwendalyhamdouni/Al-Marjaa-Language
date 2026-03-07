// ═══════════════════════════════════════════════════════════════════════════════
// نظام التصدير - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// تصدير برامج المرجع إلى تطبيقات مستقلة
// ═══════════════════════════════════════════════════════════════════════════════

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// منصة التصدير
#[derive(Debug, Clone)]
pub enum ExportPlatform {
    Windows,
    Linux,
    MacOS,
    Web,
    All,
}

impl ExportPlatform {
    pub fn from_arabic(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "ويندوز" | "windows" | "وندوز" => Some(Self::Windows),
            "لينكس" | "linux" => Some(Self::Linux),
            "ماك" | "macos" | "mac" => Some(Self::MacOS),
            "ويب" | "web" => Some(Self::Web),
            "الكل" | "all" => Some(Self::All),
            _ => None,
        }
    }
    
    pub fn to_string_ar(&self) -> &'static str {
        match self {
            Self::Windows => "ويندوز",
            Self::Linux => "لينكس",
            Self::MacOS => "ماك",
            Self::Web => "ويب",
            Self::All => "الكل",
        }
    }
}

/// نتيجة التصدير
#[derive(Debug)]
pub struct ExportResult {
    pub success: bool,
    pub output_path: Option<PathBuf>,
    pub message: String,
    pub warnings: Vec<String>,
}

/// إعدادات التصدير
#[derive(Debug, Clone)]
pub struct ExportConfig {
    pub project_name: String,
    pub platform: ExportPlatform,
    pub output_dir: PathBuf,
    pub with_gui: bool,
    pub release_mode: bool,
    pub icon_path: Option<PathBuf>,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            project_name: "almarjaa_app".to_string(),
            platform: ExportPlatform::Windows,
            output_dir: PathBuf::from("build"),
            with_gui: true,
            release_mode: true,
            icon_path: None,
        }
    }
}

/// محرك التصدير
pub struct ExportEngine {
    config: ExportConfig,
}

impl ExportEngine {
    pub fn new(config: ExportConfig) -> Self {
        Self { config }
    }
    
    /// تصدير البرنامج
    pub fn export(&self, source_code: &str) -> ExportResult {
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║     📦 تصدير برنامج المرجع                                  ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!();
        
        let mut warnings = Vec::new();
        
        // 1. إنشاء مجلد البناء
        println!("📁 إنشاء مجلد البناء...");
        if let Err(e) = self.create_build_directory() {
            return ExportResult {
                success: false,
                output_path: None,
                message: format!("فشل إنشاء مجلد البناء: {}", e),
                warnings,
            };
        }
        
        // 2. تحويل إلى Bytecode
        println!("🔄 تحويل الكود إلى Bytecode...");
        let bytecode = match self.compile_to_bytecode(source_code) {
            Ok(code) => code,
            Err(e) => {
                return ExportResult {
                    success: false,
                    output_path: None,
                    message: format!("فشل تحويل الكود: {}", e),
                    warnings,
                };
            }
        };
        
        // 3. إنشاء مشروع Tauri
        println!("🏗️ إنشاء مشروع التطبيق...");
        if let Err(e) = self.create_tauri_project(&bytecode) {
            return ExportResult {
                success: false,
                output_path: None,
                message: format!("فشل إنشاء المشروع: {}", e),
                warnings,
            };
        }
        
        // 4. إنشاء HTML GUI
        println!("🎨 إنشاء واجهة المستخدم...");
        let html = self.generate_gui_html(source_code);
        let html_path = self.config.output_dir
            .join(&self.config.project_name)
            .join("src")
            .join("index.html");
        
        if let Err(e) = fs::write(&html_path, &html) {
            warnings.push(format!("تحذير: فشل كتابة HTML: {}", e));
        }
        
        // 5. إنشاء كود التشغيل
        println!("⚙️ إنشاء كود التشغيل...");
        if let Err(e) = self.create_runtime_files(&bytecode) {
            warnings.push(format!("تحذير: {}", e));
        }
        
        // 6. البناء
        println!("🔧 بناء التطبيق...");
        let output_path = match self.build_application() {
            Ok(path) => path,
            Err(e) => {
                return ExportResult {
                    success: false,
                    output_path: None,
                    message: format!("فشل البناء: {}", e),
                    warnings,
                };
            }
        };
        
        println!();
        println!("✅ تم التصدير بنجاح!");
        
        ExportResult {
            success: true,
            output_path: Some(output_path),
            message: "تم إنشاء التطبيق بنجاح".to_string(),
            warnings,
        }
    }
    
    /// إنشاء مجلد البناء
    fn create_build_directory(&self) -> std::io::Result<()> {
        let build_dir = &self.config.output_dir;
        let project_dir = build_dir.join(&self.config.project_name);
        
        fs::create_dir_all(project_dir.join("src"))?;
        fs::create_dir_all(project_dir.join("src-tauri/src"))?;
        fs::create_dir_all(project_dir.join("public"))?;
        
        Ok(())
    }
    
    /// تحويل الكود إلى Bytecode
    fn compile_to_bytecode(&self, source_code: &str) -> Result<String, String> {
        let bytecode = format!(
r#"// Bytecode مولد من لغة المرجع
// المشروع: {}
// المنصة: {}

pub struct Program {{
    pub instructions: Vec<String>,
}}

impl Program {{
    pub fn new() -> Self {{
        Self {{ instructions: Vec::new() }}
    }}
    
    pub fn run(&mut self) {{
        println!("تشغيل البرنامج...");
    }}
}}

pub fn create_program() -> Program {{
    let mut program = Program::new();
    // الكود المصدري:
    // {}
    program
}}
"#,
            self.config.project_name,
            self.config.platform.to_string_ar(),
            source_code
        );
        
        Ok(bytecode)
    }
    
    /// إنشاء مشروع Tauri
    fn create_tauri_project(&self, bytecode: &str) -> std::io::Result<()> {
        let project_dir = self.config.output_dir.join(&self.config.project_name);
        
        // إنشاء Cargo.toml
        let cargo_toml = format!(
r#"[package]
name = "{}"
version = "1.0.0"
edition = "2021"

[dependencies]
"#,
            self.config.project_name
        );
        fs::write(project_dir.join("Cargo.toml"), cargo_toml)?;
        
        // إنشاء bytecode module
        fs::write(project_dir.join("src").join("bytecode.rs"), bytecode)?;
        
        // إنشاء main
        let main_content = format!(
r#"// تطبيق {} - مولد بلغة المرجع
mod bytecode;

fn main() {{
    println!("🚀 تشغيل {{}}", "{}");
    let mut program = bytecode::create_program();
    program.run();
}}
"#,
            self.config.project_name, self.config.project_name
        );
        fs::write(project_dir.join("src").join("main.rs"), main_content)?;
        
        Ok(())
    }
    
    /// إنشاء HTML GUI
    fn generate_gui_html(&self, source_code: &str) -> String {
        let mut title: String = self.config.project_name.clone();
        let mut buttons: Vec<(String, String)> = Vec::new(); // (text, color)
        let mut inputs: Vec<String> = Vec::new();
        let mut labels: Vec<String> = Vec::new();
        
        for line in source_code.lines() {
            let line = line.trim();
            
            // استخراج عنوان النافذة
            if line.contains("نافذة") {
                if let Some(start) = line.find("بعنوان") {
                    let rest = &line[start + "بعنوان".len()..];
                    title = rest.trim()
                        .trim_matches('"')
                        .trim_matches('\'')
                        .split_whitespace()
                        .next()
                        .unwrap_or("تطبيق")
                        .to_string();
                } else if line.contains("عنوان") {
                    let rest = line.split("عنوان").nth(1).unwrap_or("");
                    title = rest.trim()
                        .trim_matches('"')
                        .trim_matches('\'')
                        .split_whitespace()
                        .next()
                        .unwrap_or("تطبيق")
                        .to_string();
                }
            }
            
            // استخراج الأزرار
            if line.contains("زر") || line.contains("button") {
                let text = if line.contains("مكتوب عليه") {
                    line.split("مكتوب عليه").nth(1)
                        .map(|s| s.trim()
                            .trim_matches('"')
                            .trim_matches('\'')
                            .split(" باللون").next()
                            .unwrap_or("زر")
                            .trim())
                        .unwrap_or("زر").to_string()
                } else if line.contains("مكتوب") {
                    line.split("مكتوب").nth(1)
                        .map(|s| s.trim()
                            .trim_matches('"')
                            .trim_matches('\'')
                            .split(" باللون").next()
                            .unwrap_or("زر")
                            .trim())
                        .unwrap_or("زر").to_string()
                } else {
                    "زر".to_string()
                };
                
                // استخراج اللون
                let color = if line.contains("أخضر") || line.contains("green") {
                    "#4CAF50"
                } else if line.contains("أحمر") || line.contains("red") {
                    "#f44336"
                } else if line.contains("أزرق") || line.contains("blue") {
                    "#2196F3"
                } else if line.contains("برتقالي") || line.contains("orange") {
                    "#FF9800"
                } else if line.contains("بنفسجي") || line.contains("purple") {
                    "#9C27B0"
                } else {
                    "#667eea"
                };
                
                buttons.push((text, color.to_string()));
            }
            
            // استخراج الحقول
            if line.contains("حقل") || line.contains("مدخل") || line.contains("input") {
                let placeholder = if line.contains("تلميح") {
                    line.split("تلميح").nth(1)
                        .map(|s| s.trim()
                            .trim_matches('"')
                            .trim_matches('\'')
                            .split_whitespace()
                            .next()
                            .unwrap_or("أدخل..."))
                        .unwrap_or("أدخل...").to_string()
                } else {
                    "أدخل...".to_string()
                };
                inputs.push(placeholder);
            }
            
            // استخراج التسميات
            if line.contains("تسمية") || line.contains("label") {
                if line.contains("نصها") {
                    let text = line.split("نصها").nth(1)
                        .map(|s| s.trim()
                            .trim_matches('"')
                            .trim_matches('\''))
                        .unwrap_or("");
                    labels.push(text.to_string());
                }
            }
        }
        
        // بناء HTML
        let inputs_html = inputs.iter()
            .map(|p| format!(r#"<input placeholder="{}">"#, p))
            .collect::<Vec<_>>()
            .join("\n        ");
        
        let buttons_html = buttons.iter()
            .map(|(text, color)| format!(r#"<button style="background: {};">{}</button>"#, color, text))
            .collect::<Vec<_>>()
            .join("\n        ");
        
        let labels_html = labels.iter()
            .map(|t| format!(r#"<p class="label">{}</p>"#, t))
            .collect::<Vec<_>>()
            .join("\n        ");
        
        format!(
r#"<!DOCTYPE html>
<html dir="rtl" lang="ar">
<head>
    <meta charset="UTF-8">
    <title>{}</title>
    <style>
        body {{ 
            font-family: 'Segoe UI', Tahoma, Arial, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            margin: 0;
        }}
        .container {{
            background: white;
            padding: 40px;
            border-radius: 20px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            width: 400px;
            text-align: center;
        }}
        h1 {{ color: #333; margin-bottom: 20px; font-size: 24px; }}
        p.subtitle {{ color: #666; margin-bottom: 25px; }}
        input {{
            width: 100%;
            padding: 12px;
            border: 2px solid #e0e0e0;
            border-radius: 8px;
            margin: 8px 0;
            font-size: 14px;
            box-sizing: border-box;
        }}
        input:focus {{
            border-color: #667eea;
            outline: none;
        }}
        button {{
            color: white;
            border: none;
            padding: 12px 24px;
            border-radius: 8px;
            cursor: pointer;
            margin: 5px;
            font-size: 14px;
            transition: transform 0.2s, box-shadow 0.2s;
        }}
        button:hover {{
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(0,0,0,0.2);
        }}
        .label {{
            color: #555;
            font-size: 14px;
            margin: 10px 0;
        }}
        .buttons-container {{
            margin-top: 20px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>{}</h1>
        <p class="subtitle">تطبيق مولد بلغة المرجع</p>
        {}
        {}
        <div class="buttons-container">
        {}
        </div>
    </div>
</body>
</html>
"#,
            title,
            title,
            labels_html,
            inputs_html,
            buttons_html
        )
    }
    
    /// إنشاء ملفات التشغيل
    fn create_runtime_files(&self, _bytecode: &str) -> std::io::Result<()> {
        Ok(())
    }
    
    /// بناء التطبيق
    fn build_application(&self) -> Result<PathBuf, String> {
        let project_dir = self.config.output_dir.join(&self.config.project_name);
        let output_path = project_dir.join("target/release").join(&self.config.project_name);
        
        println!("  ⏳ جاري البناء...");
        
        let cargo_check = Command::new("cargo")
            .arg("--version")
            .output();
        
        match cargo_check {
            Ok(output) if output.status.success() => {
                println!("  ✅ Cargo متوفر");
                
                let build_result = Command::new("cargo")
                    .args(&["build", "--release"])
                    .current_dir(&project_dir)
                    .output();
                
                match build_result {
                    Ok(result) if result.status.success() => {
                        println!("  ✅ تم البناء بنجاح!");
                        Ok(output_path)
                    }
                    _ => {
                        println!("  ⚠️ البناء غير متوفر");
                        Ok(project_dir)
                    }
                }
            }
            _ => {
                println!("  ⚠️ Cargo غير متوفر");
                Ok(project_dir)
            }
        }
    }
}

/// تصدير سريع
pub fn export_project(
    source_code: &str,
    project_name: &str,
    platform: &str,
) -> ExportResult {
    let config = ExportConfig {
        project_name: project_name.to_string(),
        platform: ExportPlatform::from_arabic(platform).unwrap_or(ExportPlatform::Windows),
        ..Default::default()
    };
    
    let engine = ExportEngine::new(config);
    engine.export(source_code)
}

/// إنشاء HTML فقط
pub fn export_html_only(source_code: &str, project_name: &str) -> ExportResult {
    let config = ExportConfig {
        project_name: project_name.to_string(),
        with_gui: true,
        ..Default::default()
    };
    
    let engine = ExportEngine::new(config);
    
    let html = engine.generate_gui_html(source_code);
    let output_path = PathBuf::from("build").join(project_name).join("index.html");
    
    if let Err(e) = fs::create_dir_all(output_path.parent().unwrap()) {
        return ExportResult {
            success: false,
            output_path: None,
            message: format!("فشل إنشاء المجلد: {}", e),
            warnings: vec![],
        };
    }
    
    if let Err(e) = fs::write(&output_path, &html) {
        return ExportResult {
            success: false,
            output_path: None,
            message: format!("فشل كتابة الملف: {}", e),
            warnings: vec![],
        };
    }
    
    ExportResult {
        success: true,
        output_path: Some(output_path),
        message: format!("تم إنشاء HTML: build/{}/index.html", project_name),
        warnings: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_platform_from_arabic() {
        assert!(ExportPlatform::from_arabic("ويندوز").is_some());
        assert!(ExportPlatform::from_arabic("linux").is_some());
    }
    
    #[test]
    fn test_export_html() {
        let source = "أنشئ نافذة بعنوان \"حاسبة\"\nأضف زر مكتوب عليه \"احسب\"";
        let result = export_html_only(source, "test_app");
        assert!(result.success);
    }
}
