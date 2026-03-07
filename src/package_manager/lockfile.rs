// ═══════════════════════════════════════════════════════════════════════════════
// نظام Lockfile المتقدم - لغة المرجع
// Advanced Lockfile System
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// ملف القفل
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lockfile {
    /// إصدار ملف القفل
    pub version: u32,
    /// وقت الإنشاء
    pub created_at: u64,
    /// وقت آخر تحديث
    pub updated_at: u64,
    /// الحزم المقفولة
    pub packages: HashMap<String, LockedPackage>,
    /// شجرة التبعيات
    pub dependency_tree: HashMap<String, Vec<String>>,
    /// checksums
    pub checksums: HashMap<String, String>,
    /// التجزئة الكلية
    pub content_hash: String,
}

/// حزمة مقفولة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedPackage {
    /// الاسم
    pub name: String,
    /// الإصدار الدقيق
    pub version: String,
    /// المصدر
    pub source: PackageSourceLock,
    /// التجزئة
    pub checksum: String,
    /// التبعيات
    pub dependencies: Vec<String>,
    /// الإصدارات المتوافقة
    pub compatible_versions: Vec<String>,
    /// المنصة
    pub platform: Option<String>,
    /// الميزات المفعّلة
    pub features: Vec<String>,
    /// الحجم بالبايت
    pub size: u64,
    /// وقت التثبيت
    pub installed_at: u64,
}

/// مصدر الحزمة في Lockfile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageSourceLock {
    /// المستودع الرسمي
    Registry {
        url: String,
    },
    /// Git
    Git {
        url: String,
        commit: String,
        tag: Option<String>,
    },
    /// مسار محلي
    Local {
        path: String,
    },
    /// IPFS
    IPFS {
        cid: String,
    },
}

