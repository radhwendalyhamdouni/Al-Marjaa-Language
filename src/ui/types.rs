// ═══════════════════════════════════════════════════════════════════════════════
// أنواع واجهات المستخدم الأساسية - UI Basic Types
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// ═══════════════════════════════════════════════════════════════════════════════
// المعرفات والمسارات
// ═══════════════════════════════════════════════════════════════════════════════

/// معرف المكون
pub type ComponentId = String;

/// اسم الخاصية
pub type PropertyName = String;

/// مسار الربط
pub type BindingPath = String;

// ═══════════════════════════════════════════════════════════════════════════════
// الألوان
// ═══════════════════════════════════════════════════════════════════════════════

/// لون واجهة المستخدم
#[derive(Debug, Clone, PartialEq)]
pub struct UIColor {
    /// الأحمر (0-255)
    pub r: u8,
    /// الأخضر (0-255)
    pub g: u8,
    /// الأزرق (0-255)
    pub b: u8,
    /// الشفافية (0.0-1.0)
    pub a: f32,
}

impl UIColor {
    /// إنشاء لون جديد
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self { r, g, b, a: a.clamp(0.0, 1.0) }
    }
    
    /// إنشاء لون من RGB
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 1.0)
    }
    
    /// إنشاء لون من كود سداسي عشري
    pub fn hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        let a = if hex.len() > 6 {
            u8::from_str_radix(&hex[6..8], 16).unwrap_or(255) as f32 / 255.0
        } else {
            1.0
        };
        Self::new(r, g, b, a)
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        if self.a < 1.0 {
            format!("rgba({}, {}, {}, {:.2})", self.r, self.g, self.b, self.a)
        } else {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        }
    }
    
    // ألوان محددة مسبقاً
    pub fn transparent() -> Self { Self::new(0, 0, 0, 0.0) }
    pub fn white() -> Self { Self::rgb(255, 255, 255) }
    pub fn black() -> Self { Self::rgb(0, 0, 0) }
    pub fn red() -> Self { Self::rgb(255, 0, 0) }
    pub fn green() -> Self { Self::rgb(0, 255, 0) }
    pub fn blue() -> Self { Self::rgb(0, 0, 255) }
    pub fn yellow() -> Self { Self::rgb(255, 255, 0) }
    pub fn cyan() -> Self { Self::rgb(0, 255, 255) }
    pub fn magenta() -> Self { Self::rgb(255, 0, 255) }
    pub fn orange() -> Self { Self::rgb(255, 165, 0) }
    pub fn purple() -> Self { Self::rgb(128, 0, 128) }
    pub fn pink() -> Self { Self::rgb(255, 192, 203) }
    pub fn gray() -> Self { Self::rgb(128, 128, 128) }
    pub fn light_gray() -> Self { Self::rgb(211, 211, 211) }
    pub fn dark_gray() -> Self { Self::rgb(64, 64, 64) }
    
    /// تفتيح اللون
    pub fn lighten(&self, amount: f32) -> Self {
        let factor = amount.clamp(0.0, 1.0);
        Self::new(
            (self.r as f32 + (255.0 - self.r as f32) * factor) as u8,
            (self.g as f32 + (255.0 - self.g as f32) * factor) as u8,
            (self.b as f32 + (255.0 - self.b as f32) * factor) as u8,
            self.a,
        )
    }
    
    /// تغميق اللون
    pub fn darken(&self, amount: f32) -> Self {
        let factor = 1.0 - amount.clamp(0.0, 1.0);
        Self::new(
            (self.r as f32 * factor) as u8,
            (self.g as f32 * factor) as u8,
            (self.b as f32 * factor) as u8,
            self.a,
        )
    }
    
    /// تعديل الشفافية
    pub fn with_alpha(&self, a: f32) -> Self {
        Self::new(self.r, self.g, self.b, a)
    }
    
    /// خلط لونين
    pub fn blend(&self, other: &UIColor, ratio: f32) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);
        Self::new(
            (self.r as f32 * (1.0 - ratio) + other.r as f32 * ratio) as u8,
            (self.g as f32 * (1.0 - ratio) + other.g as f32 * ratio) as u8,
            (self.b as f32 * (1.0 - ratio) + other.b as f32 * ratio) as u8,
            self.a * (1.0 - ratio) + other.a * ratio,
        )
    }
}

impl Default for UIColor {
    fn default() -> Self {
        Self::black()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الخطوط
// ═══════════════════════════════════════════════════════════════════════════════

/// وزن الخط
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Normal
    }
}

/// نمط الخط
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

impl Default for FontStyle {
    fn default() -> Self {
        Self::Normal
    }
}

/// زخرفة النص
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextDecoration {
    None,
    Underline,
    Overline,
    LineThrough,
    Blink,
}

impl Default for TextDecoration {
    fn default() -> Self {
        Self::None
    }
}

