// ═══════════════════════════════════════════════════════════════════════════════
// وحدة واجهات المستخدم - UI Module
// ═══════════════════════════════════════════════════════════════════════════════
// نظام واجهات متكامل مع:
// - التخطيط التلقائي (Row/Column/Grid)
// - ربط البيانات التلقائي (Data Binding)
// - المكونات القابلة لإعادة الاستخدام
// - التصميم المتجاوب (Responsive)
// - نظام الثيمات (Themes)
// - الرسوم البيانية (Charts)
// - النوافذ المنبثقة (Modals)
// ═══════════════════════════════════════════════════════════════════════════════

pub mod types;
pub mod layout;
pub mod binding;
pub mod components;
pub mod responsive;
pub mod themes;
pub mod charts;
pub mod modals;
pub mod widgets;
pub mod animations;
pub mod events;
pub mod styling;

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير الأنواع الأساسية
// ═══════════════════════════════════════════════════════════════════════════════

pub use types::{
    UIElement, UIValue, UIColor, UIFont, UIMargin, UIPadding,
    UISize, UIPosition, UIRect, UIPoint, UITransform,
    UIEvent, UIEventHandler, UIState, UIContext,
    ComponentId, PropertyName, BindingPath,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير نظام التخطيط
// ═══════════════════════════════════════════════════════════════════════════════

pub use layout::{
    LayoutEngine, Layout, LayoutKind, LayoutDirection,
    Row, Column, Grid, GridTrack, GridPlacement,
    FlexContainer, FlexItem, FlexDirection, FlexWrap,
    JustifyContent, AlignItems, AlignContent, AlignSelf,
    Gap, GapSize, calculate_layout, LayoutResult,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير ربط البيانات
// ═══════════════════════════════════════════════════════════════════════════════

pub use binding::{
    BindingEngine, Binding, BindingKind, BindingMode,
    Observable, ObservableValue, ObservableCollection,
    Computed, ComputedValue, Watcher, WatcherCallback,
    BindingExpression, BindingResult, DataContext,
    bind, bind_two_way, bind_one_time, observe, computed,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير المكونات
// ═══════════════════════════════════════════════════════════════════════════════

pub use components::{
    Component, ComponentBase, ComponentBuilder, ComponentRegistry,
    Props, ComponentProps, ComponentState, ComponentLifecycle,
    Text, Button, TextField, TextArea, Checkbox, Radio,
    Select, SelectOption, Slider, ProgressBar, Spinner,
    Image, Icon, Container, Card, List, ListItem,
    Table, TableRow, TableCell, Form, FormField, Label,
    Divider, Spacer, Badge, Avatar, Tooltip, Popover,
    Tabs, Tab, Accordion, AccordionItem, Breadcrumb,
    Pagination, Stepper, Calendar, DatePicker, TimePicker,
    create_component, register_component, component_exists,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير التصميم المتجاوب
// ═══════════════════════════════════════════════════════════════════════════════

pub use responsive::{
    ResponsiveEngine, Breakpoint, Breakpoints, MediaQuery,
    DeviceType, Orientation, ScreenSize, ResponsiveValue,
    ResponsiveContainer, ResponsiveLayout, HideOn, ShowOn,
    responsive_value, match_breakpoint, get_device_type,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير نظام الثيمات
// ═══════════════════════════════════════════════════════════════════════════════

pub use themes::{
    Theme, ThemeColors, ThemeTypography, ThemeSpacing,
    ThemeShadows, ThemeBorders, ThemeConfig, ThemeManager,
    ThemeMode, ColorScheme, Typography, FontFamily,
    create_theme, load_theme, save_theme, apply_theme,
    default_light_theme, default_dark_theme, default_arabic_theme,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير الرسوم البيانية
// ═══════════════════════════════════════════════════════════════════════════════

pub use charts::{
    Chart, ChartKind, ChartData, ChartSeries, ChartPoint,
    ChartAxis, ChartLegend, ChartTooltip, ChartConfig,
    LineChart, BarChart, PieChart, AreaChart, ScatterChart,
    RadarChart, GaugeChart, HeatmapChart, CandlestickChart,
    ChartAnimation, ChartStyle, ChartColor, ChartTheme,
    create_chart, render_chart, chart_to_svg,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير النوافذ المنبثقة
// ═══════════════════════════════════════════════════════════════════════════════

pub use modals::{
    Modal, ModalKind, ModalConfig, ModalResult, ModalManager,
    Dialog, AlertDialog, ConfirmDialog, PromptDialog,
    Toast, ToastKind, ToastPosition, ToastManager,
    Popup, PopupPosition, PopupTrigger,
    Sheet, SheetSide, Drawer, DrawerSide,
    Notification, NotificationKind, NotificationManager,
    show_modal, show_alert, show_confirm, show_toast, show_notification,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير الأدوات المتقدمة
// ═══════════════════════════════════════════════════════════════════════════════

pub use widgets::{
    Widget, WidgetBuilder, WidgetState,
    DropDown, ComboBox, AutoComplete, MultiSelect,
    Rating, SliderRange, ColorPicker, FileUpload,
    RichTextEditor, CodeEditor, MarkdownViewer,
    Timeline, TreeView, TreeNode, VirtualList,
    Kanban, KanbanBoard, KanbanCard,
    Map, MapMarker, MapConfig,
    VideoPlayer, AudioPlayer, MediaPlayer,
    Carousel, SliderShow, Gallery,
    DataGrid, DataTable, InfiniteScroll,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير الرسوم المتحركة
// ═══════════════════════════════════════════════════════════════════════════════

pub use animations::{
    Animation, AnimationKind, AnimationConfig, AnimationManager,
    Transition, TransitionKind, TransitionConfig,
    Keyframe, Keyframes, Easing, EasingFunction,
    AnimationTimeline, AnimationState, AnimationEvent,
    animate, transition, create_animation, play_animation,
    FadeIn, FadeOut, SlideIn, SlideOut, ScaleIn, ScaleOut,
    RotateIn, RotateOut, BounceIn, BounceOut,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير الأحداث
// ═══════════════════════════════════════════════════════════════════════════════

pub use events::{
    EventManager, EventDispatcher, EventListener, EventHandler,
    MouseEvent, KeyboardEvent, FocusEvent, FormEvent,
    TouchEvent, GestureEvent, DragEvent, DropEvent,
    ScrollEvent, ResizeEvent, AnimationEvent as UIAnimationEvent,
    CustomEvent, EventPhase, EventResult,
    on_click, on_change, on_submit, on_hover, on_focus, on_blur,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير التنسيق
// ═══════════════════════════════════════════════════════════════════════════════

pub use styling::{
    Style, StyleBuilder, StyleProperty, StyleValue,
    StyleSheet, CSSRule, Selector, PseudoClass,
    Border, BorderStyle, BorderRadius, Shadow,
    Gradient, GradientKind, GradientStop,
    Transform, TransformKind, Filter, FilterKind,
    Background, Foreground, Decoration,
    style, css, class_name, inline_style,
};

// ═══════════════════════════════════════════════════════════════════════════════
// المحرك الرئيسي لواجهات المستخدم
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// محرك واجهات المستخدم الرئيسي
pub struct UIEngine {
    /// مدير الثيمات
    theme_manager: ThemeManager,
    /// محرك التخطيط
    layout_engine: LayoutEngine,
    /// محرك ربط البيانات
    binding_engine: BindingEngine,
    /// سجل المكونات
    component_registry: ComponentRegistry,
    /// مدير الأحداث
    event_manager: EventManager,
    /// مدير النوافذ المنبثقة
    modal_manager: ModalManager,
    /// مدير الرسوم المتحركة
    animation_manager: AnimationManager,
    /// المكون الجذر
    root: Option<Arc<RwLock<Box<dyn Component>>>>,
    /// حالة المحرك
    state: UIEngineState,
}

/// حالة محرك واجهات المستخدم
#[derive(Debug, Clone, PartialEq)]
pub enum UIEngineState {
    /// غير مهيأ
    Uninitialized,
    /// مهيأ
    Initialized,
    /// قيد التشغيل
    Running,
    /// متوقف مؤقتاً
    Paused,
    /// متوقف
    Stopped,
}

/// تكوين محرك واجهات المستخدم
#[derive(Debug, Clone)]
pub struct UIEngineConfig {
    /// الثيم الافتراضي
    pub default_theme: String,
    /// تفعيل التصميم المتجاوب
    pub responsive: bool,
    /// تفعيل الرسوم المتحركة
    pub animations: bool,
    /// تفعيل ربط البيانات
    pub data_binding: bool,
    /// تفعيل RTL
    pub rtl: bool,
    /// نقاط التوقف المخصصة
    pub custom_breakpoints: Option<Breakpoints>,
}

impl Default for UIEngineConfig {
    fn default() -> Self {
        Self {
            default_theme: "light".to_string(),
            responsive: true,
            animations: true,
            data_binding: true,
            rtl: true,
            custom_breakpoints: None,
        }
    }
}

impl UIEngine {
    /// إنشاء محرك جديد
    pub fn new() -> Self {
        Self::with_config(UIEngineConfig::default())
    }
    
    /// إنشاء محرك بتكوين مخصص
    pub fn with_config(config: UIEngineConfig) -> Self {
        let mut theme_manager = ThemeManager::new();
        
        // تحميل الثيم الافتراضي
        if config.default_theme == "dark" {
            theme_manager.set_theme(default_dark_theme());
        } else if config.default_theme == "arabic" {
            theme_manager.set_theme(default_arabic_theme());
        } else {
            theme_manager.set_theme(default_light_theme());
        }
        
        Self {
            theme_manager,
            layout_engine: LayoutEngine::new(),
            binding_engine: BindingEngine::new(),
            component_registry: ComponentRegistry::new(),
            event_manager: EventManager::new(),
            modal_manager: ModalManager::new(),
            animation_manager: AnimationManager::new(),
            root: None,
            state: UIEngineState::Initialized,
        }
    }
    
    /// تهيئة المحرك
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.state != UIEngineState::Uninitialized {
            return Err("المحرك مهيأ بالفعل".to_string());
        }
        
        self.state = UIEngineState::Initialized;
        Ok(())
    }
    
    /// تشغيل المحرك
    pub fn start(&mut self) -> Result<(), String> {
        if self.state == UIEngineState::Uninitialized {
            return Err("يجب تهيئة المحرك أولاً".to_string());
        }
        
        self.state = UIEngineState::Running;
        Ok(())
    }
    
    /// إيقاف المحرك مؤقتاً
    pub fn pause(&mut self) {
        self.state = UIEngineState::Paused;
    }
    
    /// استئناف المحرك
    pub fn resume(&mut self) {
        self.state = UIEngineState::Running;
    }
    
    /// إيقاف المحرك
    pub fn stop(&mut self) {
        self.state = UIEngineState::Stopped;
    }
    
    /// الحصول على حالة المحرك
    pub fn state(&self) -> &UIEngineState {
        &self.state
    }
    
    /// تعيين المكون الجذر
    pub fn set_root(&mut self, component: Box<dyn Component>) {
        self.root = Some(Arc::new(RwLock::new(component)));
    }
    
    /// الحصول على المكون الجذر
    pub fn root(&self) -> Option<Arc<RwLock<Box<dyn Component>>>> {
        self.root.clone()
    }
    
    /// تحديث المحرك
    pub fn update(&mut self, delta_time: f64) -> Result<(), String> {
        if self.state != UIEngineState::Running {
            return Ok(());
        }
        
        // تحديث التخطيط
        self.layout_engine.update()?;
        
        // تحديث ربط البيانات
        self.binding_engine.update()?;
        
        // تحديث الرسوم المتحركة
        self.animation_manager.update(delta_time)?;
        
        // تحديث النوافذ المنبثقة
        self.modal_manager.update()?;
        
        Ok(())
    }
    
    /// تصيير المحرك
    pub fn render(&self) -> Result<String, String> {
        if let Some(root) = &self.root {
            let root_lock = root.read().map_err(|e| e.to_string())?;
            root_lock.render()
        } else {
            Err("لا يوجد مكون جذر".to_string())
        }
    }
    
    /// معالجة حدث
    pub fn handle_event(&mut self, event: UIEvent) -> Result<EventResult, String> {
        self.event_manager.dispatch(event)
    }
    
    /// الحصول على مدير الثيمات
    pub fn theme_manager(&self) -> &ThemeManager {
        &self.theme_manager
    }
    
    /// الحصول على مدير الثيمات قابل للتعديل
    pub fn theme_manager_mut(&mut self) -> &mut ThemeManager {
        &mut self.theme_manager
    }
    
    /// الحصول على محرك التخطيط
    pub fn layout_engine(&self) -> &LayoutEngine {
        &self.layout_engine
    }
    
    /// الحصول على محرك ربط البيانات
    pub fn binding_engine(&self) -> &BindingEngine {
        &self.binding_engine
    }
    
    /// الحصول على سجل المكونات
    pub fn component_registry(&self) -> &ComponentRegistry {
        &self.component_registry
    }
    
    /// الحصول على مدير الأحداث
    pub fn event_manager(&self) -> &EventManager {
        &self.event_manager
    }
    
    /// الحصول على مدير النوافذ المنبثقة
    pub fn modal_manager(&self) -> &ModalManager {
        &self.modal_manager
    }
    
    /// الحصول على مدير الرسوم المتحركة
    pub fn animation_manager(&self) -> &AnimationManager {
        &self.animation_manager
    }
}

impl Default for UIEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الدوال المساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء محرك واجهات جديد
pub fn create_ui_engine() -> UIEngine {
    UIEngine::new()
}

/// إنشاء محرك واجهات بتكوين مخصص
pub fn create_ui_engine_with_config(config: UIEngineConfig) -> UIEngine {
    UIEngine::with_config(config)
}

/// بناء واجهة بسيطة
pub fn build_ui<F>(builder: F) -> Result<UIEngine, String>
where
    F: FnOnce(&mut UIEngine) -> Result<(), String>,
{
    let mut engine = UIEngine::new();
    builder(&mut engine)?;
    engine.start()?;
    Ok(engine)
}
