// ═══════════════════════════════════════════════════════════════════════════════
// نظام الثيمات - Theme System
// ═══════════════════════════════════════════════════════════════════════════════

use crate::ui::types::*;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
// وضع الثيم
// ═══════════════════════════════════════════════════════════════════════════════

/// وضع الثيم
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
    Custom,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::Light
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ألوان الثيم
// ═══════════════════════════════════════════════════════════════════════════════

/// ألوان الثيم
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeColors {
    /// اللون الأساسي
    pub primary: UIColor,
    /// اللون الأساسي الفاتح
    pub primary_light: UIColor,
    /// اللون الأساسي الداكن
    pub primary_dark: UIColor,
    /// اللون الثانوي
    pub secondary: UIColor,
    /// لون النجاح
    pub success: UIColor,
    /// لون التحذير
    pub warning: UIColor,
    /// لون الخطأ
    pub error: UIColor,
    /// لون المعلومات
    pub info: UIColor,
    /// لون الخلفية
    pub background: UIColor,
    /// لون السطح
    pub surface: UIColor,
    /// لون السطح المتغير
    pub surface_variant: UIColor,
    /// لون النص الأساسي
    pub on_background: UIColor,
    /// لون النص على السطح
    pub on_surface: UIColor,
    /// لون النص على الأساسي
    pub on_primary: UIColor,
    /// لون النص على الثانوي
    pub on_secondary: UIColor,
    /// لون الحدود
    pub border: UIColor,
    /// لون الفاصل
    pub divider: UIColor,
    /// لون التمييز
    pub highlight: UIColor,
    /// لون الظل
    pub shadow: UIColor,
}

impl ThemeColors {
    pub fn light() -> Self {
        Self {
            primary: UIColor::hex("#2196F3"),
            primary_light: UIColor::hex("#64B5F6"),
            primary_dark: UIColor::hex("#1976D2"),
            secondary: UIColor::hex("#FF9800"),
            success: UIColor::hex("#4CAF50"),
            warning: UIColor::hex("#FFC107"),
            error: UIColor::hex("#F44336"),
            info: UIColor::hex("#00BCD4"),
            background: UIColor::hex("#FFFFFF"),
            surface: UIColor::hex("#FFFFFF"),
            surface_variant: UIColor::hex("#F5F5F5"),
            on_background: UIColor::hex("#212121"),
            on_surface: UIColor::hex("#212121"),
            on_primary: UIColor::white(),
            on_secondary: UIColor::black(),
            border: UIColor::hex("#E0E0E0"),
            divider: UIColor::hex("#BDBDBD"),
            highlight: UIColor::hex("#FFF9C4"),
            shadow: UIColor::hex("#000000").with_alpha(0.1),
        }
    }
    
    pub fn dark() -> Self {
        Self {
            primary: UIColor::hex("#90CAF9"),
            primary_light: UIColor::hex("#BBDEFB"),
            primary_dark: UIColor::hex("#42A5F5"),
            secondary: UIColor::hex("#FFB74D"),
            success: UIColor::hex("#81C784"),
            warning: UIColor::hex("#FFD54F"),
            error: UIColor::hex("#E57373"),
            info: UIColor::hex("#4DD0E1"),
            background: UIColor::hex("#121212"),
            surface: UIColor::hex("#1E1E1E"),
            surface_variant: UIColor::hex("#2D2D2D"),
            on_background: UIColor::hex("#FFFFFF"),
            on_surface: UIColor::hex("#FFFFFF"),
            on_primary: UIColor::black(),
            on_secondary: UIColor::black(),
            border: UIColor::hex("#424242"),
            divider: UIColor::hex("#424242"),
            highlight: UIColor::hex("#3E2723").with_alpha(0.5),
            shadow: UIColor::hex("#000000").with_alpha(0.3),
        }
    }
    
