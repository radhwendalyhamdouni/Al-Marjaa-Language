// ═══════════════════════════════════════════════════════════════════════════════
// نظام تصدير الهواتف المحمولة - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// تصدير برامج المرجع إلى تطبيقات Android و iOS
// - دعم React Native و Flutter و Capacitor
// - توليد كود أصلي (Kotlin/Swift)
// - تكامل مع WASM للأداء الأمثل
// ═══════════════════════════════════════════════════════════════════════════════

pub mod templates;

use std::path::PathBuf;
use std::fs;

/// منصة الهاتف المحمول
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobilePlatform {
    Android,
    iOS,
    Both,
}

impl MobilePlatform {
    /// التحويل من اسم عربي
    pub fn from_arabic(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "أندرويد" | "android" | "اندرويد" => Some(Self::Android),
            "آيفون" | "ios" | "ايفون" | "iphone" => Some(Self::iOS),
            "كلاهما" | "both" | "الكل" => Some(Self::Both),
            _ => None,
        }
    }
    
    /// الاسم بالعربية
    pub fn to_string_ar(&self) -> &'static str {
        match self {
            Self::Android => "أندرويد",
            Self::iOS => "آيفون",
            Self::Both => "كلاهما",
        }
    }
}

/// إطار العمل المحمول
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobileFramework {
    ReactNative,
    Flutter,
    Native,
    Capacitor,
}

impl MobileFramework {
    pub fn from_arabic(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "ريأكت" | "react-native" | "react" => Some(Self::ReactNative),
            "فلاتر" | "flutter" => Some(Self::Flutter),
            "أصلي" | "native" => Some(Self::Native),
            "كاباسيتور" | "capacitor" => Some(Self::Capacitor),
            _ => None,
        }
    }
    
    pub fn to_string_ar(&self) -> &'static str {
        match self {
            Self::ReactNative => "React Native",
            Self::Flutter => "Flutter",
            Self::Native => "أصلي",
            Self::Capacitor => "Capacitor",
        }
    }
}

/// نتيجة التصدير
#[derive(Debug)]
pub struct MobileExportResult {
    pub success: bool,
    pub output_path: Option<PathBuf>,
    pub message: String,
    pub platform: MobilePlatform,
    pub framework: MobileFramework,
    pub build_commands: Vec<String>,
    pub warnings: Vec<String>,
}

/// إعدادات تصدير الهاتف
#[derive(Debug, Clone)]
pub struct MobileExportConfig {
    /// اسم المشروع
    pub project_name: String,
    /// المنصة المستهدفة
    pub platform: MobilePlatform,
    /// إطار العمل
    pub framework: MobileFramework,
    /// مجلد الإخراج
    pub output_dir: PathBuf,
    /// اسم الحزمة (com.example.app)
    pub package_name: String,
    /// أيقونة التطبيق
    pub icon_path: Option<PathBuf>,
    /// لون السمة الرئيسي
    pub primary_color: String,
    /// دعم RTL
    pub rtl_support: bool,
    /// وضع الإصدار
    pub release_mode: bool,
    /// تفعيل التوقيع
    pub signing: bool,
    /// keystore path للتوقيع
    pub keystore_path: Option<PathBuf>,
    /// وصف التطبيق
    pub description: String,
    /// إصدار التطبيق
    pub version: String,
}

