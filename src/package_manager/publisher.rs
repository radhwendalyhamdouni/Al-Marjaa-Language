// ═══════════════════════════════════════════════════════════════════════════════
// ناشر الحزم - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use std::path::Path;
use std::fs;
use super::config::PackageConfig;

/// ناشر الحزم
pub struct Publisher {
    /// الرمز المميز
    token: Option<String>,
    /// المستودع
    registry_url: String,
    /// التحقق قبل النشر
    pre_publish_check: bool,
}

impl Publisher {
    /// إنشاء ناشر جديد
    pub fn new() -> Self {
        Self {
            token: None,
            registry_url: super::DEFAULT_REGISTRY.to_string(),
            pre_publish_check: true,
        }
    }

    /// تعيين الرمز المميز
    pub fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    /// نشر حزمة
    pub fn publish(&self, config: &PackageConfig, project_path: &Path) -> Result<(), String> {
        println!("\n📤 نشر الحزمة {} v{}...", config.name, config.version);
        
        // التحقق من الرمز
        if self.token.is_none() {
            return Err("لم يتم تعيين رمز النشر".to_string());
        }
        
        // التحقق من الملفات المطلوبة
        self.check_required_files(project_path)?;
        
        // التحقق من جودة الكود
        if self.pre_publish_check {
            self.run_quality_checks(project_path)?;
        }
        
        // إنشاء الأرشيف
        let archive = self.create_archive(project_path)?;
        
        // رفع للمستودع
        self.upload(&config.name, &config.version, &archive)?;
        
        println!("   ✅ تم النشر بنجاح!");
        println!("   🌐 https://registry.almarjaa.io/package/{}", config.name);
        
        Ok(())
    }

    /// التحقق من الملفات المطلوبة
    fn check_required_files(&self, project_path: &Path) -> Result<(), String> {
        let required_files = [
            "مشروع.toml",
            "README.md",
        ];
        
        for file in &required_files {
            if !project_path.join(file).exists() {
                return Err(format!("الملف '{}' مطلوب للنشر", file));
            }
        }
        
        Ok(())
    }

    /// تشغيل فحوصات الجودة
    fn run_quality_checks(&self, project_path: &Path) -> Result<(), String> {
        println!("   🔍 فحص الجودة...");
        
        // التحقق من وجود مجلد المصدر
        let src_dir = project_path.join("مصدر");
        if !src_dir.exists() {
            return Err("مجلد المصدر غير موجود".to_string());
        }
        
        // التحقق من وجود ملفات
        let has_files = fs::read_dir(&src_dir)
            .map(|mut d| d.next().is_some())
            .unwrap_or(false);
        
        if !has_files {
            return Err("لا توجد ملفات في مجلد المصدر".to_string());
        }
        
        Ok(())
    }

    /// إنشاء أرشيف
    fn create_archive(&self, project_path: &Path) -> Result<Vec<u8>, String> {
        println!("   📦 إنشاء الأرشيف...");
        
        // في التطبيق الحقيقي، سنستخدم tar+gzip
        // هنا نحاكي بقراءة الملفات
        
        let mut archive_data = Vec::new();
        
        fn add_dir(dir: &Path, data: &mut Vec<u8>) -> Result<(), String> {
            for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let path = entry.path();
                
                if path.is_dir() {
                    add_dir(&path, data)?;
                } else {
                    let content = fs::read(&path).map_err(|e| e.to_string())?;
                    data.extend(content);
                }
            }
            Ok(())
        }
        
        add_dir(project_path, &mut archive_data)?;
        
        Ok(archive_data)
    }

    /// رفع للمستودع
    fn upload(&self, name: &str, version: &str, archive: &[u8]) -> Result<(), String> {
        println!("   📤 رفع {} v{} ({} bytes)...", name, version, archive.len());
        
        // محاكاة الرفع - في التطبيق الحقيقي سنستخدم HTTP
        
        Ok(())
    }
}

impl Default for Publisher {
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

    #[test]
    fn test_publisher_creation() {
        let publisher = Publisher::new();
        assert!(publisher.token.is_none());
    }

    #[test]
    fn test_publisher_with_token() {
        let publisher = Publisher::new().with_token("test_token");
        assert!(publisher.token.is_some());
    }

    #[test]
    fn test_publish_without_token() {
        let publisher = Publisher::new();
        let config = PackageConfig::new("test");
        
        let result = publisher.publish(&config, Path::new("."));
        assert!(result.is_err());
    }
}
