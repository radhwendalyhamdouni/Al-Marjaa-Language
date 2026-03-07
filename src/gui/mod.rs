// ═══════════════════════════════════════════════════════════════════════════════
// وحدة واجهات المستخدم الرسومية - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// تدعم إنشاء واجهات GUI بالكود العربي + Vibe Coding
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// نوع العنصر GUI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GUIElement {
    /// نافذة رئيسية
    Window {
        id: String,
        title: String,
        width: u32,
        height: u32,
        children: Vec<Box<GUIElement>>,
    },
    /// زر
    Button {
        id: String,
        text: String,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        color: Option<GUIColor>,
        on_click: Option<String>,
        animation: Option<GUIAnimation>,
    },
    /// حقل نص
    TextField {
        id: String,
        placeholder: String,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        value: String,
    },
    /// تسمية
    Label {
        id: String,
        text: String,
        x: i32,
        y: i32,
        color: Option<GUIColor>,
        font_size: u32,
    },
    /// صورة
    Image {
        id: String,
        source: String,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    },
    /// حاوية
    Container {
        id: String,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        color: Option<GUIColor>,
        children: Vec<Box<GUIElement>>,
    },
    /// شريط تقدم
    ProgressBar {
        id: String,
        x: i32,
        y: i32,
        width: u32,
        value: f32,
    },
    /// منزلق
    Slider {
        id: String,
        x: i32,
        y: i32,
        width: u32,
        min: f32,
        max: f32,
        value: f32,
    },
    /// قائمة منسدلة
    Dropdown {
        id: String,
        x: i32,
        y: i32,
        width: u32,
        options: Vec<String>,
        selected: usize,
    },
    /// مربع اختيار
    Checkbox {
        id: String,
        text: String,
        x: i32,
        y: i32,
        checked: bool,
    },
}

/// اللون
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GUIColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl GUIColor {
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue, alpha: 255 }
    }
    
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self { red, green, blue, alpha }
    }
    
    /// ألوان جاهزة
    pub fn red() -> Self { Self::rgb(255, 0, 0) }
    pub fn green() -> Self { Self::rgb(0, 200, 0) }
    pub fn blue() -> Self { Self::rgb(0, 100, 255) }
    pub fn white() -> Self { Self::rgb(255, 255, 255) }
    pub fn black() -> Self { Self::rgb(0, 0, 0) }
    pub fn yellow() -> Self { Self::rgb(255, 200, 0) }
    pub fn cyan() -> Self { Self::rgb(0, 200, 200) }
    pub fn magenta() -> Self { Self::rgb(200, 0, 200) }
    pub fn orange() -> Self { Self::rgb(255, 140, 0) }
    pub fn purple() -> Self { Self::rgb(128, 0, 200) }
    pub fn pink() -> Self { Self::rgb(255, 150, 180) }
    pub fn gray() -> Self { Self::rgb(128, 128, 128) }
    
    /// لون من اسم عربي
    pub fn from_arabic(name: &str) -> Option<Self> {
        match name {
            "أحمر" | "احمر" => Some(Self::red()),
            "أخضر" | "اخضر" => Some(Self::green()),
            "أزرق" | "ازرق" => Some(Self::blue()),
            "أبيض" | "ابيض" => Some(Self::white()),
            "أسود" | "اسود" => Some(Self::black()),
            "أصفر" | "اصفر" => Some(Self::yellow()),
            "سماوي" => Some(Self::cyan()),
            "بنفسجي" => Some(Self::magenta()),
            "برتقالي" => Some(Self::orange()),
            "أرجواني" | "ارجواني" => Some(Self::purple()),
            "وردي" => Some(Self::pink()),
            "رمادي" => Some(Self::gray()),
            _ => None,
        }
    }
    
    pub fn to_css(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.red, self.green, self.blue, self.alpha as f32 / 255.0)
    }
    
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

/// نوع الحركة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationType {
    FadeIn, FadeOut, SlideIn, SlideOut,
    ScaleUp, ScaleDown, Rotate, Shake,
    Bounce, Pulse, Blink,
}

