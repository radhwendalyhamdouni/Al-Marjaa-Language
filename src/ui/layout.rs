// ═══════════════════════════════════════════════════════════════════════════════
// نظام التخطيط التلقائي - Automatic Layout System
// ═══════════════════════════════════════════════════════════════════════════════
// يدعم: Row, Column, Grid, Flexbox
// ═══════════════════════════════════════════════════════════════════════════════

use crate::ui::types::*;

// ═══════════════════════════════════════════════════════════════════════════════
// أنواع التخطيط
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع التخطيط
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutKind {
    Row,
    Column,
    Grid,
    Flex,
    Stack,
    Wrap,
}

impl Default for LayoutKind {
    fn default() -> Self {
        Self::Column
    }
}

/// اتجاه التخطيط
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

impl Default for LayoutDirection {
    fn default() -> Self {
        Self::RightToLeft // للعربية
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Flexbox
// ═══════════════════════════════════════════════════════════════════════════════

/// اتجاه Flex
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl Default for FlexDirection {
    fn default() -> Self {
        Self::Row
    }
}

/// التفاف Flex
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> Self {
        Self::NoWrap
    }
}

/// تبرير المحتوى
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for JustifyContent {
    fn default() -> Self {
        Self::Start
    }
}

/// محاذاة العناصر
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

impl Default for AlignItems {
    fn default() -> Self {
        Self::Stretch
    }
}

/// محاذاة المحتوى
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignContent {
    Start,
    End,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

impl Default for AlignContent {
    fn default() -> Self {
        Self::Stretch
    }
}

/// محاذاة الذات
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignSelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

impl Default for AlignSelf {
    fn default() -> Self {
        Self::Auto
    }
}

/// حجم الفجوة
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GapSize {
    pub row: f32,
    pub column: f32,
}

impl GapSize {
    pub fn new(row: f32, column: f32) -> Self {
        Self { row, column }
    }
    
    pub fn all(value: f32) -> Self {
        Self::new(value, value)
    }
    
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Default for GapSize {
    fn default() -> Self {
        Self::zero()
    }
}

/// الفجوة
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gap {
    pub main: f32,
    pub cross: f32,
}

impl Gap {
    pub fn new(main: f32, cross: f32) -> Self {
        Self { main, cross }
    }
    
    pub fn uniform(value: f32) -> Self {
        Self::new(value, value)
    }
    
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Default for Gap {
    fn default() -> Self {
        Self::zero()
    }
}

/// عنصر Flex
#[derive(Debug, Clone, PartialEq)]
pub struct FlexItem {
    /// معرف العنصر
    pub id: String,
    /// معامل النمو
    pub grow: f32,
    /// معامل الانكماش
    pub shrink: f32,
    /// الحجم الأساسي
    pub basis: Option<UnitValue>,
    /// محاذاة الذات
    pub align_self: AlignSelf,
    /// الترتيب
    pub order: i32,
    /// الحجم المحسوب
    pub computed_rect: UIRect,
}

impl FlexItem {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            grow: 0.0,
            shrink: 1.0,
            basis: None,
            align_self: AlignSelf::Auto,
            order: 0,
            computed_rect: UIRect::default(),
        }
    }
    
    pub fn grow(mut self, value: f32) -> Self {
        self.grow = value;
        self
    }
    
    pub fn shrink(mut self, value: f32) -> Self {
        self.shrink = value;
        self
    }
    
    pub fn basis(mut self, value: UnitValue) -> Self {
        self.basis = Some(value);
        self
    }
    
    pub fn order(mut self, value: i32) -> Self {
        self.order = value;
        self
    }
}

/// حاوية Flex
#[derive(Debug, Clone, PartialEq)]
pub struct FlexContainer {
    /// المعرف
    pub id: String,
    /// الاتجاه
    pub direction: FlexDirection,
    /// الالتفاف
    pub wrap: FlexWrap,
    /// تبرير المحتوى
    pub justify_content: JustifyContent,
    /// محاذاة العناصر
    pub align_items: AlignItems,
    /// محاذاة المحتوى
    pub align_content: AlignContent,
    /// الفجوة
    pub gap: Gap,
    /// العناصر
    pub items: Vec<FlexItem>,
    /// الحجم
    pub size: UISize,
    /// المستطيل المحسوب
    pub computed_rect: UIRect,
}

impl FlexContainer {
    pub fn new() -> Self {
        Self {
            id: format!("flex_{}", uuid()),
            direction: FlexDirection::Row,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            align_content: AlignContent::Stretch,
            gap: Gap::zero(),
            items: Vec::new(),
            size: UISize::auto(),
            computed_rect: UIRect::default(),
        }
    }
    
    pub fn row() -> Self {
        Self::new()
    }
    
    pub fn column() -> Self {
        let mut container = Self::new();
        container.direction = FlexDirection::Column;
        container
    }
    
    pub fn direction(mut self, direction: FlexDirection) -> Self {
        self.direction = direction;
        self
    }
    
    pub fn wrap(mut self, wrap: FlexWrap) -> Self {
        self.wrap = wrap;
        self
    }
    
    pub fn justify(mut self, justify: JustifyContent) -> Self {
        self.justify_content = justify;
        self
    }
    
    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.align_items = align;
        self
    }
    
    pub fn gap(mut self, main: f32, cross: f32) -> Self {
        self.gap = Gap::new(main, cross);
        self
    }
    
    pub fn add_item(mut self, item: FlexItem) -> Self {
        self.items.push(item);
        self
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        let direction = match self.direction {
            FlexDirection::Row => "row",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::Column => "column",
            FlexDirection::ColumnReverse => "column-reverse",
        };
        
        let wrap = match self.wrap {
            FlexWrap::NoWrap => "nowrap",
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse => "wrap-reverse",
        };
        
        let justify = match self.justify_content {
            JustifyContent::Start => "flex-start",
            JustifyContent::End => "flex-end",
            JustifyContent::Center => "center",
            JustifyContent::SpaceBetween => "space-between",
            JustifyContent::SpaceAround => "space-around",
            JustifyContent::SpaceEvenly => "space-evenly",
        };
        
        let align = match self.align_items {
            AlignItems::Start => "flex-start",
            AlignItems::End => "flex-end",
            AlignItems::Center => "center",
            AlignItems::Stretch => "stretch",
            AlignItems::Baseline => "baseline",
        };
        
        format!(
            "display: flex; flex-direction: {}; flex-wrap: {}; justify-content: {}; align-items: {}; gap: {}px {};",
            direction, wrap, justify, align, self.gap.main, self.gap.cross
        )
    }
}

impl Default for FlexContainer {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Grid
// ═══════════════════════════════════════════════════════════════════════════════

/// مسار الشبكة
#[derive(Debug, Clone, PartialEq)]
pub enum GridTrack {
    Auto,
    Px(f32),
    Fr(f32),
    Percent(f32),
    MinContent,
    MaxContent,
    MinMax(Box<GridTrack>, Box<GridTrack>),
}

impl GridTrack {
    pub fn auto() -> Self {
        Self::Auto
    }
    
