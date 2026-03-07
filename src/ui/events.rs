// ═══════════════════════════════════════════════════════════════════════════════
// نظام الأحداث - Event System
// ═══════════════════════════════════════════════════════════════════════════════

use crate::ui::types::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// ═══════════════════════════════════════════════════════════════════════════════
// أنواع الأحداث
// ═══════════════════════════════════════════════════════════════════════════════

/// حدث الماوس
#[derive(Debug, Clone, PartialEq)]
pub struct MouseEvent {
    /// الموضع X
    pub x: f32,
    /// الموضع Y
    pub y: f32,
    /// زر الماوس
    pub button: MouseButton,
    /// الأزرار المضغوطة
    pub buttons: u8,
    /// مفاتيح التعديل
    pub modifiers: Modifiers,
}

/// زر الماوس
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    None,
}

impl Default for MouseButton {
    fn default() -> Self {
        Self::None
    }
}

/// مفاتيح التعديل
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

impl Default for Modifiers {
    fn default() -> Self {
        Self {
            ctrl: false,
            shift: false,
            alt: false,
            meta: false,
        }
    }
}

/// حدث لوحة المفاتيح
#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardEvent {
    /// المفتاح
    pub key: String,
    /// كود المفتاح
    pub code: String,
    /// كود ASCII
    pub key_code: u32,
    /// مفاتيح التعديل
    pub modifiers: Modifiers,
    /// مكرر
    pub repeat: bool,
}

/// حدث التركيز
#[derive(Debug, Clone, PartialEq)]
pub struct FocusEvent {
    /// العنصر ذو التركيز السابق
    pub related_target: Option<String>,
}

/// حدث النموذج
#[derive(Debug, Clone, PartialEq)]
pub struct FormEvent {
    /// القيمة
    pub value: UIValue,
    /// صالح
    pub valid: bool,
    /// التحقق من الصحة
    pub validation_message: Option<String>,
}

/// حدث اللمس
#[derive(Debug, Clone, PartialEq)]
pub struct TouchEvent {
    /// نقاط اللمس
    pub touches: Vec<TouchPoint>,
}

/// نقطة اللمس
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchPoint {
    pub id: u32,
    pub x: f32,
    pub y: f32,
}

/// حدث الإيماءة
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GestureEvent {
    /// نوع الإيماءة
    pub gesture: GestureType,
    /// المقياس
    pub scale: f32,
    /// الدوران
    pub rotation: f32,
}

/// نوع الإيماءة
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GestureType {
    Tap,
    DoubleTap,
    LongPress,
    Swipe,
    Pinch,
    Rotate,
    Pan,
}

/// حدث السحب
#[derive(Debug, Clone, PartialEq)]
pub struct DragEvent {
    /// البيانات المنقولة
    pub data: HashMap<String, String>,
    /// الموضع
    pub x: f32,
    pub y: f32,
}

/// حدث الإفلات
pub type DropEvent = DragEvent;

/// حدث التمرير
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollEvent {
    /// الموضع الأفقي
    pub scroll_x: f32,
    /// الموضع العمودي
    pub scroll_y: f32,
    /// الحد الأقصى للتمرير الأفقي
    pub max_scroll_x: f32,
    /// الحد الأقصى للتمرير العمودي
    pub max_scroll_y: f32,
}

/// حدث تغيير الحجم
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResizeEvent {
    /// العرض الجديد
    pub width: f32,
    /// الارتفاع الجديد
    pub height: f32,
}

/// حدث مخصص
#[derive(Debug, Clone, PartialEq)]
pub struct CustomEvent {
    /// النوع
    pub event_type: String,
    /// البيانات
    pub data: HashMap<String, UIValue>,
}

/// مرحلة الحدث
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventPhase {
    None,
    Capturing,
    AtTarget,
    Bubbling,
}

impl Default for EventPhase {
    fn default() -> Self {
        Self::None
    }
}

/// نتيجة الحدث
#[derive(Debug, Clone, PartialEq)]
pub enum EventResult {
    Handled,
    NotHandled,
    Cancelled,
    Error(String),
}

// ═══════════════════════════════════════════════════════════════════════════════
// معالج الأحداث
// ═══════════════════════════════════════════════════════════════════════════════

/// معالج الأحداث
pub type EventHandler = Arc<dyn Fn(&UIEvent) -> EventResult + Send + Sync>;

/// مستمع الأحداث
pub struct EventListener {
    /// المعرف
    pub id: String,
    /// نوع الحدث
    pub event_type: String,
    /// المعالج
    pub handler: EventHandler,
    /// نشط
    pub active: bool,
    /// مرة واحدة
    pub once: bool,
}

impl EventListener {
    pub fn new<F>(event_type: &str, handler: F) -> Self
    where
        F: Fn(&UIEvent) -> EventResult + Send + Sync + 'static,
    {
        Self {
            id: format!("listener_{}", uuid()),
            event_type: event_type.to_string(),
            handler: Arc::new(handler),
            active: true,
            once: false,
        }
    }
    
