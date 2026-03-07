// ═══════════════════════════════════════════════════════════════════════════════
// نظام التنسيق - Styling System
// ═══════════════════════════════════════════════════════════════════════════════

use crate::ui::types::*;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
// خصائص التنسيق
// ═══════════════════════════════════════════════════════════════════════════════

/// قيمة التنسيق
#[derive(Debug, Clone, PartialEq)]
pub enum StyleValue {
    None,
    Auto,
    Inherit,
    Initial,
    Unset,
    String(String),
    Number(f32),
    Color(UIColor),
    Size(UnitValue),
    List(Vec<StyleValue>),
}

impl StyleValue {
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        match self {
            Self::None => "none".to_string(),
            Self::Auto => "auto".to_string(),
            Self::Inherit => "inherit".to_string(),
            Self::Initial => "initial".to_string(),
            Self::Unset => "unset".to_string(),
            Self::String(s) => s.clone(),
            Self::Number(n) => format!("{}px", n),
            Self::Color(c) => c.to_css(),
            Self::Size(s) => s.to_css(),
            Self::List(values) => {
                values.iter()
                    .map(|v| v.to_css())
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        }
    }
}

/// خاصية التنسيق
#[derive(Debug, Clone, PartialEq)]
pub struct StyleProperty {
    /// الاسم
    pub name: String,
    /// القيمة
    pub value: StyleValue,
    /// مهم
    pub important: bool,
}

impl StyleProperty {
    pub fn new(name: &str, value: StyleValue) -> Self {
        Self {
            name: name.to_string(),
            value,
            important: false,
        }
    }
    
    pub fn important(mut self) -> Self {
        self.important = true;
        self
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        let important = if self.important { " !important" } else { "" };
        format!("{}: {}{}", self.name, self.value.to_css(), important)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الحدود
// ═══════════════════════════════════════════════════════════════════════════════

/// نمط الحدود
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderStyle {
    None,
    Solid,
    Dashed,
    Dotted,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
    Hidden,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self::Solid
    }
}

impl BorderStyle {
    pub fn to_css(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Solid => "solid",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
            Self::Double => "double",
            Self::Groove => "groove",
            Self::Ridge => "ridge",
            Self::Inset => "inset",
            Self::Outset => "outset",
            Self::Hidden => "hidden",
        }
    }
}

/// الحدود
#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    pub width: f32,
    pub style: BorderStyle,
    pub color: UIColor,
    pub radius: BorderRadius,
}

impl Border {
    pub fn new() -> Self {
        Self {
            width: 1.0,
            style: BorderStyle::Solid,
            color: UIColor::gray(),
            radius: BorderRadius::new(),
        }
    }
    
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
    
    pub fn style(mut self, style: BorderStyle) -> Self {
        self.style = style;
        self
    }
    
    pub fn color(mut self, color: UIColor) -> Self {
        self.color = color;
        self
    }
    
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = BorderRadius::all(radius);
        self
    }
    
    pub fn to_css(&self) -> String {
        format!(
            "{}px {} {}; {}",
            self.width,
            self.style.to_css(),
            self.color.to_css(),
            self.radius.to_css()
        )
    }
}

impl Default for Border {
    fn default() -> Self {
        Self::new()
    }
}

/// نصف قطر الزوايا
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl BorderRadius {
    pub fn new() -> Self {
        Self {
            top_left: 0.0,
            top_right: 0.0,
            bottom_right: 0.0,
            bottom_left: 0.0,
        }
    }
    
    pub fn all(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }
    
    pub fn horizontal_vertical(horizontal: f32, vertical: f32) -> Self {
        Self {
            top_left: horizontal,
            top_right: horizontal,
            bottom_right: vertical,
            bottom_left: vertical,
        }
    }
    
    pub fn to_css(&self) -> String {
        if self.top_left == self.top_right
            && self.top_right == self.bottom_right
            && self.bottom_right == self.bottom_left
        {
            format!("border-radius: {}px;", self.top_left)
        } else {
            format!(
                "border-radius: {}px {}px {}px {}px;",
                self.top_left, self.top_right, self.bottom_right, self.bottom_left
            )
        }
    }
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الظل
// ═══════════════════════════════════════════════════════════════════════════════

/// الظل
#[derive(Debug, Clone, PartialEq)]
pub struct Shadow {
    pub x: f32,
    pub y: f32,
    pub blur: f32,
    pub spread: f32,
    pub color: UIColor,
    pub inset: bool,
}

impl Shadow {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 4.0,
            blur: 8.0,
            spread: 0.0,
            color: UIColor::black().with_alpha(0.1),
            inset: false,
        }
    }
    
    pub fn offset(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }
    
    pub fn blur(mut self, blur: f32) -> Self {
        self.blur = blur;
        self
    }
    
    pub fn spread(mut self, spread: f32) -> Self {
        self.spread = spread;
        self
    }
    
    pub fn color(mut self, color: UIColor) -> Self {
        self.color = color;
        self
    }
    
    pub fn inset(mut self) -> Self {
        self.inset = true;
        self
    }
    
    pub fn to_css(&self) -> String {
        let inset = if self.inset { "inset " } else { "" };
        format!(
            "{}{}px {}px {}px {}px {}",
            inset, self.x, self.y, self.blur, self.spread, self.color.to_css()
        )
    }
}