impl Default for MobileExportConfig {
    fn default() -> Self {
        Self {
            project_name: "almarjaa_app".to_string(),
            platform: MobilePlatform::Android,
            framework: MobileFramework::Flutter,
            output_dir: PathBuf::from("build/mobile"),
            package_name: "com.almarjaa.app".to_string(),
            icon_path: None,
            primary_color: "#667eea".to_string(),
            rtl_support: true,
            release_mode: false,
            signing: false,
            keystore_path: None,
            description: "تطبيق مولد بلغة المرجع".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

/// محرك تصدير الهواتف المحمولة
pub struct MobileExporter {
    config: MobileExportConfig,
}

impl MobileExporter {
    /// إنشاء محرك جديد
    pub fn new(config: MobileExportConfig) -> Self {
        Self { config }
    }
    
    /// تصدير التطبيق
    pub fn export(&self, source_code: &str) -> MobileExportResult {
        self.print_banner();
        
        let mut warnings = Vec::new();
        let mut build_commands = Vec::new();
        
        // 1. التحقق من المتطلبات
        println!("🔍 التحقق من المتطلبات...");
        if let Err(e) = self.check_requirements() {
            warnings.push(format!("تحذير: {}", e));
        }
        
        // 2. إنشاء مجلد المشروع
        println!("📁 إنشاء مجلد المشروع...");
        if let Err(e) = self.create_project_directory() {
            return MobileExportResult {
                success: false,
                output_path: None,
                message: format!("فشل إنشاء المجلد: {}", e),
                platform: self.config.platform,
                framework: self.config.framework,
                build_commands,
                warnings,
            };
        }
        
        // 3. تحويل الكود إلى WASM
        println!("🔄 تحويل الكود إلى WASM...");
        let wasm_bytes = match self.compile_to_wasm(source_code) {
            Ok(bytes) => bytes,
            Err(e) => {
                return MobileExportResult {
                    success: false,
                    output_path: None,
                    message: format!("فشل تحويل WASM: {}", e),
                    platform: self.config.platform,
                    framework: self.config.framework,
                    build_commands,
                    warnings,
                };
            }
        };
        
        // 4. إنشاء المشروع حسب الإطار
        let output_path = match self.config.framework {
            MobileFramework::Flutter => {
                self.export_flutter(&wasm_bytes, &mut build_commands, &mut warnings)
            }
            MobileFramework::ReactNative => {
                self.export_react_native(&wasm_bytes, &mut build_commands, &mut warnings)
            }
            MobileFramework::Native => {
                self.export_native(&wasm_bytes, &mut build_commands, &mut warnings)
            }
            MobileFramework::Capacitor => {
                self.export_capacitor(&wasm_bytes, &mut build_commands, &mut warnings)
            }
        };
        
        match output_path {
            Ok(path) => {
                println!();
                println!("✅ تم إنشاء مشروع الهاتف بنجاح!");
                self.print_next_steps(&build_commands);
                
                MobileExportResult {
                    success: true,
                    output_path: Some(path),
                    message: "تم إنشاء المشروع بنجاح".to_string(),
                    platform: self.config.platform,
                    framework: self.config.framework,
                    build_commands,
                    warnings,
                }
            }
            Err(e) => {
                MobileExportResult {
                    success: false,
                    output_path: None,
                    message: e,
                    platform: self.config.platform,
                    framework: self.config.framework,
                    build_commands,
                    warnings,
                }
            }
        }
    }
    
    /// طباعة البانر
    fn print_banner(&self) {
        println!();
        println!("╔════════════════════════════════════════════════════════════════════════╗");
        println!("║         📱 تصدير تطبيقات الهواتف - لغة المرجع                          ║");
        println!("╠════════════════════════════════════════════════════════════════════════╣");
        println!("║  المشروع: {:<58}║", self.config.project_name);
        println!("║  المنصة: {:<59}║", self.config.platform.to_string_ar());
        println!("║  الإطار: {:<58}║", self.config.framework.to_string_ar());
        println!("╚════════════════════════════════════════════════════════════════════════╝");
        println!();
    }
    
    /// التحقق من المتطلبات
    fn check_requirements(&self) -> Result<(), String> {
        match self.config.framework {
            MobileFramework::Flutter => {
                if !self.command_exists("flutter") {
                    return Err("Flutter غير مثبت. قم بتثبيته من: https://flutter.dev".into());
                }
            }
            MobileFramework::ReactNative | MobileFramework::Capacitor => {
                if !self.command_exists("node") {
                    return Err("Node.js غير مثبت. قم بتثبيته من: https://nodejs.org".into());
                }
            }
            _ => {}
        }
        
        if self.config.platform == MobilePlatform::iOS && cfg!(not(target_os = "macos")) {
            return Err("بناء iOS يتطلب جهاز macOS".into());
        }
        
        Ok(())
    }
    
    /// التحقق من وجود أمر
    fn command_exists(&self, cmd: &str) -> bool {
        std::process::Command::new(cmd)
            .arg("--version")
            .output()
            .is_ok()
    }
    
    /// إنشاء مجلد المشروع
    fn create_project_directory(&self) -> std::io::Result<()> {
        let project_dir = self.config.output_dir.join(&self.config.project_name);
        fs::create_dir_all(&project_dir)?;
        Ok(())
    }
    
    /// تحويل إلى WASM
    fn compile_to_wasm(&self, source_code: &str) -> Result<Vec<u8>, String> {
        // إنشاء WASM بسيط للمثال
        let wasm_header = vec![
            0x00, 0x61, 0x73, 0x6D, // Magic number
            0x01, 0x00, 0x00, 0x00, // Version
        ];
        
        let source_bytes = source_code.as_bytes();
        let mut wasm = wasm_header;
        wasm.extend_from_slice(source_bytes);
        
        Ok(wasm)
    }
    
    /// تصدير Flutter
    fn export_flutter(
        &self,
        wasm_bytes: &[u8],
        build_commands: &mut Vec<String>,
        warnings: &mut Vec<String>,
    ) -> Result<PathBuf, String> {
        println!("🦋 إنشاء مشروع Flutter...");
        
        let project_dir = self.config.output_dir.join(&self.config.project_name);
        
        // إنشاء هيكل المشروع
        let dirs = vec![
            "lib",
            "lib/screens",
            "lib/widgets",
            "lib/runtime",
            "lib/i18n",
            "assets",
            "assets/wasm",
            "android/app/src/main/res/values",
            "android/app/src/main/res/values-ar",
            "ios/Runner",
        ];
        
        for dir in &dirs {
            fs::create_dir_all(project_dir.join(dir))
                .map_err(|e| format!("فشل إنشاء {}: {}", dir, e))?;
        }
        
        // إنشاء الملفات
        fs::write(project_dir.join("pubspec.yaml"), templates::flutter::generate_pubspec(&self.config))
            .map_err(|e| format!("فشل كتابة pubspec.yaml: {}", e))?;
        
        fs::write(project_dir.join("lib/main.dart"), templates::flutter::generate_main(&self.config))
            .map_err(|e| format!("فشل كتابة main.dart: {}", e))?;
        
        fs::write(project_dir.join("lib/app_theme.dart"), templates::flutter::generate_theme(&self.config))
            .map_err(|e| format!("فشل كتابة app_theme.dart: {}", e))?;
        
        fs::write(project_dir.join("lib/screens/home_screen.dart"), templates::flutter::generate_home_screen(&self.config))
            .map_err(|e| format!("فشل كتابة home_screen.dart: {}", e))?;
        
        fs::write(project_dir.join("lib/runtime/wasm_runtime.dart"), templates::flutter::generate_wasm_runtime(&self.config))
            .map_err(|e| format!("فشل كتابة wasm_runtime.dart: {}", e))?;
        
        fs::write(project_dir.join("lib/i18n/strings.dart"), templates::flutter::generate_strings(&self.config))
            .map_err(|e| format!("فشل كتابة strings.dart: {}", e))?;
        
        // حفظ WASM
        fs::write(project_dir.join("assets/wasm/app.wasm"), wasm_bytes)
            .map_err(|e| format!("فشل حفظ WASM: {}", e))?;
        
        // أوامر البناء
        build_commands.push(format!("cd {}", project_dir.display()));
        build_commands.push("flutter pub get".to_string());
        
        match self.config.platform {
            MobilePlatform::Android => {
                if self.config.release_mode {
                    build_commands.push("flutter build apk --release".to_string());
                } else {
                    build_commands.push("flutter build apk --debug".to_string());
                }
            }
            MobilePlatform::iOS => {
                if self.config.release_mode {
                    build_commands.push("flutter build ios --release".to_string());
                } else {
                    build_commands.push("flutter build ios --debug".to_string());
                }
            }
            MobilePlatform::Both => {
                build_commands.push("flutter build apk --release".to_string());
                build_commands.push("flutter build ios --release".to_string());
            }
        }
        
        Ok(project_dir)
    }
    
    /// تصدير React Native
    fn export_react_native(
        &self,
        wasm_bytes: &[u8],
        build_commands: &mut Vec<String>,
        warnings: &mut Vec<String>,
    ) -> Result<PathBuf, String> {
        println!("⚛️ إنشاء مشروع React Native...");
        
        let project_dir = self.config.output_dir.join(&self.config.project_name);
        
        let dirs = vec![
            "src",
            "src/screens",
            "src/components",
            "src/runtime",
            "src/i18n",
            "assets/wasm",
        ];
        
        for dir in &dirs {
            fs::create_dir_all(project_dir.join(dir))
                .map_err(|e| format!("فشل إنشاء {}: {}", dir, e))?;
        }
        
        // إنشاء الملفات
        fs::write(project_dir.join("package.json"), templates::react_native::generate_package_json(&self.config))
            .map_err(|e| format!("فشل كتابة package.json: {}", e))?;
        
        fs::write(project_dir.join("App.tsx"), templates::react_native::generate_app(&self.config))
            .map_err(|e| format!("فشل كتابة App.tsx: {}", e))?;
        
        fs::write(project_dir.join("src/screens/HomeScreen.tsx"), templates::react_native::generate_home_screen(&self.config))
            .map_err(|e| format!("فشل كتابة HomeScreen.tsx: {}", e))?;
        
        fs::write(project_dir.join("src/runtime/wasmRuntime.ts"), templates::react_native::generate_wasm_runtime(&self.config))
            .map_err(|e| format!("فشل كتابة wasmRuntime.ts: {}", e))?;
        
        fs::write(project_dir.join("src/i18n/rtl.ts"), templates::react_native::generate_rtl_utils(&self.config))
            .map_err(|e| format!("فشل كتابة rtl.ts: {}", e))?;
        
        // حفظ WASM
        fs::write(project_dir.join("assets/wasm/app.wasm"), wasm_bytes)
            .map_err(|e| format!("فشل حفظ WASM: {}", e))?;
        
        // أوامر البناء
        build_commands.push(format!("cd {}", project_dir.display()));
        build_commands.push("npm install".to_string());
        
        match self.config.platform {
            MobilePlatform::Android => {
                build_commands.push("npx react-native run-android".to_string());
            }
            MobilePlatform::iOS => {
                build_commands.push("cd ios && pod install && cd ..".to_string());
                build_commands.push("npx react-native run-ios".to_string());
            }
            MobilePlatform::Both => {
                build_commands.push("npx react-native run-android".to_string());
                build_commands.push("cd ios && pod install && cd ..".to_string());
                build_commands.push("npx react-native run-ios".to_string());
            }
        }
        
        Ok(project_dir)
    }
    
    /// تصدير Native
    fn export_native(
        &self,
        wasm_bytes: &[u8],
        build_commands: &mut Vec<String>,
        warnings: &mut Vec<String>,
    ) -> Result<PathBuf, String> {
        let project_dir = self.config.output_dir.join(&self.config.project_name);
        
        match self.config.platform {
            MobilePlatform::Android | MobilePlatform::Both => {
                self.export_android_native(wasm_bytes, build_commands, &project_dir)?;
            }
            _ => {}
        }
        
        match self.config.platform {
            MobilePlatform::iOS | MobilePlatform::Both => {
                if cfg!(target_os = "macos") {
                    self.export_ios_native(wasm_bytes, build_commands, &project_dir)?;
                } else {
                    warnings.push("تخطي iOS: يتطلب macOS".to_string());
                }
            }
            _ => {}
        }
        
        Ok(project_dir)
    }
    
    /// تصدير Android Native
    fn export_android_native(
        &self,
        wasm_bytes: &[u8],
        build_commands: &mut Vec<String>,
        project_dir: &PathBuf,
    ) -> Result<(), String> {
        println!("🤖 إنشاء مشروع Android Native...");
        
        let android_dir = project_dir.join("android");
        
        let dirs = vec![
            "app/src/main/java/com/almarjaa/app",
            "app/src/main/res/layout",
            "app/src/main/res/values",
            "app/src/main/res/values-ar",
            "app/src/main/res/drawable",
            "app/src/main/assets/wasm",
        ];
        
        for dir in &dirs {
            fs::create_dir_all(android_dir.join(dir))
                .map_err(|e| format!("فشل إنشاء {}: {}", dir, e))?;
        }
        
        // إنشاء الملفات
        fs::write(android_dir.join("app/build.gradle"), templates::android::generate_build_gradle(&self.config))
            .map_err(|e| format!("فشل كتابة build.gradle: {}", e))?;
        
        fs::write(android_dir.join("settings.gradle"), templates::android::generate_settings_gradle(&self.config))
            .map_err(|e| format!("فشل كتابة settings.gradle: {}", e))?;
        
        fs::write(android_dir.join("build.gradle"), templates::android::generate_root_build_gradle(&self.config))
            .map_err(|e| format!("فشل كتابة root build.gradle: {}", e))?;
        
        fs::write(
            android_dir.join("app/src/main/java/com/almarjaa/app/MainActivity.kt"),
            templates::android::generate_main_activity(&self.config)
        ).map_err(|e| format!("فشل كتابة MainActivity.kt: {}", e))?;
        
        fs::write(
            android_dir.join("app/src/main/java/com/almarjaa/app/WasmRuntime.kt"),
            templates::android::generate_wasm_runtime(&self.config)
        ).map_err(|e| format!("فشل كتابة WasmRuntime.kt: {}", e))?;
        
        fs::write(
            android_dir.join("app/src/main/AndroidManifest.xml"),
            templates::android::generate_manifest(&self.config)
        ).map_err(|e| format!("فشل كتابة AndroidManifest.xml: {}", e))?;
        
        fs::write(
            android_dir.join("app/src/main/res/layout/activity_main.xml"),
            templates::android::generate_main_layout(&self.config)
        ).map_err(|e| format!("فشل كتابة activity_main.xml: {}", e))?;
        
        fs::write(
            android_dir.join("app/src/main/res/values/strings.xml"),
            templates::android::generate_strings(&self.config)
        ).map_err(|e| format!("فشل كتابة strings.xml: {}", e))?;
        
        fs::write(
            android_dir.join("app/src/main/res/values-ar/strings.xml"),
            templates::android::generate_strings_ar(&self.config)
        ).map_err(|e| format!("فشل كتابة strings-ar.xml: {}", e))?;
        
        // حفظ WASM
        fs::write(android_dir.join("app/src/main/assets/wasm/app.wasm"), wasm_bytes)
            .map_err(|e| format!("فشل حفظ WASM: {}", e))?;
        
        // أوامر البناء
        build_commands.push(format!("cd {}", android_dir.display()));
        build_commands.push("./gradlew assembleDebug".to_string());
        
        if self.config.release_mode {
            build_commands.push("./gradlew assembleRelease".to_string());
        }
        
        Ok(())
    }
    
    /// تصدير iOS Native
    fn export_ios_native(
        &self,
        wasm_bytes: &[u8],
        build_commands: &mut Vec<String>,
        project_dir: &PathBuf,
    ) -> Result<(), String> {
        println!("🍎 إنشاء مشروع iOS Native...");
        
        let ios_dir = project_dir.join("ios");
        
        let dirs = vec![
            "AlmarjaaApp",
            "AlmarjaaApp/Resources",
            "AlmarjaaApp/Resources/ar.lproj",
            "AlmarjaaApp/Resources/Base.lproj",
        ];
        
        for dir in &dirs {
            fs::create_dir_all(ios_dir.join(dir))
                .map_err(|e| format!("فشل إنشاء {}: {}", dir, e))?;
        }
        
        // إنشاء الملفات
        fs::write(ios_dir.join("AlmarjaaApp/AppDelegate.swift"), templates::ios::generate_app_delegate(&self.config))
            .map_err(|e| format!("فشل كتابة AppDelegate.swift: {}", e))?;
        
        fs::write(ios_dir.join("AlmarjaaApp/ViewController.swift"), templates::ios::generate_view_controller(&self.config))
            .map_err(|e| format!("فشل كتابة ViewController.swift: {}", e))?;
        
        fs::write(ios_dir.join("AlmarjaaApp/WasmRuntime.swift"), templates::ios::generate_wasm_runtime(&self.config))
            .map_err(|e| format!("فشل كتابة WasmRuntime.swift: {}", e))?;
        
        fs::write(ios_dir.join("AlmarjaaApp/Info.plist"), templates::ios::generate_info_plist(&self.config))
            .map_err(|e| format!("فشل كتابة Info.plist: {}", e))?;
        
        fs::write(
            ios_dir.join("AlmarjaaApp/Resources/Base.lproj/Main.storyboard"),
            templates::ios::generate_main_storyboard(&self.config)
        ).map_err(|e| format!("فشل كتابة Main.storyboard: {}", e))?;
        
        fs::write(
            ios_dir.join("AlmarjaaApp/Resources/ar.lproj/Localizable.strings"),
            templates::ios::generate_localizable(&self.config)
        ).map_err(|e| format!("فشل كتابة Localizable.strings: {}", e))?;
        
        // حفظ WASM
        fs::write(ios_dir.join("AlmarjaaApp/Resources/app.wasm"), wasm_bytes)
            .map_err(|e| format!("فشل حفظ WASM: {}", e))?;
        
        // أوامر البناء
        build_commands.push(format!("cd {}", ios_dir.display()));
        build_commands.push("xcodebuild -scheme AlmarjaaApp -sdk iphoneos".to_string());
        
        Ok(())
    }
    
    /// تصدير Capacitor
    fn export_capacitor(
        &self,
        wasm_bytes: &[u8],
        build_commands: &mut Vec<String>,
        warnings: &mut Vec<String>,
    ) -> Result<PathBuf, String> {
        println!("⚡ إنشاء مشروع Capacitor...");
        
        let project_dir = self.config.output_dir.join(&self.config.project_name);
        
        let dirs = vec!["src", "public", "android", "ios"];
        
        for dir in &dirs {
            fs::create_dir_all(project_dir.join(dir))
                .map_err(|e| format!("فشل إنشاء {}: {}", dir, e))?;
        }
        
        // إنشاء الملفات
        fs::write(project_dir.join("package.json"), templates::capacitor::generate_package_json(&self.config))
            .map_err(|e| format!("فشل كتابة package.json: {}", e))?;
        
        fs::write(project_dir.join("capacitor.config.json"), templates::capacitor::generate_config(&self.config))
            .map_err(|e| format!("فشل كتابة capacitor.config.json: {}", e))?;
        
        fs::write(project_dir.join("public/index.html"), templates::capacitor::generate_index_html(&self.config))
            .map_err(|e| format!("فشل كتابة index.html: {}", e))?;
        
        fs::write(project_dir.join("src/main.js"), templates::capacitor::generate_main_js(&self.config))
            .map_err(|e| format!("فشل كتابة main.js: {}", e))?;
        
        fs::write(project_dir.join("src/styles.css"), templates::capacitor::generate_styles(&self.config))
            .map_err(|e| format!("فشل كتابة styles.css: {}", e))?;
        
        // حفظ WASM
        fs::write(project_dir.join("public/app.wasm"), wasm_bytes)
            .map_err(|e| format!("فشل حفظ WASM: {}", e))?;
        
        // أوامر البناء
        build_commands.push(format!("cd {}", project_dir.display()));
        build_commands.push("npm install".to_string());
        build_commands.push("npx cap add android".to_string());
        
        if self.config.platform == MobilePlatform::iOS || self.config.platform == MobilePlatform::Both {
            build_commands.push("npx cap add ios".to_string());
        }
        
        build_commands.push("npx cap sync".to_string());
        
        Ok(project_dir)
    }
    
    /// طباعة الخطوات التالية
    fn print_next_steps(&self, build_commands: &[String]) {
        println!();
        println!("📋 الخطوات التالية:");
        println!("─────────────────────────────────────────────────────────────");
        for cmd in build_commands {
            println!("  $ {}", cmd);
        }
        println!("─────────────────────────────────────────────────────────────");
    }
}

/// تصدير سريع
pub fn export_mobile(
    source_code: &str,
    project_name: &str,
    platform: &str,
    framework: &str,
) -> MobileExportResult {
    let config = MobileExportConfig {
        project_name: project_name.to_string(),
        platform: MobilePlatform::from_arabic(platform).unwrap_or(MobilePlatform::Android),
        framework: MobileFramework::from_arabic(framework).unwrap_or(MobileFramework::Flutter),
        ..Default::default()
    };
    
    let exporter = MobileExporter::new(config);
    exporter.export(source_code)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_platform_from_arabic() {
        assert!(MobilePlatform::from_arabic("أندرويد").is_some());
        assert!(MobilePlatform::from_arabic("ios").is_some());
        assert!(MobilePlatform::from_arabic("كلاهما").is_some());
    }
    
    #[test]
    fn test_framework_from_arabic() {
        assert!(MobileFramework::from_arabic("فلاتر").is_some());
        assert!(MobileFramework::from_arabic("react-native").is_some());
        assert!(MobileFramework::from_arabic("أصلي").is_some());
    }
    
    #[test]
    fn test_mobile_export_config_default() {
        let config = MobileExportConfig::default();
        assert_eq!(config.project_name, "almarjaa_app");
        assert!(config.rtl_support);
    }
    
    #[test]
    fn test_mobile_exporter_creation() {
        let config = MobileExportConfig::default();
        let _exporter = MobileExporter::new(config);
    }
}