    pub fn px(value: f32) -> Self {
        Self::Px(value)
    }
    
    pub fn fr(value: f32) -> Self {
        Self::Fr(value)
    }
    
    pub fn percent(value: f32) -> Self {
        Self::Percent(value)
    }
    
    pub fn min_content() -> Self {
        Self::MinContent
    }
    
    pub fn max_content() -> Self {
        Self::MaxContent
    }
    
    pub fn min_max(min: GridTrack, max: GridTrack) -> Self {
        Self::MinMax(Box::new(min), Box::new(max))
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        match self {
            Self::Auto => "auto".to_string(),
            Self::Px(v) => format!("{}px", v),
            Self::Fr(v) => format!("{}fr", v),
            Self::Percent(v) => format!("{}%", v),
            Self::MinContent => "min-content".to_string(),
            Self::MaxContent => "max-content".to_string(),
            Self::MinMax(min, max) => format!("minmax({}, {})", min.to_css(), max.to_css()),
        }
    }
}

/// موضع الشبكة
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridPlacement {
    pub row_start: i32,
    pub row_end: i32,
    pub column_start: i32,
    pub column_end: i32,
}

impl GridPlacement {
    pub fn new(row: i32, column: i32) -> Self {
        Self {
            row_start: row,
            row_end: row + 1,
            column_start: column,
            column_end: column + 1,
        }
    }
    