impl Lockfile {
    /// إنشاء ملف قفل جديد
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            version: 1,
            created_at: now,
            updated_at: now,
            packages: HashMap::new(),
            dependency_tree: HashMap::new(),
            checksums: HashMap::new(),
            content_hash: String::new(),
        }
    }
    
    /// إضافة حزمة
    pub fn add_package(&mut self, package: LockedPackage) {
        let name = package.name.clone();
        let deps = package.dependencies.clone();
        self.packages.insert(name.clone(), package);
        self.dependency_tree.insert(name, deps);
        self.update_hash();
    }
    
    /// إزالة حزمة
    pub fn remove_package(&mut self, name: &str) -> Option<LockedPackage> {
        let removed = self.packages.remove(name);
        self.dependency_tree.remove(name);
        self.update_hash();
        removed
    }
    
    /// الحصول على حزمة
    pub fn get_package(&self, name: &str) -> Option<&LockedPackage> {
        self.packages.get(name)
    }
    
    /// تحديث التجزئة
    fn update_hash(&mut self) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        
        // ترتيب الحزم أبجدياً للتجزئة المتسقة
        let mut packages: Vec<_> = self.packages.iter().collect();
        packages.sort_by_key(|(k, _)| *k);
        
        for (name, pkg) in packages {
            name.hash(&mut hasher);
            pkg.version.hash(&mut hasher);
            pkg.checksum.hash(&mut hasher);
        }
        
        self.content_hash = format!("{:016x}", hasher.finish());
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// حفظ ملف القفل
    pub fn save(&self, path: &Path) -> Result<(), String> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("فشل تحويل ملف القفل: {}", e))?;
        
        std::fs::write(path, content)
            .map_err(|e| format!("فشل حفظ ملف القفل: {}", e))
    }
    
    /// تحميل ملف القفل
    pub fn load(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Err("ملف القفل غير موجود".to_string());
        }
        
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("فشل قراءة ملف القفل: {}", e))?;
        
        toml::from_str(&content)
            .map_err(|e| format!("فشل تحليل ملف القفل: {}", e))
    }
    
    /// التحقق من التكامل
    pub fn verify(&self) -> Result<bool, String> {
        let mut hasher = crc32fast::Hasher::new();
        
        for (name, pkg) in &self.packages {
            hasher.update(name.as_bytes());
            hasher.update(pkg.version.as_bytes());
            hasher.update(pkg.checksum.as_bytes());
        }
        
        let calculated = format!("{:08x}", hasher.finalize());
        
        // التحقق من كل حزمة
        for (name, pkg) in &self.packages {
            if let Some(expected) = self.checksums.get(name) {
                if expected != &pkg.checksum {
                    return Err(format!("الحزمة '{}' تم التلاعب بها", name));
                }
            }
        }
        
        Ok(true)
    }
    
    /// التحقق من وجود حزمة
    pub fn has_package(&self, name: &str) -> bool {
        self.packages.contains_key(name)
    }
    
    /// عدد الحزم
    pub fn package_count(&self) -> usize {
        self.packages.len()
    }
    
    /// الحجم الكلي
    pub fn total_size(&self) -> u64 {
        self.packages.values().map(|p| p.size).sum()
    }
    
    /// قائمة الحزم
    pub fn package_names(&self) -> Vec<String> {
        self.packages.keys().cloned().collect()
    }
    
    /// التحقق من التوافق
    pub fn check_compatibility(&self, package: &str, version: &str) -> bool {
        if let Some(pkg) = self.packages.get(package) {
            return pkg.compatible_versions.contains(&version.to_string()) 
                || pkg.version == version;
        }
        false
    }
    
    /// الحصول على التبعيات العكسية
    pub fn get_reverse_dependencies(&self, package: &str) -> Vec<String> {
        let mut reverse_deps = Vec::new();
        
        for (name, deps) in &self.dependency_tree {
            if deps.contains(&package.to_string()) {
                reverse_deps.push(name.clone());
            }
        }
        
        reverse_deps
    }
    
    /// فرز الحزم حسب التبعيات
    pub fn topological_sort(&self) -> Vec<String> {
        let mut sorted = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut temp = std::collections::HashSet::new();
        
        fn visit(
            name: &str,
            packages: &HashMap<String, LockedPackage>,
            visited: &mut std::collections::HashSet<String>,
            temp: &mut std::collections::HashSet<String>,
            sorted: &mut Vec<String>,
        ) {
            if visited.contains(name) {
                return;
            }
            if temp.contains(name) {
                // دورة - نتجاهلها
                return;
            }
            
            temp.insert(name.to_string());
            
            if let Some(pkg) = packages.get(name) {
                for dep in &pkg.dependencies {
                    visit(dep, packages, visited, temp, sorted);
                }
            }
            
            temp.remove(name);
            visited.insert(name.to_string());
            sorted.push(name.to_string());
        }
        
        for name in self.packages.keys() {
            visit(name, &self.packages, &mut visited, &mut temp, &mut sorted);
        }
        
        sorted
    }
    
    /// التحويل إلى TOML
    pub fn to_toml(&self) -> Result<String, String> {
        toml::to_string_pretty(self)
            .map_err(|e| format!("فشل تحويل إلى TOML: {}", e))
    }
    
    /// التحويل من TOML
    pub fn from_toml(content: &str) -> Result<Self, String> {
        toml::from_str(content)
            .map_err(|e| format!("فشل تحليل TOML: {}", e))
    }
    
    /// مقارنة مع ملف قفل آخر
    pub fn diff(&self, other: &Lockfile) -> LockfileDiff {
        let mut added = Vec::new();
        let mut removed = Vec::new();
        let mut changed = Vec::new();
        
        // الحزم المضافة
        for (name, pkg) in &self.packages {
            if !other.packages.contains_key(name) {
                added.push(pkg.clone());
            } else if let Some(other_pkg) = other.packages.get(name) {
                if pkg.version != other_pkg.version {
                    changed.push(LockfileChange {
                        name: name.clone(),
                        old_version: other_pkg.version.clone(),
                        new_version: pkg.version.clone(),
                    });
                }
            }
        }
        
        // الحزم المحذوفة
        for name in other.packages.keys() {
            if !self.packages.contains_key(name) {
                if let Some(pkg) = other.get_package(name) {
                    removed.push(pkg.clone());
                }
            }
        }
        
        LockfileDiff {
            added,
            removed,
            changed,
        }
    }
}