    pub fn arabic() -> Self {
        Self {
            primary: UIColor::hex("#1B5E20"), // أخضر إسلامي
            primary_light: UIColor::hex("#4CAF50"),
            primary_dark: UIColor::hex("#0D3311"),
            secondary: UIColor::hex("#D4AF37"), // ذهبي
            success: UIColor::hex("#2E7D32"),
            warning: UIColor::hex("#F9A825"),
            error: UIColor::hex("#C62828"),
            info: UIColor::hex("#00838F"),
            background: UIColor::hex("#FAFAFA"),
            surface: UIColor::hex("#FFFFFF"),
            surface_variant: UIColor::hex("#F5F5F5"),
            on_background: UIColor::hex("#1A1A1A"),
            on_surface: UIColor::hex("#1A1A1A"),
            on_primary: UIColor::white(),
            on_secondary: UIColor::black(),
            border: UIColor::hex("#E0E0E0"),
            divider: UIColor::hex("#BDBDBD"),
            highlight: UIColor::hex("#FFF8E1"),
            shadow: UIColor::hex("#000000").with_alpha(0.1),
        }
    }
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self::light()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// طباعة الثيم
// ═══════════════════════════════════════════════════════════════════════════════

/// عائلة الخط
#[derive(Debug, Clone, PartialEq)]
pub struct FontFamily {
    /// الخط الأساسي
    pub primary: String,
    /// خط العناوين
    pub heading: String,
    /// الخط أحادي المسافة
    pub monospace: String,
    /// خط عربي
    pub arabic: String,
}

impl FontFamily {
    pub fn default_family() -> Self {
        Self {
            primary: "Arial, sans-serif".to_string(),
            heading: "Arial Black, sans-serif".to_string(),
            monospace: "Consolas, monospace".to_string(),
            arabic: "Amiri, 'Traditional Arabic', serif".to_string(),
        }
    }
    
    pub fn arabic() -> Self {
        Self {
            primary: "Amiri, 'Traditional Arabic', Arial, serif".to_string(),
            heading: "'Amiri Quran', 'Traditional Arabic', serif".to_string(),
            monospace: "'Courier New', monospace".to_string(),
            arabic: "Amiri, 'Traditional Arabic', serif".to_string(),
        }
    }
}

impl Default for FontFamily {
    fn default() -> Self {
        Self::default_family()
    }
}

/// الطباعة
#[derive(Debug, Clone, PartialEq)]
pub struct Typography {
    /// عائلات الخطوط
    pub font_family: FontFamily,
    /// حجم الخط الأساسي
    pub base_size: f32,
    /// أحجام العناوين
    pub heading_sizes: HashMap<String, f32>,
    /// أوزان الخطوط
    pub font_weights: HashMap<String, FontWeight>,
    /// ارتفاع السطر
    pub line_heights: HashMap<String, f32>,
}

impl Typography {
    pub fn new() -> Self {
        let mut heading_sizes = HashMap::new();
        heading_sizes.insert("h1".to_string(), 32.0);
        heading_sizes.insert("h2".to_string(), 28.0);
        heading_sizes.insert("h3".to_string(), 24.0);
        heading_sizes.insert("h4".to_string(), 20.0);
        heading_sizes.insert("h5".to_string(), 18.0);
        heading_sizes.insert("h6".to_string(), 16.0);
        
        let mut font_weights = HashMap::new();
        font_weights.insert("normal".to_string(), FontWeight::Normal);
        font_weights.insert("bold".to_string(), FontWeight::Bold);
        font_weights.insert("light".to_string(), FontWeight::Light);
        
        let mut line_heights = HashMap::new();
        line_heights.insert("normal".to_string(), 1.5);
        line_heights.insert("heading".to_string(), 1.2);
        line_heights.insert("tight".to_string(), 1.25);
        line_heights.insert("loose".to_string(), 1.75);
        
        Self {
            font_family: FontFamily::default(),
            base_size: 16.0,
            heading_sizes,
            font_weights,
            line_heights,
        }
    }
    