impl AnimationType {
    pub fn from_arabic(name: &str) -> Option<Self> {
        match name {
            "تلاشي_دخول" | "ظهور" => Some(Self::FadeIn),
            "تلاشي_خروج" | "اختفاء" => Some(Self::FadeOut),
            "انزلاق_دخول" | "انزلاق" => Some(Self::SlideIn),
            "انزلاق_خروج" => Some(Self::SlideOut),
            "تكبير" => Some(Self::ScaleUp),
            "تصغير" => Some(Self::ScaleDown),
            "دوران" => Some(Self::Rotate),
            "اهتزاز" => Some(Self::Shake),
            "نط" | "قفز" => Some(Self::Bounce),
            "نبض" => Some(Self::Pulse),
            "وميض" | "غمز" => Some(Self::Blink),
            _ => None,
        }
    }
}

/// الحركة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GUIAnimation {
    pub animation_type: AnimationType,
    pub duration_ms: u32,
    pub delay_ms: u32,
    pub repeat: bool,
}

impl GUIAnimation {
    pub fn new(animation_type: AnimationType, duration_ms: u32) -> Self {
        Self { animation_type, duration_ms, delay_ms: 0, repeat: false }
    }
    
    pub fn from_arabic(name: &str, duration: u32) -> Option<Self> {
        AnimationType::from_arabic(name).map(|t| Self::new(t, duration))
    }
}

/// محرك GUI
#[derive(Debug, Clone)]
pub struct GUIEngine {
    elements: HashMap<String, GUIElement>,
    current_window: Option<String>,
    next_id: u64,
}

impl GUIEngine {
    pub fn new() -> Self {
        Self { elements: HashMap::new(), current_window: None, next_id: 1 }
    }
    
    fn generate_id(&mut self, prefix: &str) -> String {
        let id = format!("{}_{}", prefix, self.next_id);
        self.next_id += 1;
        id
    }
    
    pub fn create_window(&mut self, title: &str, width: u32, height: u32) -> String {
        let id = self.generate_id("window");
        let window = GUIElement::Window {
            id: id.clone(),
            title: title.to_string(),
            width,
            height,
            children: Vec::new(),
        };
        self.elements.insert(id.clone(), window);
        self.current_window = Some(id.clone());
        id
    }
    
    pub fn add_element(&mut self, element: GUIElement) -> String {
        let id = match &element {
            GUIElement::Window { id, .. } => id.clone(),
            GUIElement::Button { id, .. } => id.clone(),
            GUIElement::TextField { id, .. } => id.clone(),
            GUIElement::Label { id, .. } => id.clone(),
            GUIElement::Image { id, .. } => id.clone(),
            GUIElement::Container { id, .. } => id.clone(),
            GUIElement::ProgressBar { id, .. } => id.clone(),
            GUIElement::Slider { id, .. } => id.clone(),
            GUIElement::Dropdown { id, .. } => id.clone(),
            GUIElement::Checkbox { id, .. } => id.clone(),
        };
        
        if let Some(window_id) = &self.current_window {
            if let Some(GUIElement::Window { children, .. }) = self.elements.get_mut(window_id) {
                children.push(Box::new(element.clone()));
            }
        }
        
        self.elements.insert(id.clone(), element);
        id
    }
    
    pub fn create_button(&mut self, text: &str, x: i32, y: i32) -> String {
        let id = self.generate_id("btn");
        let button = GUIElement::Button {
            id: id.clone(),
            text: text.to_string(),
            x, y,
            width: 120, height: 40,
            color: None,
            on_click: None,
            animation: None,
        };
        self.add_element(button)
    }
    
    pub fn create_button_advanced(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        color: Option<GUIColor>,
        animation: Option<GUIAnimation>,
    ) -> String {
        let id = self.generate_id("btn");
        let button = GUIElement::Button {
            id: id.clone(),
            text: text.to_string(),
            x, y, width, height,
            color, on_click: None, animation,
        };
        self.add_element(button)
    }
    
    pub fn create_textfield(&mut self, placeholder: &str, x: i32, y: i32, width: u32) -> String {
        let id = self.generate_id("txt");
        let textfield = GUIElement::TextField {
            id: id.clone(),
            placeholder: placeholder.to_string(),
            x, y, width,
            height: 35,
            value: String::new(),
        };
        self.add_element(textfield)
    }
    
    pub fn create_label(&mut self, text: &str, x: i32, y: i32) -> String {
        let id = self.generate_id("lbl");
        let label = GUIElement::Label {
            id: id.clone(),
            text: text.to_string(),
            x, y,
            color: None,
            font_size: 16,
        };
        self.add_element(label)
    }
    
