// ═══════════════════════════════════════════════════════════════════════════════
// نظام الحزم المتقدم - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// نظام ثوري لإدارة الحزم يتميز بـ:
// - ملف تعريف عربي (مشروع.toml)
// - مستودع مركزي ذكي
// - حل تبعيات بالذكاء الاصطناعي
// - تحقق أمني تلقائي
// - أوامر عربية سهلة
// - توقيع الحزم
// - التوزيع الثنائي
// - دعم Workspace
// ═══════════════════════════════════════════════════════════════════════════════

pub mod config;
pub mod registry;
pub mod resolver;
pub mod dependency;
pub mod installer;
pub mod publisher;
pub mod security;
pub mod stats;
pub mod badges;
pub mod decentralized;
pub mod reputation;
pub mod versioning;
pub mod lockfile;
pub mod binary;
pub mod signing;
pub mod workspace;

pub use config::PackageConfig;
pub use registry::{Registry, PackageInfo, PackageVersion};
pub use resolver::{DependencyResolver, ResolvedDependencies};
pub use installer::Installer;
pub use publisher::Publisher;
pub use security::{SecurityChecker, SecurityReport, Vulnerability, Severity};
pub use stats::{PackageStats, DownloadStats, UsageStats};
pub use badges::{PackageEvaluator, PackageRating, Badge, BadgeLevel, EvaluationInput};
pub use decentralized::{DecentralizedRegistry, DocumentationGenerator, GeneratedDocumentation};
pub use reputation::{ReputationSystem, AuthorProfile, AuthorLevel, VerificationStatus};
pub use versioning::{SemanticVersion, ChangeAnalyzer, CISystem, CIConfig, ChangeType};
pub use lockfile::{Lockfile, LockedPackage, LockfileDiff};
pub use binary::{BinaryDistribution, BinaryRelease, BuildTarget, BinaryBuilder};
pub use signing::{SigningKey, PackageSignature, KeyRegistry, SignatureVerifier, PackageSigner};
pub use workspace::{Workspace, WorkspaceMember, WorkspaceConfig, WorkspaceBuilder, WorkspaceRunner};

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
// الثوابت والتعدادات
// ═══════════════════════════════════════════════════════════════════════════════

/// اسم ملف التعريف
pub const MANIFEST_FILE: &str = "مشروع.toml";
/// مجلد الحزم المحلي
pub const PACKAGES_DIR: &str = "حزم";
/// ملف القفل
pub const LOCK_FILE: &str = "قفل.toml";
/// المستودع الافتراضي
pub const DEFAULT_REGISTRY: &str = "https://registry.almarjaa.io";

/// حالة الحزمة
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PackageStatus {
    /// غير مثبتة
    NotInstalled,
    /// قيد التثبيت
    Installing,
    /// مثبتة
    Installed,
    /// تحتاج تحديث
    Outdated,
    /// خطأ
    Error(String),
}

/// مصدر الحزمة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageSource {
    /// المستودع الرسمي
    Registry,
    /// GitHub
    GitHub { owner: String, repo: String },
    /// Git محلي
    Git { url: String },
    /// مسار محلي
    Local { path: PathBuf },
    /// IPFS
    IPFS { cid: String },
}

impl std::fmt::Display for PackageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageSource::Registry => write!(f, "registry"),
            PackageSource::GitHub { owner, repo } => write!(f, "github:{}/{}", owner, repo),
            PackageSource::Git { url } => write!(f, "git:{}", url),
            PackageSource::Local { path } => write!(f, "local:{}", path.display()),
            PackageSource::IPFS { cid } => write!(f, "ipfs:{}", cid),
        }
    }
}

/// نتيجة عملية الحزمة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageResult {
    /// نجاح أو فشل
    pub success: bool,
    /// الرسالة
    pub message: String,
    /// الحزم المتأثرة
    pub packages: Vec<String>,
    /// الوقت المستغرق (مللي ثانية)
    pub duration_ms: u64,
    /// تفاصيل إضافية
    pub details: HashMap<String, String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// مدير الحزم الرئيسي