    pub fn span(row_span: i32, column_span: i32) -> Self {
        Self {
            row_start: 1,
            row_end: row_span + 1,
            column_start: 1,
            column_end: column_span + 1,
        }
    }
    
    pub fn area(row_start: i32, row_end: i32, col_start: i32, col_end: i32) -> Self {
        Self {
            row_start,
            row_end,
            column_start: col_start,
            column_end: col_end,
        }
    }
}

impl Default for GridPlacement {
    fn default() -> Self {
        Self::new(1, 1)
    }
}

/// شبكة
#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    /// المعرف
    pub id: String,
    /// الأعمدة
    pub columns: Vec<GridTrack>,
    /// الصفوف
    pub rows: Vec<GridTrack>,
    /// الفجوة
    pub gap: GapSize,
    /// تبريط المحتوى
    pub justify_items: JustifyContent,
    /// محاذاة المحتوى
    pub align_items: AlignItems,
    /// الحجم
    pub size: UISize,
    /// المستطيل المحسوب
    pub computed_rect: UIRect,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            id: format!("grid_{}", uuid()),
            columns: vec![GridTrack::fr(1.0)],
            rows: vec![GridTrack::auto()],
            gap: GapSize::zero(),
            justify_items: JustifyContent::Start,
            align_items: AlignItems::Stretch,
            size: UISize::auto(),
            computed_rect: UIRect::default(),
        }
    }
    
    pub fn with_columns(columns: Vec<GridTrack>) -> Self {
        Self {
            columns,
            ..Self::new()
        }
    }
    
    pub fn columns(mut self, columns: Vec<GridTrack>) -> Self {
        self.columns = columns;
        self
    }
    
    pub fn rows(mut self, rows: Vec<GridTrack>) -> Self {
        self.rows = rows;
        self
    }
    
    pub fn gap(mut self, row: f32, column: f32) -> Self {
        self.gap = GapSize::new(row, column);
        self
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        let columns: String = self.columns.iter()
            .map(|t| t.to_css())
            .collect::<Vec<_>>()
            .join(" ");
        
        let rows: String = self.rows.iter()
            .map(|t| t.to_css())
            .collect::<Vec<_>>()
            .join(" ");
        
        format!(
            "display: grid; grid-template-columns: {}; grid-template-rows: {}; gap: {}px {};",
            columns, rows, self.gap.row, self.gap.column
        )
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Row و Column
// ═══════════════════════════════════════════════════════════════════════════════

/// صف أفقي
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    /// حاوية Flex
    pub flex: FlexContainer,
}

impl Row {
    pub fn new() -> Self {
        Self {
            flex: FlexContainer::row(),
        }
    }
    
    pub fn gap(mut self, value: f32) -> Self {
        self.flex.gap = Gap::new(value, 0.0);
        self
    }
    
    pub fn justify(mut self, justify: JustifyContent) -> Self {
        self.flex.justify_content = justify;
        self
    }
    
    pub fn align(mut self, align: AlignItems) -> Self {
        self.flex.align_items = align;
        self
    }
    
    pub fn wrap(mut self) -> Self {
        self.flex.wrap = FlexWrap::Wrap;
        self
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        self.flex.to_css()
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

/// عمود عمودي
#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    /// حاوية Flex
    pub flex: FlexContainer,
}

impl Column {
    pub fn new() -> Self {
        Self {
            flex: FlexContainer::column(),
        }
    }
    