    pub fn create_label_advanced(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        color: Option<GUIColor>,
        font_size: u32,
    ) -> String {
        let id = self.generate_id("lbl");
        let label = GUIElement::Label { id: id.clone(), text: text.to_string(), x, y, color, font_size };
        self.add_element(label)
    }
    
    pub fn create_container(&mut self, x: i32, y: i32, width: u32, height: u32, color: Option<GUIColor>) -> String {
        let id = self.generate_id("container");
        let container = GUIElement::Container { id: id.clone(), x, y, width, height, color, children: Vec::new() };
        self.add_element(container)
    }
    
    pub fn create_progressbar(&mut self, x: i32, y: i32, width: u32) -> String {
        let id = self.generate_id("prog");
        let progress = GUIElement::ProgressBar { id: id.clone(), x, y, width, value: 0.0 };
        self.add_element(progress)
    }
    
    pub fn create_slider(&mut self, x: i32, y: i32, width: u32, min: f32, max: f32) -> String {
        let id = self.generate_id("slider");
        let slider = GUIElement::Slider { id: id.clone(), x, y, width, min, max, value: min };
        self.add_element(slider)
    }
    
    pub fn create_dropdown(&mut self, x: i32, y: i32, width: u32, options: Vec<&str>) -> String {
        let id = self.generate_id("drop");
        let dropdown = GUIElement::Dropdown {
            id: id.clone(),
            x, y, width,
            options: options.iter().map(|s| s.to_string()).collect(),
            selected: 0,
        };
        self.add_element(dropdown)
    }
    
    pub fn create_checkbox(&mut self, text: &str, x: i32, y: i32) -> String {
        let id = self.generate_id("chk");
        let checkbox = GUIElement::Checkbox { id: id.clone(), text: text.to_string(), x, y, checked: false };
        self.add_element(checkbox)
    }
    
    pub fn bind_click(&mut self, element_id: &str, handler: &str) -> bool {
        if let Some(element) = self.elements.get_mut(element_id) {
            if let GUIElement::Button { on_click, .. } = element {
                *on_click = Some(handler.to_string());
                return true;
            }
        }
        false
    }
    
    pub fn update_value(&mut self, element_id: &str, value: &str) -> bool {
        if let Some(element) = self.elements.get_mut(element_id) {
            match element {
                GUIElement::TextField { value: v, .. } => { *v = value.to_string(); true }
                GUIElement::Label { text, .. } => { *text = value.to_string(); true }
                GUIElement::ProgressBar { value: v, .. } => { *v = value.parse().unwrap_or(0.0); true }
                _ => false,
            }
        } else { false }
    }
    
    pub fn update_color(&mut self, element_id: &str, color: GUIColor) -> bool {
        if let Some(element) = self.elements.get_mut(element_id) {
            match element {
                GUIElement::Button { color: c, .. } => { *c = Some(color); true }
                GUIElement::Label { color: c, .. } => { *c = Some(color); true }
                GUIElement::Container { color: c, .. } => { *c = Some(color); true }
                _ => false,
            }
        } else { false }
    }
    
    pub fn get_element(&self, id: &str) -> Option<&GUIElement> {
        self.elements.get(id)
    }
    
