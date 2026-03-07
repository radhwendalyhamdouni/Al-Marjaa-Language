// ═══════════════════════════════════════════════════════════════════════════════
// نظام Workspace - لغة المرجع
// Workspace System for Monorepos
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

/// مساحة العمل
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    /// اسم المساحة
    pub name: String,
    /// المسار الجذر
    pub root: PathBuf,
    /// الحزم الأعضاء
    pub members: Vec<WorkspaceMember>,
    /// التبعيات المشتركة
    pub shared_dependencies: HashMap<String, String>,
    /// التكوين
    pub config: WorkspaceConfig,
}

/// عضو في المساحة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMember {
    /// اسم الحزمة
    pub name: String,
    /// المسار النسبي
    pub path: PathBuf,
    /// الإصدار
    pub version: String,
    /// هل هي الحزمة الرئيسية؟
    pub is_root: bool,
    /// التبعيات المحلية
    pub local_dependencies: Vec<String>,
}

/// تكوين المساحة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// نسخ التبعيات المشتركة
    pub hoist_dependencies: bool,
    /// التحقق من التوافق
    pub enforce_versions: bool,
    /// البناء المتوازي
    pub parallel_builds: bool,
    /// عدد العمليات المتوازية
    pub max_parallel: usize,
    /// التجاهل
    pub exclude: Vec<String>,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            hoist_dependencies: true,
            enforce_versions: true,
            parallel_builds: true,
            max_parallel: num_cpus::get(),
            exclude: Vec::new(),
        }
    }
}

impl Workspace {
    /// إنشاء مساحة عمل جديدة
    pub fn new(name: &str, root: &Path) -> Self {
        Self {
            name: name.to_string(),
            root: root.to_path_buf(),
            members: Vec::new(),
            shared_dependencies: HashMap::new(),
            config: WorkspaceConfig::default(),
        }
    }
    
    /// إضافة عضو
    pub fn add_member(&mut self, member: WorkspaceMember) {
        self.members.push(member);
    }
    
    /// إزالة عضو
    pub fn remove_member(&mut self, name: &str) -> Option<WorkspaceMember> {
        if let Some(pos) = self.members.iter().position(|m| m.name == name) {
            Some(self.members.remove(pos))
        } else {
            None
        }
    }
    
    /// الحصول على عضو
    pub fn get_member(&self, name: &str) -> Option<&WorkspaceMember> {
        self.members.iter().find(|m| m.name == name)
    }
    
    /// الحصول على العضو الجذري
    pub fn root_member(&self) -> Option<&WorkspaceMember> {
        self.members.iter().find(|m| m.is_root)
    }
    
    /// عدد الأعضاء
    pub fn member_count(&self) -> usize {
        self.members.len()
    }
    
    /// أسماء الأعضاء
    pub fn member_names(&self) -> Vec<String> {
        self.members.iter().map(|m| m.name.clone()).collect()
    }
    
    /// تبعيات العضو
    pub fn member_dependencies(&self, name: &str) -> Vec<String> {
        let mut all_deps = Vec::new();
        
        if let Some(member) = self.get_member(name) {
            // التبعيات المحلية
            all_deps.extend(member.local_dependencies.clone());
            
            // التبعيات المشتركة
            all_deps.extend(self.shared_dependencies.keys().cloned());
        }
        
        all_deps
    }
    
    /// شجرة التبعيات
    pub fn dependency_tree(&self) -> HashMap<String, Vec<String>> {
        let mut tree = HashMap::new();
        
        for member in &self.members {
            tree.insert(member.name.clone(), member.local_dependencies.clone());
        }
        
        tree
    }
    
    /// فرز الأعضاء حسب التبعيات
    pub fn topological_sort(&self) -> Vec<String> {
        let mut sorted = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut temp = std::collections::HashSet::new();
        
        fn visit(
            name: &str,
            members: &[WorkspaceMember],
            visited: &mut std::collections::HashSet<String>,
            temp: &mut std::collections::HashSet<String>,
            sorted: &mut Vec<String>,
        ) {
            if visited.contains(name) {
                return;
            }
            if temp.contains(name) {
                return;
            }
            
            temp.insert(name.to_string());
            
            if let Some(member) = members.iter().find(|m| m.name == name) {
                for dep in &member.local_dependencies {
                    visit(dep, members, visited, temp, sorted);
                }
            }
            
            temp.remove(name);
            visited.insert(name.to_string());
            sorted.push(name.to_string());
        }
        
        for member in &self.members {
            visit(&member.name, &self.members, &mut visited, &mut temp, &mut sorted);
        }
        
        sorted
    }
    
