// ═══════════════════════════════════════════════════════════════════════════════
// المكونات القابلة لإعادة الاستخدام - Reusable Components
// ═══════════════════════════════════════════════════════════════════════════════

use crate::ui::types::*;
use crate::ui::layout::*;
use crate::ui::styling::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// ═══════════════════════════════════════════════════════════════════════════════
// الخصائص والحالة
// ═══════════════════════════════════════════════════════════════════════════════

/// خصائص المكون
pub trait ComponentProps: Clone + Send + Sync {
    /// إنشاء خصائص افتراضية
    fn default() -> Self;
}

/// خصائص عامة
#[derive(Debug, Clone, Default)]
pub struct Props {
    /// الخصائص
    pub values: HashMap<String, UIValue>,
}

impl Props {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    
    pub fn set(mut self, key: &str, value: UIValue) -> Self {
        self.values.insert(key.to_string(), value);
        self
    }
    
    pub fn get(&self, key: &str) -> Option<&UIValue> {
        self.values.get(key)
    }
}

/// حالة المكون
#[derive(Debug, Clone)]
pub struct ComponentState {
    /// الحالة
    values: HashMap<String, UIValue>,
}

impl ComponentState {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, key: &str, value: UIValue) {
        self.values.insert(key.to_string(), value);
    }
    
    pub fn get(&self, key: &str) -> Option<&UIValue> {
        self.values.get(key)
    }
}

impl Default for ComponentState {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دورة حياة المكون
// ═══════════════════════════════════════════════════════════════════════════════

/// دورة حياة المكون
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComponentLifecycle {
    Created,
    Mounted,
    Updated,
    Unmounted,
}

// ═══════════════════════════════════════════════════════════════════════════════
// المكون الأساسي
// ═══════════════════════════════════════════════════════════════════════════════

/// المكون الأساسي
pub trait Component: Send + Sync {
    /// الحصول على المعرف
    fn id(&self) -> &str;
    
    /// الحصول على النوع
    fn component_type(&self) -> &str;
    
    /// تصيير المكون
    fn render(&self) -> Result<String, String>;
    
    /// معالجة الحدث
    fn handle_event(&mut self, event: &UIEvent) -> bool {
        let _ = event;
        false
    }
    
    /// الحصول على الحالة
    fn state(&self) -> UIState;
    
    /// تحديث المكون
    fn update(&mut self) -> Result<(), String> {
        Ok(())
    }
}

/// قاعدة المكون
#[derive(Debug, Clone)]
pub struct ComponentBase {
    /// المعرف
    pub id: String,
    /// النوع
    pub component_type: String,
    /// الخصائص
    pub props: Props,
    /// الحالة
    pub state: ComponentState,
    /// حالة UI
    pub ui_state: UIState,
    /// التنسيق
    pub style: Style,
    /// التخطيط
    pub layout: Option<Layout>,
    /// الأطفال
    pub children: Vec<Arc<RwLock<Box<dyn Component>>>>,
    /// دورة الحياة
    pub lifecycle: ComponentLifecycle,
}

impl ComponentBase {
    pub fn new(component_type: &str) -> Self {
        Self {
            id: format!("{}_{}", component_type, uuid()),
            component_type: component_type.to_string(),
            props: Props::new(),
            state: ComponentState::new(),
            ui_state: UIState::Idle,
            style: Style::new(),
            layout: None,
            children: Vec::new(),
            lifecycle: ComponentLifecycle::Created,
        }
    }
    
    pub fn with_id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }
    