/// محاذاة النص
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAlign {
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
}

impl Default for TextAlign {
    fn default() -> Self {
        Self::Start
    }
}

/// خط واجهة المستخدم
#[derive(Debug, Clone, PartialEq)]
pub struct UIFont {
    /// عائلة الخط
    pub family: String,
    /// الحجم بالبكسل
    pub size: f32,
    /// الوزن
    pub weight: FontWeight,
    /// النمط
    pub style: FontStyle,
    /// الزخرفة
    pub decoration: TextDecoration,
    /// المحاذاة
    pub align: TextAlign,
}

impl UIFont {
    /// إنشاء خط جديد
    pub fn new(family: &str, size: f32) -> Self {
        Self {
            family: family.to_string(),
            size,
            weight: FontWeight::Normal,
            style: FontStyle::Normal,
            decoration: TextDecoration::None,
            align: TextAlign::Start,
        }
    }
    
    /// تعيين الوزن
    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }
    
    /// تعيين النمط
    pub fn style(mut self, style: FontStyle) -> Self {
        self.style = style;
        self
    }
    
    /// تعيين الزخرفة
    pub fn decoration(mut self, decoration: TextDecoration) -> Self {
        self.decoration = decoration;
        self
    }
    
    /// تعيين المحاذاة
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        let style = match self.style {
            FontStyle::Normal => "normal",
            FontStyle::Italic => "italic",
            FontStyle::Oblique => "oblique",
        };
        let decoration = match self.decoration {
            TextDecoration::None => "none",
            TextDecoration::Underline => "underline",
            TextDecoration::Overline => "overline",
            TextDecoration::LineThrough => "line-through",
            TextDecoration::Blink => "blink",
        };
        format!(
            "font: {} {} {}px {}; text-decoration: {}",
            style, self.weight as i32, self.size, self.family, decoration
        )
    }
}

impl Default for UIFont {
    fn default() -> Self {
        Self::new("Arial", 14.0)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// المسافات
// ═══════════════════════════════════════════════════════════════════════════════

/// وحدة القياس
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Px,
    Em,
    Rem,
    Percent,
    Vw,
    Vh,
    Auto,
}

impl Default for Unit {
    fn default() -> Self {
        Self::Px
    }
}

/// قيمة مع وحدة
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnitValue {
    pub value: f32,
    pub unit: Unit,
}

impl UnitValue {
    pub fn px(value: f32) -> Self {
        Self { value, unit: Unit::Px }
    }
    
    pub fn em(value: f32) -> Self {
        Self { value, unit: Unit::Em }
    }
    
    pub fn rem(value: f32) -> Self {
        Self { value, unit: Unit::Rem }
    }
    
    pub fn percent(value: f32) -> Self {
        Self { value, unit: Unit::Percent }
    }
    
    pub fn vw(value: f32) -> Self {
        Self { value, unit: Unit::Vw }
    }
    
    pub fn vh(value: f32) -> Self {
        Self { value, unit: Unit::Vh }
    }
    
    pub fn auto() -> Self {
        Self { value: 0.0, unit: Unit::Auto }
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        match self.unit {
            Unit::Px => format!("{}px", self.value),
            Unit::Em => format!("{}em", self.value),
            Unit::Rem => format!("{}rem", self.value),
            Unit::Percent => format!("{}%", self.value),
            Unit::Vw => format!("{}vw", self.value),
            Unit::Vh => format!("{}vh", self.value),
            Unit::Auto => "auto".to_string(),
        }
    }
}

/// الهوامش
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UIMargin {
    pub top: UnitValue,
    pub right: UnitValue,
    pub bottom: UnitValue,
    pub left: UnitValue,
}

impl UIMargin {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top: UnitValue::px(top),
            right: UnitValue::px(right),
            bottom: UnitValue::px(bottom),
            left: UnitValue::px(left),
        }
    }
    
    pub fn all(value: f32) -> Self {
        Self::new(value, value, value, value)
    }
    
    pub fn horizontal_vertical(h: f32, v: f32) -> Self {
        Self::new(v, h, v, h)
    }
    
    pub fn zero() -> Self {
        Self::all(0.0)
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        format!(
            "margin: {} {} {} {}",
            self.top.to_css(),
            self.right.to_css(),
            self.bottom.to_css(),
            self.left.to_css()
        )
    }
}

impl Default for UIMargin {
    fn default() -> Self {
        Self::zero()
    }
}

/// الحشو
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UIPadding {
    pub top: UnitValue,
    pub right: UnitValue,
    pub bottom: UnitValue,
    pub left: UnitValue,
}