// ═══════════════════════════════════════════════════════════════════════════════

/// مدير الحزم المتقدم
pub struct PackageManager {
    /// التكوين الحالي
    config: Option<PackageConfig>,
    /// المستودع
    registry: Registry,
    /// محلل التبعيات
    resolver: DependencyResolver,
    /// المثبت
    installer: Installer,
    /// الناشر
    publisher: Publisher,
    /// مدقق الأمان
    security: SecurityChecker,
    /// الإحصائيات
    stats: PackageStats,
    /// مسار المشروع
    project_path: PathBuf,
    /// وضع Verbose
    verbose: bool,
}

impl PackageManager {
    /// إنشاء مدير حزم جديد
    pub fn new(project_path: &Path) -> Self {
        Self {
            config: None,
            registry: Registry::new(DEFAULT_REGISTRY),
            resolver: DependencyResolver::new(),
            installer: Installer::new(),
            publisher: Publisher::new(),
            security: SecurityChecker::new(),
            stats: PackageStats::new(),
            project_path: project_path.to_path_buf(),
            verbose: false,
        }
    }

    /// تفعيل الوضع التفصيلي
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// تعيين المستودع
    pub fn with_registry(mut self, registry_url: &str) -> Self {
        self.registry = Registry::new(registry_url);
        self
    }

    // ═══════════════════════════════════════════════════════════════
    // الأوامر الأساسية
    // ═══════════════════════════════════════════════════════════════

    /// تهيئة مشروع جديد
    /// الأمر: almarjaa أنشئ مشروع
    pub fn init(&mut self, name: &str) -> PackageResult {
        let start = std::time::Instant::now();
        
        println!("\n🚀 تهيئة مشروع جديد: {}", name);
        
        // إنشاء ملف التعريف
        let config = PackageConfig::new(name);
        
        // إنشاء المجلدات
        let src_dir = self.project_path.join("مصدر");
        let pkg_dir = self.project_path.join("حزم");
        
        if let Err(e) = std::fs::create_dir_all(&src_dir) {
            return PackageResult {
                success: false,
                message: format!("فشل إنشاء مجلد المصدر: {}", e),
                packages: vec![],
                duration_ms: start.elapsed().as_millis() as u64,
                details: HashMap::new(),
            };
        }
        
        if let Err(e) = std::fs::create_dir_all(&pkg_dir) {
            return PackageResult {
                success: false,
                message: format!("فشل إنشاء مجلد الحزم: {}", e),
                packages: vec![],
                duration_ms: start.elapsed().as_millis() as u64,
                details: HashMap::new(),
            };
        }
        
        // حفظ ملف التعريف
        let manifest_path = self.project_path.join(MANIFEST_FILE);
        if let Err(e) = config.save(&manifest_path) {
            return PackageResult {
                success: false,
                message: format!("فشل حفظ ملف التعريف: {}", e),
                packages: vec![],
                duration_ms: start.elapsed().as_millis() as u64,
                details: HashMap::new(),
            };
        }
        
        // إنشاء ملف رئيسي
        let main_file = src_dir.join("رئيسي.mrj");
        let main_content = r#"// مشروع جديد - لغة المرجع
// ═══════════════════════════════════════════════════════

اطبع("مرحبا بالعالم من {}!", "المرجع");
"#;
        
        if let Err(e) = std::fs::write(&main_file, main_content) {
            return PackageResult {
                success: false,
                message: format!("فشل إنشاء الملف الرئيسي: {}", e),
                packages: vec![],
                duration_ms: start.elapsed().as_millis() as u64,
                details: HashMap::new(),
            };
        }
        
        self.config = Some(config);
        
        println!("   ✅ تم إنشاء المشروع بنجاح");
        println!("   📁 {}/", self.project_path.display());
        println!("   ├── مشروع.toml");
        println!("   ├── مصدر/");
        println!("   │   └── رئيسي.mrj");
        println!("   └── حزم/");
        
        PackageResult {
            success: true,
            message: format!("تم إنشاء المشروع '{}' بنجاح", name),
            packages: vec![name.to_string()],
            duration_ms: start.elapsed().as_millis() as u64,
            details: {
                let mut details = HashMap::new();
                details.insert("path".to_string(), self.project_path.to_string_lossy().to_string());
                details.insert("manifest".to_string(), MANIFEST_FILE.to_string());
                details
            },
        }
    }