    pub fn prop(mut self, key: &str, value: UIValue) -> Self {
        self.props.values.insert(key.to_string(), value);
        self
    }
    
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = Some(layout);
        self
    }
    
    pub fn child(mut self, child: Box<dyn Component>) -> Self {
        self.children.push(Arc::new(RwLock::new(child)));
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// بناء المكونات
// ═══════════════════════════════════════════════════════════════════════════════

/// بناء المكونات
pub struct ComponentBuilder {
    base: ComponentBase,
}

impl ComponentBuilder {
    pub fn new(component_type: &str) -> Self {
        Self {
            base: ComponentBase::new(component_type),
        }
    }
    
    pub fn id(mut self, id: &str) -> Self {
        self.base.id = id.to_string();
        self
    }
    
    pub fn prop(mut self, key: &str, value: UIValue) -> Self {
        self.base.props.values.insert(key.to_string(), value);
        self
    }
    
    pub fn style(mut self, style: Style) -> Self {
        self.base.style = style;
        self
    }
    
    pub fn layout(mut self, layout: Layout) -> Self {
        self.base.layout = Some(layout);
        self
    }
    
    pub fn child(mut self, child: Box<dyn Component>) -> Self {
        self.base.children.push(Arc::new(RwLock::new(child)));
        self
    }
    
    pub fn build(self) -> ComponentBase {
        self.base
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// سجل المكونات
// ═══════════════════════════════════════════════════════════════════════════════

/// منشئ المكون
pub type ComponentFactory = Box<dyn Fn() -> Box<dyn Component> + Send + Sync>;

/// سجل المكونات
pub struct ComponentRegistry {
    /// المكونات المسجلة
    components: HashMap<String, ComponentFactory>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            components: HashMap::new(),
        };
        
        // تسجيل المكونات الافتراضية
        registry.register_defaults();
        
        registry
    }
    
    /// تسجيل المكونات الافتراضية
    fn register_defaults(&mut self) {
        // يتم تسجيل المكونات الأساسية هنا
    }
    
    /// تسجيل مكون
    pub fn register<F>(&mut self, name: &str, factory: F)
    where
        F: Fn() -> Box<dyn Component> + Send + Sync + 'static,
    {
        self.components.insert(name.to_string(), Box::new(factory));
    }
    
    /// إنشاء مكون
    pub fn create(&self, name: &str) -> Option<Box<dyn Component>> {
        self.components.get(name).map(|f| f())
    }
    
    /// التحقق من وجود مكون
    pub fn exists(&self, name: &str) -> bool {
        self.components.contains_key(name)
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// المكونات الأساسية
// ═══════════════════════════════════════════════════════════════════════════════

/// نص
#[derive(Debug, Clone)]
pub struct Text {
    pub base: ComponentBase,
    pub content: String,
}

impl Text {
    pub fn new(content: &str) -> Self {
        Self {
            base: ComponentBase::new("text"),
            content: content.to_string(),
        }
    }
}

impl Component for Text {
    fn id(&self) -> &str {
        &self.base.id
    }
    
    fn component_type(&self) -> &str {
        &self.base.component_type
    }
    
    fn render(&self) -> Result<String, String> {
        Ok(format!("<span id=\"{}\" style=\"{}\">{}</span>",
            self.base.id,
            self.base.style.to_css(),
            self.content
        ))
    }
    
    fn state(&self) -> UIState {
        self.base.ui_state
    }
}

/// زر
#[derive(Debug, Clone)]
pub struct Button {
    pub base: ComponentBase,
    pub label: String,
    pub disabled: bool,
    pub on_click: Option<String>,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Self {
            base: ComponentBase::new("button"),
            label: label.to_string(),
            disabled: false,
            on_click: None,
        }
    }
    
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    
    pub fn on_click(mut self, handler: &str) -> Self {
        self.on_click = Some(handler.to_string());
        self
    }
}

impl Component for Button {
    fn id(&self) -> &str {
        &self.base.id
    }
    
    fn component_type(&self) -> &str {
        &self.base.component_type
    }
    
    fn render(&self) -> Result<String, String> {
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        let onclick_attr = if let Some(ref handler) = self.on_click {
            format!(" onclick=\"{}\"", handler)
        } else {
            String::new()
        };
        
        Ok(format!(
            "<button id=\"{}\" style=\"{}\"{}{}>{}</button>",
            self.base.id,
            self.base.style.to_css(),
            disabled_attr,
            onclick_attr,
            self.label
        ))
    }
    
    fn handle_event(&mut self, event: &UIEvent) -> bool {
        matches!(event.kind, UIEventKind::Click)
    }
    
    fn state(&self) -> UIState {
        if self.disabled {
            UIState::Disabled
        } else {
            self.base.ui_state
        }
    }
}

/// حقل نصي
#[derive(Debug, Clone)]
pub struct TextField {
    pub base: ComponentBase,
    pub value: String,
    pub placeholder: String,
    pub disabled: bool,
    pub on_change: Option<String>,
}

impl TextField {
    pub fn new() -> Self {
        Self {
            base: ComponentBase::new("textfield"),
            value: String::new(),
            placeholder: String::new(),
            disabled: false,
            on_change: None,
        }
    }
    
    pub fn value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }
    
    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }
}

