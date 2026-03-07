// ═══════════════════════════════════════════════════════════════════════════════
// النوافذ المنبثقة - Modals
// ═══════════════════════════════════════════════════════════════════════════════

use crate::ui::types::*;
use crate::ui::components::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// ═══════════════════════════════════════════════════════════════════════════════
// أنواع النوافذ المنبثقة
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع النافذة المنبثقة
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModalKind {
    Dialog,
    Alert,
    Confirm,
    Prompt,
    Sheet,
    Drawer,
    Toast,
    Notification,
    Popup,
}

impl Default for ModalKind {
    fn default() -> Self {
        Self::Dialog
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// تكوين النافذة المنبثقة
// ═══════════════════════════════════════════════════════════════════════════════

/// تكوين النافذة المنبثقة
#[derive(Debug, Clone, PartialEq)]
pub struct ModalConfig {
    /// العنوان
    pub title: String,
    /// المحتوى
    pub content: String,
    /// العرض
    pub width: Option<UnitValue>,
    /// الارتفاع
    pub height: Option<UnitValue>,
    /// قابل للإغلاق
    pub closable: bool,
    /// إغلاق عند النقر خارج
    pub close_on_outside_click: bool,
    /// إغلاق بالضغط على Escape
    pub close_on_escape: bool,
    /// الخلفية المعتمة
    pub overlay: bool,
    /// لون الخلفية
    pub overlay_color: UIColor,
    /// الرسوم المتحركة
    pub animated: bool,
    /// الموضع
    pub position: ModalPosition,
}

impl ModalConfig {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            content: String::new(),
            width: None,
            height: None,
            closable: true,
            close_on_outside_click: true,
            close_on_escape: true,
            overlay: true,
            overlay_color: UIColor::black().with_alpha(0.5),
            animated: true,
            position: ModalPosition::Center,
        }
    }
    
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }
    
    pub fn content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }
    
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = Some(UnitValue::px(width));
        self.height = Some(UnitValue::px(height));
        self
    }
}

