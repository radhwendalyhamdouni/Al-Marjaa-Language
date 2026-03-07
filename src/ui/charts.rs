// ═══════════════════════════════════════════════════════════════════════════════
// الرسوم البيانية - Charts
// ═══════════════════════════════════════════════════════════════════════════════

use crate::ui::types::*;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
// أنواع الرسوم البيانية
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع الرسم البياني
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChartKind {
    Line,
    Bar,
    Pie,
    Area,
    Scatter,
    Radar,
    Gauge,
    Heatmap,
    Candlestick,
    Donut,
    Bubble,
    Treemap,
}

impl Default for ChartKind {
    fn default() -> Self {
        Self::Bar
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// بيانات الرسم البياني
// ═══════════════════════════════════════════════════════════════════════════════

/// نقطة البيانات
#[derive(Debug, Clone, PartialEq)]
pub struct ChartPoint {
    /// التسمية
    pub label: String,
    /// القيمة
    pub value: f64,
    /// القيمة الثانية (للرسوم المركبة)
    pub value2: Option<f64>,
    /// اللون
    pub color: Option<UIColor>,
}

impl ChartPoint {
    pub fn new(label: &str, value: f64) -> Self {
        Self {
            label: label.to_string(),
            value,
            value2: None,
            color: None,
        }
    }
    
    pub fn with_color(mut self, color: UIColor) -> Self {
        self.color = Some(color);
        self
    }
}

/// سلسلة البيانات
#[derive(Debug, Clone, PartialEq)]
pub struct ChartSeries {
    /// اسم السلسلة
    pub name: String,
    /// النقاط
    pub points: Vec<ChartPoint>,
    /// اللون
    pub color: UIColor,
    /// نوع الرسم (للرسوم المختلطة)
    pub kind: Option<ChartKind>,
}

impl ChartSeries {
    pub fn new(name: &str, points: Vec<ChartPoint>) -> Self {
        Self {
            name: name.to_string(),
            points,
            color: UIColor::blue(),
            kind: None,
        }
    }
    
    pub fn color(mut self, color: UIColor) -> Self {
        self.color = color;
        self
    }
}

/// بيانات الرسم البياني
#[derive(Debug, Clone, PartialEq)]
pub struct ChartData {
    /// السلاسل
    pub series: Vec<ChartSeries>,
    /// التسميات
    pub labels: Vec<String>,
}

impl ChartData {
    pub fn new() -> Self {
        Self {
            series: Vec::new(),
            labels: Vec::new(),
        }
    }
    
    pub fn with_series(mut self, series: ChartSeries) -> Self {
        self.series.push(series);
        self
    }
    
    pub fn with_labels(mut self, labels: Vec<&str>) -> Self {
        self.labels = labels.iter().map(|s| s.to_string()).collect();
        self
    }
}

impl Default for ChartData {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// محاور الرسم البياني
// ═══════════════════════════════════════════════════════════════════════════════

/// محور الرسم البياني
#[derive(Debug, Clone, PartialEq)]
pub struct ChartAxis {
    /// العنوان
    pub title: String,
    /// مرئي
    pub visible: bool,
    /// الحد الأدنى
    pub min: Option<f64>,
    /// الحد الأقصى
    pub max: Option<f64>,
    /// تنسيق التسميات
    pub label_format: Option<String>,
}

impl ChartAxis {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            visible: true,
            min: None,
            max: None,
            label_format: None,
        }
    }
    
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }
    
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }
}