impl Default for TextField {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for TextField {
    fn id(&self) -> &str {
        &self.base.id
    }
    
    fn component_type(&self) -> &str {
        &self.base.component_type
    }
    
    fn render(&self) -> Result<String, String> {
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        Ok(format!(
            "<input type=\"text\" id=\"{}\" style=\"{}\" value=\"{}\" placeholder=\"{}\"{}>",
            self.base.id,
            self.base.style.to_css(),
            self.value,
            self.placeholder,
            disabled_attr
        ))
    }
    
    fn handle_event(&mut self, event: &UIEvent) -> bool {
        if let UIEventKind::Change = event.kind {
            if let Some(ref value) = event.value {
                if let Some(s) = value.as_string() {
                    self.value = s.to_string();
                    return true;
                }
            }
        }
        false
    }
    
    fn state(&self) -> UIState {
        if self.disabled {
            UIState::Disabled
        } else {
            self.base.ui_state
        }
    }
}

/// منطقة نص
#[derive(Debug, Clone)]
pub struct TextArea {
    pub base: ComponentBase,
    pub value: String,
    pub placeholder: String,
    pub rows: u32,
}

impl TextArea {
    pub fn new() -> Self {
        Self {
            base: ComponentBase::new("textarea"),
            value: String::new(),
            placeholder: String::new(),
            rows: 4,
        }
    }
}

impl Default for TextArea {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for TextArea {
    fn id(&self) -> &str {
        &self.base.id
    }
    
    fn component_type(&self) -> &str {
        &self.base.component_type
    }
    
    fn render(&self) -> Result<String, String> {
        Ok(format!(
            "<textarea id=\"{}\" style=\"{}\" rows=\"{}\" placeholder=\"{}\">{}</textarea>",
            self.base.id,
            self.base.style.to_css(),
            self.rows,
            self.placeholder,
            self.value
        ))
    }
    
    fn state(&self) -> UIState {
        self.base.ui_state
    }
}

/// خانة اختيار
#[derive(Debug, Clone)]
pub struct Checkbox {
    pub base: ComponentBase,
    pub checked: bool,
    pub label: String,
}

impl Checkbox {
    pub fn new(label: &str) -> Self {
        Self {
            base: ComponentBase::new("checkbox"),
            checked: false,
            label: label.to_string(),
        }
    }
    
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
}

impl Component for Checkbox {
    fn id(&self) -> &str {
        &self.base.id
    }
    
    fn component_type(&self) -> &str {
        &self.base.component_type
    }
    
    fn render(&self) -> Result<String, String> {
        let checked_attr = if self.checked { " checked" } else { "" };
        Ok(format!(
            "<label><input type=\"checkbox\" id=\"{}\" style=\"{}\"{}> {}</label>",
            self.base.id,
            self.base.style.to_css(),
            checked_attr,
            self.label
        ))
    }
    
    fn state(&self) -> UIState {
        self.base.ui_state
    }
}

/// زر راديو
#[derive(Debug, Clone)]
pub struct Radio {
    pub base: ComponentBase,
    pub checked: bool,
    pub label: String,
    pub group: String,
}

impl Radio {
    pub fn new(label: &str, group: &str) -> Self {
        Self {
            base: ComponentBase::new("radio"),
            checked: false,
            label: label.to_string(),
            group: group.to_string(),
        }
    }
}

impl Component for Radio {
    fn id(&self) -> &str {
        &self.base.id
    }
    