    pub fn gap(mut self, value: f32) -> Self {
        self.flex.gap = Gap::new(0.0, value);
        self
    }
    
    pub fn justify(mut self, justify: JustifyContent) -> Self {
        self.flex.justify_content = justify;
        self
    }
    
    pub fn align(mut self, align: AlignItems) -> Self {
        self.flex.align_items = align;
        self
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        self.flex.to_css()
    }
}

impl Default for Column {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// التخطيط
// ═══════════════════════════════════════════════════════════════════════════════

/// تخطيط
#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    /// المعرف
    pub id: String,
    /// النوع
    pub kind: LayoutKind,
    /// الاتجاه
    pub direction: LayoutDirection,
    /// Flex
    pub flex: Option<FlexContainer>,
    /// Grid
    pub grid: Option<Grid>,
    /// الحجم
    pub size: UISize,
    /// الهوامش
    pub margin: UIMargin,
    /// الحشو
    pub padding: UIPadding,
    /// المستطيل المحسوب
    pub computed_rect: UIRect,
}

impl Layout {
    pub fn new(kind: LayoutKind) -> Self {
        Self {
            id: format!("layout_{}", uuid()),
            kind,
            direction: LayoutDirection::RightToLeft,
            flex: None,
            grid: None,
            size: UISize::auto(),
            margin: UIMargin::zero(),
            padding: UIPadding::zero(),
            computed_rect: UIRect::default(),
        }
    }
    
    pub fn row() -> Self {
        let mut layout = Self::new(LayoutKind::Row);
        layout.flex = Some(FlexContainer::row());
        layout
    }
    
    pub fn column() -> Self {
        let mut layout = Self::new(LayoutKind::Column);
        layout.flex = Some(FlexContainer::column());
        layout
    }
    
    pub fn grid(columns: Vec<GridTrack>) -> Self {
        let mut layout = Self::new(LayoutKind::Grid);
        layout.grid = Some(Grid::with_columns(columns));
        layout
    }
    
    pub fn flex() -> Self {
        let mut layout = Self::new(LayoutKind::Flex);
        layout.flex = Some(FlexContainer::new());
        layout
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        let mut css_parts = Vec::new();
        
        if let Some(ref flex) = self.flex {
            css_parts.push(flex.to_css());
        }
        
        if let Some(ref grid) = self.grid {
            css_parts.push(grid.to_css());
        }
        
        css_parts.push(self.margin.to_css());
        css_parts.push(self.padding.to_css());
        
        css_parts.join("; ")
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// محرك التخطيط
// ═══════════════════════════════════════════════════════════════════════════════

/// نتيجة التخطيط
#[derive(Debug, Clone, PartialEq)]
pub struct LayoutResult {
    /// المستطيلات المحسوبة
    pub rects: HashMap<String, UIRect>,
    /// الحجم الكلي
    pub total_size: UISize,
}

/// محرك التخطيط
#[derive(Debug)]
pub struct LayoutEngine {
    /// التخطيطات
    layouts: HashMap<String, Layout>,
    /// النتائج المحسوبة
    results: HashMap<String, LayoutResult>,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            layouts: HashMap::new(),
            results: HashMap::new(),
        }
    }
    
    /// إضافة تخطيط
    pub fn add_layout(&mut self, layout: Layout) {
        self.layouts.insert(layout.id.clone(), layout);
    }
    
    /// حساب التخطيط
    pub fn calculate(&mut self, id: &str, available_width: f32, available_height: f32) -> Option<LayoutResult> {
        let layout = self.layouts.get(id)?;
        
        // حساب التخطيط بناءً على النوع
        let result = match layout.kind {
            LayoutKind::Row | LayoutKind::Column | LayoutKind::Flex => {
                self.calculate_flex_layout(layout, available_width, available_height)
            }
            LayoutKind::Grid => {
                self.calculate_grid_layout(layout, available_width, available_height)
            }
            LayoutKind::Stack => {
                self.calculate_stack_layout(layout, available_width, available_height)
            }
            LayoutKind::Wrap => {
                self.calculate_wrap_layout(layout, available_width, available_height)
            }
        };
        
        self.results.insert(id.to_string(), result.clone());
        Some(result)
    }
    
