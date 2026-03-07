// ═══════════════════════════════════════════════════════════════════════════════
// محلل التبعيات - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::{HashMap, HashSet, BTreeMap};
use serde::{Deserialize, Serialize};

/// محلل التبعيات الذكي
pub struct DependencyResolver {
    /// التبعيات المحلولة
    resolved: HashMap<String, ResolvedPackage>,
    /// التبعيات المتضاربة
    conflicts: Vec<DependencyConflict>,
    /// السجل
    log: Vec<String>,
}

/// حزمة محلولة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedPackage {
    /// الاسم
    pub name: String,
    /// الإصدار المحلول
    pub version: String,
    /// الإصدار المطلوب
    pub requested_version: String,
    /// المصدر
    pub source: String,
    /// التبعيات الفرعية
    pub dependencies: Vec<String>,
    /// العمق في شجرة التبعيات
    pub depth: usize,
}

/// تضارب التبعيات
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyConflict {
    /// اسم الحزمة
    pub package: String,
    /// الإصدارات المتضاربة
    pub versions: Vec<String>,
    /// الحزم التي تطلب كل إصدار
    pub requested_by: HashMap<String, String>,
    /// الحل المقترح
    pub suggested_resolution: String,
}

/// نتيجة التحليل
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedDependencies {
    /// التبعيات المحلولة
    pub packages: BTreeMap<String, ResolvedPackage>,
    /// التضاربات
    pub conflicts: Vec<DependencyConflict>,
    /// إجمالي الحزم
    pub total_packages: usize,
    /// أقصى عمق
    pub max_depth: usize,
    /// الوقت المستغرق (مللي ثانية)
    pub resolution_time_ms: u64,
}

impl DependencyResolver {
    /// إنشاء محلل جديد
    pub fn new() -> Self {
        Self {
            resolved: HashMap::new(),
            conflicts: Vec::new(),
            log: Vec::new(),
        }
    }

    /// تحليل التبعيات
    pub fn resolve(&mut self, dependencies: &HashMap<String, String>) -> Result<ResolvedDependencies, String> {
        let start = std::time::Instant::now();
        
        self.log.push(format!("🔍 بدء تحليل {} تبعية", dependencies.len()));
        
        // مسح التحليل السابق
        self.resolved.clear();
        self.conflicts.clear();
        
        // تحليل كل تبعية
        for (name, version) in dependencies {
            self.resolve_package(name, version, 0)?;
        }
        
        // التحقق من التضاربات
        self.detect_conflicts();
        
        let duration = start.elapsed().as_millis() as u64;
        let max_depth = self.resolved.values().map(|p| p.depth).max().unwrap_or(0);
        
        // تحويل إلى BTreeMap للترتيب
        let packages: BTreeMap<String, ResolvedPackage> = self.resolved.clone().into_iter().collect();
        
        self.log.push(format!("✅ تم تحليل {} حزمة في {}ms", packages.len(), duration));
        
        Ok(ResolvedDependencies {
            packages,
            conflicts: self.conflicts.clone(),
            total_packages: self.resolved.len(),
            max_depth,
            resolution_time_ms: duration,
        })
    }

    /// تحليل حزمة واحدة
    fn resolve_package(&mut self, name: &str, version: &str, depth: usize) -> Result<(), String> {
        // التحقق من وجود تحليل سابق
        if let Some(existing) = self.resolved.get(name) {
            // التحقق من التوافق
            if self.versions_compatible(&existing.version, version) {
                self.log.push(format!("  ✓ {} محلول مسبقاً", name));
                return Ok(());
            } else {
                // تضارب
                self.conflicts.push(DependencyConflict {
                    package: name.to_string(),
                    versions: vec![existing.version.clone(), version.to_string()],
                    requested_by: HashMap::new(),
                    suggested_resolution: self.suggest_resolution(&existing.version, version),
                });
                return Err(format!("تضارب في إصدار الحزمة '{}' : {} vs {}", name, existing.version, version));
            }
        }
        
        self.log.push(format!("{}📦 تحليل {}@{}", "  ".repeat(depth), name, version));
        
        // إضافة الحزمة
        let resolved_pkg = ResolvedPackage {
            name: name.to_string(),
            version: version.to_string(),
            requested_version: version.to_string(),
            source: "registry".to_string(),
            dependencies: vec![],
            depth,
        };
        
        self.resolved.insert(name.to_string(), resolved_pkg);
        
        // تحليل التبعيات الفرعية (محاكاة)
        let sub_deps = self.get_package_dependencies(name, version)?;
        for (sub_name, sub_version) in sub_deps {
            self.resolve_package(&sub_name, &sub_version, depth + 1)?;
            
            // إضافة التبعية الفرعية للحزمة
            if let Some(pkg) = self.resolved.get_mut(name) {
                pkg.dependencies.push(sub_name);
            }
        }
        
        Ok(())
    }

    /// الحصول على تبعيات الحزمة
    fn get_package_dependencies(&self, name: &str, version: &str) -> Result<HashMap<String, String>, String> {
        // محاكاة - في التطبيق الحقيقي سيتم جلب من Registry
        let deps = match name {
            "ويب" => {
                let mut map = HashMap::new();
                map.insert("json".to_string(), "1.0.0".to_string());
                map
            }
            "قاعدة_بيانات" => {
                let mut map = HashMap::new();
                map.insert("ملفات".to_string(), "1.0.0".to_string());
                map
            }
            "بريد" => {
                let mut map = HashMap::new();
                map.insert("ويب".to_string(), "1.0.0".to_string());
                map
            }
            _ => HashMap::new(),
        };
        
        Ok(deps)
    }