    fn component_type(&self) -> &str {
        &self.base.component_type
    }
    
    fn render(&self) -> Result<String, String> {
        let checked_attr = if self.checked { " checked" } else { "" };
        Ok(format!(
            "<label><input type=\"radio\" id=\"{}\" name=\"{}\" style=\"{}\"{}> {}</label>",
            self.base.id,
            self.group,
            self.base.style.to_css(),
            checked_attr,
            self.label
        ))
    }
    
    fn state(&self) -> UIState {
        self.base.ui_state
    }
}

/// خيار القائمة
#[derive(Debug, Clone)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub selected: bool,
}

impl SelectOption {
    pub fn new(value: &str, label: &str) -> Self {
        Self {
            value: value.to_string(),
            label: label.to_string(),
            selected: false,
        }
    }
}

/// قائمة اختيار
#[derive(Debug, Clone)]
pub struct Select {
    pub base: ComponentBase,
    pub options: Vec<SelectOption>,
    pub selected_index: Option<usize>,
}

impl Select {
    pub fn new() -> Self {
        Self {
            base: ComponentBase::new("select"),
            options: Vec::new(),
            selected_index: None,
        }
    }
    
    pub fn option(mut self, value: &str, label: &str) -> Self {
        self.options.push(SelectOption::new(value, label));
        self
    }
}

impl Default for Select {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Select {
    fn id(&self) -> &str {
        &self.base.id
    }
    
    fn component_type(&self) -> &str {
        &self.base.component_type
    }
    
    fn render(&self) -> Result<String, String> {
        let options: String = self.options.iter()
            .map(|opt| {
                let selected = if opt.selected { " selected" } else { "" };
                format!("<option value=\"{}\"{}>{}</option>", opt.value, selected, opt.label)
            })
            .collect();
        
        Ok(format!(
            "<select id=\"{}\" style=\"{}\">{}</select>",
            self.base.id,
            self.base.style.to_css(),
            options
        ))
    }
    
