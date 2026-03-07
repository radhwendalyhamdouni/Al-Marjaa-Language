// ═══════════════════════════════════════════════════════════════════════════════
// لغة المرجع - Al-Marjaa Language
// ═══════════════════════════════════════════════════════════════════════════════
// © 2026 رضوان دالي حمدوني | RADHWEN DALY HAMDOUNI
// جميع الحقوق محفوظة | All Rights Reserved
// ═══════════════════════════════════════════════════════════════════════════════
// لغة برمجة عربية متكاملة مع ذكاء اصطناعي وواجهات متقدمة
// الإصدار 3.2.0
// المؤلف: رضوان دالي حمدوني
// البريد: almarjaa.project@hotmail.com
// ═══════════════════════════════════════════════════════════════════════════════
// تحذير: هذا المشروع محمي بموجب حقوق الملكية الفكرية.
// الاستخدام التجاري يتطلب إذناً كتابياً صريحاً من المؤلف.
// WARNING: This project is protected by intellectual property rights.
// Commercial use requires explicit written permission from the author.
// ═══════════════════════════════════════════════════════════════════════════════

pub mod ai_engine;
pub mod bytecode;
pub mod error;
pub mod exporter;
pub mod fine_tuning;
pub mod formatter;
pub mod gui;
pub mod integration;
pub mod interpreter;
pub mod lexer;
pub mod linter;
pub mod lsp_bridge;
pub mod onnx;
pub mod package_manager;
pub mod parser;
pub mod runtime;
pub mod ui;

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير محرك الذكاء الاصطناعي
// ═══════════════════════════════════════════════════════════════════════════════