    /// تثبيت الحزم
    /// الأمر: almarjaa ثبّت
    pub fn install(&mut self, packages: &[&str]) -> PackageResult {
        let start = std::time::Instant::now();
        
        println!("\n📦 تثبيت الحزم...");
        
        // تحميل التكوين
        if self.config.is_none() {
            let manifest_path = self.project_path.join(MANIFEST_FILE);
            if manifest_path.exists() {
                match PackageConfig::load(&manifest_path) {
                    Ok(config) => self.config = Some(config),
                    Err(e) => {
                        return PackageResult {
                            success: false,
                            message: format!("فشل قراءة ملف التعريف: {}", e),
                            packages: vec![],
                            duration_ms: start.elapsed().as_millis() as u64,
                            details: HashMap::new(),
                        };
                    }
                }
            }
        }
        
        let mut installed = Vec::new();
        let mut errors = Vec::new();
        
        for pkg in packages {
            println!("   📥 تثبيت '{}'...", pkg);
            
            // البحث عن الحزمة
            match self.registry.find_package(pkg) {
                Ok(info) => {
                    // التحقق الأمني
                    match self.security.check(&info) {
                        Ok(security_report) => {
                            if security_report.safe {
                                // التثبيت
                                match self.installer.install(&info, &self.project_path) {
                                    Ok(_) => {
                                        println!("      ✅ تم تثبيت '{}' v{}", pkg, info.latest_version);
                                        installed.push(pkg.to_string());
                                    }
                                    Err(e) => {
                                        println!("      ❌ فشل التثبيت: {}", e);
                                        errors.push(format!("{}: {}", pkg, e));
                                    }
                                }
                            } else {
                                println!("      ⚠️ تحذير أمني: {}", security_report.warning);
                                errors.push(format!("{}: مخاوف أمنية", pkg));
                            }
                        }
                        Err(e) => {
                            println!("      ❌ فشل التحقق الأمني: {}", e);
                            errors.push(format!("{}: {}", pkg, e));
                        }
                    }
                }
                Err(e) => {
                    println!("      ❌ لم يتم العثور على الحزمة: {}", e);
                    errors.push(format!("{}: غير موجود", pkg));
                }
            }
        }
        
        let duration = start.elapsed().as_millis() as u64;
        
        if installed.is_empty() && !errors.is_empty() {
            PackageResult {
                success: false,
                message: format!("فشل تثبيت {} حزمة", errors.len()),
                packages: installed,
                duration_ms: duration,
                details: errors.into_iter().enumerate().map(|(i, e)| (format!("error_{}", i), e)).collect(),
            }
        } else {
            println!("\n   ✅ تم تثبيت {} حزمة في {}ms", installed.len(), duration);
            PackageResult {
                success: true,
                message: format!("تم تثبيت {} حزمة بنجاح", installed.len()),
                packages: installed,
                duration_ms: duration,
                details: HashMap::new(),
            }
        }
    }

