// ═══════════════════════════════════════════════════════════════════════════════
// ربط البيانات التلقائي - Automatic Data Binding
// ═══════════════════════════════════════════════════════════════════════════════

use crate::ui::types::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// ═══════════════════════════════════════════════════════════════════════════════
// أنواع الربط
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع الربط
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BindingMode {
    /// ربط أحادي الاتجاه (من المصدر للهدف)
    OneWay,
    /// ربط أحادي الاتجاه (من الهدف للمصدر)
    OneWayToSource,
    /// ربط ثنائي الاتجاه
    TwoWay,
    /// ربط مرة واحدة
    OneTime,
}

impl Default for BindingMode {
    fn default() -> Self {
        Self::OneWay
    }
}

/// نوع الربط المتقدم
#[derive(Debug, Clone, PartialEq)]
pub enum BindingKind {
    /// ربط بسيط
    Simple {
        source_path: BindingPath,
        target_property: PropertyName,
    },
    /// ربط مع تحويل
    Transform {
        source_path: BindingPath,
        target_property: PropertyName,
        transform: String, // اسم دالة التحويل
    },
    /// ربط شرطي
    Conditional {
        condition_path: BindingPath,
        true_value: UIValue,
        false_value: UIValue,
        target_property: PropertyName,
    },
    /// ربط متعدد
    Multi {
        source_paths: Vec<BindingPath>,
        target_property: PropertyName,
        combiner: String, // اسم دالة الدمج
    },
    /// ربط مع تنسيق
    Format {
        source_path: BindingPath,
        format: String,
        target_property: PropertyName,
    },
}

// ═══════════════════════════════════════════════════════════════════════════════
// القيم القابلة للملاحظة
// ═══════════════════════════════════════════════════════════════════════════════

/// قيمة قابلة للملاحظة
pub trait Observable: Send + Sync {
    /// الحصول على القيمة
    fn get(&self) -> UIValue;
    
    /// تعيين القيمة
    fn set(&mut self, value: UIValue);
    
    /// الاشتراك في التغييرات
    fn subscribe(&mut self, callback: WatcherCallback);
}

/// دالة المراقبة
pub type WatcherCallback = Arc<dyn Fn(&UIValue) + Send + Sync>;

/// قيمة قابلة للملاحظة بسيطة
#[derive(Debug, Clone)]
pub struct ObservableValue {
    /// القيمة الحالية
    value: UIValue,
    /// المراقبين
    watchers: Vec<WatcherCallback>,
}

impl ObservableValue {
    pub fn new(value: UIValue) -> Self {
        Self {
            value,
            watchers: Vec::new(),
        }
    }
    
    pub fn get(&self) -> UIValue {
        self.value.clone()
    }
    
    pub fn set(&mut self, value: UIValue) {
        if self.value != value {
            self.value = value.clone();
            for watcher in &self.watchers {
                watcher(&value);
            }
        }
    }
    
    pub fn subscribe(&mut self, callback: WatcherCallback) {
        self.watchers.push(callback);
    }
}

impl Default for ObservableValue {
    fn default() -> Self {
        Self::new(UIValue::Null)
    }
}

/// مجموعة قابلة للملاحظة
#[derive(Debug, Clone)]
pub struct ObservableCollection {
    /// العناصر
    items: Vec<UIValue>,
    /// المراقبين
    watchers: Vec<WatcherCallback>,
}

impl ObservableCollection {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            watchers: Vec::new(),
        }
    }
    
    pub fn get(&self, index: usize) -> Option<&UIValue> {
        self.items.get(index)
    }
    
    pub fn push(&mut self, value: UIValue) {
        self.items.push(value);
        self.notify();
    }
    
    pub fn remove(&mut self, index: usize) -> Option<UIValue> {
        if index < self.items.len() {
            let value = self.items.remove(index);
            self.notify();
            Some(value)
        } else {
            None
        }
    }
    
    pub fn len(&self) -> usize {
        self.items.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &UIValue> {
        self.items.iter()
    }
    
    fn notify(&self) {
        // إشعار المراقبين
    }
}

impl Default for ObservableCollection {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// القيم المحسوبة
// ═══════════════════════════════════════════════════════════════════════════════

/// قيمة محسوبة
pub struct ComputedValue {
    /// القيمة المخزنة
    cached_value: Option<UIValue>,
    /// الدالة الحاسبة
    computer: Box<dyn Fn() -> UIValue + Send + Sync>,
    /// التبعيات
    dependencies: Vec<String>,
    /// المراقبين
    watchers: Vec<WatcherCallback>,
}

impl ComputedValue {
    pub fn new<F>(computer: F, dependencies: Vec<String>) -> Self
    where
        F: Fn() -> UIValue + Send + Sync + 'static,
    {
        Self {
            cached_value: None,
            computer: Box::new(computer),
            dependencies,
            watchers: Vec::new(),
        }
    }
    
    pub fn get(&mut self) -> UIValue {
        if self.cached_value.is_none() {
            self.cached_value = Some((self.computer)());
        }
        self.cached_value.clone().unwrap()
    }
    