impl Default for ChartAxis {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// عناصر إضافية
// ═══════════════════════════════════════════════════════════════════════════════

/// وسيلة الإيضاح
#[derive(Debug, Clone, PartialEq)]
pub struct ChartLegend {
    /// مرئية
    pub visible: bool,
    /// الموضع
    pub position: LegendPosition,
}

/// موضع وسيلة الإيضاح
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LegendPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl Default for ChartLegend {
    fn default() -> Self {
        Self {
            visible: true,
            position: LegendPosition::Bottom,
        }
    }
}

/// التلميح
#[derive(Debug, Clone, PartialEq)]
pub struct ChartTooltip {
    /// مفعل
    pub enabled: bool,
    /// القالب
    pub template: Option<String>,
}

impl Default for ChartTooltip {
    fn default() -> Self {
        Self {
            enabled: true,
            template: None,
        }
    }
}

/// لون الرسم البياني
pub type ChartColor = UIColor;

/// ثيم الرسم البياني
#[derive(Debug, Clone, PartialEq)]
pub struct ChartTheme {
    /// الألوان
    pub colors: Vec<UIColor>,
    /// لون الخلفية
    pub background: UIColor,
    /// لون النص
    pub text_color: UIColor,
    /// لون الشبكة
    pub grid_color: UIColor,
}

impl ChartTheme {
    pub fn default_theme() -> Self {
        Self {
            colors: vec![
                UIColor::hex("#2196F3"),
                UIColor::hex("#FF9800"),
                UIColor::hex("#4CAF50"),
                UIColor::hex("#E91E63"),
                UIColor::hex("#9C27B0"),
                UIColor::hex("#00BCD4"),
            ],
            background: UIColor::white(),
            text_color: UIColor::dark_gray(),
            grid_color: UIColor::light_gray(),
        }
    }
}

impl Default for ChartTheme {
    fn default() -> Self {
        Self::default_theme()
    }
}

/// تنسيق الرسم البياني
#[derive(Debug, Clone, PartialEq)]
pub struct ChartStyle {
    /// لون الخلفية
    pub background: Option<UIColor>,
    /// عرض الخط
    pub line_width: f32,
    /// عرض الحدود
    pub border_width: f32,
    /// نصف قطر الزوايا
    pub border_radius: f32,
}

impl Default for ChartStyle {
    fn default() -> Self {
        Self {
            background: None,
            line_width: 2.0,
            border_width: 1.0,
            border_radius: 4.0,
        }
    }
}

/// رسوم متحركة للرسم البياني
#[derive(Debug, Clone, PartialEq)]
pub struct ChartAnimation {
    /// مفعلة
    pub enabled: bool,
    /// المدة بالميلي ثانية
    pub duration: u32,
    /// نوع التخفيف
    pub easing: String,
}

impl Default for ChartAnimation {
    fn default() -> Self {
        Self {
            enabled: true,
            duration: 1000,
            easing: "easeInOut".to_string(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// تكوين الرسم البياني
// ═══════════════════════════════════════════════════════════════════════════════

/// تكوين الرسم البياني
#[derive(Debug, Clone, PartialEq)]
pub struct ChartConfig {
    /// العنوان
    pub title: String,
    /// العرض
    pub width: u32,
    /// الارتفاع
    pub height: u32,
    /// المحور الأفقي
    pub x_axis: ChartAxis,
    /// المحور العمودي
    pub y_axis: ChartAxis,
    /// وسيلة الإيضاح
    pub legend: ChartLegend,
    /// التلميح
    pub tooltip: ChartTooltip,
    /// الثيم
    pub theme: ChartTheme,
    /// التنسيق
    pub style: ChartStyle,
    /// الرسوم المتحركة
    pub animation: ChartAnimation,
}

impl ChartConfig {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            width: 600,
            height: 400,
            x_axis: ChartAxis::new(),
            y_axis: ChartAxis::new(),
            legend: ChartLegend::default(),
            tooltip: ChartTooltip::default(),
            theme: ChartTheme::default(),
            style: ChartStyle::default(),
            animation: ChartAnimation::default(),
        }
    }
    
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }
    
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

impl Default for ChartConfig {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الرسم البياني
// ═══════════════════════════════════════════════════════════════════════════════

/// الرسم البياني
#[derive(Debug, Clone, PartialEq)]
pub struct Chart {
    /// المعرف
    pub id: String,
    /// النوع
    pub kind: ChartKind,
    /// البيانات
    pub data: ChartData,
    /// التكوين
    pub config: ChartConfig,
}

impl Chart {
    pub fn new(kind: ChartKind) -> Self {
        Self {
            id: format!("chart_{}", uuid()),
            kind,
            data: ChartData::new(),
            config: ChartConfig::new(),
        }
    }
    
    pub fn with_data(mut self, data: ChartData) -> Self {
        self.data = data;
        self
    }
    
    pub fn with_config(mut self, config: ChartConfig) -> Self {
        self.config = config;
        self
    }
    
    /// تحويل إلى SVG
    pub fn to_svg(&self) -> String {
        let width = self.config.width;
        let height = self.config.height;
        
        let mut svg = format!(
            r#"<svg id="{}" width="{}" height="{}" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            self.id, width, height, width, height
        );
        
        // الخلفية
        svg.push_str(&format!(
            r#"<rect width="{}" height="{}" fill="{}"/>"#,
            width, height, self.config.theme.background.to_css()
        ));
        
        // العنوان
        if !self.config.title.is_empty() {
            svg.push_str(&format!(
                r#"<text x="{}" y="30" text-anchor="middle" fill="{}" font-size="18" font-weight="bold">{}</text>"#,
                width / 2, self.config.theme.text_color.to_css(), self.config.title
            ));
        }
        
        // الرسم حسب النوع
        match self.kind {
            ChartKind::Bar => self.render_bars(&mut svg),
            ChartKind::Line => self.render_lines(&mut svg),
            ChartKind::Pie => self.render_pie(&mut svg),
            ChartKind::Area => self.render_area(&mut svg),
            _ => {}
        }
        
        svg.push_str("</svg>");
        svg
    }
    
    fn render_bars(&self, svg: &mut String) {
        let width = self.config.width as f32;
        let height = self.config.height as f32;
        let padding = 60.0;
        
        if let Some(series) = self.data.series.first() {
            let bar_count = series.points.len();
            if bar_count == 0 { return; }
            
            let max_value = series.points.iter().map(|p| p.value).fold(0.0, f64::max);
            let bar_width = (width - padding * 2.0) / bar_count as f32 * 0.8;
            let bar_gap = (width - padding * 2.0) / bar_count as f32 * 0.2;
            
            for (i, point) in series.points.iter().enumerate() {
                let x = padding + i as f32 * (bar_width + bar_gap);
                let bar_height = (point.value as f32 / max_value as f32) * (height - padding * 2.0);
                let y = height - padding - bar_height;
                
                let color = point.color.as_ref().unwrap_or(&series.color);
                
                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" rx="{}"/>"#,
                    x, y, bar_width, bar_height, color.to_css(), self.config.style.border_radius
                ));
                
                // التسمية
                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" text-anchor="middle" fill="{}" font-size="12">{}</text>"#,
                    x + bar_width / 2.0, height - 20, self.config.theme.text_color.to_css(), point.label
                ));
            }
        }
    }
    