    fn state(&self) -> UIState {
        self.base.ui_state
    }
}

/// منزلق
#[derive(Debug, Clone)]
pub struct Slider {
    pub base: ComponentBase,
    pub min: f64,
    pub max: f64,
    pub value: f64,
    pub step: f64,
}

impl Slider {
    pub fn new() -> Self {
        Self {
            base: ComponentBase::new("slider"),
            min: 0.0,
            max: 100.0,
            value: 50.0,
            step: 1.0,
        }
    }
    
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }
    
    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Slider {
    fn id(&self) -> &str {
        &self.base.id
    }
    
    fn component_type(&self) -> &str {
        &self.base.component_type
    }
    
    fn render(&self) -> Result<String, String> {
        Ok(format!(
            "<input type=\"range\" id=\"{}\" style=\"{}\" min=\"{}\" max=\"{}\" value=\"{}\" step=\"{}\">",
            self.base.id,
            self.base.style.to_css(),
            self.min,
            self.max,
            self.value,
            self.step
        ))
    }
    
    fn state(&self) -> UIState {
        self.base.ui_state
    }
}

/// شريط التقدم
#[derive(Debug, Clone)]
pub struct ProgressBar {
    pub base: ComponentBase,
    pub value: f64,
    pub max: f64,
}

impl ProgressBar {
    pub fn new() -> Self {
        Self {
            base: ComponentBase::new("progressbar"),
            value: 0.0,
            max: 100.0,
        }
    }
    
    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for ProgressBar {
    fn id(&self) -> &str {
        &self.base.id
    }
    
    fn component_type(&self) -> &str {
        &self.base.component_type
    }
    
    fn render(&self) -> Result<String, String> {
        let percent = (self.value / self.max * 100.0).min(100.0);
        Ok(format!(
            "<div id=\"{}\" style=\"{}\"><div style=\"width: {}%\"></div></div>",
            self.base.id,
            self.base.style.to_css(),
            percent
        ))
    }
    
    fn state(&self) -> UIState {
        self.base.ui_state
    }
}

/// مؤشر التحميل
#[derive(Debug, Clone)]
pub struct Spinner {
    pub base: ComponentBase,
    pub size: f32,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            base: ComponentBase::new("spinner"),
            size: 24.0,
        }
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Spinner {
    fn id(&self) -> &str {
        &self.base.id
    }
    
    fn component_type(&self) -> &str {
        &self.base.component_type
    }
    
    fn render(&self) -> Result<String, String> {
        Ok(format!(
            "<div id=\"{}\" class=\"spinner\" style=\"width: {}px; height: {}px; {}\"></div>",
            self.base.id,
            self.size,
            self.size,
            self.base.style.to_css()
        ))
    }
    
    fn state(&self) -> UIState {
        UIState::Loading
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// المكونات الهيكلية
// ═══════════════════════════════════════════════════════════════════════════════

/// حاوية
#[derive(Debug, Clone)]
pub struct Container {
    pub base: ComponentBase,
}

impl Container {
    pub fn new() -> Self {
        Self {
            base: ComponentBase::new("container"),
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Container {
    fn id(&self) -> &str {
        &self.base.id
    }
    
    fn component_type(&self) -> &str {
        &self.base.component_type
    }
    
    fn render(&self) -> Result<String, String> {
        let children: String = self.base.children.iter()
            .filter_map(|c| c.read().ok()?.render().ok())
            .collect();
        
        Ok(format!(
            "<div id=\"{}\" style=\"{}\">{}</div>",
            self.base.id,
            self.base.style.to_css(),
            children
        ))
    }
    
    fn state(&self) -> UIState {
        self.base.ui_state
    }
}

/// بطاقة
#[derive(Debug, Clone)]
pub struct Card {
    pub base: ComponentBase,
    pub title: String,
}

impl Card {
    pub fn new(title: &str) -> Self {
        Self {
            base: ComponentBase::new("card"),
            title: title.to_string(),
        }
    }
}

impl Component for Card {
    fn id(&self) -> &str {
        &self.base.id
    }
    
    fn component_type(&self) -> &str {
        &self.base.component_type
    }
    
    fn render(&self) -> Result<String, String> {
        let children: String = self.base.children.iter()
            .filter_map(|c| c.read().ok()?.render().ok())
            .collect();
        
        Ok(format!(
            "<div id=\"{}\" class=\"card\" style=\"{}\"><h3>{}</h3><div class=\"card-content\">{}</div></div>",
            self.base.id,
            self.base.style.to_css(),
            self.title,
            children
        ))
    }
    
    fn state(&self) -> UIState {
        self.base.ui_state
    }
}

/// تسمية
#[derive(Debug, Clone)]
pub struct Label {
    pub base: ComponentBase,
    pub text: String,
    pub for_id: Option<String>,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self {
            base: ComponentBase::new("label"),
            text: text.to_string(),
            for_id: None,
        }
    }
}

impl Component for Label {
    fn id(&self) -> &str {
        &self.base.id
    }
    
    fn component_type(&self) -> &str {
        &self.base.component_type
    }
    
    fn render(&self) -> Result<String, String> {
        let for_attr = if let Some(ref for_id) = self.for_id {
            format!(" for=\"{}\"", for_id)
        } else {
            String::new()
        };
        
        Ok(format!(
            "<label id=\"{}\" style=\"{}\"{}>{}</label>",
            self.base.id,
            self.base.style.to_css(),
            for_attr,
            self.text
        ))
    }
    
    fn state(&self) -> UIState {
        self.base.ui_state
    }
}

/// فاصل
#[derive(Debug, Clone)]
pub struct Divider {
    pub base: ComponentBase,
    pub vertical: bool,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            base: ComponentBase::new("divider"),
            vertical: false,
        }
    }
    
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Divider {
    fn id(&self) -> &str {
        &self.base.id
    }
    
    fn component_type(&self) -> &str {
        &self.base.component_type
    }
    
    fn render(&self) -> Result<String, String> {
        let border_style = if self.vertical {
            "border-right: 1px solid #ccc"
        } else {
            "border-top: 1px solid #ccc"
        };
        
        Ok(format!(
            "<div id=\"{}\" style=\"{}; {}\"></div>",
            self.base.id,
            self.base.style.to_css(),
            border_style
        ))
    }
    
    fn state(&self) -> UIState {
        self.base.ui_state
    }
}

/// مساحة فارغة
#[derive(Debug, Clone)]
pub struct Spacer {
    pub base: ComponentBase,
    pub flex: f32,
}

impl Spacer {
    pub fn new() -> Self {
        Self {
            base: ComponentBase::new("spacer"),
            flex: 1.0,
        }
    }
    
    pub fn flex(mut self, flex: f32) -> Self {
        self.flex = flex;
        self
    }
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Spacer {
    fn id(&self) -> &str {
        &self.base.id
    }
    
    fn component_type(&self) -> &str {
        &self.base.component_type
    }
    
    fn render(&self) -> Result<String, String> {
        Ok(format!(
            "<div id=\"{}\" style=\"flex: {}\"></div>",
            self.base.id,
            self.flex
        ))
    }
    
    fn state(&self) -> UIState {
        self.base.ui_state
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// المكونات المتبقية (stub implementations for compilation)
// ═══════════════════════════════════════════════════════════════════════════════

pub struct Image { pub base: ComponentBase, pub src: String, pub alt: String }
pub struct Icon { pub base: ComponentBase, pub name: String }
pub struct List { pub base: ComponentBase, pub items: Vec<String> }
pub struct ListItem { pub base: ComponentBase, pub content: String }
pub struct Table { pub base: ComponentBase }
pub struct TableRow { pub base: ComponentBase }
pub struct TableCell { pub base: ComponentBase, pub content: String }
pub struct Form { pub base: ComponentBase }
pub struct FormField { pub base: ComponentBase, pub label: String }
pub struct Badge { pub base: ComponentBase, pub text: String }
pub struct Avatar { pub base: ComponentBase, pub src: String }
pub struct Tooltip { pub base: ComponentBase, pub text: String }
pub struct Popover { pub base: ComponentBase, pub content: String }
pub struct Tabs { pub base: ComponentBase }
pub struct Tab { pub base: ComponentBase, pub label: String }
pub struct Accordion { pub base: ComponentBase }
pub struct AccordionItem { pub base: ComponentBase, pub title: String }
pub struct Breadcrumb { pub base: ComponentBase, pub items: Vec<String> }
pub struct Pagination { pub base: ComponentBase, pub total: u32, pub current: u32 }
pub struct Stepper { pub base: ComponentBase, pub steps: Vec<String>, pub current: u32 }
pub struct Calendar { pub base: ComponentBase }
pub struct DatePicker { pub base: ComponentBase }
pub struct TimePicker { pub base: ComponentBase }

// ═══════════════════════════════════════════════════════════════════════════════
// الدوال المساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء مكون
pub fn create_component(name: &str) -> Option<Box<dyn Component>> {
    match name {
        "text" => Some(Box::new(Text::new(""))),
        "button" => Some(Box::new(Button::new(""))),
        "textfield" => Some(Box::new(TextField::new())),
        "container" => Some(Box::new(Container::new())),
        _ => None,
    }
}

/// تسجيل مكون
pub fn register_component(registry: &mut ComponentRegistry, name: &str) {
    // تسجيل المكون في السجل
    let _ = registry;
    let _ = name;
}

/// التحقق من وجود مكون
pub fn component_exists(registry: &ComponentRegistry, name: &str) -> bool {
    registry.exists(name)
}

/// إنشاء معرف فريد
fn uuid() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