    /// حفظ التكوين
    pub fn save(&self) -> Result<(), String> {
        let config_path = self.root.join("مساحة.toml");
        
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("فشل تحويل التكوين: {}", e))?;
        
        std::fs::write(&config_path, content)
            .map_err(|e| format!("فشل حفظ التكوين: {}", e))
    }
    
    /// تحميل التكوين
    pub fn load(root: &Path) -> Result<Self, String> {
        let config_path = root.join("مساحة.toml");
        
        if !config_path.exists() {
            return Err("ملف تكوين المساحة غير موجود".to_string());
        }
        
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("فشل قراءة التكوين: {}", e))?;
        
        toml::from_str(&content)
            .map_err(|e| format!("فشل تحليل التكوين: {}", e))
    }
    
    /// التحقق من صحة المساحة
    pub fn validate(&self) -> Result<Vec<String>, String> {
        let mut warnings = Vec::new();
        
        // التحقق من وجود العضو الجذري
        if self.root_member().is_none() {
            warnings.push("لا يوجد عضو جذري".to_string());
        }
        
        // التحقق من التبعيات المحلية
        for member in &self.members {
            for dep in &member.local_dependencies {
                if !self.members.iter().any(|m| m.name == *dep) {
                    return Err(format!(
                        "العضو '{}' يعتمد على '{}' غير الموجود",
                        member.name, dep
                    ));
                }
            }
        }
        
        // التحقق من الدورات
        let sorted = self.topological_sort();
        if sorted.len() != self.members.len() {
            warnings.push("يوجد دورات في التبعيات".to_string());
        }
        
        Ok(warnings)
    }
    
    /// تحديث الإصدارات
    pub fn bump_versions(&mut self, package: &str, new_version: &str) {
        if let Some(member) = self.members.iter_mut().find(|m| m.name == package) {
            member.version = new_version.to_string();
        }
    }
    
    /// البحث عن حزمة
    pub fn find_package(&self, name: &str) -> Option<PathBuf> {
        self.members
            .iter()
            .find(|m| m.name == name)
            .map(|m| self.root.join(&m.path))
    }
}

impl WorkspaceMember {
    /// إنشاء عضو جديد
    pub fn new(name: &str, path: &Path, version: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_path_buf(),
            version: version.to_string(),
            is_root: false,
            local_dependencies: Vec::new(),
        }
    }
    
    /// جعله جذرياً
    pub fn as_root(mut self) -> Self {
        self.is_root = true;
        self
    }
    
    /// إضافة تبعية محلية
    pub fn add_dependency(mut self, name: &str) -> Self {
        if !self.local_dependencies.contains(&name.to_string()) {
            self.local_dependencies.push(name.to_string());
        }
        self
    }
}

/// منشئ المساحة
pub struct WorkspaceBuilder {
    name: String,
    root: PathBuf,
    members: Vec<WorkspaceMember>,
    shared_deps: HashMap<String, String>,
    config: WorkspaceConfig,
}

impl WorkspaceBuilder {
    /// إنشاء منشئ جديد
    pub fn new(name: &str, root: &Path) -> Self {
        Self {
            name: name.to_string(),
            root: root.to_path_buf(),
            members: Vec::new(),
            shared_deps: HashMap::new(),
            config: WorkspaceConfig::default(),
        }
    }
    
    /// إضافة عضو
    pub fn member(mut self, member: WorkspaceMember) -> Self {
        self.members.push(member);
        self
    }
    
    /// إضافة تبعية مشتركة
    pub fn shared_dep(mut self, name: &str, version: &str) -> Self {
        self.shared_deps.insert(name.to_string(), version.to_string());
        self
    }
    