    /// التحقق من توافق الإصدارات
    fn versions_compatible(&self, v1: &str, v2: &str) -> bool {
        // إذا كانت متطابقة
        if v1 == v2 {
            return true;
        }
        
        // التحقق من التوافق مع ^
        if v1.starts_with('^') || v2.starts_with('^') {
            let clean_v1 = v1.trim_start_matches('^');
            let clean_v2 = v2.trim_start_matches('^');
            return self.same_major_version(clean_v1, clean_v2);
        }
        
        false
    }

    /// التحقق من نفس الإصدار الرئيسي
    fn same_major_version(&self, v1: &str, v2: &str) -> bool {
        let parts1: Vec<&str> = v1.split('.').collect();
        let parts2: Vec<&str> = v2.split('.').collect();
        
        if parts1.is_empty() || parts2.is_empty() {
            return false;
        }
        
        parts1[0] == parts2[0]
    }

    /// اقتراح حل للتضارب
    fn suggest_resolution(&self, v1: &str, v2: &str) -> String {
        // اختيار الإصدار الأحدث
        let clean_v1 = v1.trim_start_matches('^');
        let clean_v2 = v2.trim_start_matches('^');
        
        let parts1: Vec<u32> = clean_v1.split('.')
            .filter_map(|p| p.parse().ok())
            .collect();
        let parts2: Vec<u32> = clean_v2.split('.')
            .filter_map(|p| p.parse().ok())
            .collect();
        
        if parts1.len() == 3 && parts2.len() == 3 {
            // مقارنة الإصدارات
            if parts1[0] != parts2[0] {
                format!("ترقية رئيسية مطلوبة - اختر {} أو {}", v1, v2)
            } else if parts1[1] > parts2[1] || (parts1[1] == parts2[1] && parts1[2] > parts2[2]) {
                format!("استخدم {} (الأحدث)", v1)
            } else {
                format!("استخدم {} (الأحدث)", v2)
            }
        } else {
            format!("اختر الإصدار المناسب يدوياً")
        }
    }

    /// كشف التضاربات
    fn detect_conflicts(&mut self) {
        // التضاربات تم كشفها أثناء التحليل
    }

    /// الحصول على السجل
    pub fn get_log(&self) -> &[String] {
        &self.log
    }

    /// طباعة شجرة التبعيات
    pub fn print_tree(&self, dependencies: &ResolvedDependencies) {
        println!("\n🌳 شجرة التبعيات:");
        
        for (name, pkg) in &dependencies.packages {
            let indent = "  ".repeat(pkg.depth);
            let deps_str = if pkg.dependencies.is_empty() {
                String::new()
            } else {
                format!(" → [{}]", pkg.dependencies.join(", "))
            };
            println!("{}├── {}@{}{}", indent, name, pkg.version, deps_str);
        }
        
        println!("\n📊 الإحصائيات:");
        println!("   • إجمالي الحزم: {}", dependencies.total_packages);
        println!("   • أقصى عمق: {}", dependencies.max_depth);
        println!("   • وقت التحليل: {}ms", dependencies.resolution_time_ms);
    }

    /// إنشاء ملف قفل
    pub fn generate_lockfile(&self, resolved: &ResolvedDependencies) -> String {
        let mut lock = String::new();
        
        lock.push_str("# ═══════════════════════════════════════════════════════\n");
        lock.push_str("# ملف القفل - لغة المرجع\n");
        lock.push_str("# ═══════════════════════════════════════════════════════\n");
        lock.push_str("# تم إنشاؤه تلقائياً - لا تعدل هذا الملف يدوياً\n\n");
        
        lock.push_str("[حزم]\n");
        for (name, pkg) in &resolved.packages {
            lock.push_str(&format!("{} = \"{}\"\n", name, pkg.version));
        }
        
        lock
    }
}

impl Default for DependencyResolver {
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
    fn test_resolver_creation() {
        let resolver = DependencyResolver::new();
        assert!(resolver.resolved.is_empty());
    }

    #[test]
    fn test_resolve_empty() {
        let mut resolver = DependencyResolver::new();
        let deps = HashMap::new();
        let result = resolver.resolve(&deps).unwrap();
        
        assert_eq!(result.total_packages, 0);
    }

    #[test]
    fn test_resolve_single_package() {
        let mut resolver = DependencyResolver::new();
        let mut deps = HashMap::new();
        deps.insert("json".to_string(), "1.0.0".to_string());
        
        let result = resolver.resolve(&deps).unwrap();
        
        assert_eq!(result.total_packages, 1);
        assert!(result.packages.contains_key("json"));
    }

    #[test]
    fn test_resolve_with_dependencies() {
        let mut resolver = DependencyResolver::new();
        let mut deps = HashMap::new();
        deps.insert("ويب".to_string(), "1.0.0".to_string());
        
        let result = resolver.resolve(&deps).unwrap();
        
        // يجب أن تحل 'ويب' وتبعيتها 'json'
        assert!(result.total_packages >= 1);
    }

    #[test]
    fn test_versions_compatible() {
        let resolver = DependencyResolver::new();
        
        assert!(resolver.versions_compatible("1.0.0", "1.0.0"));
        assert!(!resolver.versions_compatible("1.0.0", "2.0.0"));
    }

    #[test]
    fn test_lockfile_generation() {
        let mut resolver = DependencyResolver::new();
        let mut deps = HashMap::new();
        deps.insert("json".to_string(), "1.0.0".to_string());
        
        let result = resolver.resolve(&deps).unwrap();
        let lock = resolver.generate_lockfile(&result);
        
        assert!(lock.contains("json"));
        assert!(lock.contains("1.0.0"));
    }
}
