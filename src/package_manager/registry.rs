// ═══════════════════════════════════════════════════════════════════════════════
// مستودع الحزم - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::PackageSource;

/// المستودع المركزي
pub struct Registry {
    /// عنوان المستودع
    url: String,
    /// الكاش المحلي
    cache: HashMap<String, PackageInfo>,
    /// مهلة الطلب (مللي ثانية)
    timeout: u64,
}

/// معلومات الحزمة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    /// الاسم
    pub name: String,
    /// أحدث إصدار
    pub latest_version: String,
    /// الوصف
    pub description: String,
    /// المؤلف
    pub author: String,
    /// الرخصة
    pub license: String,
    /// الإصدارات المتاحة
    pub versions: Vec<String>,
    /// عدد التنزيلات
    pub downloads: u64,
    /// عدد النجوم
    pub stars: u64,
    /// عدد الفروع
    pub forks: u64,
    /// رابط GitHub
    pub github_url: Option<String>,
    /// رابط التوثيق
    pub documentation_url: Option<String>,
    /// الكلمات المفتاحية
    pub keywords: Vec<String>,
    /// التبعيات
    pub dependencies: HashMap<String, String>,
    /// التاريخ
    pub created_at: String,
    /// آخر تحديث
    pub updated_at: String,
    /// المصدر
    pub source: PackageSource,
    /// الحجم بالبايت
    pub size: u64,
    /// checksum
    pub checksum: String,
}

/// إصدار الحزمة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageVersion {
    /// رقم الإصدار
    pub version: String,
    /// التاريخ
    pub date: String,
    /// التغييرات
    pub changelog: String,
    /// التبعيات
    pub dependencies: HashMap<String, String>,
    /// الحجم
    pub size: u64,
    /// checksum
    pub checksum: String,
    /// yarn lock
    pub integrity: String,
    /// الرابط
    pub tarball: String,
}

/// نتيجة البحث
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// الحزم
    pub packages: Vec<PackageInfo>,
    /// إجمالي النتائج
    pub total: usize,
    /// الوقت المستغرق
    pub took_ms: u64,
}