    fn render_lines(&self, svg: &mut String) {
        let width = self.config.width as f32;
        let height = self.config.height as f32;
        let padding = 60.0;
        
        for series in &self.data.series {
            let point_count = series.points.len();
            if point_count < 2 { continue; }
            
            let max_value = series.points.iter().map(|p| p.value).fold(0.0, f64::max);
            let mut path = String::from("M ");
            
            for (i, point) in series.points.iter().enumerate() {
                let x = padding + (i as f32 / (point_count - 1) as f32) * (width - padding * 2.0);
                let y = height - padding - (point.value as f32 / max_value as f32) * (height - padding * 2.0);
                
                path.push_str(&format!("{} {} ", x, y));
                
                if i < point_count - 1 {
                    path.push_str("L ");
                }
            }
            
            svg.push_str(&format!(
                r#"<path d="{}" fill="none" stroke="{}" stroke-width="{}"/>"#,
                path, series.color.to_css(), self.config.style.line_width
            ));
        }
    }
    
    fn render_pie(&self, svg: &mut String) {
        let width = self.config.width as f32;
        let height = self.config.height as f32;
        let center_x = width / 2.0;
        let center_y = height / 2.0;
        let radius = (width.min(height) / 2.0 - 40.0).max(10.0);
        
        if let Some(series) = self.data.series.first() {
            let total: f64 = series.points.iter().map(|p| p.value).sum();
            if total == 0.0 { return; }
            
            let mut current_angle = -std::f32::consts::PI / 2.0;
            
            for point in &series.points {
                let angle = (point.value as f32 / total as f32) * 2.0 * std::f32::consts::PI;
                let color = point.color.as_ref().unwrap_or(&series.color);
                
                let x1 = center_x + radius * current_angle.cos();
                let y1 = center_y + radius * current_angle.sin();
                let x2 = center_x + radius * (current_angle + angle).cos();
                let y2 = center_y + radius * (current_angle + angle).sin();
                
                let large_arc = if angle > std::f32::consts::PI { 1 } else { 0 };
                
                svg.push_str(&format!(
                    r#"<path d="M {} {} L {} {} A {} {} 0 {} 1 {} {} Z" fill="{}"/>"#,
                    center_x, center_y, x1, y1, radius, radius, large_arc, x2, y2, color.to_css()
                ));
                
                current_angle += angle;
            }
        }
    }
    