    /// تعيين التكوين
    pub fn config(mut self, config: WorkspaceConfig) -> Self {
        self.config = config;
        self
    }
    
    /// البناء
    pub fn build(self) -> Result<Workspace, String> {
        let workspace = Workspace {
            name: self.name,
            root: self.root,
            members: self.members,
            shared_dependencies: self.shared_deps,
            config: self.config,
        };
        
        // التحقق
        workspace.validate()?;
        
        Ok(workspace)
    }
}

/// مشغل المهام على المساحة
pub struct WorkspaceRunner {
    workspace: Workspace,
}

impl WorkspaceRunner {
    /// إنشاء مشغل جديد
    pub fn new(workspace: Workspace) -> Self {
        Self { workspace }
    }
    
    /// تنفيذ أمر على كل الأعضاء
    pub fn run_all(&self, command: &str) -> Result<HashMap<String, bool>, String> {
        let mut results = HashMap::new();
        let sorted = self.workspace.topological_sort();
        
        for name in sorted {
            println!("📦 {} - تنفيذ: {}", name, command);
            
            if let Some(member) = self.workspace.get_member(&name) {
                let member_path = self.workspace.root.join(&member.path);
                
                // محاكاة التنفيذ
                let success = true; // في التنفيذ الحقيقي، ننفذ الأمر
                
                results.insert(name, success);
                
                if success {
                    println!("   ✅ نجاح");
                } else {
                    println!("   ❌ فشل");
                }
            }
        }
        
        Ok(results)
    }
    
    /// تنفيذ أمر على عضو محدد
    pub fn run_one(&self, name: &str, command: &str) -> Result<bool, String> {
        let member = self.workspace.get_member(name)
            .ok_or_else(|| format!("العضو '{}' غير موجود", name))?;
        
        let member_path = self.workspace.root.join(&member.path);
        
        println!("📦 {} - تنفيذ: {}", name, command);
        
        // محاكاة التنفيذ
        Ok(true)
    }
    
    /// بناء كل الأعضاء
    pub fn build_all(&self) -> Result<HashMap<String, bool>, String> {
        self.run_all("build")
    }
    
    /// اختبار كل الأعضاء
    pub fn test_all(&self) -> Result<HashMap<String, bool>, String> {
        self.run_all("test")
    }
    
    /// نشر كل الأعضاء
    pub fn publish_all(&self) -> Result<HashMap<String, bool>, String> {
        let sorted = self.workspace.topological_sort();
        let mut results = HashMap::new();
        
        for name in sorted {
            println!("📤 نشر: {}", name);
            results.insert(name, true);
        }
        
        Ok(results)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_workspace_creation() {
        let workspace = Workspace::new("my-workspace", Path::new("."));
        
        assert_eq!(workspace.name, "my-workspace");
        assert!(workspace.members.is_empty());
    }
    
    #[test]
    fn test_workspace_member() {
        let member = WorkspaceMember::new("core", Path::new("packages/core"), "1.0.0");
        
        assert_eq!(member.name, "core");
        assert!(!member.is_root);
    }
    
    #[test]
    fn test_workspace_builder() {
        let workspace = WorkspaceBuilder::new("test", Path::new("."))
            .member(WorkspaceMember::new("core", Path::new("core"), "1.0.0").as_root())
            .member(WorkspaceMember::new("utils", Path::new("utils"), "1.0.0").add_dependency("core"))
            .shared_dep("serde", "1.0")
            .build()
            .unwrap();
        
        assert_eq!(workspace.member_count(), 2);
    }
    
    #[test]
    fn test_topological_sort() {
        let workspace = WorkspaceBuilder::new("test", Path::new("."))
            .member(WorkspaceMember::new("A", Path::new("a"), "1.0.0").add_dependency("B"))
            .member(WorkspaceMember::new("B", Path::new("b"), "1.0.0"))
            .build()
            .unwrap();
        
        let sorted = workspace.topological_sort();
        
        let b_pos = sorted.iter().position(|n| n == "B").unwrap();
        let a_pos = sorted.iter().position(|n| n == "A").unwrap();
        
        assert!(b_pos < a_pos);
    }
}