impl Registry {
    /// إنشاء مستودع جديد
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            cache: HashMap::new(),
            timeout: 30000,
        }
    }

    /// تعيين المهلة
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    // ═══════════════════════════════════════════════════════════════
    // العمليات الأساسية
    // ═══════════════════════════════════════════════════════════════

    /// البحث عن حزمة
    pub fn find_package(&self, name: &str) -> Result<PackageInfo, String> {
        // التحقق من الكاش أولاً
        if let Some(info) = self.cache.get(name) {
            return Ok(info.clone());
        }
        
        // محاولة الحصول من المستودع
        self.fetch_package(name)
    }

    /// البحث في GitHub
    pub fn find_on_github(&self, name: &str) -> Result<PackageInfo, String> {
        // محاكاة البحث في GitHub
        // في التطبيق الحقيقي، سنستخدم GitHub API
        
        let parts: Vec<&str> = name.split('/').collect();
        if parts.len() == 2 {
            let owner = parts[0];
            let repo = parts[1];
            
            Ok(PackageInfo {
                name: repo.to_string(),
                latest_version: "0.1.0".to_string(),
                description: format!("من GitHub: {}/{}", owner, repo),
                author: owner.to_string(),
                license: "MIT".to_string(),
                versions: vec!["0.1.0".to_string()],
                downloads: 0,
                stars: 0,
                forks: 0,
                github_url: Some(format!("https://github.com/{}/{}", owner, repo)),
                documentation_url: None,
                keywords: vec![],
                dependencies: HashMap::new(),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
                source: super::PackageSource::GitHub {
                    owner: owner.to_string(),
                    repo: repo.to_string(),
                },
                size: 0,
                checksum: String::new(),
            })
        } else {
            Err("صيغة GitHub غير صحيحة. استخدم: owner/repo".to_string())
        }
    }

    /// البحث عن حزم
    pub fn search(&self, query: &str) -> Result<Vec<PackageInfo>, String> {
        let results = self.search_in_registry(query)?;
        Ok(results)
    }

    /// الحصول على إصدار محدد
    pub fn get_version(&self, name: &str, version: &str) -> Result<PackageVersion, String> {
        // محاكاة الحصول على إصدار
        Ok(PackageVersion {
            version: version.to_string(),
            date: chrono::Utc::now().to_rfc3339(),
            changelog: String::new(),
            dependencies: HashMap::new(),
            size: 0,
            checksum: String::new(),
            integrity: String::new(),
            tarball: format!("{}/packages/{}/{}", self.url, name, version),
        })
    }

    /// تحميل حزمة
    pub fn download(&self, name: &str, version: &str) -> Result<Vec<u8>, String> {
        // محاكاة التحميل
        let _info = self.find_package(name)?;
        let _version_info = self.get_version(name, version)?;
        
        // في التطبيق الحقيقي، سنقوم بالتحميل الفعلي
        Ok(vec![])
    }

    // ═══════════════════════════════════════════════════════════════
    // الحزم المدمجة
    // ═══════════════════════════════════════════════════════════════

    /// الحصول على قائمة الحزم المدمجة
    pub fn built_in_packages() -> Vec<PackageInfo> {
        vec![
            Self::create_builtin("json", "تحليل وإنشاء JSON", "1.0.0"),
            Self::create_builtin("http", "طلبات HTTP الشبكية", "1.0.0"),
            Self::create_builtin("قاعدة_بيانات", "الاتصال بقواعد البيانات", "1.0.0"),
            Self::create_builtin("رياضيات", "دوال رياضية متقدمة", "1.0.0"),
            Self::create_builtin("ملفات", "قراءة وكتابة الملفات", "1.0.0"),
            Self::create_builtin("تاريخ", "التعامل مع التاريخ والوقت", "1.0.0"),
            Self::create_builtin("عشوائي", "أرقام وقيم عشوائية", "1.0.0"),
            Self::create_builtin("تشفير", "خوارزميات التشفير", "1.0.0"),
            Self::create_builtin("ضغط", "ضغط وفك ضغط الملفات", "1.0.0"),
            Self::create_builtin("ويب", "خوادم الويب", "1.0.0"),
            Self::create_builtin("بريد", "إرسال البريد الإلكتروني", "1.0.0"),
            Self::create_builtin("صور", "معالجة الصور", "1.0.0"),
            Self::create_builtin("اختبارات", "إطار الاختبارات", "1.0.0"),
            Self::create_builtin("سجلات", "تسجيل الأحداث", "1.0.0"),
            Self::create_builtin("محادثة", "برمجيات المحادثات", "1.0.0"),
        ]
    }

    /// إنشاء حزمة مدمجة
    fn create_builtin(name: &str, desc: &str, version: &str) -> PackageInfo {
        PackageInfo {
            name: name.to_string(),
            latest_version: version.to_string(),
            description: desc.to_string(),
            author: "المرجع".to_string(),
            license: "MIT".to_string(),
            versions: vec![version.to_string()],
            downloads: 0,
            stars: 0,
            forks: 0,
            github_url: None,
            documentation_url: Some(format!("https://docs.almarjaa.io/packages/{}", name)),
            keywords: vec![name.to_string()],
            dependencies: HashMap::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            source: super::PackageSource::Registry,
            size: 0,
            checksum: String::new(),
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // العمليات الداخلية
    // ═══════════════════════════════════════════════════════════════

    /// جلب حزمة من المستودع
    fn fetch_package(&self, name: &str) -> Result<PackageInfo, String> {
        // أولاً، التحقق من الحزم المدمجة
        for pkg in Self::built_in_packages() {
            if pkg.name == name {
                return Ok(pkg);
            }
        }
        
        // محاولة من المستودع البعيد
        // في التطبيق الحقيقي، سنستخدم HTTP client
        
        Err(format!("الحزمة '{}' غير موجودة", name))
    }

    /// البحث في المستودع
    fn search_in_registry(&self, query: &str) -> Result<Vec<PackageInfo>, String> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        
        // البحث في الحزم المدمجة
        for pkg in Self::built_in_packages() {
            if pkg.name.to_lowercase().contains(&query_lower) ||
               pkg.description.to_lowercase().contains(&query_lower) ||
               pkg.keywords.iter().any(|k| k.to_lowercase().contains(&query_lower)) {
                results.push(pkg);
            }
        }
        
        Ok(results)
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new(super::DEFAULT_REGISTRY)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = Registry::new("https://test.io");
        assert_eq!(registry.url, "https://test.io");
    }

    #[test]
    fn test_built_in_packages() {
        let packages = Registry::built_in_packages();
        assert!(!packages.is_empty());
        assert!(packages.iter().any(|p| p.name == "json"));
    }

    #[test]
    fn test_find_builtin_package() {
        let registry = Registry::default();
        let result = registry.find_package("json");
        
        assert!(result.is_ok());
        let pkg = result.unwrap();
        assert_eq!(pkg.name, "json");
    }

    #[test]
    fn test_find_nonexistent_package() {
        let registry = Registry::default();
        let result = registry.find_package("حزمة_غير_موجودة_12345");
        
        assert!(result.is_err());
    }

    #[test]
    fn test_search() {
        let registry = Registry::default();
        let results = registry.search("json").unwrap();
        
        assert!(!results.is_empty());
    }

    #[test]
    fn test_github_search() {
        let registry = Registry::default();
        let result = registry.find_on_github("user/repo");
        
        assert!(result.is_ok());
        let pkg = result.unwrap();
        assert_eq!(pkg.name, "repo");
    }

    #[test]
    fn test_invalid_github_format() {
        let registry = Registry::default();
        let result = registry.find_on_github("invalid-format");
        
        assert!(result.is_err());
    }
}
