// ═══════════════════════════════════════════════════════════════════════════════
// محلل التبعيات - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

use super::registry::PackageInfo;
use std::collections::{HashMap, HashSet, VecDeque};

/// تبعية
#[derive(Debug, Clone)]
pub struct Dependency {
    /// اسم الحزمة
    pub name: String,
    /// الإصدار المطلوب
    pub version: String,
    /// هل هي تبعية تطوير
    pub dev: bool,
}

/// رسم بياني للتبعيات
#[derive(Debug, Default, Clone)]
pub struct DependencyGraph {
    /// العقد (الحزم)
    nodes: HashMap<String, Vec<String>>,
    /// ترتيب طوبولوجي
    order: Vec<String>,
}

impl DependencyGraph {
    /// إنشاء رسم بياني جديد
    pub fn new() -> Self {
        Self::default()
    }
    
    /// إضافة علاقة تبعية
    pub fn add_dependency(&mut self, package: &str, depends_on: &str) {
        self.nodes
            .entry(package.to_string())
            .or_default()
            .push(depends_on.to_string());
        
        if !self.nodes.contains_key(depends_on) {
            self.nodes.entry(depends_on.to_string()).or_default();
        }
    }
    
    /// الترتيب الطوبولوجي
    pub fn topological_sort(&mut self) -> Result<Vec<String>, String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        
        // حساب درجة الدخول لكل عقدة
        for node in self.nodes.keys() {
            in_degree.entry(node.clone()).or_insert(0);
        }
        
        for deps in self.nodes.values() {
            for dep in deps {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }
        
        // عكس الاتجاه للترتيب الصحيح
        let mut reversed: HashMap<String, Vec<String>> = HashMap::new();
        for (pkg, deps) in &self.nodes {
            for dep in deps {
                reversed
                    .entry(dep.clone())
                    .or_default()
                    .push(pkg.clone());
            }
        }
        
        // BFS للترتيب الطوبولوجي
        let mut queue: VecDeque<String> = VecDeque::new();
        for (node, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node.clone());
            }
        }
        
        let mut result = Vec::new();
        
        while let Some(node) = queue.pop_front() {
            result.push(node.clone());
            
            if let Some(neighbors) = reversed.get(&node) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }
        
        if result.len() != self.nodes.len() {
            return Err("تم اكتشاف تبعية دائرية!".to_string());
        }
        
        self.order = result.clone();
        Ok(result)
    }
    
    /// الحصول على الترتيب
    pub fn order(&self) -> &[String] {
        &self.order
    }
}

/// محلل التبعيات
#[derive(Debug, Default, Clone)]
pub struct DependencyResolver {
    /// التبعيات المحفوظة
    cache: HashMap<String, Vec<String>>,
}

impl DependencyResolver {
    /// إنشاء محلل جديد
    pub fn new() -> Self {
        Self::default()
    }
    
    /// حل التبعيات
    pub fn resolve(&mut self, info: &PackageInfo, _version: &str) -> Result<Vec<String>, String> {
        // التحقق من الـ cache
        if let Some(cached) = self.cache.get(&info.name) {
            return Ok(cached.clone());
        }
        
        // بناء الرسم البياني
        let mut graph = DependencyGraph::new();
        let mut visited = HashSet::new();
        
        self.build_graph(info, &mut graph, &mut visited)?;
        
        // الترتيب الطوبولوجي
        let order = graph.topological_sort()?;
        
        // فلترة الحزمة الرئيسية من القائمة
        let dependencies: Vec<String> = order
            .into_iter()
            .filter(|name| name != &info.name)
            .collect();
        
        // تخزين في الـ cache
        self.cache.insert(info.name.clone(), dependencies.clone());
        
        Ok(dependencies)
    }
    
    /// بناء الرسم البياني للتبعيات
    fn build_graph(
        &self,
        info: &PackageInfo,
        graph: &mut DependencyGraph,
        visited: &mut HashSet<String>,
    ) -> Result<(), String> {
        if visited.contains(&info.name) {
            return Ok(());
        }
        visited.insert(info.name.clone());
        
        for dep in &info.dependencies {
            graph.add_dependency(&info.name, dep);
        }
        
        Ok(())
    }
    
    /// التحقق من التوافق
    pub fn check_compatibility(&self, deps: &[Dependency]) -> Result<(), String> {
        let mut packages: HashMap<String, String> = HashMap::new();
        
        for dep in deps {
            if let Some(existing_version) = packages.get(&dep.name) {
                if existing_version != &dep.version {
                    return Err(format!(
                        "تضارب في الإصدارات: {} يطلب {} و {}",
                        dep.name, existing_version, dep.version
                    ));
                }
            } else {
                packages.insert(dep.name.clone(), dep.version.clone());
            }
        }
        
        Ok(())
    }
    
    /// مسح الـ cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    
    /// الحصول على حجم الـ cache
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}