    pub fn get_current_window(&self) -> Option<&GUIElement> {
        self.current_window.as_ref().and_then(|id| self.elements.get(id))
    }
    
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self.get_current_window()).unwrap_or_default()
    }
    
    /// تصدير إلى HTML كامل
    pub fn to_html(&self) -> String {
        if let Some(GUIElement::Window { title, width, height, children, .. }) = self.get_current_window() {
            let mut html = format!(
                r#"<!DOCTYPE html>
<html dir="rtl" lang="ar">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{ 
            font-family: 'Segoe UI', Tahoma, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
        }}
        .window {{
            width: {}px;
            height: {}px;
            background: white;
            border-radius: 15px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            position: relative;
            overflow: hidden;
        }}
        .window-header {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 15px 20px;
            font-size: 18px;
            font-weight: bold;
        }}
        .window-content {{
            padding: 20px;
            position: relative;
            height: calc(100% - 50px);
        }}
        .btn {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 8px;
            cursor: pointer;
            font-size: 14px;
            transition: all 0.3s;
            position: absolute;
        }}
        .btn:hover {{
            transform: translateY(-2px);
            box-shadow: 0 5px 20px rgba(102, 126, 234, 0.4);
        }}
        .txt {{
            border: 2px solid #e0e0e0;
            padding: 10px 15px;
            border-radius: 8px;
            font-size: 14px;
            position: absolute;
            direction: rtl;
        }}
        .txt:focus {{
            outline: none;
            border-color: #667eea;
        }}
        .lbl {{
            position: absolute;
            color: #333;
        }}
        .container {{
            position: absolute;
            border-radius: 10px;
            padding: 15px;
            background: #f5f5f5;
        }}
        .progress {{
            position: absolute;
            height: 8px;
            background: #e0e0e0;
            border-radius: 4px;
            overflow: hidden;
        }}
        .progress-bar {{
            height: 100%;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            transition: width 0.3s;
        }}
        .slider {{
            position: absolute;
            -webkit-appearance: none;
            height: 8px;
            background: #e0e0e0;
            border-radius: 4px;
        }}
        .slider::-webkit-slider-thumb {{
            -webkit-appearance: none;
            width: 20px;
            height: 20px;
            background: #667eea;
            border-radius: 50%;
            cursor: pointer;
        }}
        select {{
            position: absolute;
            padding: 10px;
            border-radius: 8px;
            border: 2px solid #e0e0e0;
            background: white;
        }}
        .checkbox-label {{
            position: absolute;
            display: flex;
            align-items: center;
            gap: 8px;
        }}
        @keyframes fadeIn {{ from {{ opacity: 0; }} to {{ opacity: 1; }} }}
        @keyframes slideIn {{ from {{ transform: translateX(-100%); }} to {{ transform: translateX(0); }} }}
        @keyframes scaleUp {{ from {{ transform: scale(0); }} to {{ transform: scale(1); }} }}
        @keyframes rotate {{ from {{ transform: rotate(0deg); }} to {{ transform: rotate(360deg); }} }}
        @keyframes shake {{ 0%, 100% {{ transform: translateX(0); }} 25% {{ transform: translateX(-10px); }} 75% {{ transform: translateX(10px); }} }}
        @keyframes bounce {{ 0%, 100% {{ transform: translateY(0); }} 50% {{ transform: translateY(-20px); }} }}
        @keyframes pulse {{ 0%, 100% {{ transform: scale(1); }} 50% {{ transform: scale(1.1); }} }}
        @keyframes blink {{ 0%, 100% {{ opacity: 1; }} 50% {{ opacity: 0; }} }}
        .anim-fadeIn {{ animation: fadeIn 0.5s ease-out; }}
        .anim-slideIn {{ animation: slideIn 0.5s ease-out; }}
        .anim-scaleUp {{ animation: scaleUp 0.3s ease-out; }}
        .anim-rotate {{ animation: rotate 1s linear infinite; }}
        .anim-shake {{ animation: shake 0.5s ease-out; }}
        .anim-bounce {{ animation: bounce 0.5s ease-out; }}
        .anim-pulse {{ animation: pulse 1s ease-out infinite; }}
        .anim-blink {{ animation: blink 1s ease-out infinite; }}
    </style>
</head>
<body>
    <div class="window">
        <div class="window-header">{}</div>
        <div class="window-content">