    pub fn arabic() -> Self {
        Self {
            font_family: FontFamily::arabic(),
            base_size: 18.0,
            ..Self::new()
        }
    }
}

impl Default for Typography {
    fn default() -> Self {
        Self::new()
    }
}

/// طباعة الثيم
pub type ThemeTypography = Typography;

// ═══════════════════════════════════════════════════════════════════════════════
// تباعد الثيم
// ═══════════════════════════════════════════════════════════════════════════════

/// تباعد الثيم
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThemeSpacing {
    /// الوحدة الأساسية
    pub unit: f32,
    /// الهوامش
    pub margins: [f32; 6],
    /// الحشو
    pub paddings: [f32; 6],
    /// الفجوات
    pub gaps: [f32; 6],
}

impl ThemeSpacing {
    pub fn new() -> Self {
        Self {
            unit: 8.0,
            margins: [4.0, 8.0, 16.0, 24.0, 32.0, 48.0],
            paddings: [4.0, 8.0, 16.0, 24.0, 32.0, 48.0],
            gaps: [4.0, 8.0, 12.0, 16.0, 24.0, 32.0],
        }
    }
}

impl Default for ThemeSpacing {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ظلال الثيم
// ═══════════════════════════════════════════════════════════════════════════════

/// ظلال الثيم
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeShadows {
    /// الظلال
    pub shadows: HashMap<String, String>,
}

impl ThemeShadows {
    pub fn new() -> Self {
        let mut shadows = HashMap::new();
        shadows.insert("sm".to_string(), "0 1px 2px rgba(0,0,0,0.05)".to_string());
        shadows.insert("md".to_string(), "0 4px 6px rgba(0,0,0,0.1)".to_string());
        shadows.insert("lg".to_string(), "0 10px 15px rgba(0,0,0,0.1)".to_string());
        shadows.insert("xl".to_string(), "0 20px 25px rgba(0,0,0,0.15)".to_string());
        shadows.insert("2xl".to_string(), "0 25px 50px rgba(0,0,0,0.25)".to_string());
        
        Self { shadows }
    }
}

impl Default for ThemeShadows {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// حدود الثيم
// ═══════════════════════════════════════════════════════════════════════════════

/// حدود الثيم
#[derive(Debug, Clone, PartialEq)]
pub struct ThemeBorders {
    /// نصف قطر الزوايا
    pub radius: HashMap<String, f32>,
    /// عرض الحدود
    pub widths: HashMap<String, f32>,
}

impl ThemeBorders {
    pub fn new() -> Self {
        let mut radius = HashMap::new();
        radius.insert("none".to_string(), 0.0);
        radius.insert("sm".to_string(), 2.0);
        radius.insert("md".to_string(), 4.0);
        radius.insert("lg".to_string(), 8.0);
        radius.insert("xl".to_string(), 12.0);
        radius.insert("full".to_string(), 9999.0);
        
        let mut widths = HashMap::new();
        widths.insert("thin".to_string(), 1.0);
        widths.insert("medium".to_string(), 2.0);
        widths.insert("thick".to_string(), 4.0);
        
        Self { radius, widths }
    }
}

impl Default for ThemeBorders {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الثيم
// ═══════════════════════════════════════════════════════════════════════════════

/// مخطط الألوان
pub type ColorScheme = ThemeColors;

/// الثيم
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    /// اسم الثيم
    pub name: String,
    /// الوضع
    pub mode: ThemeMode,
    /// الألوان
    pub colors: ThemeColors,
    /// الطباعة
    pub typography: ThemeTypography,
    /// التباعد
    pub spacing: ThemeSpacing,
    /// الظلال
    pub shadows: ThemeShadows,
    /// الحدود
    pub borders: ThemeBorders,
    /// متغيرات مخصصة
    pub custom: HashMap<String, UIValue>,
}

impl Theme {
    pub fn new(name: &str, mode: ThemeMode) -> Self {
        Self {
            name: name.to_string(),
            mode,
            colors: match mode {
                ThemeMode::Dark => ThemeColors::dark(),
                _ => ThemeColors::light(),
            },
            typography: ThemeTypography::new(),
            spacing: ThemeSpacing::new(),
            shadows: ThemeShadows::new(),
            borders: ThemeBorders::new(),
            custom: HashMap::new(),
        }
    }
    
