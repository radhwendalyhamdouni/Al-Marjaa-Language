// ═══════════════════════════════════════════════════════════════════════════════
// التصميم المتجاوب - Responsive Design
// ═══════════════════════════════════════════════════════════════════════════════

use crate::ui::types::*;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
// نقاط التوقف
// ═══════════════════════════════════════════════════════════════════════════════

/// نقطة توقف
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Breakpoint {
    /// الحد الأدنى للعرض بالبكسل
    pub min_width: u32,
    /// الحد الأقصى للعرض (اختياري)
    pub max_width: Option<u32>,
    /// اسم النقطة
    pub name: &'static str,
}

impl Breakpoint {
    pub fn new(name: &'static str, min_width: u32) -> Self {
        Self {
            min_width,
            max_width: None,
            name,
        }
    }
    
    pub fn range(name: &'static str, min_width: u32, max_width: u32) -> Self {
        Self {
            min_width,
            max_width: Some(max_width),
            name,
        }
    }
    
    pub fn matches(&self, width: u32) -> bool {
        width >= self.min_width
            && self.max_width.map_or(true, |max| width <= max)
    }
}

/// نقاط التوقف الافتراضية
#[derive(Debug, Clone)]
pub struct Breakpoints {
    /// الهواتف الصغيرة
    pub xs: Breakpoint,
    /// الهواتف
    pub sm: Breakpoint,
    /// الأجهزة اللوحية
    pub md: Breakpoint,
    /// الحواسيب المحمولة
    pub lg: Breakpoint,
    /// الحواسيب المكتبية
    pub xl: Breakpoint,
    /// الشاشات الكبيرة
    pub xxl: Breakpoint,
}

impl Breakpoints {
    pub fn new() -> Self {
        Self {
            xs: Breakpoint::range("xs", 0, 575),
            sm: Breakpoint::range("sm", 576, 767),
            md: Breakpoint::range("md", 768, 991),
            lg: Breakpoint::range("lg", 992, 1199),
            xl: Breakpoint::range("xl", 1200, 1399),
            xxl: Breakpoint::new("xxl", 1400),
        }
    }
    
    pub fn custom(breakpoints: HashMap<String, Breakpoint>) -> Self {
        let default = Self::new();
        default
    }
    
    /// الحصول على نقطة التوقف الحالية
    pub fn get_current(&self, width: u32) -> &Breakpoint {
        if self.xxl.matches(width) {
            &self.xxl
        } else if self.xl.matches(width) {
            &self.xl
        } else if self.lg.matches(width) {
            &self.lg
        } else if self.md.matches(width) {
            &self.md
        } else if self.sm.matches(width) {
            &self.sm
        } else {
            &self.xs
        }
    }
    
    /// التحقق من تطابق نقطة توقف
    pub fn is(&self, width: u32, name: &str) -> bool {
        match name {
            "xs" => self.xs.matches(width),
            "sm" => self.sm.matches(width),
            "md" => self.md.matches(width),
            "lg" => self.lg.matches(width),
            "xl" => self.xl.matches(width),
            "xxl" => self.xxl.matches(width),
            _ => false,
        }
    }
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// نوع الجهاز
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع الجهاز
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceType {
    Mobile,
    Tablet,
    Desktop,
    LargeDesktop,
    TV,
}

impl DeviceType {
    pub fn from_width(width: u32) -> Self {
        match width {
            0..=575 => Self::Mobile,
            576..=991 => Self::Tablet,
            992..=1399 => Self::Desktop,
            1400..=1919 => Self::LargeDesktop,
            _ => Self::TV,
        }
    }
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mobile => write!(f, "mobile"),
            Self::Tablet => write!(f, "tablet"),
            Self::Desktop => write!(f, "desktop"),
            Self::LargeDesktop => write!(f, "large_desktop"),
            Self::TV => write!(f, "tv"),
        }
    }
}

/// الاتجاه
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Orientation {
    Portrait,
    Landscape,
}

impl Orientation {
    pub fn from_size(width: u32, height: u32) -> Self {
        if width > height {
            Self::Landscape
        } else {
            Self::Portrait
        }
    }
}

/// حجم الشاشة
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenSize {
    pub width: u32,
    pub height: u32,
    pub device_type: DeviceType,
    pub orientation: Orientation,
}

impl ScreenSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            device_type: DeviceType::from_width(width),
            orientation: Orientation::from_size(width, height),
        }
    }
}