impl Default for Lockfile {
    fn default() -> Self {
        Self::new()
    }
}

/// فرق بين ملفي قفل
#[derive(Debug, Clone)]
pub struct LockfileDiff {
    /// الحزم المضافة
    pub added: Vec<LockedPackage>,
    /// الحزم المحذوفة
    pub removed: Vec<LockedPackage>,
    /// الحزم المتغيرة
    pub changed: Vec<LockfileChange>,
}

/// تغيير في حزمة
#[derive(Debug, Clone)]
pub struct LockfileChange {
    /// اسم الحزمة
    pub name: String,
    /// الإصدار القديم
    pub old_version: String,
    /// الإصدار الجديد
    pub new_version: String,
}

impl LockfileDiff {
    /// هل فارغ؟
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty()
    }
    
    /// طباعة الفرق
    pub fn print(&self) {
        if self.is_empty() {
            println!("✅ لا توجد تغييرات");
            return;
        }
        
        if !self.added.is_empty() {
            println!("📦 حزم مضافة:");
            for pkg in &self.added {
                println!("   + {} v{}", pkg.name, pkg.version);
            }
        }
        
        if !self.removed.is_empty() {
            println!("🗑️ حزم محذوفة:");
            for pkg in &self.removed {
                println!("   - {} v{}", pkg.name, pkg.version);
            }
        }
        
        if !self.changed.is_empty() {
            println!("🔄 حزم متغيرة:");
            for change in &self.changed {
                println!("   ~ {} {} → {}", change.name, change.old_version, change.new_version);
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lockfile_creation() {
        let lockfile = Lockfile::new();
        assert_eq!(lockfile.version, 1);
        assert!(lockfile.packages.is_empty());
    }
    
    #[test]
    fn test_add_package() {
        let mut lockfile = Lockfile::new();
        
        let pkg = LockedPackage {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            source: PackageSourceLock::Registry { url: "https://registry.almarjaa.io".to_string() },
            checksum: "abc123".to_string(),
            dependencies: vec![],
            compatible_versions: vec!["1.0.0".to_string(), "1.0.1".to_string()],
            platform: None,
            features: vec![],
            size: 1024,
            installed_at: 0,
        };
        
        lockfile.add_package(pkg);
        
        assert_eq!(lockfile.package_count(), 1);
        assert!(lockfile.has_package("test"));
    }
    
    #[test]
    fn test_topological_sort() {
        let mut lockfile = Lockfile::new();
        
        // حزمة A تعتمد على B
        lockfile.add_package(LockedPackage {
            name: "A".to_string(),
            version: "1.0.0".to_string(),
            source: PackageSourceLock::Registry { url: String::new() },
            checksum: String::new(),
            dependencies: vec!["B".to_string()],
            compatible_versions: vec![],
            platform: None,
            features: vec![],
            size: 0,
            installed_at: 0,
        });
        
        lockfile.add_package(LockedPackage {
            name: "B".to_string(),
            version: "1.0.0".to_string(),
            source: PackageSourceLock::Registry { url: String::new() },
            checksum: String::new(),
            dependencies: vec![],
            compatible_versions: vec![],
            platform: None,
            features: vec![],
            size: 0,
            installed_at: 0,
        });
        
        let sorted = lockfile.topological_sort();
        
        // B يجب أن يأتي قبل A
        let b_pos = sorted.iter().position(|n| n == "B").unwrap();
        let a_pos = sorted.iter().position(|n| n == "A").unwrap();
        assert!(b_pos < a_pos);
    }
}