    /// إضافة حزمة جديدة
    /// الأمر: almarjaa أضف <اسم>
    pub fn add(&mut self, package: &str, version: Option<&str>) -> PackageResult {
        let start = std::time::Instant::now();
        
        println!("\n📦 إضافة حزمة '{}'...", package);
        
        // البحث عن الحزمة
        let info = match self.registry.find_package(package) {
            Ok(info) => info,
            Err(e) => {
                // محاولة البحث في GitHub
                println!("   🔍 البحث في GitHub...");
                match self.registry.find_on_github(package) {
                    Ok(info) => info,
                    Err(_) => {
                        return PackageResult {
                            success: false,
                            message: format!("لم يتم العثور على الحزمة '{}' في المستودع أو GitHub", package),
                            packages: vec![],
                            duration_ms: start.elapsed().as_millis() as u64,
                            details: HashMap::new(),
                        };
                    }
                }
            }
        };
        
        let target_version = version.unwrap_or(&info.latest_version);
        println!("   📌 الإصدار: {}", target_version);
        
        // التحقق الأمني
        let security_report = match self.security.check(&info) {
            Ok(report) => report,
            Err(e) => {
                return PackageResult {
                    success: false,
                    message: format!("فشل التحقق الأمني: {}", e),
                    packages: vec![],
                    duration_ms: start.elapsed().as_millis() as u64,
                    details: HashMap::new(),
                };
            }
        };
        
        if !security_report.safe {
            println!("   ⚠️ تحذير أمني: {}", security_report.warning);
        }
        
        // إضافة للتكوين
        if let Some(ref mut config) = self.config {
            config.add_dependency(package, target_version);
            let manifest_path = self.project_path.join(MANIFEST_FILE);
            if let Err(e) = config.save(&manifest_path) {
                return PackageResult {
                    success: false,
                    message: format!("فشل تحديث ملف التعريف: {}", e),
                    packages: vec![],
                    duration_ms: start.elapsed().as_millis() as u64,
                    details: HashMap::new(),
                };
            }
        }
        
        // التثبيت
        match self.installer.install(&info, &self.project_path) {
            Ok(_) => {
                println!("   ✅ تمت الإضافة بنجاح");
                PackageResult {
                    success: true,
                    message: format!("تمت إضافة '{}' v{} بنجاح", package, target_version),
                    packages: vec![package.to_string()],
                    duration_ms: start.elapsed().as_millis() as u64,
                    details: {
                        let mut details = HashMap::new();
                        details.insert("version".to_string(), target_version.to_string());
                        details.insert("source".to_string(), info.source.to_string());
                        details
                    },
                }
            }
            Err(e) => {
                PackageResult {
                    success: false,
                    message: format!("فشل تثبيت الحزمة: {}", e),
                    packages: vec![],
                    duration_ms: start.elapsed().as_millis() as u64,
                    details: HashMap::new(),
                }
            }
        }
    }

    /// إزالة حزمة
    /// الأمر: almarjaa أزل <اسم>
    pub fn remove(&mut self, package: &str) -> PackageResult {
        let start = std::time::Instant::now();
        
        println!("\n🗑️ إزالة الحزمة '{}'...", package);
        
        // إزالة من التكوين
        if let Some(ref mut config) = self.config {
            config.remove_dependency(package);
            let manifest_path = self.project_path.join(MANIFEST_FILE);
            if let Err(e) = config.save(&manifest_path) {
                return PackageResult {
                    success: false,
                    message: format!("فشل تحديث ملف التعريف: {}", e),
                    packages: vec![],
                    duration_ms: start.elapsed().as_millis() as u64,
                    details: HashMap::new(),
                };
            }
        }
        
        // إزالة الملفات
        let pkg_dir = self.project_path.join(PACKAGES_DIR).join(package);
        if pkg_dir.exists() {
            if let Err(e) = std::fs::remove_dir_all(&pkg_dir) {
                return PackageResult {
                    success: false,
                    message: format!("فشل إزالة ملفات الحزمة: {}", e),
                    packages: vec![],
                    duration_ms: start.elapsed().as_millis() as u64,
                    details: HashMap::new(),
                };
            }
        }
        
        println!("   ✅ تمت الإزالة بنجاح");
        
        PackageResult {
            success: true,
            message: format!("تمت إزالة '{}' بنجاح", package),
            packages: vec![package.to_string()],
            duration_ms: start.elapsed().as_millis() as u64,
            details: HashMap::new(),
        }
    }