    pub fn once(mut self) -> Self {
        self.once = true;
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// موجه الأحداث
// ═══════════════════════════════════════════════════════════════════════════════

/// موجه الأحداث
pub struct EventDispatcher {
    /// المستمعين
    listeners: HashMap<String, Vec<EventListener>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }
    
    /// إضافة مستمع
    pub fn add_listener(&mut self, listener: EventListener) -> String {
        let id = listener.id.clone();
        let event_type = listener.event_type.clone();
        
        if let Some(listeners) = self.listeners.get_mut(&event_type) {
            listeners.push(listener);
        } else {
            self.listeners.insert(event_type, vec![listener]);
        }
        
        id
    }
    
    /// إزالة مستمع
    pub fn remove_listener(&mut self, id: &str) {
        for listeners in self.listeners.values_mut() {
            listeners.retain(|l| l.id != id);
        }
    }
    
    /// إرسال حدث
    pub fn dispatch(&self, event: &UIEvent) -> EventResult {
        let event_type = match &event.kind {
            UIEventKind::Click => "click",
            UIEventKind::DoubleClick => "dblclick",
            UIEventKind::MouseDown => "mousedown",
            UIEventKind::MouseUp => "mouseup",
            UIEventKind::MouseMove => "mousemove",
            UIEventKind::MouseEnter => "mouseenter",
            UIEventKind::MouseLeave => "mouseleave",
            UIEventKind::KeyDown => "keydown",
            UIEventKind::KeyUp => "keyup",
            UIEventKind::KeyPress => "keypress",
            UIEventKind::Focus => "focus",
            UIEventKind::Blur => "blur",
            UIEventKind::Change => "change",
            UIEventKind::Submit => "submit",
            UIEventKind::Scroll => "scroll",
            UIEventKind::Resize => "resize",
            UIEventKind::Custom(ref name) => name.as_str(),
            _ => "unknown",
        };
        
        if let Some(listeners) = self.listeners.get(event_type) {
            for listener in listeners {
                if listener.active {
                    let result = (listener.handler)(event);
                    if matches!(result, EventResult::Handled | EventResult::Cancelled) {
                        return result;
                    }
                }
            }
        }
        
        EventResult::NotHandled
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// مدير الأحداث
// ═══════════════════════════════════════════════════════════════════════════════

/// مدير الأحداث
#[derive(Debug)]
pub struct EventManager {
    /// الموجهات
    dispatchers: HashMap<String, EventDispatcher>,
    /// قائمة الأحداث المنتظرة
    pending_events: Vec<UIEvent>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            dispatchers: HashMap::new(),
            pending_events: Vec::new(),
        }
    }
    
    /// تسجيل مكون
    pub fn register_component(&mut self, component_id: &str) {
        self.dispatchers.insert(component_id.to_string(), EventDispatcher::new());
    }
    
    /// إضافة مستمع
    pub fn add_listener(&mut self, component_id: &str, listener: EventListener) -> Result<String, String> {
        if let Some(dispatcher) = self.dispatchers.get_mut(component_id) {
            Ok(dispatcher.add_listener(listener))
        } else {
            Err(format!("المكون '{}' غير مسجل", component_id))
        }
    }
    
    /// إزالة مستمع
    pub fn remove_listener(&mut self, id: &str) {
        for dispatcher in self.dispatchers.values_mut() {
            dispatcher.remove_listener(id);
        }
    }
    
    /// إرسال حدث
    pub fn dispatch(&mut self, event: UIEvent) -> Result<EventResult, String> {
        if let Some(dispatcher) = self.dispatchers.get(&event.source) {
            Ok(dispatcher.dispatch(&event))
        } else {
            Err(format!("المكون '{}' غير مسجل", event.source))
        }
    }
    
    /// إضافة حدث للطابور
    pub fn queue_event(&mut self, event: UIEvent) {
        self.pending_events.push(event);
    }
    
    /// معالجة الأحداث المنتظرة
    pub fn process_pending(&mut self) -> Vec<Result<EventResult, String>> {
        let events: Vec<UIEvent> = self.pending_events.drain(..).collect();
        events.into_iter()
            .map(|e| self.dispatch(e))
            .collect()
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الدوال المساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء معالج نقر
pub fn on_click<F>(handler: F) -> EventListener
where
    F: Fn(&UIEvent) -> EventResult + Send + Sync + 'static,
{
    EventListener::new("click", handler)
}

/// إنشاء معالج تغيير
pub fn on_change<F>(handler: F) -> EventListener
where
    F: Fn(&UIEvent) -> EventResult + Send + Sync + 'static,
{
    EventListener::new("change", handler)
}

/// إنشاء معالج إرسال
pub fn on_submit<F>(handler: F) -> EventListener
where
    F: Fn(&UIEvent) -> EventResult + Send + Sync + 'static,
{
    EventListener::new("submit", handler)
}

/// إنشاء معالج تمرير
pub fn on_hover<F>(handler: F) -> EventListener
where
    F: Fn(&UIEvent) -> EventResult + Send + Sync + 'static,
{
    EventListener::new("mouseenter", handler)
}

/// إنشاء معالج تركيز
pub fn on_focus<F>(handler: F) -> EventListener
where
    F: Fn(&UIEvent) -> EventResult + Send + Sync + 'static,
{
    EventListener::new("focus", handler)
}

/// إنشاء معالج فقدان التركيز
pub fn on_blur<F>(handler: F) -> EventListener
where
    F: Fn(&UIEvent) -> EventResult + Send + Sync + 'static,
{
    EventListener::new("blur", handler)
}

fn uuid() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