    pub fn invalidate(&mut self) {
        let old_value = self.cached_value.clone();
        self.cached_value = None;
        let new_value = self.get();
        
        if old_value.as_ref() != Some(&new_value) {
            for watcher in &self.watchers {
                watcher(&new_value);
            }
        }
    }
}

/// قيمة محسوبة بسيطة
#[derive(Debug, Clone)]
pub struct Computed {
    /// اسم القيمة
    pub name: String,
    /// التعبير
    pub expression: String,
    /// التبعيات
    pub dependencies: Vec<String>,
}

impl Computed {
    pub fn new(name: &str, expression: &str, dependencies: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            expression: expression.to_string(),
            dependencies,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
/// المراقب
// ═══════════════════════════════════════════════════════════════════════════════

/// مراقب
pub struct Watcher {
    /// المعرف
    pub id: String,
    /// المسار المراقب
    pub path: BindingPath,
    /// دالة الاستدعاء
    pub callback: WatcherCallback,
    /// نشط
    pub active: bool,
}

impl Watcher {
    pub fn new(path: BindingPath, callback: WatcherCallback) -> Self {
        Self {
            id: format!("watcher_{}", uuid()),
            path,
            callback,
            active: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// تعبير الربط
// ═══════════════════════════════════════════════════════════════════════════════

/// تعبير ربط
#[derive(Debug, Clone)]
pub struct BindingExpression {
    /// المصدر
    pub source: BindingPath,
    /// الهدف
    pub target: PropertyName,
    /// الوضع
    pub mode: BindingMode,
    /// التحويل
    pub converter: Option<String>,
    /// التنسيق
    pub format: Option<String>,
    /// القيمة الافتراضية
    pub default: Option<UIValue>,
}

impl BindingExpression {
    pub fn new(source: &str, target: &str) -> Self {
        Self {
            source: source.to_string(),
            target: target.to_string(),
            mode: BindingMode::OneWay,
            converter: None,
            format: None,
            default: None,
        }
    }
    
    pub fn two_way(mut self) -> Self {
        self.mode = BindingMode::TwoWay;
        self
    }
    
    pub fn one_time(mut self) -> Self {
        self.mode = BindingMode::OneTime;
        self
    }
    
    pub fn converter(mut self, converter: &str) -> Self {
        self.converter = Some(converter.to_string());
        self
    }
    
    pub fn format(mut self, format: &str) -> Self {
        self.format = Some(format.to_string());
        self
    }
    
    pub fn default(mut self, value: UIValue) -> Self {
        self.default = Some(value);
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الربط
// ═══════════════════════════════════════════════════════════════════════════════

/// ربط
#[derive(Debug)]
pub struct Binding {
    /// المعرف
    pub id: String,
    /// المكون الهدف
    pub target_component: ComponentId,
    /// الخاصية الهدف
    pub target_property: PropertyName,
    /// مصدر البيانات
    pub source_path: BindingPath,
    /// الوضع
    pub mode: BindingMode,
    /// التحويل
    pub converter: Option<Arc<dyn Fn(&UIValue) -> UIValue + Send + Sync>>,
    /// القيمة الأخيرة
    pub last_value: Option<UIValue>,
}

impl Binding {
    pub fn new(
        target_component: &str,
        target_property: &str,
        source_path: &str,
        mode: BindingMode,
    ) -> Self {
        Self {
            id: format!("binding_{}", uuid()),
            target_component: target_component.to_string(),
            target_property: target_property.to_string(),
            source_path: source_path.to_string(),
            mode,
            converter: None,
            last_value: None,
        }
    }
    
    pub fn converter<F>(mut self, converter: F) -> Self
    where
        F: Fn(&UIValue) -> UIValue + Send + Sync + 'static,
    {
        self.converter = Some(Arc::new(converter));
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
/// سياق البيانات
// ═══════════════════════════════════════════════════════════════════════════════

/// سياق البيانات
#[derive(Debug)]
pub struct DataContext {
    /// البيانات
    data: HashMap<String, ObservableValue>,
    /// السياق الأب
    parent: Option<Arc<RwLock<DataContext>>>,
}

impl DataContext {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            parent: None,
        }
    }
    
    pub fn with_parent(parent: Arc<RwLock<DataContext>>) -> Self {
        Self {
            data: HashMap::new(),
            parent: Some(parent),
        }
    }
    
    /// تعيين قيمة
    pub fn set(&mut self, key: &str, value: UIValue) {
        if let Some(obs) = self.data.get_mut(key) {
            obs.set(value);
        } else {
            self.data.insert(key.to_string(), ObservableValue::new(value));
        }
    }
    
    /// الحصول على قيمة
    pub fn get(&self, key: &str) -> Option<UIValue> {
        if let Some(obs) = self.data.get(key) {
            Some(obs.get())
        } else if let Some(ref parent) = self.parent {
            if let Ok(parent_lock) = parent.read() {
                parent_lock.get(key)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// الحصول على قيمة بمسار
    pub fn get_path(&self, path: &str) -> Option<UIValue> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return None;
        }
        
        let mut current = self.get(parts[0])?;
        
        for part in &parts[1..] {
            match current {
                UIValue::Map(map) => {
                    current = map.get(*part).cloned()?;
                }
                _ => return None,
            }
        }
        
        Some(current)
    }
    
    /// تعيين قيمة بمسار
    pub fn set_path(&mut self, path: &str, value: UIValue) {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return;
        }
        
        if parts.len() == 1 {
            self.set(parts[0], value);
        } else {
            // التعامل مع المسارات المتداخلة
            // TODO: تنفيذ كامل
        }
    }
    
    /// الاشتراك في التغييرات
    pub fn subscribe(&mut self, key: &str, callback: WatcherCallback) {
        if let Some(obs) = self.data.get_mut(key) {
            obs.subscribe(callback);
        }
    }
}

impl Default for DataContext {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
/// نتيجة الربط
// ═══════════════════════════════════════════════════════════════════════════════

/// نتيجة الربط
#[derive(Debug, Clone, PartialEq)]
pub enum BindingResult {
    Success(UIValue),
    Error(String),
    NotFound,
}

// ═══════════════════════════════════════════════════════════════════════════════
// محرك الربط
// ═══════════════════════════════════════════════════════════════════════════════

/// محرك ربط البيانات
#[derive(Debug)]
pub struct BindingEngine {
    /// الارتباطات
    bindings: HashMap<String, Binding>,
    /// سياقات البيانات
    contexts: HashMap<String, Arc<RwLock<DataContext>>>,
    /// المراقبين
    watchers: HashMap<String, Watcher>,
}

impl BindingEngine {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            contexts: HashMap::new(),
            watchers: HashMap::new(),
        }
    }
    
    /// إضافة ربط
    pub fn add_binding(&mut self, binding: Binding) {
        self.bindings.insert(binding.id.clone(), binding);
    }
    
    /// إزالة ربط
    pub fn remove_binding(&mut self, id: &str) {
        self.bindings.remove(id);
    }
    
    /// إضافة سياق بيانات
    pub fn add_context(&mut self, component_id: &str, context: DataContext) {
        self.contexts.insert(
            component_id.to_string(),
            Arc::new(RwLock::new(context)),
        );
    }
    
    /// الحصول على سياق البيانات
    pub fn get_context(&self, component_id: &str) -> Option<Arc<RwLock<DataContext>>> {
        self.contexts.get(component_id).cloned()
    }
    
    /// تحديث قيمة
    pub fn update_value(&mut self, path: &str, value: UIValue) -> BindingResult {
        // تحديث جميع الارتباطات المتأثرة
        for binding in self.bindings.values_mut() {
            if binding.source_path == path {
                binding.last_value = Some(value.clone());
            }
        }
        BindingResult::Success(value)
    }
    
    /// الحصول على قيمة الربط
    pub fn get_binding_value(&self, binding_id: &str) -> BindingResult {
        if let Some(binding) = self.bindings.get(binding_id) {
            if let Some(ref value) = binding.last_value {
                let final_value = if let Some(ref converter) = binding.converter {
                    converter(value)
                } else {
                    value.clone()
                };
                BindingResult::Success(final_value)
            } else {
                BindingResult::NotFound
            }
        } else {
            BindingResult::NotFound
        }
    }
    
    /// تحديث المحرك
    pub fn update(&mut self) -> Result<(), String> {
        // تحديث جميع الارتباطات
        Ok(())
    }
    
    /// تطبيق الربط
    pub fn apply(&mut self, binding_id: &str) -> BindingResult {
        if let Some(binding) = self.bindings.get(binding_id) {
            let path = binding.source_path.clone();
            let parts: Vec<&str> = path.split('.').collect();
            
            if let Some(context) = self.contexts.get(&binding.target_component) {
                if let Ok(ctx) = context.read() {
                    if let Some(value) = ctx.get_path(&binding.source_path) {
                        return BindingResult::Success(value);
                    }
                }
            }
        }
        BindingResult::NotFound
    }
}

impl Default for BindingEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الدوال المساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء ربط أحادي
pub fn bind(source: &str, target: &str) -> BindingExpression {
    BindingExpression::new(source, target)
}

/// إنشاء ربط ثنائي
pub fn bind_two_way(source: &str, target: &str) -> BindingExpression {
    BindingExpression::new(source, target).two_way()
}

/// إنشاء ربط مرة واحدة
pub fn bind_one_time(source: &str, target: &str) -> BindingExpression {
    BindingExpression::new(source, target).one_time()
}

/// إنشاء قيمة قابلة للملاحظة
pub fn observe(value: UIValue) -> ObservableValue {
    ObservableValue::new(value)
}

/// إنشاء قيمة محسوبة
pub fn computed<F>(name: &str, dependencies: Vec<String>, computer: F) -> Computed
where
    F: Fn() -> UIValue + Send + Sync + 'static,
{
    Computed::new(name, "computed", dependencies)
}

/// إنشاء معرف فريد
fn uuid() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