impl UIPadding {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top: UnitValue::px(top),
            right: UnitValue::px(right),
            bottom: UnitValue::px(bottom),
            left: UnitValue::px(left),
        }
    }
    
    pub fn all(value: f32) -> Self {
        Self::new(value, value, value, value)
    }
    
    pub fn horizontal_vertical(h: f32, v: f32) -> Self {
        Self::new(v, h, v, h)
    }
    
    pub fn zero() -> Self {
        Self::all(0.0)
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        format!(
            "padding: {} {} {} {}",
            self.top.to_css(),
            self.right.to_css(),
            self.bottom.to_css(),
            self.left.to_css()
        )
    }
}

impl Default for UIPadding {
    fn default() -> Self {
        Self::zero()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الأحجام والمواضع
// ═══════════════════════════════════════════════════════════════════════════════

/// الحجم
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UISize {
    pub width: UnitValue,
    pub height: UnitValue,
    pub min_width: Option<UnitValue>,
    pub max_width: Option<UnitValue>,
    pub min_height: Option<UnitValue>,
    pub max_height: Option<UnitValue>,
}

impl UISize {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width: UnitValue::px(width),
            height: UnitValue::px(height),
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
        }
    }
    
    pub fn fill() -> Self {
        Self {
            width: UnitValue::percent(100.0),
            height: UnitValue::percent(100.0),
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
        }
    }
    
    pub fn auto() -> Self {
        Self {
            width: UnitValue::auto(),
            height: UnitValue::auto(),
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
        }
    }
}

impl Default for UISize {
    fn default() -> Self {
        Self::auto()
    }
}

/// نقطة
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UIPoint {
    pub x: f32,
    pub y: f32,
}

impl UIPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Default for UIPoint {
    fn default() -> Self {
        Self::zero()
    }
}

/// مستطيل
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UIRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl UIRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn from_point_size(origin: UIPoint, size: UISize) -> Self {
        Self {
            x: origin.x,
            y: origin.y,
            width: size.width.value,
            height: size.height.value,
        }
    }
    
    pub fn contains(&self, point: UIPoint) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
    
    pub fn intersects(&self, other: &UIRect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}

impl Default for UIRect {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

/// الموضع
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UIPosition {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

impl Default for UIPosition {
    fn default() -> Self {
        Self::Static
    }
}

/// التحويل
#[derive(Debug, Clone, PartialEq)]
pub struct UITransform {
    pub translate: UIPoint,
    pub rotate: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub skew_x: f32,
    pub skew_y: f32,
}

impl UITransform {
    pub fn new() -> Self {
        Self {
            translate: UIPoint::zero(),
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            skew_x: 0.0,
            skew_y: 0.0,
        }
    }
    
    pub fn translate(x: f32, y: f32) -> Self {
        Self {
            translate: UIPoint::new(x, y),
            ..Self::new()
        }
    }
    
    pub fn rotate(degrees: f32) -> Self {
        Self {
            rotate: degrees,
            ..Self::new()
        }
    }
    
    pub fn scale(sx: f32, sy: f32) -> Self {
        Self {
            scale_x: sx,
            scale_y: sy,
            ..Self::new()
        }
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        let mut transforms = Vec::new();
        
        if self.translate.x != 0.0 || self.translate.y != 0.0 {
            transforms.push(format!("translate({}px, {}px)", self.translate.x, self.translate.y));
        }
        
        if self.rotate != 0.0 {
            transforms.push(format!("rotate({}deg)", self.rotate));
        }
        
        if self.scale_x != 1.0 || self.scale_y != 1.0 {
            transforms.push(format!("scale({}, {})", self.scale_x, self.scale_y));
        }
        
        if self.skew_x != 0.0 || self.skew_y != 0.0 {
            transforms.push(format!("skew({}deg, {}deg)", self.skew_x, self.skew_y));
        }
        
        if transforms.is_empty() {
            "none".to_string()
        } else {
            transforms.join(" ")
        }
    }
}

impl Default for UITransform {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// القيم
// ═══════════════════════════════════════════════════════════════════════════════

/// قيمة واجهة المستخدم
#[derive(Debug, Clone, PartialEq)]
pub enum UIValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Color(UIColor),
    Font(UIFont),
    Size(UISize),
    Margin(UIMargin),
    Padding(UIPadding),
    Transform(UITransform),
    List(Vec<UIValue>),
    Map(HashMap<String, UIValue>),
}

impl UIValue {
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
    
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }
    
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }
    
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }
    
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }
    
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            _ => None,
        }
    }
    
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }
}

impl Default for UIValue {
    fn default() -> Self {
        Self::Null
    }
}