impl Default for Shadow {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// التدرج
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع التدرج
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GradientKind {
    Linear,
    Radial,
    Conic,
}

impl Default for GradientKind {
    fn default() -> Self {
        Self::Linear
    }
}

/// نقطة التدرج
#[derive(Debug, Clone, PartialEq)]
pub struct GradientStop {
    pub color: UIColor,
    pub position: f32,
}

impl GradientStop {
    pub fn new(color: UIColor, position: f32) -> Self {
        Self { color, position }
    }
}

/// التدرج
#[derive(Debug, Clone, PartialEq)]
pub struct Gradient {
    pub kind: GradientKind,
    pub angle: f32,
    pub stops: Vec<GradientStop>,
}

impl Gradient {
    pub fn linear() -> Self {
        Self {
            kind: GradientKind::Linear,
            angle: 0.0,
            stops: Vec::new(),
        }
    }
    
    pub fn radial() -> Self {
        Self {
            kind: GradientKind::Radial,
            angle: 0.0,
            stops: Vec::new(),
        }
    }
    
    pub fn angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }
    
    pub fn stop(mut self, color: UIColor, position: f32) -> Self {
        self.stops.push(GradientStop::new(color, position));
        self
    }
    
    pub fn to_css(&self) -> String {
        let stops: String = self.stops.iter()
            .map(|s| format!("{} {}%", s.color.to_css(), s.position * 100.0))
            .collect::<Vec<_>>()
            .join(", ");
        
        match self.kind {
            GradientKind::Linear => {
                format!("linear-gradient({}deg, {})", self.angle, stops)
            }
            GradientKind::Radial => {
                format!("radial-gradient({})", stops)
            }
            GradientKind::Conic => {
                format!("conic-gradient(from {}deg, {})", self.angle, stops)
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
/// التحويل
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع التحويل
#[derive(Debug, Clone, PartialEq)]
pub enum TransformKind {
    Translate(f32, f32),
    Rotate(f32),
    Scale(f32, f32),
    Skew(f32, f32),
    Matrix(f32, f32, f32, f32, f32, f32),
}

impl TransformKind {
    pub fn to_css(&self) -> String {
        match self {
            Self::Translate(x, y) => format!("translate({}px, {}px)", x, y),
            Self::Rotate(deg) => format!("rotate({}deg)", deg),
            Self::Scale(x, y) => format!("scale({}, {})", x, y),
            Self::Skew(x, y) => format!("skew({}deg, {}deg)", x, y),
            Self::Matrix(a, b, c, d, e, f) => {
                format!("matrix({}, {}, {}, {}, {}, {})", a, b, c, d, e, f)
            }
        }
    }
}

/// التحويل
#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub transforms: Vec<TransformKind>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            transforms: Vec::new(),
        }
    }
    
    pub fn translate(mut self, x: f32, y: f32) -> Self {
        self.transforms.push(TransformKind::Translate(x, y));
        self
    }
    
    pub fn rotate(mut self, deg: f32) -> Self {
        self.transforms.push(TransformKind::Rotate(deg));
        self
    }
    
    pub fn scale(mut self, x: f32, y: f32) -> Self {
        self.transforms.push(TransformKind::Scale(x, y));
        self
    }
    
    pub fn to_css(&self) -> String {
        let transforms: String = self.transforms.iter()
            .map(|t| t.to_css())
            .collect::<Vec<_>>()
            .join(" ");
        
        if transforms.is_empty() {
            "transform: none;".to_string()
        } else {
            format!("transform: {};", transforms)
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
/// المرشح
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع المرشح
#[derive(Debug, Clone, PartialEq)]
pub enum FilterKind {
    Blur(f32),
    Brightness(f32),
    Contrast(f32),
    Grayscale(f32),
    HueRotate(f32),
    Invert(f32),
    Opacity(f32),
    Saturate(f32),
    Sepia(f32),
    DropShadow(Shadow),
}

impl FilterKind {
    pub fn to_css(&self) -> String {
        match self {
            Self::Blur(px) => format!("blur({}px)", px),
            Self::Brightness(val) => format!("brightness({}%)", val * 100.0),
            Self::Contrast(val) => format!("contrast({}%)", val * 100.0),
            Self::Grayscale(val) => format!("grayscale({}%)", val * 100.0),
            Self::HueRotate(deg) => format!("hue-rotate({}deg)", deg),
            Self::Invert(val) => format!("invert({}%)", val * 100.0),
            Self::Opacity(val) => format!("opacity({}%)", val * 100.0),
            Self::Saturate(val) => format!("saturate({}%)", val * 100.0),
            Self::Sepia(val) => format!("sepia({}%)", val * 100.0),
            Self::DropShadow(shadow) => format!("drop-shadow({})", shadow.to_css()),
        }
    }
}

/// المرشح
#[derive(Debug, Clone, PartialEq)]
pub struct Filter {
    pub filters: Vec<FilterKind>,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }
    
    pub fn blur(mut self, px: f32) -> Self {
        self.filters.push(FilterKind::Blur(px));
        self
    }
    
    pub fn brightness(mut self, val: f32) -> Self {
        self.filters.push(FilterKind::Brightness(val));
        self
    }
    
    pub fn grayscale(mut self) -> Self {
        self.filters.push(FilterKind::Grayscale(1.0));
        self
    }
    
    pub fn to_css(&self) -> String {
        let filters: String = self.filters.iter()
            .map(|f| f.to_css())
            .collect::<Vec<_>>()
            .join(" ");
        
        if filters.is_empty() {
            "filter: none;".to_string()
        } else {
            format!("filter: {};", filters)
        }
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الخلفية والمقدمة
// ═══════════════════════════════════════════════════════════════════════════════

/// الخلفية
#[derive(Debug, Clone, PartialEq)]
pub struct Background {
    pub color: Option<UIColor>,
    pub image: Option<String>,
    pub gradient: Option<Gradient>,
    pub size: Option<String>,
    pub position: Option<String>,
    pub repeat: Option<String>,
}

impl Background {
    pub fn new() -> Self {
        Self {
            color: None,
            image: None,
            gradient: None,
            size: None,
            position: None,
            repeat: None,
        }
    }
    
    pub fn color(mut self, color: UIColor) -> Self {
        self.color = Some(color);
        self
    }
    
    pub fn gradient(mut self, gradient: Gradient) -> Self {
        self.gradient = Some(gradient);
        self
    }
    
    pub fn image(mut self, url: &str) -> Self {
        self.image = Some(format!("url({})", url));
        self
    }
    
    pub fn to_css(&self) -> String {
        let mut parts = Vec::new();
        
        if let Some(ref color) = self.color {
            parts.push(format!("background-color: {};", color.to_css()));
        }
        
        if let Some(ref image) = self.image {
            parts.push(format!("background-image: {};", image));
        } else if let Some(ref gradient) = self.gradient {
            parts.push(format!("background-image: {};", gradient.to_css()));
        }
        
        parts.join(" ")
    }
}

impl Default for Background {
    fn default() -> Self {
        Self::new()
    }
}

/// المقدمة
pub type Foreground = UIColor;

/// الزخرفة
#[derive(Debug, Clone, Default)]
pub struct Decoration {
    pub text_decoration: Option<String>,
    pub text_shadow: Option<Shadow>,
}

// ═══════════════════════════════════════════════════════════════════════════════
/// التنسيق
// ═══════════════════════════════════════════════════════════════════════════════

/// التنسيق
#[derive(Debug, Clone, Default)]
pub struct Style {
    pub properties: HashMap<String, StyleValue>,
}

impl Style {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }
    
    pub fn set(mut self, name: &str, value: StyleValue) -> Self {
        self.properties.insert(name.to_string(), value);
        self
    }
    
    pub fn color(mut self, color: UIColor) -> Self {
        self.properties.insert("color".to_string(), StyleValue::Color(color));
        self
    }
    
    pub fn background(mut self, color: UIColor) -> Self {
        self.properties.insert("background-color".to_string(), StyleValue::Color(color));
        self
    }
    
    pub fn font_size(mut self, size: f32) -> Self {
        self.properties.insert("font-size".to_string(), StyleValue::String(format!("{}px", size)));
        self
    }
    
    pub fn padding(mut self, padding: UIPadding) -> Self {
        self.properties.insert("padding".to_string(), StyleValue::String(padding.to_css()));
        self
    }
    
    pub fn margin(mut self, margin: UIMargin) -> Self {
        self.properties.insert("margin".to_string(), StyleValue::String(margin.to_css()));
        self
    }
    
    pub fn border(mut self, border: Border) -> Self {
        self.properties.insert("border".to_string(), StyleValue::String(border.to_css()));
        self
    }
    
    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.properties.insert("box-shadow".to_string(), StyleValue::String(shadow.to_css()));
        self
    }
    
    pub fn transform(mut self, transform: Transform) -> Self {
        self.properties.insert("transform".to_string(), StyleValue::String(transform.to_css()));
        self
    }
    
    pub fn filter(mut self, filter: Filter) -> Self {
        self.properties.insert("filter".to_string(), StyleValue::String(filter.to_css()));
        self
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        self.properties.iter()
            .map(|(name, value)| format!("{}: {}", name, value.to_css()))
            .collect::<Vec<_>>()
            .join("; ")
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
/// بناء التنسيق
// ═══════════════════════════════════════════════════════════════════════════════

/// بناء التنسيق
pub struct StyleBuilder {
    style: Style,
}

impl StyleBuilder {
    pub fn new() -> Self {
        Self {
            style: Style::new(),
        }
    }
    
    pub fn set(mut self, name: &str, value: StyleValue) -> Self {
        self.style.properties.insert(name.to_string(), value);
        self
    }
    
    pub fn build(self) -> Style {
        self.style
    }
}

impl Default for StyleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
/// ورقة الأنماط
// ═══════════════════════════════════════════════════════════════════════════════

/// المحدد
#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub pseudo_class: Option<PseudoClass>,
}

impl Selector {
    pub fn new() -> Self {
        Self {
            tag: None,
            id: None,
            classes: Vec::new(),
            pseudo_class: None,
        }
    }
    
    pub fn tag(mut self, tag: &str) -> Self {
        self.tag = Some(tag.to_string());
        self
    }
    
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }
    
    pub fn class(mut self, class: &str) -> Self {
        self.classes.push(class.to_string());
        self
    }
    
    pub fn pseudo(mut self, pseudo: PseudoClass) -> Self {
        self.pseudo_class = Some(pseudo);
        self
    }
    
    pub fn to_css(&self) -> String {
        let mut selector = String::new();
        
        if let Some(ref tag) = self.tag {
            selector.push_str(tag);
        }
        
        if let Some(ref id) = self.id {
            selector.push_str(&format!("#{}", id));
        }
        
        for class in &self.classes {
            selector.push_str(&format!(".{}", class));
        }
        
        if let Some(ref pseudo) = self.pseudo_class {
            selector.push_str(&pseudo.to_css());
        }
        
        selector
    }
}

impl Default for Selector {
    fn default() -> Self {
        Self::new()
    }
}

/// الفئة الزائفة
#[derive(Debug, Clone, PartialEq)]
pub enum PseudoClass {
    Hover,
    Active,
    Focus,
    FocusWithin,
    FirstChild,
    LastChild,
    NthChild(i32),
    Disabled,
    Checked,
    Custom(String),
}

impl PseudoClass {
    pub fn to_css(&self) -> String {
        match self {
            Self::Hover => ":hover".to_string(),
            Self::Active => ":active".to_string(),
            Self::Focus => ":focus".to_string(),
            Self::FocusWithin => ":focus-within".to_string(),
            Self::FirstChild => ":first-child".to_string(),
            Self::LastChild => ":last-child".to_string(),
            Self::NthChild(n) => format!(":nth-child({})", n),
            Self::Disabled => ":disabled".to_string(),
            Self::Checked => ":checked".to_string(),
            Self::Custom(s) => format!(":{}", s),
        }
    }
}

/// قاعدة CSS
#[derive(Debug, Clone, PartialEq)]
pub struct CSSRule {
    pub selector: Selector,
    pub properties: Vec<StyleProperty>,
}

impl CSSRule {
    pub fn new(selector: Selector) -> Self {
        Self {
            selector,
            properties: Vec::new(),
        }
    }
    
    pub fn property(mut self, property: StyleProperty) -> Self {
        self.properties.push(property);
        self
    }
    
    pub fn to_css(&self) -> String {
        let props: String = self.properties.iter()
            .map(|p| format!("  {}", p.to_css()))
            .collect::<Vec<_>>()
            .join(";\n");
        
        format!("{} {{\n{};\n}}", self.selector.to_css(), props)
    }
}

/// ورقة الأنماط
#[derive(Debug, Clone, Default)]
pub struct StyleSheet {
    pub rules: Vec<CSSRule>,
}

impl StyleSheet {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }
    
    pub fn rule(mut self, rule: CSSRule) -> Self {
        self.rules.push(rule);
        self
    }
    
    pub fn to_css(&self) -> String {
        self.rules.iter()
            .map(|r| r.to_css())
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الدوال المساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء تنسيق
pub fn style() -> Style {
    Style::new()
}

/// إنشاء قاعدة CSS
pub fn css(selector: &str) -> CSSRule {
    CSSRule::new(Selector::new().tag(selector))
}

/// إنشاء اسم فئة
pub fn class_name(name: &str) -> String {
    format!("class=\"{}\"", name)
}

/// إنشاء تنسيق مضمّن
pub fn inline_style(style: &Style) -> String {
    format!("style=\"{}\"", style.to_css())
}