"#,
                title, width, height, title
            );
            
            for child in children {
                html.push_str(&self.element_to_html(child));
            }
            
            html.push_str(r#"        </div>
    </div>
</body>
</html>"#);
            
            html
        } else {
            String::new()
        }
    }
    
    fn element_to_html(&self, element: &GUIElement) -> String {
        match element {
            GUIElement::Button { id, text, x, y, width, height, color, animation, .. } => {
                let style = format!(
                    "left: {}px; top: {}px; width: {}px; height: {}px;{}",
                    x, y, width, height,
                    color.as_ref().map(|c| format!(" background: {};", c.to_css())).unwrap_or_default()
                );
                let anim_class = animation.as_ref().map(|a| {
                    match a.animation_type {
                        AnimationType::FadeIn => " anim-fadeIn",
                        AnimationType::SlideIn => " anim-slideIn",
                        AnimationType::ScaleUp => " anim-scaleUp",
                        AnimationType::Rotate => " anim-rotate",
                        AnimationType::Shake => " anim-shake",
                        AnimationType::Bounce => " anim-bounce",
                        AnimationType::Pulse => " anim-pulse",
                        AnimationType::Blink => " anim-blink",
                        _ => "",
                    }
                }).unwrap_or_default();
                format!(r#"            <button id="{}" class="btn{}" style="{}">{}</button>
"#, id, anim_class, style, text)
            }
            GUIElement::TextField { id, placeholder, x, y, width, value, .. } => {
                format!(r#"            <input id="{}" class="txt" type="text" placeholder="{}" value="{}" style="left: {}px; top: {}px; width: {}px;">
"#, id, placeholder, value, x, y, width)
            }
            GUIElement::Label { id, text, x, y, color, font_size } => {
                let style = format!(
                    "left: {}px; top: {}px; font-size: {}px;{}",
                    x, y, font_size,
                    color.as_ref().map(|c| format!(" color: {};", c.to_css())).unwrap_or_default()
                );
                format!(r#"            <span id="{}" class="lbl" style="{}">{}</span>
"#, id, style, text)
            }
            GUIElement::Container { id, x, y, width, height, color, children } => {
                let style = format!(
                    "left: {}px; top: {}px; width: {}px; height: {}px;{}",
                    x, y, width, height,
                    color.as_ref().map(|c| format!(" background: {};", c.to_css())).unwrap_or_default()
                );
                let mut html = format!(r#"            <div id="{}" class="container" style="{}">
"#, id, style);
                for child in children {
                    html.push_str(&self.element_to_html(child));
                }
                html.push_str("            </div>\n");
                html
            }
            GUIElement::ProgressBar { id, x, y, width, value } => {
                format!(r#"            <div id="{}" class="progress" style="left: {}px; top: {}px; width: {}px;">
                <div class="progress-bar" style="width: {}%;"></div>
            </div>
"#, id, x, y, width, value * 100.0)
            }
            GUIElement::Slider { id, x, y, width, min, max, value } => {
                format!(r#"            <input id="{}" class="slider" type="range" min="{}" max="{}" value="{}" style="left: {}px; top: {}px; width: {}px;">
"#, id, min, max, value, x, y, width)
            }
            GUIElement::Dropdown { id, x, y, width, options, selected } => {
                let options_html: String = options.iter().enumerate()
                    .map(|(i, opt)| format!(r#"                <option{}>{}</option>"#, 
                        if i == *selected { " selected" } else { "" }, opt))
                    .collect::<Vec<_>>().join("\n");
                format!(r#"            <select id="{}" style="left: {}px; top: {}px; width: {}px;">
{}
            </select>
"#, id, x, y, width, options_html)
            }
            GUIElement::Checkbox { id, text, x, y, checked } => {
                format!(r#"            <label class="checkbox-label" style="left: {}px; top: {}px;">
                <input id="{}" type="checkbox"{}> {}
            </label>
"#, x, y, id, if *checked { " checked" } else { "" }, text)
            }
            _ => String::new()
        }
    }
    
    pub fn clear(&mut self) {
        self.elements.clear();
        self.current_window = None;
        self.next_id = 1;
    }
}

impl Default for GUIEngine {
    fn default() -> Self { Self::new() }
}

// ═══════════════════════════════════════════════════════════════════════════════
// محرك Vibe GUI - تحويل النص العربي إلى واجهات
// ═══════════════════════════════════════════════════════════════════════════════

pub struct VibeGUIEngine {
    gui_engine: GUIEngine,
}

impl VibeGUIEngine {
    pub fn new() -> Self {
        Self { gui_engine: GUIEngine::new() }
    }
    
    /// تحويل نص عربي إلى GUI
    pub fn text_to_gui(&mut self, text: &str) -> Result<String, String> {
        let lower = text.to_lowercase();
        
        if lower.contains("نافذة") || lower.contains("شاشة") {
            self.parse_window(text)
        } else if lower.contains("زر") {
            self.parse_button(text)
        } else if lower.contains("حقل") || lower.contains("مدخل") || lower.contains("إدخال") {
            self.parse_textfield(text)
        } else if lower.contains("تسمية") || lower.contains("عنوان") {
            self.parse_label(text)
        } else if lower.contains("حاوية") || lower.contains("صندوق") {
            self.parse_container(text)
        } else if lower.contains("قائمة") {
            self.parse_dropdown(text)
        } else if lower.contains("اختيار") || lower.contains("مربع") {
            self.parse_checkbox(text)
        } else if lower.contains("منزلق") {
            self.parse_slider(text)
        } else if lower.contains("تقدم") || lower.contains("شريط") {
            self.parse_progress(text)
        } else {
            Err(format!("لم أفهم الأمر: {}", text))
        }
    }
    
    fn parse_window(&mut self, text: &str) -> Result<String, String> {
        let title = text.split("عنوانها").nth(1)
            .and_then(|s| s.split_whitespace().next())
            .unwrap_or("مشروع جديد");
        
        let width = text.split("عرض").nth(1)
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse().ok())
            .unwrap_or(800);
            
        let height = text.split("ارتفاع").nth(1)
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse().ok())
            .unwrap_or(600);
        
        let id = self.gui_engine.create_window(title, width, height);
        Ok(format!("نافذة(\"{}\"، {}، {}) // {}", title, width, height, id))
    }
    
    fn parse_button(&mut self, text: &str) -> Result<String, String> {
        let btn_text = text.split("مكتوب").nth(1)
            .or_else(|| text.split("عليه").nth(1))
            .map(|s| s.trim().trim_matches('"'))
            .unwrap_or("زر");
        
        let x = text.split("س").nth(1)
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse().ok())
            .unwrap_or(50);
            
        let y = text.split("ص").nth(1)
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse().ok())
            .unwrap_or(50);
        
        let color = self.extract_color(text);
        let animation = self.extract_animation(text);
        
        let id = self.gui_engine.create_button_advanced(btn_text, x, y, 120, 40, color, animation);
        Ok(format!("زر(\"{}\"، {}، {}) // {}", btn_text, x, y, id))
    }
    
    fn parse_textfield(&mut self, text: &str) -> Result<String, String> {
        let placeholder = text.split("نص").nth(1)
            .or_else(|| text.split("تلميح").nth(1))
            .map(|s| s.trim())
            .unwrap_or("أدخل...");
        
        let x = text.split("س").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(50);
        let y = text.split("ص").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(100);
        let width = text.split("عرض").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(200);
        
        let id = self.gui_engine.create_textfield(placeholder, x, y, width);
        Ok(format!("حقل_نص(\"{}\"، {}، {}) // {}", placeholder, x, y, id))
    }
    
    fn parse_label(&mut self, text: &str) -> Result<String, String> {
        let label_text = text.split("نصها").nth(1)
            .or_else(|| text.split("بمحتوى").nth(1))
            .map(|s| s.trim())
            .unwrap_or("تسمية");
        
        let x = text.split("س").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(50);
        let y = text.split("ص").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(150);
        let color = self.extract_color(text);
        let font_size = text.split("حجم").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(16);
        
        let id = self.gui_engine.create_label_advanced(label_text, x, y, color, font_size);
        Ok(format!("تسمية(\"{}\"، {}، {}) // {}", label_text, x, y, id))
    }
    
    fn parse_container(&mut self, text: &str) -> Result<String, String> {
        let x = text.split("س").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(50);
        let y = text.split("ص").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(200);
        let width = text.split("عرض").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(300);
        let height = text.split("ارتفاع").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(200);
        let color = self.extract_color(text);
        
        let id = self.gui_engine.create_container(x, y, width, height, color);
        Ok(format!("حاوية({}، {}، {}، {}) // {}", x, y, width, height, id))
    }
    
    fn parse_dropdown(&mut self, text: &str) -> Result<String, String> {
        let x = text.split("س").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(50);
        let y = text.split("ص").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(100);
        let width = text.split("عرض").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(150);
        
        let id = self.gui_engine.create_dropdown(x, y, width, vec!["خيار 1", "خيار 2", "خيار 3"]);
        Ok(format!("قائمة_منسدلة({}، {}، {}) // {}", x, y, width, id))
    }
    
    fn parse_checkbox(&mut self, text: &str) -> Result<String, String> {
        let checkbox_text = text.split("نص").nth(1).map(|s| s.trim()).unwrap_or("خيار");
        let x = text.split("س").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(50);
        let y = text.split("ص").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(100);
        
        let id = self.gui_engine.create_checkbox(checkbox_text, x, y);
        Ok(format!("مربع_اختيار(\"{}\"، {}، {}) // {}", checkbox_text, x, y, id))
    }
    
    fn parse_slider(&mut self, text: &str) -> Result<String, String> {
        let x = text.split("س").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(50);
        let y = text.split("ص").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(100);
        let width = text.split("عرض").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(200);
        
        let id = self.gui_engine.create_slider(x, y, width, 0.0, 100.0);
        Ok(format!("منزلق({}، {}، {}) // {}", x, y, width, id))
    }
    
    fn parse_progress(&mut self, text: &str) -> Result<String, String> {
        let x = text.split("س").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(50);
        let y = text.split("ص").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(100);
        let width = text.split("عرض").nth(1).and_then(|s| s.split_whitespace().next()).and_then(|s| s.parse().ok()).unwrap_or(200);
        
        let id = self.gui_engine.create_progressbar(x, y, width);
        Ok(format!("شريط_تقدم({}، {}، {}) // {}", x, y, width, id))
    }
    
    fn extract_color(&self, text: &str) -> Option<GUIColor> {
        let lower = text.to_lowercase();
        if lower.contains("أحمر") || lower.contains("احمر") { Some(GUIColor::red()) }
        else if lower.contains("أخضر") || lower.contains("اخضر") { Some(GUIColor::green()) }
        else if lower.contains("أزرق") || lower.contains("ازرق") { Some(GUIColor::blue()) }
        else if lower.contains("أصفر") || lower.contains("اصفر") { Some(GUIColor::yellow()) }
        else if lower.contains("برتقالي") { Some(GUIColor::orange()) }
        else if lower.contains("وردي") { Some(GUIColor::pink()) }
        else if lower.contains("رمادي") { Some(GUIColor::gray()) }
        else { None }
    }
    
    fn extract_animation(&self, text: &str) -> Option<GUIAnimation> {
        let lower = text.to_lowercase();
        if lower.contains("تلاشي") || lower.contains("ظهور") { Some(GUIAnimation::new(AnimationType::FadeIn, 500)) }
        else if lower.contains("انزلاق") { Some(GUIAnimation::new(AnimationType::SlideIn, 500)) }
        else if lower.contains("تكبير") { Some(GUIAnimation::new(AnimationType::ScaleUp, 300)) }
        else if lower.contains("دوران") { Some(GUIAnimation::new(AnimationType::Rotate, 1000)) }
        else if lower.contains("اهتزاز") { Some(GUIAnimation::new(AnimationType::Shake, 500)) }
        else if lower.contains("نط") || lower.contains("قفز") { Some(GUIAnimation::new(AnimationType::Bounce, 500)) }
        else if lower.contains("نبض") { Some(GUIAnimation::new(AnimationType::Pulse, 1000)) }
        else if lower.contains("وميض") { Some(GUIAnimation::new(AnimationType::Blink, 1000)) }
        else { None }
    }
    
    pub fn get_html(&self) -> String { self.gui_engine.to_html() }
    pub fn get_engine(&self) -> &GUIEngine { &self.gui_engine }
    pub fn get_engine_mut(&mut self) -> &mut GUIEngine { &mut self.gui_engine }
}

impl Default for VibeGUIEngine {
    fn default() -> Self { Self::new() }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_window() {
        let mut engine = GUIEngine::new();
        let id = engine.create_window("مشروعي", 800, 600);
        assert!(id.starts_with("window_"));
    }

    #[test]
    fn test_create_button() {
        let mut engine = GUIEngine::new();
        engine.create_window("test", 400, 300);
        let id = engine.create_button("تأكيد", 50, 50);
        assert!(id.starts_with("btn_"));
    }

    #[test]
    fn test_color_from_arabic() {
        assert!(GUIColor::from_arabic("أحمر").is_some());
        assert!(GUIColor::from_arabic("أخضر").is_some());
    }

    #[test]
    fn test_vibe_gui() {
        let mut engine = VibeGUIEngine::new();
        let result = engine.text_to_gui("أنشئ نافذة عنوانها مشروعي");
        assert!(result.is_ok());
    }

    #[test]
    fn test_html_generation() {
        let mut engine = VibeGUIEngine::new();
        engine.text_to_gui("أنشئ نافذة عنوانها اختبار").unwrap();
        engine.text_to_gui("أضف زر مكتوب عليه موافق").unwrap();
        let html = engine.get_html();
        assert!(html.contains("اختبار"));
        assert!(html.contains("موافق"));
    }
}