    /// إضافة متغير مخصص
    pub fn with_custom(mut self, key: &str, value: UIValue) -> Self {
        self.custom.insert(key.to_string(), value);
        self
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        let mut css = String::new();
        
        // متغيرات الألوان
        css.push_str(":root {\n");
        css.push_str(&format!("  --color-primary: {};\n", self.colors.primary.to_css()));
        css.push_str(&format!("  --color-secondary: {};\n", self.colors.secondary.to_css()));
        css.push_str(&format!("  --color-success: {};\n", self.colors.success.to_css()));
        css.push_str(&format!("  --color-warning: {};\n", self.colors.warning.to_css()));
        css.push_str(&format!("  --color-error: {};\n", self.colors.error.to_css()));
        css.push_str(&format!("  --color-background: {};\n", self.colors.background.to_css()));
        css.push_str(&format!("  --color-surface: {};\n", self.colors.surface.to_css()));
        css.push_str(&format!("  --color-on-background: {};\n", self.colors.on_background.to_css()));
        css.push_str(&format!("  --color-border: {};\n", self.colors.border.to_css()));
        
        // متغيرات التباعد
        css.push_str(&format!("  --spacing-unit: {}px;\n", self.spacing.unit));
        
        // متغيرات الحدود
        css.push_str(&format!("  --border-radius-md: {}px;\n", 
            self.borders.radius.get("md").unwrap_or(&4.0)));
        
        css.push_str("}\n");
        
        css
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new("default", ThemeMode::Light)
    }
}

/// تكوين الثيم
pub type ThemeConfig = Theme;

// ═══════════════════════════════════════════════════════════════════════════════
// مدير الثيمات
// ═══════════════════════════════════════════════════════════════════════════════

/// مدير الثيمات
#[derive(Debug)]
pub struct ThemeManager {
    /// الثيمات المتاحة
    themes: HashMap<String, Theme>,
    /// الثيم الحالي
    current_theme: String,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        
        // إضافة الثيمات الافتراضية
        themes.insert("light".to_string(), default_light_theme());
        themes.insert("dark".to_string(), default_dark_theme());
        themes.insert("arabic".to_string(), default_arabic_theme());
        
        Self {
            themes,
            current_theme: "light".to_string(),
        }
    }
    
    /// إضافة ثيم
    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.insert(theme.name.clone(), theme);
    }
    
    /// تعيين الثيم الحالي
    pub fn set_theme(&mut self, theme: Theme) {
        let name = theme.name.clone();
        self.themes.insert(name.clone(), theme);
        self.current_theme = name;
    }
    
    /// تحويل إلى ثيم
    pub fn switch_to(&mut self, name: &str) -> Result<(), String> {
        if self.themes.contains_key(name) {
            self.current_theme = name.to_string();
            Ok(())
        } else {
            Err(format!("الثيم '{}' غير موجود", name))
        }
    }
    
    /// الحصول على الثيم الحالي
    pub fn current(&self) -> Option<&Theme> {
        self.themes.get(&self.current_theme)
    }
    
    /// الحصول على ثيم
    pub fn get(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }
    
    /// الحصول على اسم الثيم الحالي
    pub fn current_name(&self) -> &str {
        &self.current_theme
    }
    
    /// قائمة الثيمات المتاحة
    pub fn available_themes(&self) -> Vec<&str> {
        self.themes.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الدوال المساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء ثيم
pub fn create_theme(name: &str, mode: ThemeMode) -> Theme {
    Theme::new(name, mode)
}

/// تحميل ثيم من ملف
pub fn load_theme(_path: &str) -> Result<Theme, String> {
    // TODO: تنفيذ التحميل من ملف
    Ok(Theme::default())
}

/// حفظ ثيم إلى ملف
pub fn save_theme(_theme: &Theme, _path: &str) -> Result<(), String> {
    // TODO: تنفيذ الحفظ إلى ملف
    Ok(())
}

/// تطبيق ثيم
pub fn apply_theme(_theme: &Theme) -> String {
    // TODO: تطبيق الثيم على المكونات
    String::new()
}

/// الثيم الفاتح الافتراضي
pub fn default_light_theme() -> Theme {
    Theme {
        name: "light".to_string(),
        mode: ThemeMode::Light,
        colors: ThemeColors::light(),
        typography: ThemeTypography::new(),
        spacing: ThemeSpacing::new(),
        shadows: ThemeShadows::new(),
        borders: ThemeBorders::new(),
        custom: HashMap::new(),
    }
}

/// الثيم الداكن الافتراضي
pub fn default_dark_theme() -> Theme {
    Theme {
        name: "dark".to_string(),
        mode: ThemeMode::Dark,
        colors: ThemeColors::dark(),
        typography: ThemeTypography::new(),
        spacing: ThemeSpacing::new(),
        shadows: ThemeShadows::new(),
        borders: ThemeBorders::new(),
        custom: HashMap::new(),
    }
}

/// الثيم العربي الافتراضي
pub fn default_arabic_theme() -> Theme {
    Theme {
        name: "arabic".to_string(),
        mode: ThemeMode::Light,
        colors: ThemeColors::arabic(),
        typography: ThemeTypography::arabic(),
        spacing: ThemeSpacing::new(),
        shadows: ThemeShadows::new(),
        borders: ThemeBorders::new(),
        custom: HashMap::new(),
    }
}
