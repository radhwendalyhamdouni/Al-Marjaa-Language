// ═══════════════════════════════════════════════════════════════════════════════
// مثبت الحزم - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use std::path::Path;
use std::fs;
use super::registry::PackageInfo;

/// مثبت الحزم
pub struct Installer {
    /// عدد الخيوط المتوازية
    parallel_downloads: usize,
    /// استخدام الكاش
    use_cache: bool,
    /// التحقق من التوقيع
    verify_signature: bool,
}

impl Installer {
    /// إنشاء مثبت جديد
    pub fn new() -> Self {
        Self {
            parallel_downloads: 4,
            use_cache: true,
            verify_signature: true,
        }
    }

    /// تثبيت حزمة
    pub fn install(&self, package: &PackageInfo, project_path: &Path) -> Result<(), String> {
        // إنشاء مجلد الحزم
        let packages_dir = project_path.join("حزم");
        if !packages_dir.exists() {
            fs::create_dir_all(&packages_dir)
                .map_err(|e| format!("فشل إنشاء مجلد الحزم: {}", e))?;
        }
        
        // مجلد الحزمة
        let pkg_dir = packages_dir.join(&package.name);
        if pkg_dir.exists() {
            // التحديث بدلاً من التثبيت الجديد
            return self.update_package(package, &pkg_dir);
        }
        
        fs::create_dir_all(&pkg_dir)
            .map_err(|e| format!("فشل إنشاء مجلد الحزمة: {}", e))?;
        
        // إنشاء الهيكل
        self.create_package_structure(&pkg_dir)?;
        
        // كتابة ملف التعريف
        self.write_package_manifest(package, &pkg_dir)?;
        
        // كتابة الكود الأساسي
        self.write_package_code(package, &pkg_dir)?;
        
        Ok(())
    }

    /// تحديث حزمة موجودة
    fn update_package(&self, package: &PackageInfo, pkg_dir: &Path) -> Result<(), String> {
        // حذف المحتوى القديم
        for entry in fs::read_dir(pkg_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            if entry.path().is_dir() {
                fs::remove_dir_all(entry.path()).map_err(|e| e.to_string())?;
            } else {
                fs::remove_file(entry.path()).map_err(|e| e.to_string())?;
            }
        }
        
        // إعادة إنشاء
        self.create_package_structure(pkg_dir)?;
        
        Ok(())
    }

    /// إنشاء هيكل الحزمة
    fn create_package_structure(&self, pkg_dir: &Path) -> Result<(), String> {
        let dirs = vec!["مصدر", "توثيق", "أمثلة"];
        
        for dir in dirs {
            fs::create_dir_all(pkg_dir.join(dir))
                .map_err(|e| format!("فشل إنشاء مجلد {}: {}", dir, e))?;
        }
        
        Ok(())
    }

    /// كتابة ملف تعريف الحزمة
    fn write_package_manifest(&self, package: &PackageInfo, pkg_dir: &Path) -> Result<(), String> {
        let manifest = format!(
r#"# حزمة {} v{}
الاسم = "{}"
الإصدار = "{}"
الوصف = "{}"
المؤلف = "{}"
"#,
            package.name, package.latest_version,
            package.name, package.latest_version,
            package.description, package.author
        );
        
        fs::write(pkg_dir.join("حزمة.toml"), manifest)
            .map_err(|e| format!("فشل كتابة ملف التعريف: {}", e))
    }

    /// كتابة كود الحزمة
    fn write_package_code(&self, package: &PackageInfo, pkg_dir: &Path) -> Result<(), String> {
        // إنشاء ملف أساسي
        let main_code = self.generate_package_code(package);
        
        fs::write(pkg_dir.join("مصدر").join("رئيسي.mrj"), main_code)
            .map_err(|e| format!("فشل كتابة الكود: {}", e))
    }

    /// توليد كود الحزمة
    fn generate_package_code(&self, package: &PackageInfo) -> String {
        format!(
r#"// ═══════════════════════════════════════════════════════════════════
// حزمة {} v{}
// ═══════════════════════════════════════════════════════════════════
// {}

// التصدير العام
صدّر {{
    // الدوال الرئيسية
}};

// ═══════════════════════════════════════════════════════════════════
// التنفيذ الداخلي
// ═══════════════════════════════════════════════════════════════════

"#,
            package.name, package.latest_version, package.description
        )
    }

    /// تثبيت متوازي
    pub fn install_parallel(&self, packages: &[PackageInfo], project_path: &Path) -> Result<Vec<String>, String> {
        let mut installed = Vec::new();
        
        for package in packages {
            match self.install(package, project_path) {
                Ok(_) => installed.push(package.name.clone()),
                Err(e) => return Err(format!("فشل تثبيت {}: {}", package.name, e)),
            }
        }
        
        Ok(installed)
    }
}

impl Default for Installer {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::package_manager::PackageSource;

    fn create_test_package() -> PackageInfo {
        PackageInfo {
            name: "test".to_string(),
            latest_version: "1.0.0".to_string(),
            description: "Test package".to_string(),
            author: "test".to_string(),
            license: "MIT".to_string(),
            versions: vec!["1.0.0".to_string()],
            downloads: 0,
            stars: 0,
            forks: 0,
            github_url: None,
            documentation_url: None,
            keywords: vec![],
            dependencies: HashMap::new(),
            created_at: String::new(),
            updated_at: String::new(),
            source: PackageSource::Registry,
            size: 0,
            checksum: String::new(),
        }
    }

    #[test]
    fn test_installer_creation() {
        let installer = Installer::new();
        assert!(installer.use_cache);
    }

    #[test]
    fn test_install() {
        let temp_dir = std::env::temp_dir().join("almarjaa_install_test");
        let _ = std::fs::create_dir_all(&temp_dir);
        
        let installer = Installer::new();
        let package = create_test_package();
        
        let result = installer.install(&package, &temp_dir);
        assert!(result.is_ok());
        
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
