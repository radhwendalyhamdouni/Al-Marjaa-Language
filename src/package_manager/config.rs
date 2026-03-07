// ═══════════════════════════════════════════════════════════════════════════════
// تكوين الحزمة - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};

/// تكوين الحزمة (ملف مشروع.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    /// اسم الحزمة
    pub name: String,
    /// الإصدار
    pub version: String,
    /// الوصف
    #[serde(default)]
    pub description: String,
    /// المؤلف
    #[serde(default)]
    pub author: String,
    /// الرخصة
    #[serde(default = "default_license")]
    pub license: String,
    /// الكلمات المفتاحية
    #[serde(default)]
    pub keywords: Vec<String>,
    /// موقع المستودع
    #[serde(default)]
    pub repository: Option<String>,
    /// موقع التوثيق
    #[serde(default)]
    pub documentation: Option<String>,
    /// ملف القراءة
    #[serde(default = "default_readme")]
    pub readme: String,
    /// التبعيات
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    /// تبعيات التطوير
    #[serde(default)]
    pub dev_dependencies: HashMap<String, String>,
    /// الملف الرئيسي
    #[serde(default = "default_entry")]
    pub entry: String,
    /// الملفات المضمنة
    #[serde(default = "default_include")]
    pub include: Vec<String>,
    /// الملفات المستثناة
    #[serde(default = "default_exclude")]
    pub exclude: Vec<String>,
    /// الميزات
    #[serde(default)]
    pub features: HashMap<String, Vec<String>>,
    /// الإعدادات المتقدمة
    #[serde(default)]
    pub settings: PackageSettings,
}

fn default_license() -> String { "MIT".to_string() }
fn default_readme() -> String { "README.md".to_string() }
fn default_entry() -> String { "مصدر/رئيسي.mrj".to_string() }
fn default_include() -> Vec<String> { vec!["مصدر/**/*".to_string()] }
fn default_exclude() -> Vec<String> { vec!["حزم/**/*".to_string(), "هدف/**/*".to_string()] }

/// إعدادات الحزمة المتقدمة
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PackageSettings {
    /// المستوى الأدنى للإصدار
    pub minimum_version: Option<String>,
    /// التجميع الأمثل
    #[serde(default)]
    pub optimize: bool,
    /// تصحيح الأخطاء
    #[serde(default)]
    pub debug: bool,
    /// الهدف
    #[serde(default)]
    pub target: String,
    /// إضافات المترجم
    #[serde(default)]
    pub compiler_flags: Vec<String>,
    /// تعريف الأنواع
    #[serde(default)]
    pub type_definitions: Vec<String>,
}

impl PackageConfig {
    /// إنشاء تكوين جديد
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            description: String::new(),
            author: String::new(),
            license: default_license(),
            keywords: Vec::new(),
            repository: None,
            documentation: None,
            readme: default_readme(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            entry: default_entry(),
            include: default_include(),
            exclude: default_exclude(),
            features: HashMap::new(),
            settings: PackageSettings::default(),
        }
    }

    /// تحميل من ملف
    pub fn load(path: &Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("فشل قراءة الملف: {}", e))?;
        
        Self::parse(&content)
    }

    /// تحليل النص
    pub fn parse(content: &str) -> Result<Self, String> {
        // تحليل TOML
        toml::from_str(content)
            .map_err(|e| format!("خطأ في تحليل TOML: {}", e))
    }

    /// حفظ إلى ملف
    pub fn save(&self, path: &Path) -> Result<(), String> {
        let content = self.to_toml()?;
        std::fs::write(path, content)
            .map_err(|e| format!("فشل كتابة الملف: {}", e))
    }

    /// تحويل إلى TOML
    pub fn to_toml(&self) -> Result<String, String> {
        toml::to_string_pretty(self)
            .map_err(|e| format!("خطأ في تحويل TOML: {}", e))
    }

    /// إضافة تبعية
    pub fn add_dependency(&mut self, name: &str, version: &str) {
        self.dependencies.insert(name.to_string(), version.to_string());
    }

    /// إزالة تبعية
    pub fn remove_dependency(&mut self, name: &str) {
        self.dependencies.remove(name);
    }

    /// إضافة تبعية تطوير
    pub fn add_dev_dependency(&mut self, name: &str, version: &str) {
        self.dev_dependencies.insert(name.to_string(), version.to_string());
    }

    /// التحقق من صحة التكوين
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // التحقق من الاسم
        if self.name.is_empty() {
            errors.push("اسم الحزمة مطلوب".to_string());
        } else if !self.name.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            errors.push("اسم الحزمة يجب أن يحتوي فقط على أحرف وأرقام و _ و -".to_string());
        }
        
        // التحقق من الإصدار
        if self.version.is_empty() {
            errors.push("الإصدار مطلوب".to_string());
        } else if !self.is_valid_version(&self.version) {
            errors.push("صيغة الإصدار غير صحيحة (استخدم x.y.z)".to_string());
        }
        
        // التحقق من التبعيات
        for (name, version) in &self.dependencies {
            if !self.is_valid_version(version) && !version.starts_with('^') && !version.starts_with('~') {
                errors.push(format!("إصدار التبعية '{}' غير صالح: {}", name, version));
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// التحقق من صحة الإصدار
    fn is_valid_version(&self, version: &str) -> bool {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return false;
        }
        parts.iter().all(|p| p.parse::<u32>().is_ok())
    }

    /// إنشاء قالب TOML افتراضي
    pub fn template(name: &str) -> String {
        format!(
r#"# ═══════════════════════════════════════════════════════════════════
# ملف تعريف المشروع - لغة المرجع
# ═══════════════════════════════════════════════════════════════════

# معلومات أساسية
اسم = "{}"
إصدار = "0.1.0"
وصف = "مشروع جديد بلغة المرجع"
مؤلف = ""
رخصة = "MIT"

# الكلمات المفتاحية
كلمات = ["مرجع", "عربي"]

# الروابط
[روابط]
مستودع = ""
توثيق = ""

# التبعيات
[تبعيات]
# json = "1.0.0"

# تبعيات التطوير
[تبعيات_تطوير]
# اختبارات = "1.0.0"

# الإعدادات
[إعدادات]
تحسين = true
تصحيح = false
"#,
            name
        )
    }
}

impl Default for PackageConfig {
    fn default() -> Self {
        Self::new("مشروع-جديد")
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = PackageConfig::new("اختبار");
        assert_eq!(config.name, "اختبار");
        assert_eq!(config.version, "0.1.0");
    }

    #[test]
    fn test_add_dependency() {
        let mut config = PackageConfig::new("اختبار");
        config.add_dependency("json", "1.0.0");
        
        assert!(config.dependencies.contains_key("json"));
        assert_eq!(config.dependencies.get("json"), Some(&"1.0.0".to_string()));
    }

    #[test]
    fn test_remove_dependency() {
        let mut config = PackageConfig::new("اختبار");
        config.add_dependency("json", "1.0.0");
        config.remove_dependency("json");
        
        assert!(!config.dependencies.contains_key("json"));
    }

    #[test]
    fn test_validate() {
        let config = PackageConfig::new("اختبار");
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_name() {
        let config = PackageConfig::new("");
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_toml_conversion() {
        let config = PackageConfig::new("اختبار");
        let toml = config.to_toml().unwrap();
        assert!(toml.contains("اختبار"));
    }

    #[test]
    fn test_template() {
        let template = PackageConfig::template("مشروعي");
        assert!(template.contains("مشروعي"));
        assert!(template.contains("تبعيات"));
    }
}