pub use ai_engine::{
    PipelineEngine, PipelineResult, Intent,
    run_pipeline, run_example, parse_intent, generate_code,
    AIEngine, ModelConfig, ModelType, InferenceResult,
    create_engine, text_to_code, text_to_intent_json,
    GGUFEngine, GGUFConfig, GGUFResult,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير Bytecode VM مع JIT المتقدم
// ═══════════════════════════════════════════════════════════════════════════════

pub use bytecode::{
    run_bytecode, run_bytecode_benchmark, 
    run_all_benchmarks, print_benchmark_results,
    Chunk, Compiler, VM, OpCode, BenchmarkResult,
    // JIT exports
    JitCompiler, JitStats, OptimizedExecutor, CompiledCode, HotSpotInfo,
    // Advanced JIT exports
    AdvancedJitCompiler, AdvancedJitStats,
    TierLevel, TierInfo, TierThresholds,
    TracingRecorder, Trace, TraceEntry, TraceState, CompiledTrace,
    SimdProcessor, SimdStats, SimdOperation,
    ThreadedCodeExecutor, ThreadedStats, ThreadPool,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير Fine-tuning
// ═══════════════════════════════════════════════════════════════════════════════

pub use fine_tuning::{
    FineTuningInterface, TrainingConfig, TrainingExample, TrainingResult,
    fine_tune_model, fine_tune_with_config, evaluate_model,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير GUI
// ═══════════════════════════════════════════════════════════════════════════════

pub use gui::{
    GUIEngine, GUIElement, GUIColor, GUIAnimation, AnimationType,
    VibeGUIEngine,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير نظام التصدير
// ═══════════════════════════════════════════════════════════════════════════════

pub use exporter::{
    ExportEngine, ExportConfig, ExportPlatform, ExportResult,
    export_project, export_html_only,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير محرك التكامل
// ═══════════════════════════════════════════════════════════════════════════════

pub use integration::{
    IntegrationEngine, IntegrationResult, natural_to_app,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير المكونات الأساسية
// ═══════════════════════════════════════════════════════════════════════════════

pub use error::{AlMarjaaError, ErrorCode, Position, Severity, Span};
pub use formatter::format_source;
pub use interpreter::Interpreter;
pub use lexer::Lexer;
pub use linter::{
    lint_program, lint_program_with_config, lint_source, lint_source_with_config,
    LintConfig, LintDiagnostic, LintLevel,
};
pub use parser::Parser;

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير مدير الحزم
// ═══════════════════════════════════════════════════════════════════════════════

pub use package_manager::{
    PackageManager, PackageConfig, PackageInfo, PackageVersion,
    PackageSource, PackageStatus, PackageResult,
    Registry, Installer, Publisher, SecurityChecker, PackageStats,
    DependencyResolver, ResolvedDependencies,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير ONNX
// ═══════════════════════════════════════════════════════════════════════════════

pub use onnx::{
    ONNXEngine, ONNXSession, ONNXConfig, ONNXModel,
    ONNXTensor, ONNXShape, ONNXDataType, ONNXModelInfo,
    ONNXExporter, ExportOptions, ExportResult,
    ONNXInference, InferenceOptions, InferenceResult,
    onnx_load, onnx_engine, onnx_infer, onnx_export,
    tensor_to_onnx, onnx_to_tensor, LayerSpec,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصدير نظام واجهات المستخدم
// ═══════════════════════════════════════════════════════════════════════════════

pub use ui::{
    // المحرك الرئيسي
    UIEngine, UIEngineConfig, UIEngineState, create_ui_engine, build_ui,
    // الأنواع الأساسية
    UIElement, UIValue, UIColor, UIFont, UIMargin, UIPadding,
    UISize, UIPosition, UIRect, UIPoint, UITransform,
    UIEvent, UIEventHandler, UIState, UIContext,
    ComponentId, PropertyName, BindingPath,
    // نظام التخطيط
    LayoutEngine, Layout, LayoutKind, LayoutDirection,
    Row, Column, Grid, GridTrack, GridPlacement,
    FlexContainer, FlexItem, FlexDirection, FlexWrap,
    JustifyContent, AlignItems, AlignContent, AlignSelf,
    Gap, GapSize, calculate_layout, LayoutResult,
    // ربط البيانات
    BindingEngine, Binding, BindingKind, BindingMode,
    Observable, ObservableValue, ObservableCollection,
    Computed, ComputedValue, Watcher, WatcherCallback,
    BindingExpression, BindingResult, DataContext,
    bind, bind_two_way, bind_one_time, observe, computed,
    // المكونات
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
    // التصميم المتجاوب
    ResponsiveEngine, Breakpoint, Breakpoints, MediaQuery,
    DeviceType, Orientation, ScreenSize, ResponsiveValue,
    ResponsiveContainer, ResponsiveLayout, HideOn, ShowOn,
    responsive_value, match_breakpoint, get_device_type,
    // نظام الثيمات
    Theme, ThemeColors, ThemeTypography, ThemeSpacing,
    ThemeShadows, ThemeBorders, ThemeConfig, ThemeManager,
    ThemeMode, ColorScheme, Typography, FontFamily,
    create_theme, load_theme, save_theme, apply_theme,
    default_light_theme, default_dark_theme, default_arabic_theme,
    // الرسوم البيانية
    Chart, ChartKind, ChartData, ChartSeries, ChartPoint,
    ChartAxis, ChartLegend, ChartTooltip, ChartConfig,
    LineChart, BarChart, PieChart, AreaChart, ScatterChart,
    RadarChart, GaugeChart, HeatmapChart, CandlestickChart,
    ChartAnimation, ChartStyle, ChartColor, ChartTheme,
    create_chart, render_chart, chart_to_svg,
    // النوافذ المنبثقة
    Modal, ModalKind, ModalConfig, ModalResult, ModalManager,
    Dialog, AlertDialog, ConfirmDialog, PromptDialog,
    Toast, ToastKind, ToastPosition, ToastManager,
    Popup, PopupPosition, PopupTrigger,
    Sheet, SheetSide, Drawer, DrawerSide,
    Notification, NotificationKind, NotificationManager,
    show_modal, show_alert, show_confirm, show_toast, show_notification,
    // الأدوات المتقدمة
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
    // الرسوم المتحركة
    Animation, AnimationKind, AnimationConfig, AnimationManager,
    Transition, TransitionKind, TransitionConfig,
    Keyframe, Keyframes, Easing, EasingFunction,
    AnimationTimeline, AnimationState,
    animate, transition, create_animation, play_animation,
    FadeIn, FadeOut, SlideIn, SlideOut, ScaleIn, ScaleOut,
    RotateIn, RotateOut, BounceIn, BounceOut,
    // الأحداث
    EventManager, EventDispatcher, EventListener, EventHandler,
    MouseEvent, KeyboardEvent, FocusEvent, FormEvent,
    TouchEvent, GestureEvent, DragEvent, DropEvent,
    ScrollEvent, ResizeEvent,
    CustomEvent, EventPhase, EventResult,
    on_click, on_change, on_submit, on_hover, on_focus, on_blur,
    // التنسيق
    Style, StyleBuilder, StyleProperty, StyleValue,
    StyleSheet, CSSRule, Selector, PseudoClass,
    Border, BorderStyle, BorderRadius, Shadow,
    Gradient, GradientKind, GradientStop,
    Transform, TransformKind, Filter, FilterKind,
    Background, Foreground, Decoration,
    style, css, class_name, inline_style,
};

/// الإصدار الحالي للغة
pub const VERSION: &str = "3.2.0";

/// معلومات عن اللغة
pub fn info() -> &'static str {
    r#"
    ╔═══════════════════════════════════════════════════════════════╗
    ║         لغة المرجع - Al-Marjaa Language                      ║
    ║         لغة برمجة عربية مع ذكاء اصطناعي                       ║
    ╠═══════════════════════════════════════════════════════════════╣
    ║  الإصدار: 3.2.0                                              ║
    ║  Vibe Coding: ✅ مفعّل                                        ║
    ║  Bytecode VM: ✅ مفعّل                                        ║
    ║  JIT Compiler: ✅ مفعّل (5 مستويات)                           ║
    ║  Parallel GC: ✅ مفعّل                                        ║
    ║  GUI Builder: ✅ مفعّل                                        ║
    ║  Fine-tuning: ✅ مفعّل                                        ║
    ║  ONNX Support: ✅ مفعّل                                       ║
    ║  UI System: ✅ مفعّل (تخطيط، ربط، ثيمات، رسوم)               ║
    ║  المؤلف: رضوان دالي حمدوني                                   ║
    ║  GitHub: github.com/radhwendalyhamdouni/Al-Marjaa-Language     ║
    ╚═══════════════════════════════════════════════════════════════╝
"#
}