impl From<bool> for UIValue {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl From<i32> for UIValue {
    fn from(v: i32) -> Self {
        Self::Number(v as f64)
    }
}

impl From<f64> for UIValue {
    fn from(v: f64) -> Self {
        Self::Number(v)
    }
}

impl From<&str> for UIValue {
    fn from(v: &str) -> Self {
        Self::String(v.to_string())
    }
}

impl From<String> for UIValue {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الأحداث
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع حدث واجهة المستخدم
#[derive(Debug, Clone, PartialEq)]
pub enum UIEventKind {
    Click,
    DoubleClick,
    RightClick,
    MouseDown,
    MouseUp,
    MouseMove,
    MouseEnter,
    MouseLeave,
    KeyDown,
    KeyUp,
    KeyPress,
    Focus,
    Blur,
    Change,
    Submit,
    Scroll,
    Resize,
    DragStart,
    DragEnd,
    DragOver,
    Drop,
    TouchStart,
    TouchEnd,
    TouchMove,
    Custom(String),
}

/// حدث واجهة المستخدم
#[derive(Debug, Clone)]
pub struct UIEvent {
    /// نوع الحدث
    pub kind: UIEventKind,
    /// معرف المكون المصدر
    pub source: ComponentId,
    /// موضع الماوس (إن وجد)
    pub mouse_position: Option<UIPoint>,
    /// المفتاح المضغوط (إن وجد)
    pub key: Option<String>,
    /// القيمة الجديدة (إن وجدت)
    pub value: Option<UIValue>,
    /// البيانات الإضافية
    pub data: HashMap<String, UIValue>,
    /// تم إلغاء الحدث
    pub cancelled: bool,
}

impl UIEvent {
    pub fn new(kind: UIEventKind, source: ComponentId) -> Self {
        Self {
            kind,
            source,
            mouse_position: None,
            key: None,
            value: None,
            data: HashMap::new(),
            cancelled: false,
        }
    }
    
    pub fn click(source: ComponentId) -> Self {
        Self::new(UIEventKind::Click, source)
    }
    
    pub fn change(source: ComponentId, value: UIValue) -> Self {
        let mut event = Self::new(UIEventKind::Change, source);
        event.value = Some(value);
        event
    }
    
    pub fn cancel(&mut self) {
        self.cancelled = true;
    }
}

/// معالج حدث واجهة المستخدم
pub type UIEventHandler = Box<dyn Fn(&UIEvent) -> bool + Send + Sync>;

// ═══════════════════════════════════════════════════════════════════════════════
// الحالة والسياق
// ═══════════════════════════════════════════════════════════════════════════════

/// حالة المكون
#[derive(Debug, Clone, PartialEq)]
pub enum UIState {
    Idle,
    Hovered,
    Pressed,
    Focused,
    Disabled,
    Loading,
    Error,
}

impl Default for UIState {
    fn default() -> Self {
        Self::Idle
    }
}

/// سياق واجهة المستخدم
#[derive(Debug, Clone)]
pub struct UIContext {
    /// الثيم الحالي
    pub theme_name: String,
    /// اتجاه النص
    pub rtl: bool,
    /// حجم الشاشة
    pub screen_size: (f32, f32),
    /// نوع الجهاز
    pub device_type: String,
    /// البيانات المشتركة
    pub shared_data: HashMap<String, UIValue>,
}

impl UIContext {
    pub fn new() -> Self {
        Self {
            theme_name: "light".to_string(),
            rtl: true,
            screen_size: (1920.0, 1080.0),
            device_type: "desktop".to_string(),
            shared_data: HashMap::new(),
        }
    }
}

impl Default for UIContext {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// العنصر الأساسي
// ═══════════════════════════════════════════════════════════════════════════════

/// عنصر واجهة المستخدم
#[derive(Debug, Clone)]
pub struct UIElement {
    /// المعرف
    pub id: ComponentId,
    /// النوع
    pub element_type: String,
    /// الخصائص
    pub props: HashMap<PropertyName, UIValue>,
    /// الأطفال
    pub children: Vec<UIElement>,
    /// الحالة
    pub state: UIState,
    /// الموضع
    pub position: UIPosition,
    /// المستطيل المحسوب
    pub computed_rect: UIRect,
    /// التنسيق
    pub style: HashMap<String, String>,
}

impl UIElement {
    pub fn new(element_type: &str) -> Self {
        Self {
            id: format!("{}_{}", element_type, uuid()),
            element_type: element_type.to_string(),
            props: HashMap::new(),
            children: Vec::new(),
            state: UIState::Idle,
            position: UIPosition::Static,
            computed_rect: UIRect::default(),
            style: HashMap::new(),
        }
    }
    
    pub fn with_id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }
    
    pub fn prop(mut self, name: &str, value: UIValue) -> Self {
        self.props.insert(name.to_string(), value);
        self
    }
    
    pub fn child(mut self, child: UIElement) -> Self {
        self.children.push(child);
        self
    }
    
    pub fn style(mut self, name: &str, value: &str) -> Self {
        self.style.insert(name.to_string(), value.to_string());
        self
    }
}

/// إنشاء معرف فريد بسيط
fn uuid() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