    /// حساب تخطيط Flex
    fn calculate_flex_layout(&self, layout: &Layout, width: f32, height: f32) -> LayoutResult {
        let mut rects = HashMap::new();
        
        if let Some(ref flex) = layout.flex {
            let is_row = matches!(flex.direction, FlexDirection::Row | FlexDirection::RowReverse);
            let main_size = if is_row { width } else { height };
            let cross_size = if is_row { height } else { width };
            
            // حساب الأحجام الأساسية
            let mut current_pos = 0.0;
            let gap = flex.gap.main;
            
            for item in &flex.items {
                let item_main_size = item.basis
                    .as_ref()
                    .map(|b| b.value)
                    .unwrap_or(100.0); // حجم افتراضي
                
                let rect = if is_row {
                    UIRect::new(current_pos, 0.0, item_main_size, cross_size)
                } else {
                    UIRect::new(0.0, current_pos, cross_size, item_main_size)
                };
                
                rects.insert(item.id.clone(), rect);
                current_pos += item_main_size + gap;
            }
        }
        
        LayoutResult {
            rects,
            total_size: UISize::new(width, height),
        }
    }
    
    /// حساب تخطيط Grid
    fn calculate_grid_layout(&self, layout: &Layout, width: f32, height: f32) -> LayoutResult {
        let mut rects = HashMap::new();
        
        if let Some(ref grid) = layout.grid {
            // حساب أحجام الأعمدة
            let col_count = grid.columns.len().max(1);
            let col_width = width / col_count as f32;
            
            // حساب أحجام الصفوف
            let row_count = grid.rows.len().max(1);
            let row_height = height / row_count as f32;
            
            // إنشاء المستطيلات لكل خلية
            for row in 0..row_count {
                for col in 0..col_count {
                    let id = format!("cell_{}_{}", row, col);
                    let rect = UIRect::new(
                        col as f32 * col_width,
                        row as f32 * row_height,
                        col_width,
                        row_height,
                    );
                    rects.insert(id, rect);
                }
            }
        }
        
        LayoutResult {
            rects,
            total_size: UISize::new(width, height),
        }
    }
    
    /// حساب تخطيط Stack
    fn calculate_stack_layout(&self, layout: &Layout, width: f32, height: f32) -> LayoutResult {
        let mut rects = HashMap::new();
        rects.insert("stack".to_string(), UIRect::new(0.0, 0.0, width, height));
        
        LayoutResult {
            rects,
            total_size: UISize::new(width, height),
        }
    }
    
    /// حساب تخطيط Wrap
    fn calculate_wrap_layout(&self, layout: &Layout, width: f32, height: f32) -> LayoutResult {
        let mut rects = HashMap::new();
        rects.insert("wrap".to_string(), UIRect::new(0.0, 0.0, width, height));
        
        LayoutResult {
            rects,
            total_size: UISize::new(width, height),
        }
    }
    
    /// تحديث المحرك
    pub fn update(&mut self) -> Result<(), String> {
        Ok(())
    }
    
    /// الحصول على نتيجة
    pub fn get_result(&self, id: &str) -> Option<&LayoutResult> {
        self.results.get(id)
    }
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الدوال المساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// حساب التخطيط
pub fn calculate_layout(layout: &Layout, width: f32, height: f32) -> LayoutResult {
    let mut engine = LayoutEngine::new();
    let id = layout.id.clone();
    engine.add_layout(layout.clone());
    engine.calculate(&id, width, height).unwrap()
}

/// إنشاء معرف فريد
fn uuid() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