impl Default for ModalConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// موضع النافذة المنبثقة
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModalPosition {
    Center,
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Default for ModalPosition {
    fn default() -> Self {
        Self::Center
    }
}

/// نتيجة النافذة المنبثقة
#[derive(Debug, Clone, PartialEq)]
pub enum ModalResult {
    Confirmed,
    Cancelled,
    Dismissed,
    Value(String),
    Error(String),
}

// ═══════════════════════════════════════════════════════════════════════════════
/// النافذة المنبثقة
// ═══════════════════════════════════════════════════════════════════════════════

/// النافذة المنبثقة
#[derive(Debug, Clone)]
pub struct Modal {
    /// المعرف
    pub id: String,
    /// النوع
    pub kind: ModalKind,
    /// التكوين
    pub config: ModalConfig,
    /// مرئية
    pub visible: bool,
    /// الأزرار
    pub buttons: Vec<ModalButton>,
    /// المحتوى المخصص
    pub custom_content: Option<String>,
}

/// زر النافذة المنبثقة
#[derive(Debug, Clone, PartialEq)]
pub struct ModalButton {
    /// التسمية
    pub label: String,
    /// نوع الزر
    pub button_type: ModalButtonType,
    /// النتيجة عند النقر
    pub result: ModalResult,
}

impl ModalButton {
    pub fn new(label: &str, button_type: ModalButtonType) -> Self {
        Self {
            label: label.to_string(),
            button_type,
            result: ModalResult::Confirmed,
        }
    }
    
    pub fn confirm(label: &str) -> Self {
        Self::new(label, ModalButtonType::Primary)
    }
    
    pub fn cancel(label: &str) -> Self {
        let mut btn = Self::new(label, ModalButtonType::Secondary);
        btn.result = ModalResult::Cancelled;
        btn
    }
}

/// نوع زر النافذة المنبثقة
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModalButtonType {
    Primary,
    Secondary,
    Danger,
    Success,
}

impl Modal {
    pub fn new(kind: ModalKind, config: ModalConfig) -> Self {
        Self {
            id: format!("modal_{}", uuid()),
            kind,
            config,
            visible: false,
            buttons: Vec::new(),
            custom_content: None,
        }
    }
    
    pub fn button(mut self, button: ModalButton) -> Self {
        self.buttons.push(button);
        self
    }
    
    /// فتح النافذة
    pub fn open(&mut self) {
        self.visible = true;
    }
    
    /// إغلاق النافذة
    pub fn close(&mut self) {
        self.visible = false;
    }
    
    /// تصيير النافذة
    pub fn render(&self) -> String {
        if !self.visible {
            return String::new();
        }
        
        let mut html = String::new();
        
        // الخلفية المعتمة
        if self.config.overlay {
            html.push_str(&format!(
                r#"<div class="modal-overlay" style="background: {};" onclick="if({}) closeModal('{}')"></div>"#,
                self.config.overlay_color.to_css(),
                self.config.close_on_outside_click,
                self.id
            ));
        }
        
        // النافذة
        let position_style = match self.config.position {
            ModalPosition::Center => "top: 50%; left: 50%; transform: translate(-50%, -50%);",
            ModalPosition::Top => "top: 0; left: 50%; transform: translateX(-50%);",
            ModalPosition::Bottom => "bottom: 0; left: 50%; transform: translateX(-50%);",
            ModalPosition::Left => "top: 50%; left: 0; transform: translateY(-50%);",
            ModalPosition::Right => "top: 50%; right: 0; transform: translateY(-50%);",
            _ => "top: 50%; left: 50%; transform: translate(-50%, -50%);",
        };
        
        let width_style = self.config.width.as_ref()
            .map(|w| format!("width: {};", w.to_css()))
            .unwrap_or_default();
        
        let height_style = self.config.height.as_ref()
            .map(|h| format!("height: {};", h.to_css()))
            .unwrap_or_default();
        
        html.push_str(&format!(
            r#"<div id="{}" class="modal" style="{} {} {}">"#,
            self.id, position_style, width_style, height_style
        ));
        
        // الرأس
        if !self.config.title.is_empty() || self.config.closable {
            html.push_str("<div class=\"modal-header\">");
            
            if !self.config.title.is_empty() {
                html.push_str(&format!("<h3>{}</h3>", self.config.title));
            }
            
            if self.config.closable {
                html.push_str(&format!(
                    r#"<button class="modal-close" onclick="closeModal('{}')">&times;</button>"#,
                    self.id
                ));
            }
            
            html.push_str("</div>");
        }
        
        // المحتوى
        html.push_str("<div class=\"modal-content\">");
        if let Some(ref content) = self.custom_content {
            html.push_str(content);
        } else {
            html.push_str(&self.config.content);
        }
        html.push_str("</div>");
        
        // الأزرار
        if !self.buttons.is_empty() {
            html.push_str("<div class=\"modal-footer\">");
            for button in &self.buttons {
                let btn_class = match button.button_type {
                    ModalButtonType::Primary => "btn-primary",
                    ModalButtonType::Secondary => "btn-secondary",
                    ModalButtonType::Danger => "btn-danger",
                    ModalButtonType::Success => "btn-success",
                };
                html.push_str(&format!(
                    r#"<button class="{}" onclick="closeModal('{}')">{}</button>"#,
                    btn_class, self.id, button.label
                ));
            }
            html.push_str("</div>");
        }
        
        html.push_str("</div>");
        html
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الحوار
// ═══════════════════════════════════════════════════════════════════════════════

/// حوار
pub type Dialog = Modal;

/// حوار تنبيه
#[derive(Debug, Clone)]
pub struct AlertDialog {
    pub modal: Modal,
}

impl AlertDialog {
    pub fn new(message: &str) -> Self {
        let config = ModalConfig::new()
            .title("تنبيه")
            .content(message);
        
        let mut modal = Modal::new(ModalKind::Alert, config);
        modal.buttons.push(ModalButton::confirm("حسناً"));
        
        Self { modal }
    }
    
    pub fn with_title(mut self, title: &str) -> Self {
        self.modal.config.title = title.to_string();
        self
    }
}

/// حوار تأكيد
#[derive(Debug, Clone)]
pub struct ConfirmDialog {
    pub modal: Modal,
}

impl ConfirmDialog {
    pub fn new(message: &str) -> Self {
        let config = ModalConfig::new()
            .title("تأكيد")
            .content(message);
        
        let mut modal = Modal::new(ModalKind::Confirm, config);
        modal.buttons.push(ModalButton::cancel("إلغاء"));
        modal.buttons.push(ModalButton::confirm("تأكيد"));
        
        Self { modal }
    }
    
    pub fn with_buttons(mut self, confirm: &str, cancel: &str) -> Self {
        self.modal.buttons.clear();
        self.modal.buttons.push(ModalButton::cancel(cancel));
        self.modal.buttons.push(ModalButton::confirm(confirm));
        self
    }
}

/// حوار إدخال
#[derive(Debug, Clone)]
pub struct PromptDialog {
    pub modal: Modal,
    pub default_value: String,
}

impl PromptDialog {
    pub fn new(message: &str) -> Self {
        let config = ModalConfig::new()
            .title("إدخال")
            .content(message);
        
        let mut modal = Modal::new(ModalKind::Prompt, config);
        modal.buttons.push(ModalButton::cancel("إلغاء"));
        modal.buttons.push(ModalButton::confirm("موافق"));
        
        Self {
            modal,
            default_value: String::new(),
        }
    }
    
    pub fn default_value(mut self, value: &str) -> Self {
        self.default_value = value.to_string();
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Toast والإشعارات
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع Toast
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastKind {
    Info,
    Success,
    Warning,
    Error,
}

impl Default for ToastKind {
    fn default() -> Self {
        Self::Info
    }
}

/// موضع Toast
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastPosition {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    TopCenter,
    BottomCenter,
}

impl Default for ToastPosition {
    fn default() -> Self {
        Self::BottomRight
    }
}

/// Toast
#[derive(Debug, Clone)]
pub struct Toast {
    /// المعرف
    pub id: String,
    /// الرسالة
    pub message: String,
    /// النوع
    pub kind: ToastKind,
    /// المدة بالميلي ثانية
    pub duration: u32,
    /// الموضع
    pub position: ToastPosition,
    /// قابل للإغلاق
    pub closable: bool,
}

impl Toast {
    pub fn new(message: &str) -> Self {
        Self {
            id: format!("toast_{}", uuid()),
            message: message.to_string(),
            kind: ToastKind::Info,
            duration: 3000,
            position: ToastPosition::BottomRight,
            closable: true,
        }
    }
    
    pub fn success(message: &str) -> Self {
        Self {
            kind: ToastKind::Success,
            ..Self::new(message)
        }
    }
    
    pub fn error(message: &str) -> Self {
        Self {
            kind: ToastKind::Error,
            ..Self::new(message)
        }
    }
    
    pub fn warning(message: &str) -> Self {
        Self {
            kind: ToastKind::Warning,
            ..Self::new(message)
        }
    }
    
    pub fn duration(mut self, duration: u32) -> Self {
        self.duration = duration;
        self
    }
    
    /// تصيير Toast
    pub fn render(&self) -> String {
        let bg_color = match self.kind {
            ToastKind::Info => "#2196F3",
            ToastKind::Success => "#4CAF50",
            ToastKind::Warning => "#FF9800",
            ToastKind::Error => "#F44336",
        };
        
        let position_style = match self.position {
            ToastPosition::TopRight => "top: 20px; right: 20px;",
            ToastPosition::TopLeft => "top: 20px; left: 20px;",
            ToastPosition::BottomRight => "bottom: 20px; right: 20px;",
            ToastPosition::BottomLeft => "bottom: 20px; left: 20px;",
            ToastPosition::TopCenter => "top: 20px; left: 50%; transform: translateX(-50%);",
            ToastPosition::BottomCenter => "bottom: 20px; left: 50%; transform: translateX(-50%);",
        };
        
        let close_btn = if self.closable {
            format!(r#"<button class="toast-close" onclick="closeToast('{}')">&times;</button>"#, self.id)
        } else {
            String::new()
        };
        
        format!(
            r#"<div id="{}" class="toast toast-{}" style="background: {}; {}; animation: slideIn 0.3s ease-out;">{}{}</div>"#,
            self.id, 
            match self.kind {
                ToastKind::Info => "info",
                ToastKind::Success => "success",
                ToastKind::Warning => "warning",
                ToastKind::Error => "error",
            },
            bg_color,
            position_style,
            self.message,
            close_btn
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
/// Popup
// ═══════════════════════════════════════════════════════════════════════════════

/// موضع Popup
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PopupPosition {
    Top,
    Bottom,
    Left,
    Right,
    Auto,
}

impl Default for PopupPosition {
    fn default() -> Self {
        Self::Auto
    }
}

/// محفز Popup
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PopupTrigger {
    Click,
    Hover,
    Focus,
    Manual,
}

impl Default for PopupTrigger {
    fn default() -> Self {
        Self::Click
    }
}

/// Popup
#[derive(Debug, Clone)]
pub struct Popup {
    /// المعرف
    pub id: String,
    /// المحتوى
    pub content: String,
    /// الموضع
    pub position: PopupPosition,
    /// المحفز
    pub trigger: PopupTrigger,
}

impl Popup {
    pub fn new(content: &str) -> Self {
        Self {
            id: format!("popup_{}", uuid()),
            content: content.to_string(),
            position: PopupPosition::Auto,
            trigger: PopupTrigger::Click,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
/// Sheet و Drawer
// ═══════════════════════════════════════════════════════════════════════════════

/// جانب Sheet
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SheetSide {
    Top,
    Bottom,
    Left,
    Right,
}

impl Default for SheetSide {
    fn default() -> Self {
        Self::Bottom
    }
}

/// Sheet
#[derive(Debug, Clone)]
pub struct Sheet {
    pub modal: Modal,
    pub side: SheetSide,
}

impl Sheet {
    pub fn new() -> Self {
        let mut modal = Modal::new(ModalKind::Sheet, ModalConfig::new());
        modal.config.position = ModalPosition::Bottom;
        Self {
            modal,
            side: SheetSide::Bottom,
        }
    }
}

impl Default for Sheet {
    fn default() -> Self {
        Self::new()
    }
}

/// جانب Drawer
pub type DrawerSide = SheetSide;

/// Drawer
pub type Drawer = Sheet;

// ═══════════════════════════════════════════════════════════════════════════════
/// الإشعارات
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع الإشعار
pub type NotificationKind = ToastKind;

/// الإشعار
#[derive(Debug, Clone)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub kind: NotificationKind,
    pub icon: Option<String>,
    pub action: Option<String>,
}

impl Notification {
    pub fn new(title: &str, message: &str) -> Self {
        Self {
            id: format!("notification_{}", uuid()),
            title: title.to_string(),
            message: message.to_string(),
            kind: NotificationKind::Info,
            icon: None,
            action: None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// مدير النوافذ المنبثقة
// ═══════════════════════════════════════════════════════════════════════════════

/// مدير النوافذ المنبثقة
#[derive(Debug)]
pub struct ModalManager {
    /// النوافذ المنبثقة
    modals: HashMap<String, Modal>,
    /// Toast
    toasts: Vec<Toast>,
    /// الإشعارات
    notifications: Vec<Notification>,
}

impl ModalManager {
    pub fn new() -> Self {
        Self {
            modals: HashMap::new(),
            toasts: Vec::new(),
            notifications: Vec::new(),
        }
    }
    
    /// إضافة نافذة منبثقة
    pub fn add_modal(&mut self, modal: Modal) {
        self.modals.insert(modal.id.clone(), modal);
    }
    
    /// فتح نافذة منبثقة
    pub fn open_modal(&mut self, id: &str) -> Result<(), String> {
        if let Some(modal) = self.modals.get_mut(id) {
            modal.open();
            Ok(())
        } else {
            Err(format!("النافذة '{}' غير موجودة", id))
        }
    }
    
    /// إغلاق نافذة منبثقة
    pub fn close_modal(&mut self, id: &str) {
        if let Some(modal) = self.modals.get_mut(id) {
            modal.close();
        }
    }
    
    /// إضافة Toast
    pub fn add_toast(&mut self, toast: Toast) {
        self.toasts.push(toast);
    }
    
    /// إزالة Toast
    pub fn remove_toast(&mut self, id: &str) {
        self.toasts.retain(|t| t.id != id);
    }
    
    /// إضافة إشعار
    pub fn add_notification(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }
    
    /// تحديث المدير
    pub fn update(&mut self) -> Result<(), String> {
        Ok(())
    }
    
    /// تصيير جميع النوافذ المنبثقة
    pub fn render(&self) -> String {
        let mut html = String::new();
        
        // تصيير النوافذ المنبثقة
        for modal in self.modals.values() {
            html.push_str(&modal.render());
        }
        
        // تصيير Toast
        for toast in &self.toasts {
            html.push_str(&toast.render());
        }
        
        html
    }
}

impl Default for ModalManager {
    fn default() -> Self {
        Self::new()
    }
}

/// مدير الإشعارات
pub type NotificationManager = ModalManager;

/// مدير Toast
pub type ToastManager = ModalManager;

// ═══════════════════════════════════════════════════════════════════════════════
// الدوال المساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// عرض نافذة منبثقة
pub fn show_modal(modal: Modal) -> String {
    let id = modal.id.clone();
    let mut manager = ModalManager::new();
    manager.add_modal(modal);
    manager.open_modal(&id).unwrap();
    id
}

/// عرض تنبيه
pub fn show_alert(message: &str) -> String {
    let alert = AlertDialog::new(message);
    show_modal(alert.modal)
}

/// عرض تأكيد
pub fn show_confirm(message: &str) -> String {
    let confirm = ConfirmDialog::new(message);
    show_modal(confirm.modal)
}

/// عرض Toast
pub fn show_toast(message: &str) -> String {
    let toast = Toast::new(message);
    let id = toast.id.clone();
    let mut manager = ModalManager::new();
    manager.add_toast(toast);
    id
}

/// عرض إشعار
pub fn show_notification(title: &str, message: &str) -> String {
    let notification = Notification::new(title, message);
    notification.id.clone()
}

/// إنشاء معرف فريد
fn uuid() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