impl Default for ScreenSize {
    fn default() -> Self {
        Self::new(1920, 1080)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// استعلام الوسائط
// ═══════════════════════════════════════════════════════════════════════════════

/// استعلام الوسائط
#[derive(Debug, Clone)]
pub struct MediaQuery {
    /// الحد الأدنى للعرض
    pub min_width: Option<u32>,
    /// الحد الأقصى للعرض
    pub max_width: Option<u32>,
    /// الحد الأدنى للارتفاع
    pub min_height: Option<u32>,
    /// الحد الأقصى للارتفاع
    pub max_height: Option<u32>,
    /// الاتجاه
    pub orientation: Option<Orientation>,
    /// نوع الوسائط
    pub media_type: Option<String>,
}

impl MediaQuery {
    pub fn new() -> Self {
        Self {
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            orientation: None,
            media_type: None,
        }
    }
    
    pub fn min_width(mut self, width: u32) -> Self {
        self.min_width = Some(width);
        self
    }
    
    pub fn max_width(mut self, width: u32) -> Self {
        self.max_width = Some(width);
        self
    }
    
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
    
    /// التحقق من التطابق
    pub fn matches(&self, screen: &ScreenSize) -> bool {
        if let Some(min) = self.min_width {
            if screen.width < min {
                return false;
            }
        }
        
        if let Some(max) = self.max_width {
            if screen.width > max {
                return false;
            }
        }
        
        if let Some(min) = self.min_height {
            if screen.height < min {
                return false;
            }
        }
        
        if let Some(max) = self.max_height {
            if screen.height > max {
                return false;
            }
        }
        
        if let Some(ref orientation) = self.orientation {
            if screen.orientation != *orientation {
                return false;
            }
        }
        
        true
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        let mut conditions = Vec::new();
        
        if let Some(min) = self.min_width {
            conditions.push(format!("(min-width: {}px)", min));
        }
        
        if let Some(max) = self.max_width {
            conditions.push(format!("(max-width: {}px)", max));
        }
        
        if let Some(min) = self.min_height {
            conditions.push(format!("(min-height: {}px)", min));
        }
        
        if let Some(max) = self.max_height {
            conditions.push(format!("(max-height: {}px)", max));
        }
        
        if let Some(ref orientation) = self.orientation {
            let orient = match orientation {
                Orientation::Portrait => "portrait",
                Orientation::Landscape => "landscape",
            };
            conditions.push(format!("(orientation: {})", orient));
        }
        
        if conditions.is_empty() {
            "all".to_string()
        } else {
            format!("@media {}", conditions.join(" and "))
        }
    }
}

impl Default for MediaQuery {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// القيم المتجاوبة
// ═══════════════════════════════════════════════════════════════════════════════

/// قيمة متجاوبة
#[derive(Debug, Clone, PartialEq)]
pub struct ResponsiveValue<T> {
    /// القيمة الافتراضية
    pub default: T,
    /// قيمة xs
    pub xs: Option<T>,
    /// قيمة sm
    pub sm: Option<T>,
    /// قيمة md
    pub md: Option<T>,
    /// قيمة lg
    pub lg: Option<T>,
    /// قيمة xl
    pub xl: Option<T>,
    /// قيمة xxl
    pub xxl: Option<T>,
}

impl<T: Clone> ResponsiveValue<T> {
    pub fn new(default: T) -> Self {
        Self {
            default,
            xs: None,
            sm: None,
            md: None,
            lg: None,
            xl: None,
            xxl: None,
        }
    }
    
    pub fn xs(mut self, value: T) -> Self {
        self.xs = Some(value);
        self
    }
    
    pub fn sm(mut self, value: T) -> Self {
        self.sm = Some(value);
        self
    }
    
    pub fn md(mut self, value: T) -> Self {
        self.md = Some(value);
        self
    }
    
    pub fn lg(mut self, value: T) -> Self {
        self.lg = Some(value);
        self
    }
    
    pub fn xl(mut self, value: T) -> Self {
        self.xl = Some(value);
        self
    }
    
    pub fn xxl(mut self, value: T) -> Self {
        self.xxl = Some(value);
        self
    }
    
    /// الحصول على القيمة المناسبة
    pub fn get(&self, width: u32) -> &T {
        if width >= 1400 {
            self.xxl.as_ref().unwrap_or(
                self.xl.as_ref().unwrap_or(&self.default)
            )
        } else if width >= 1200 {
            self.xl.as_ref().unwrap_or(
                self.lg.as_ref().unwrap_or(&self.default)
            )
        } else if width >= 992 {
            self.lg.as_ref().unwrap_or(
                self.md.as_ref().unwrap_or(&self.default)
            )
        } else if width >= 768 {
            self.md.as_ref().unwrap_or(
                self.sm.as_ref().unwrap_or(&self.default)
            )
        } else if width >= 576 {
            self.sm.as_ref().unwrap_or(
                self.xs.as_ref().unwrap_or(&self.default)
            )
        } else {
            self.xs.as_ref().unwrap_or(&self.default)
        }
    }
}

impl<T: Clone + Default> Default for ResponsiveValue<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الحاوية المتجاوبة
// ═══════════════════════════════════════════════════════════════════════════════

/// حاوية متجاوبة
#[derive(Debug, Clone)]
pub struct ResponsiveContainer {
    /// المعرف
    pub id: String,
    /// أقصى عرض
    pub max_width: ResponsiveValue<UnitValue>,
    /// الحشو
    pub padding: ResponsiveValue<UIPadding>,
}

impl ResponsiveContainer {
    pub fn new() -> Self {
        Self {
            id: format!("container_{}", uuid()),
            max_width: ResponsiveValue::new(UnitValue::px(1200))
                .xs(UnitValue::percent(100.0))
                .sm(UnitValue::px(540))
                .md(UnitValue::px(720))
                .lg(UnitValue::px(960))
                .xl(UnitValue::px(1140))
                .xxl(UnitValue::px(1320)),
            padding: ResponsiveValue::new(UIPadding::horizontal_vertical(16.0, 12.0)),
        }
    }
    
    /// الحصول على CSS
    pub fn to_css(&self, width: u32) -> String {
        let max_width = self.max_width.get(width);
        let padding = self.padding.get(width);
        
        format!(
            "#{} {{ max-width: {}; {}; }}",
            self.id,
            max_width.to_css(),
            padding.to_css()
        )
    }
}

impl Default for ResponsiveContainer {
    fn default() -> Self {
        Self::new()
    }
}

/// تخطيط متجاوب
#[derive(Debug, Clone)]
pub struct ResponsiveLayout {
    /// الحاوية
    pub container: ResponsiveContainer,
    /// الأعمدة
    pub columns: ResponsiveValue<u32>,
    /// الفجوة
    pub gap: ResponsiveValue<f32>,
}

impl ResponsiveLayout {
    pub fn new() -> Self {
        Self {
            container: ResponsiveContainer::new(),
            columns: ResponsiveValue::new(12)
                .xs(4)
                .sm(6)
                .md(8)
                .lg(12),
            gap: ResponsiveValue::new(16.0)
                .xs(8.0)
                .sm(12.0)
                .md(16.0),
        }
    }
}

impl Default for ResponsiveLayout {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// مكونات متجاوبة
// ═══════════════════════════════════════════════════════════════════════════════

/// إخفاء عند
#[derive(Debug, Clone)]
pub struct HideOn {
    /// نقاط التوقف للإخفاء
    pub breakpoints: Vec<String>,
}

impl HideOn {
    pub fn new(breakpoints: Vec<&str>) -> Self {
        Self {
            breakpoints: breakpoints.iter().map(|s| s.to_string()).collect(),
        }
    }
    
    pub fn should_hide(&self, width: u32) -> bool {
        let breakpoints = Breakpoints::new();
        let current = breakpoints.get_current(width);
        self.breakpoints.contains(&current.name.to_string())
    }
}

/// إظهار عند
#[derive(Debug, Clone)]
pub struct ShowOn {
    /// نقاط التوقف للإظهار
    pub breakpoints: Vec<String>,
}

impl ShowOn {
    pub fn new(breakpoints: Vec<&str>) -> Self {
        Self {
            breakpoints: breakpoints.iter().map(|s| s.to_string()).collect(),
        }
    }
    
    pub fn should_show(&self, width: u32) -> bool {
        let breakpoints = Breakpoints::new();
        let current = breakpoints.get_current(width);
        self.breakpoints.contains(&current.name.to_string())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// محرك التصميم المتجاوب
// ═══════════════════════════════════════════════════════════════════════════════

/// محرك التصميم المتجاوب
#[derive(Debug)]
pub struct ResponsiveEngine {
    /// نقاط التوقف
    pub breakpoints: Breakpoints,
    /// حجم الشاشة الحالي
    pub screen_size: ScreenSize,
    /// استعلامات الوسائط
    pub media_queries: HashMap<String, MediaQuery>,
}

impl ResponsiveEngine {
    pub fn new() -> Self {
        Self {
            breakpoints: Breakpoints::new(),
            screen_size: ScreenSize::default(),
            media_queries: HashMap::new(),
        }
    }
    
    /// تحديث حجم الشاشة
    pub fn update_screen_size(&mut self, width: u32, height: u32) {
        self.screen_size = ScreenSize::new(width, height);
    }
    
    /// الحصول على نقطة التوقف الحالية
    pub fn current_breakpoint(&self) -> &Breakpoint {
        self.breakpoints.get_current(self.screen_size.width)
    }
    
    /// التحقق من تطابق نقطة توقف
    pub fn is_breakpoint(&self, name: &str) -> bool {
        self.breakpoints.is(self.screen_size.width, name)
    }
    
    /// إضافة استعلام وسائط
    pub fn add_media_query(&mut self, name: &str, query: MediaQuery) {
        self.media_queries.insert(name.to_string(), query);
    }
    
    /// التحقق من تطابق استعلام
    pub fn matches_query(&self, name: &str) -> bool {
        if let Some(query) = self.media_queries.get(name) {
            query.matches(&self.screen_size)
        } else {
            false
        }
    }
}

impl Default for ResponsiveEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الدوال المساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء قيمة متجاوبة
pub fn responsive_value<T: Clone>(default: T) -> ResponsiveValue<T> {
    ResponsiveValue::new(default)
}

/// مطابقة نقطة توقف
pub fn match_breakpoint(width: u32) -> &'static str {
    let breakpoints = Breakpoints::new();
    breakpoints.get_current(width).name
}

/// الحصول على نوع الجهاز
pub fn get_device_type(width: u32) -> DeviceType {
    DeviceType::from_width(width)
}

/// إنشاء معرف فريد
fn uuid() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