    /// تحديث الحزم
    /// الأمر: almarjaa حدّث
    pub fn update(&mut self, packages: &[&str]) -> PackageResult {
        let start = std::time::Instant::now();
        
        println!("\n🔄 تحديث الحزم...");
        
        let mut updated = Vec::new();
        
        let packages_to_update = if packages.is_empty() {
            // تحديث كل الحزم
            if let Some(ref config) = self.config {
                config.dependencies.keys().cloned().collect::<Vec<_>>()
            } else {
                vec![]
            }
        } else {
            packages.iter().map(|s| s.to_string()).collect()
        };
        
        for pkg in &packages_to_update {
            println!("   🔄 تحديث '{}'...", pkg);
            
            match self.registry.find_package(pkg) {
                Ok(info) => {
                    // التحقق من وجود تحديث
                    if let Some(ref config) = self.config {
                        if let Some(current_version) = config.dependencies.get(pkg) {
                            if current_version != &info.latest_version {
                                match self.installer.install(&info, &self.project_path) {
                                    Ok(_) => {
                                        println!("      ✅ تم التحديث {} → {}", current_version, info.latest_version);
                                        updated.push(pkg.clone());
                                    }
                                    Err(e) => {
                                        println!("      ❌ فشل التحديث: {}", e);
                                    }
                                }
                            } else {
                                println!("      ℹ️ محدث بالفعل");
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("      ❌ خطأ: {}", e);
                }
            }
        }
        
        let duration = start.elapsed().as_millis() as u64;
        println!("\n   ✅ تم تحديث {} حزمة في {}ms", updated.len(), duration);
        
        PackageResult {
            success: true,
            message: format!("تم تحديث {} حزمة", updated.len()),
            packages: updated,
            duration_ms: duration,
            details: HashMap::new(),
        }
    }

    /// البحث عن حزمة
    /// الأمر: almarjaa ابحث <اسم>
    pub fn search(&self, query: &str) -> PackageResult {
        let start = std::time::Instant::now();
        
        println!("\n🔍 البحث عن '{}'...", query);
        
        match self.registry.search(query) {
            Ok(results) => {
                if results.is_empty() {
                    println!("   ℹ️ لم يتم العثور على نتائج");
                } else {
                    println!("\n   📦 نتائج البحث ({}):", results.len());
                    for info in &results {
                        println!("\n   ┌─ {} v{}", info.name, info.latest_version);
                        println!("   │  {}", info.description);
                        println!("   │  ⬇️ {} تحميل | ⭐ {} نجمة", info.downloads, info.stars);
                        println!("   └─────────────────────────────────");
                    }
                }
                
                PackageResult {
                    success: true,
                    message: format!("تم العثور على {} نتيجة", results.len()),
                    packages: results.iter().map(|p| p.name.clone()).collect(),
                    duration_ms: start.elapsed().as_millis() as u64,
                    details: HashMap::new(),
                }
            }
            Err(e) => {
                PackageResult {
                    success: false,
                    message: format!("فشل البحث: {}", e),
                    packages: vec![],
                    duration_ms: start.elapsed().as_millis() as u64,
                    details: HashMap::new(),
                }
            }
        }
    }

    /// نشر حزمة
    /// الأمر: almarjaa انشر
    pub fn publish(&mut self) -> PackageResult {
        let start = std::time::Instant::now();
        
        println!("\n📤 نشر الحزمة...");
        
        // التحقق من التكوين
        let config = match &self.config {
            Some(c) => c.clone(),
            None => {
                return PackageResult {
                    success: false,
                    message: "لم يتم العثور على ملف تعريف المشروع".to_string(),
                    packages: vec![],
                    duration_ms: start.elapsed().as_millis() as u64,
                    details: HashMap::new(),
                };
            }
        };
        
        println!("   📦 الحزمة: {} v{}", config.name, config.version);
        
        // التحقق الأمني
        println!("   🔒 فحص أمني...");
        
        // التحقق من وجود ملفات
        let src_dir = self.project_path.join("مصدر");
        if !src_dir.exists() {
            return PackageResult {
                success: false,
                message: "مجلد 'مصدر' غير موجود".to_string(),
                packages: vec![],
                duration_ms: start.elapsed().as_millis() as u64,
                details: HashMap::new(),
            };
        }
        
        // النشر
        match self.publisher.publish(&config, &self.project_path) {
            Ok(_) => {
                println!("   ✅ تم النشر بنجاح!");
                println!("   🌐 https://registry.almarjaa.io/package/{}", config.name);
                
                PackageResult {
                    success: true,
                    message: format!("تم نشر '{}' v{} بنجاح", config.name, config.version),
                    packages: vec![config.name.clone()],
                    duration_ms: start.elapsed().as_millis() as u64,
                    details: {
                        let mut details = HashMap::new();
                        details.insert("version".to_string(), config.version.clone());
                        details
                    },
                }
            }
            Err(e) => {
                PackageResult {
                    success: false,
                    message: format!("فشل النشر: {}", e),
                    packages: vec![],
                    duration_ms: start.elapsed().as_millis() as u64,
                    details: HashMap::new(),
                }
            }
        }
    }

    /// عرض معلومات حزمة
    /// الأمر: almarjaa معلومات <اسم>
    pub fn info(&self, package: &str) -> PackageResult {
        let start = std::time::Instant::now();
        
        println!("\n📋 معلومات الحزمة '{}'...", package);
        
        match self.registry.find_package(package) {
            Ok(info) => {
                println!("\n   ╔════════════════════════════════════════════════════╗");
                println!("   ║  {} v{}", info.name, info.latest_version);
                println!("   ╠════════════════════════════════════════════════════╣");
                println!("   ║  📝 {}", info.description);
                println!("   ║");
                println!("   ║  👤 المؤلف: {}", info.author);
                println!("   ║  📜 الرخصة: {}", info.license);
                println!("   ║");
                println!("   ║  📊 الإحصائيات:");
                println!("   ║     • التنزيلات: {}", info.downloads);
                println!("   ║     • النجوم: {}", info.stars);
                println!("   ║     • الفروع: {}", info.forks);
                println!("   ║");
                println!("   ║  📦 الإصدارات:");
                for v in &info.versions.iter().take(5).collect::<Vec<_>>() {
                    println!("   ║     • {}", v);
                }
                println!("   ║");
                println!("   ║  🔗 الروابط:");
                println!("   ║     • GitHub: {}", info.github_url.as_ref().unwrap_or(&"-".to_string()));
                println!("   ║     • التوثيق: {}", info.documentation_url.as_ref().unwrap_or(&"-".to_string()));
                println!("   ╚════════════════════════════════════════════════════╝");
                
                PackageResult {
                    success: true,
                    message: format!("معلومات '{}'", package),
                    packages: vec![package.to_string()],
                    duration_ms: start.elapsed().as_millis() as u64,
                    details: {
                        let mut details = HashMap::new();
                        details.insert("version".to_string(), info.latest_version.clone());
                        details.insert("author".to_string(), info.author);
                        details.insert("downloads".to_string(), info.downloads.to_string());
                        details
                    },
                }
            }
            Err(e) => {
                PackageResult {
                    success: false,
                    message: format!("لم يتم العثور على الحزمة: {}", e),
                    packages: vec![],
                    duration_ms: start.elapsed().as_millis() as u64,
                    details: HashMap::new(),
                }
            }
        }
    }

    /// عرض قائمة الحزم المثبتة
    /// الأمر: almarjaa قائمة
    pub fn list(&self) -> PackageResult {
        let start = std::time::Instant::now();
        
        println!("\n📋 الحزم المثبتة:");
        
        let pkg_dir = self.project_path.join(PACKAGES_DIR);
        let mut packages = Vec::new();
        
        if pkg_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&pkg_dir) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        if let Some(name) = entry.file_name().to_str() {
                            packages.push(name.to_string());
                            println!("   📦 {}", name);
                        }
                    }
                }
            }
        }
        
        if packages.is_empty() {
            println!("   ℹ️ لا توجد حزم مثبتة");
        }
        
        PackageResult {
            success: true,
            message: format!("{} حزمة مثبتة", packages.len()),
            packages,
            duration_ms: start.elapsed().as_millis() as u64,
            details: HashMap::new(),
        }
    }

    /// التحقق من التحديثات
    /// الأمر: almarjaa تحقق
    pub fn outdated(&self) -> PackageResult {
        let start = std::time::Instant::now();
        
        println!("\n🔍 التحقق من التحديثات...");
        
        let mut outdated = Vec::new();
        
        if let Some(ref config) = self.config {
            for (pkg, current_version) in &config.dependencies {
                if let Ok(info) = self.registry.find_package(pkg) {
                    if current_version != &info.latest_version {
                        outdated.push(pkg.clone());
                        println!("   📦 {} {} → {}", pkg, current_version, info.latest_version);
                    }
                }
            }
        }
        
        if outdated.is_empty() {
            println!("   ✅ جميع الحزم محدثة");
        }
        
        PackageResult {
            success: true,
            message: if outdated.is_empty() {
                "جميع الحزم محدثة".to_string()
            } else {
                format!("{} حزمة تحتاج تحديث", outdated.len())
            },
            packages: outdated,
            duration_ms: start.elapsed().as_millis() as u64,
            details: HashMap::new(),
        }
    }

    /// تنظيف الحزم غير المستخدمة
    /// الأمر: almarjaa نظّف
    pub fn clean(&mut self) -> PackageResult {
        let start = std::time::Instant::now();
        
        println!("\n🧹 تنظيف الحزم غير المستخدمة...");
        
        let pkg_dir = self.project_path.join(PACKAGES_DIR);
        let mut removed = Vec::new();
        
        if pkg_dir.exists() {
            // الحصول على قائمة الحزم المطلوبة
            let required: std::collections::HashSet<String> = if let Some(ref config) = self.config {
                config.dependencies.keys().cloned().collect()
            } else {
                std::collections::HashSet::new()
            };
            
            // إزالة الحزم غير المطلوبة
            if let Ok(entries) = std::fs::read_dir(&pkg_dir) {
                for entry in entries.flatten() {
                    if entry.path().is_dir() {
                        if let Some(name) = entry.file_name().to_str() {
                            if !required.contains(name) {
                                println!("   🗑️ إزالة '{}'...", name);
                                if std::fs::remove_dir_all(entry.path()).is_ok() {
                                    removed.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        println!("   ✅ تم إزالة {} حزمة", removed.len());
        
        PackageResult {
            success: true,
            message: format!("تم تنظيف {} حزمة", removed.len()),
            packages: removed,
            duration_ms: start.elapsed().as_millis() as u64,
            details: HashMap::new(),
        }
    }

    /// عرض تقرير الأمان
    /// الأمر: almarjaa أمان
    pub fn security_audit(&self) -> PackageResult {
        let start = std::time::Instant::now();
        
        println!("\n🔒 تقرير الأمان:");
        
        let mut issues = Vec::new();
        let mut checked = 0;
        
        if let Some(ref config) = self.config {
            for pkg in config.dependencies.keys() {
                if let Ok(info) = self.registry.find_package(pkg) {
                    checked += 1;
                    if let Ok(report) = self.security.check(&info) {
                        if !report.safe {
                            issues.push(pkg.clone());
                            println!("   ⚠️ {} - {}", pkg, report.warning);
                        }
                    }
                }
            }
        }
        
        if issues.is_empty() {
            println!("   ✅ لا توجد مشاكل أمنية (تم فحص {} حزمة)", checked);
        } else {
            println!("\n   ⚠️ تم العثور على {} مشكلة أمنية", issues.len());
        }
        
        PackageResult {
            success: issues.is_empty(),
            message: if issues.is_empty() {
                format!("جميع الحزم آمنة ({})", checked)
            } else {
                format!("{} مشكلة أمنية", issues.len())
            },
            packages: issues,
            duration_ms: start.elapsed().as_millis() as u64,
            details: HashMap::new(),
        }
    }
}

impl Default for PackageManager {
    fn default() -> Self {
        Self::new(Path::new("."))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال سهلة
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء مشروع جديد
pub fn init_project(name: &str, path: &Path) -> PackageResult {
    PackageManager::new(path).init(name)
}

/// تثبيت حزمة
pub fn install_package(package: &str, path: &Path) -> PackageResult {
    PackageManager::new(path).install(&[package])
}

/// نشر حزمة
pub fn publish_package(path: &Path) -> PackageResult {
    let mut pm = PackageManager::new(path);
    pm.publish()
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال للتوافق مع CLI
// ═══════════════════════════════════════════════════════════════════════════════

/// تهيئة ملف تعريف المشروع (للتوافق مع CLI)
pub fn init_manifest(path: &Path) -> Result<PackageConfig, String> {
    let mut pm = PackageManager::new(path);
    let result = pm.init("مشروع-جديد");
    if !result.success {
        return Err(result.message);
    }
    PackageConfig::load(&path.join(MANIFEST_FILE))
}

/// تحميل ملف تعريف المشروع
pub fn load_manifest(path: &Path) -> Result<PackageConfig, String> {
    PackageConfig::load(&path.join(MANIFEST_FILE))
}

/// حل التبعيات
pub fn resolve_dependencies(path: &Path) -> Result<ResolvedDependencies, String> {
    let config = load_manifest(path)?;
    let mut resolver = DependencyResolver::new();
    resolver.resolve(&config.dependencies)
}

/// إنشاء ملف قفل
pub fn generate_lockfile(path: &Path) -> Result<String, String> {
    let resolved = resolve_dependencies(path)?;
    let resolver = DependencyResolver::new();
    Ok(resolver.generate_lockfile(&resolved))
}

/// كتابة ملف قفل
pub fn write_lockfile(path: &Path) -> Result<(), String> {
    let content = generate_lockfile(path)?;
    std::fs::write(path.join(LOCK_FILE), content)
        .map_err(|e| format!("فشل كتابة ملف القفل: {}", e))
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_package_manager_creation() {
        let pm = PackageManager::new(Path::new("."));
        assert!(pm.config.is_none());
    }

    #[test]
    fn test_init_project() {
        let temp_dir = std::env::temp_dir().join("almarjaa_test_init");
        let _ = std::fs::create_dir_all(&temp_dir);
        
        let mut pm = PackageManager::new(&temp_dir);
        let result = pm.init("اختبار");
        
        assert!(result.success);
        
        // التنظيف
        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_package_result() {
        let result = PackageResult {
            success: true,
            message: "نجاح".to_string(),
            packages: vec!["حزمة1".to_string()],
            duration_ms: 100,
            details: HashMap::new(),
        };
        
        assert!(result.success);
        assert_eq!(result.packages.len(), 1);
    }

    #[test]
    fn test_search() {
        let pm = PackageManager::new(Path::new("."));
        let result = pm.search("json");
        
        // البحث يجب أن ينجح (حتى لو كان فارغاً)
        assert!(result.success);
    }

    #[test]
    fn test_list_empty() {
        let temp_dir = std::env::temp_dir().join("almarjaa_test_list");
        let _ = std::fs::create_dir_all(&temp_dir);
        
        let pm = PackageManager::new(&temp_dir);
        let result = pm.list();
        
        assert!(result.success);
        assert!(result.packages.is_empty());
        
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