    fn render_area(&self, svg: &mut String) {
        let width = self.config.width as f32;
        let height = self.config.height as f32;
        let padding = 60.0;
        
        for series in &self.data.series {
            let point_count = series.points.len();
            if point_count < 2 { continue; }
            
            let max_value = series.points.iter().map(|p| p.value).fold(0.0, f64::max);
            let mut path = format!("M {} {} ", padding, height - padding);
            
            for (i, point) in series.points.iter().enumerate() {
                let x = padding + (i as f32 / (point_count - 1) as f32) * (width - padding * 2.0);
                let y = height - padding - (point.value as f32 / max_value as f32) * (height - padding * 2.0);
                path.push_str(&format!("L {} {} ", x, y));
            }
            
            path.push_str(&format!("L {} {} Z", width - padding, height - padding));
            
            let fill_color = series.color.with_alpha(0.3);
            svg.push_str(&format!(
                r#"<path d="{}" fill="{}" stroke="{}" stroke-width="{}"/>"#,
                path, fill_color.to_css(), series.color.to_css(), self.config.style.line_width
            ));
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// أنواع الرسوم البيانية المحددة
// ═══════════════════════════════════════════════════════════════════════════════

/// رسم خطي
pub type LineChart = Chart;

/// رسم أعمدة
pub type BarChart = Chart;

/// رسم دائري
pub type PieChart = Chart;

/// رسم مساحي
pub type AreaChart = Chart;

/// رسم نقطي
pub type ScatterChart = Chart;

/// رسم رادار
pub type RadarChart = Chart;

/// رسم قياس
pub type GaugeChart = Chart;

/// رسم خريطة حرارية
pub type HeatmapChart = Chart;

/// رسم شمعدان
pub type CandlestickChart = Chart;

// ═══════════════════════════════════════════════════════════════════════════════
// الدوال المساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء رسم بياني
pub fn create_chart(kind: ChartKind) -> Chart {
    Chart::new(kind)
}

/// تصيير رسم بياني
pub fn render_chart(chart: &Chart) -> String {
    chart.to_svg()
}

/// تحويل الرسم البياني إلى SVG
pub fn chart_to_svg(chart: &Chart) -> String {
    chart.to_svg()
}

/// إنشاء معرف فريد
fn uuid() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
