// ═══════════════════════════════════════════════════════════════════════════════
// الرسوم المتحركة - Animations
// ═══════════════════════════════════════════════════════════════════════════════

use crate::ui::types::*;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
// أنواع الرسوم المتحركة
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع الرسوم المتحركة
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationKind {
    Fade,
    Slide,
    Scale,
    Rotate,
    Bounce,
    Shake,
    Pulse,
    Spin,
    Custom(String),
}

impl Default for AnimationKind {
    fn default() -> Self {
        Self::Fade
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// التخفيف
// ═══════════════════════════════════════════════════════════════════════════════

/// دالة التخفيف
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f32, f32, f32, f32),
    Spring,
    Bounce,
    Elastic,
}

impl Default for Easing {
    fn default() -> Self {
        Self::Ease
    }
}

impl Easing {
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        match self {
            Self::Linear => "linear".to_string(),
            Self::Ease => "ease".to_string(),
            Self::EaseIn => "ease-in".to_string(),
            Self::EaseOut => "ease-out".to_string(),
            Self::EaseInOut => "ease-in-out".to_string(),
            Self::CubicBezier(a, b, c, d) => {
                format!("cubic-bezier({}, {}, {}, {})", a, b, c, d)
            }
            Self::Spring => "cubic-bezier(0.175, 0.885, 0.32, 1.275)".to_string(),
            Self::Bounce => "cubic-bezier(0.68, -0.55, 0.265, 1.55)".to_string(),
            Self::Elastic => "cubic-bezier(0.68, -0.55, 0.265, 1.55)".to_string(),
        }
    }
}

/// دالة التخفيف
pub type EasingFunction = Easing;

// ═══════════════════════════════════════════════════════════════════════════════
/// الإطارات الرئيسية
// ═══════════════════════════════════════════════════════════════════════════════

/// إطار رئيسي
#[derive(Debug, Clone, PartialEq)]
pub struct Keyframe {
    /// النسبة المئوية
    pub percent: f32,
    /// الخصائص
    pub properties: HashMap<String, String>,
}

impl Keyframe {
    pub fn new(percent: f32) -> Self {
        Self {
            percent,
            properties: HashMap::new(),
        }
    }
    
    pub fn property(mut self, name: &str, value: &str) -> Self {
        self.properties.insert(name.to_string(), value.to_string());
        self
    }
}

/// الإطارات الرئيسية
#[derive(Debug, Clone, PartialEq)]
pub struct Keyframes {
    /// الاسم
    pub name: String,
    /// الإطارات
    pub frames: Vec<Keyframe>,
}

impl Keyframes {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            frames: Vec::new(),
        }
    }
    
    pub fn frame(mut self, frame: Keyframe) -> Self {
        self.frames.push(frame);
        self
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        let frames: String = self.frames.iter()
            .map(|f| {
                let props: String = f.properties.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join("; ");
                format!("{}% {{ {} }}", f.percent, props)
            })
            .collect::<Vec<_>>()
            .join(" ");
        
        format!("@keyframes {} {{ {} }}", self.name, frames)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// تكوين الرسوم المتحركة
// ═══════════════════════════════════════════════════════════════════════════════

/// تكوين الرسوم المتحركة
#[derive(Debug, Clone, PartialEq)]
pub struct AnimationConfig {
    /// المدة بالميلي ثانية
    pub duration: u32,
    /// التأخير بالميلي ثانية
    pub delay: u32,
    /// دالة التخفيف
    pub easing: Easing,
    /// عدد التكرارات (0 = لا نهائي)
    pub iterations: u32,
    /// الاتجاه
    pub direction: AnimationDirection,
    /// حالة التعبئة
    pub fill_mode: AnimationFillMode,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            duration: 300,
            delay: 0,
            easing: Easing::Ease,
            iterations: 1,
            direction: AnimationDirection::Normal,
            fill_mode: AnimationFillMode::Forwards,
        }
    }
}

/// اتجاه الرسوم المتحركة
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

impl Default for AnimationDirection {
    fn default() -> Self {
        Self::Normal
    }
}

/// حالة التعبئة
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

impl Default for AnimationFillMode {
    fn default() -> Self {
        Self::Forwards
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
/// الرسوم المتحركة
// ═══════════════════════════════════════════════════════════════════════════════

/// الرسوم المتحركة
#[derive(Debug, Clone, PartialEq)]
pub struct Animation {
    /// المعرف
    pub id: String,
    /// النوع
    pub kind: AnimationKind,
    /// التكوين
    pub config: AnimationConfig,
    /// الإطارات الرئيسية
    pub keyframes: Option<Keyframes>,
    /// الحالة
    pub state: AnimationState,
}

/// حالة الرسوم المتحركة
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnimationState {
    Idle,
    Running,
    Paused,
    Finished,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self::Idle
    }
}

/// حدث الرسوم المتحركة
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationEvent {
    Start,
    End,
    Iteration,
    Cancel,
}

impl Animation {
    pub fn new(kind: AnimationKind) -> Self {
        Self {
            id: format!("anim_{}", uuid()),
            kind,
            config: AnimationConfig::default(),
            keyframes: None,
            state: AnimationState::Idle,
        }
    }
    
    pub fn duration(mut self, duration: u32) -> Self {
        self.config.duration = duration;
        self
    }
    
    pub fn delay(mut self, delay: u32) -> Self {
        self.config.delay = delay;
        self
    }
    
    pub fn easing(mut self, easing: Easing) -> Self {
        self.config.easing = easing;
        self
    }
    
    pub fn infinite(mut self) -> Self {
        self.config.iterations = 0;
        self
    }
    
    pub fn iterations(mut self, count: u32) -> Self {
        self.config.iterations = count;
        self
    }
    
    pub fn keyframes(mut self, keyframes: Keyframes) -> Self {
        self.keyframes = Some(keyframes);
        self
    }
    
    /// بدء الرسوم المتحركة
    pub fn play(&mut self) {
        self.state = AnimationState::Running;
    }
    
    /// إيقاف مؤقت
    pub fn pause(&mut self) {
        self.state = AnimationState::Paused;
    }
    
    /// إعادة
    pub fn reset(&mut self) {
        self.state = AnimationState::Idle;
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        let name = if let Some(ref keyframes) = self.keyframes {
            keyframes.name.clone()
        } else {
            self.kind_to_name()
        };
        
        let duration = format!("{}ms", self.config.duration);
        let delay = if self.config.delay > 0 {
            format!(" {}ms", self.config.delay)
        } else {
            String::new()
        };
        
        let iterations = if self.config.iterations == 0 {
            "infinite".to_string()
        } else {
            self.config.iterations.to_string()
        };
        
        let direction = match self.config.direction {
            AnimationDirection::Normal => "normal",
            AnimationDirection::Reverse => "reverse",
            AnimationDirection::Alternate => "alternate",
            AnimationDirection::AlternateReverse => "alternate-reverse",
        };
        
        let fill_mode = match self.config.fill_mode {
            AnimationFillMode::None => "none",
            AnimationFillMode::Forwards => "forwards",
            AnimationFillMode::Backwards => "backwards",
            AnimationFillMode::Both => "both",
        };
        
        format!(
            "animation: {} {} {} {} {} {}{};",
            name, duration, self.config.easing.to_css(), iterations, direction, fill_mode, delay
        )
    }
    
    fn kind_to_name(&self) -> String {
        match &self.kind {
            AnimationKind::Fade => "fade".to_string(),
            AnimationKind::Slide => "slide".to_string(),
            AnimationKind::Scale => "scale".to_string(),
            AnimationKind::Rotate => "rotate".to_string(),
            AnimationKind::Bounce => "bounce".to_string(),
            AnimationKind::Shake => "shake".to_string(),
            AnimationKind::Pulse => "pulse".to_string(),
            AnimationKind::Spin => "spin".to_string(),
            AnimationKind::Custom(name) => name.clone(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الانتقالات
// ═══════════════════════════════════════════════════════════════════════════════

/// نوع الانتقال
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionKind {
    All,
    Property(String),
    Multiple(Vec<String>),
}

impl Default for TransitionKind {
    fn default() -> Self {
        Self::All
    }
}

/// تكوين الانتقال
#[derive(Debug, Clone, PartialEq)]
pub struct TransitionConfig {
    /// الخصائص
    pub properties: TransitionKind,
    /// المدة
    pub duration: u32,
    /// التأخير
    pub delay: u32,
    /// التخفيف
    pub easing: Easing,
}

impl Default for TransitionConfig {
    fn default() -> Self {
        Self {
            properties: TransitionKind::All,
            duration: 200,
            delay: 0,
            easing: Easing::Ease,
        }
    }
}

/// انتقال
#[derive(Debug, Clone, PartialEq)]
pub struct Transition {
    /// التكوين
    pub config: TransitionConfig,
}

impl Transition {
    pub fn new() -> Self {
        Self {
            config: TransitionConfig::default(),
        }
    }
    
    pub fn duration(mut self, duration: u32) -> Self {
        self.config.duration = duration;
        self
    }
    
    pub fn property(mut self, property: &str) -> Self {
        self.config.properties = TransitionKind::Property(property.to_string());
        self
    }
    
    pub fn easing(mut self, easing: Easing) -> Self {
        self.config.easing = easing;
        self
    }
    
    /// تحويل إلى CSS
    pub fn to_css(&self) -> String {
        let properties = match &self.config.properties {
            TransitionKind::All => "all".to_string(),
            TransitionKind::Property(p) => p.clone(),
            TransitionKind::Multiple(props) => props.join(", "),
        };
        
        format!(
            "transition: {} {}ms {} {}ms;",
            properties,
            self.config.duration,
            self.config.easing.to_css(),
            self.config.delay
        )
    }
}

impl Default for Transition {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// خط زمني الرسوم المتحركة
// ═══════════════════════════════════════════════════════════════════════════════

/// خط زمني للرسوم المتحركة
#[derive(Debug)]
pub struct AnimationTimeline {
    /// المعرف
    pub id: String,
    /// الرسوم المتحركة
    pub animations: Vec<Animation>,
    /// الحالة
    pub state: AnimationState,
    /// الوقت الحالي
    pub current_time: f64,
}

impl AnimationTimeline {
    pub fn new() -> Self {
        Self {
            id: format!("timeline_{}", uuid()),
            animations: Vec::new(),
            state: AnimationState::Idle,
            current_time: 0.0,
        }
    }
    
    pub fn add(mut self, animation: Animation) -> Self {
        self.animations.push(animation);
        self
    }
    
    pub fn play(&mut self) {
        self.state = AnimationState::Running;
        for anim in &mut self.animations {
            anim.play();
        }
    }
    
    pub fn pause(&mut self) {
        self.state = AnimationState::Paused;
        for anim in &mut self.animations {
            anim.pause();
        }
    }
}

impl Default for AnimationTimeline {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// مدير الرسوم المتحركة
// ═══════════════════════════════════════════════════════════════════════════════

/// مدير الرسوم المتحركة
#[derive(Debug)]
pub struct AnimationManager {
    /// الرسوم المتحركة
    animations: HashMap<String, Animation>,
    /// الخطوط الزمنية
    timelines: HashMap<String, AnimationTimeline>,
}

impl AnimationManager {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            timelines: HashMap::new(),
        }
    }
    
    pub fn add_animation(&mut self, animation: Animation) {
        self.animations.insert(animation.id.clone(), animation);
    }
    
    pub fn play(&mut self, id: &str) -> Result<(), String> {
        if let Some(anim) = self.animations.get_mut(id) {
            anim.play();
            Ok(())
        } else {
            Err(format!("الرسوم المتحركة '{}' غير موجودة", id))
        }
    }
    
    pub fn pause(&mut self, id: &str) {
        if let Some(anim) = self.animations.get_mut(id) {
            anim.pause();
        }
    }
    
    pub fn update(&mut self, delta_time: f64) -> Result<(), String> {
        // تحديث جميع الرسوم المتحركة
        for timeline in self.timelines.values_mut() {
            if timeline.state == AnimationState::Running {
                timeline.current_time += delta_time;
            }
        }
        Ok(())
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الرسوم المتحركة المحددة مسبقاً
// ═══════════════════════════════════════════════════════════════════════════════

/// تلاشي للداخل
pub struct FadeIn(pub Animation);
impl FadeIn {
    pub fn new() -> Self {
        Self(Animation::new(AnimationKind::Fade)
            .duration(300)
            .keyframes(Keyframes::new("fadeIn")
                .frame(Keyframe::new(0.0).property("opacity", "0"))
                .frame(Keyframe::new(100.0).property("opacity", "1"))))
    }
}

impl Default for FadeIn {
    fn default() -> Self {
        Self::new()
    }
}

/// تلاشي للخارج
pub struct FadeOut(pub Animation);
impl FadeOut {
    pub fn new() -> Self {
        Self(Animation::new(AnimationKind::Fade)
            .duration(300)
            .keyframes(Keyframes::new("fadeOut")
                .frame(Keyframe::new(0.0).property("opacity", "1"))
                .frame(Keyframe::new(100.0).property("opacity", "0"))))
    }
}

impl Default for FadeOut {
    fn default() -> Self {
        Self::new()
    }
}

/// انزلاق للداخل
pub struct SlideIn(pub Animation);
impl SlideIn {
    pub fn new() -> Self {
        Self(Animation::new(AnimationKind::Slide)
            .duration(300)
            .keyframes(Keyframes::new("slideIn")
                .frame(Keyframe::new(0.0).property("transform", "translateX(-100%)"))
                .frame(Keyframe::new(100.0).property("transform", "translateX(0)"))))
    }
}

impl Default for SlideIn {
    fn default() -> Self {
        Self::new()
    }
}

/// انزلاق للخارج
pub struct SlideOut(pub Animation);
impl SlideOut {
    pub fn new() -> Self {
        Self(Animation::new(AnimationKind::Slide)
            .duration(300)
            .keyframes(Keyframes::new("slideOut")
                .frame(Keyframe::new(0.0).property("transform", "translateX(0)"))
                .frame(Keyframe::new(100.0).property("transform", "translateX(100%)"))))
    }
}

impl Default for SlideOut {
    fn default() -> Self {
        Self::new()
    }
}

/// تكبير للداخل
pub struct ScaleIn(pub Animation);
impl ScaleIn {
    pub fn new() -> Self {
        Self(Animation::new(AnimationKind::Scale)
            .duration(300)
            .easing(Easing::Spring)
            .keyframes(Keyframes::new("scaleIn")
                .frame(Keyframe::new(0.0).property("transform", "scale(0)"))
                .frame(Keyframe::new(100.0).property("transform", "scale(1)"))))
    }
}

impl Default for ScaleIn {
    fn default() -> Self {
        Self::new()
    }
}

/// تكبير للخارج
pub struct ScaleOut(pub Animation);
impl ScaleOut {
    pub fn new() -> Self {
        Self(Animation::new(AnimationKind::Scale)
            .duration(300)
            .keyframes(Keyframes::new("scaleOut")
                .frame(Keyframe::new(0.0).property("transform", "scale(1)"))
                .frame(Keyframe::new(100.0).property("transform", "scale(0)"))))
    }
}

impl Default for ScaleOut {
    fn default() -> Self {
        Self::new()
    }
}

/// تدوير للداخل
pub struct RotateIn(pub Animation);
impl RotateIn {
    pub fn new() -> Self {
        Self(Animation::new(AnimationKind::Rotate)
            .duration(300)
            .keyframes(Keyframes::new("rotateIn")
                .frame(Keyframe::new(0.0).property("transform", "rotate(-180deg)"))
                .frame(Keyframe::new(100.0).property("transform", "rotate(0)"))))
    }
}

impl Default for RotateIn {
    fn default() -> Self {
        Self::new()
    }
}

/// تدوير للخارج
pub struct RotateOut(pub Animation);
impl RotateOut {
    pub fn new() -> Self {
        Self(Animation::new(AnimationKind::Rotate)
            .duration(300)
            .keyframes(Keyframes::new("rotateOut")
                .frame(Keyframe::new(0.0).property("transform", "rotate(0)"))
                .frame(Keyframe::new(100.0).property("transform", "rotate(180deg)"))))
    }
}

impl Default for RotateOut {
    fn default() -> Self {
        Self::new()
    }
}

/// ارتداد للداخل
pub struct BounceIn(pub Animation);
impl BounceIn {
    pub fn new() -> Self {
        Self(Animation::new(AnimationKind::Bounce)
            .duration(500)
            .easing(Easing::Bounce)
            .keyframes(Keyframes::new("bounceIn")
                .frame(Keyframe::new(0.0).property("transform", "scale(0)"))
                .frame(Keyframe::new(50.0).property("transform", "scale(1.1)"))
                .frame(Keyframe::new(100.0).property("transform", "scale(1)"))))
    }
}

impl Default for BounceIn {
    fn default() -> Self {
        Self::new()
    }
}

/// ارتداد للخارج
pub struct BounceOut(pub Animation);
impl BounceOut {
    pub fn new() -> Self {
        Self(Animation::new(AnimationKind::Bounce)
            .duration(500)
            .keyframes(Keyframes::new("bounceOut")
                .frame(Keyframe::new(0.0).property("transform", "scale(1)"))
                .frame(Keyframe::new(50.0).property("transform", "scale(1.1)"))
                .frame(Keyframe::new(100.0).property("transform", "scale(0)"))))
    }
}

impl Default for BounceOut {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الدوال المساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء رسوم متحركة
pub fn animate(kind: AnimationKind) -> Animation {
    Animation::new(kind)
}

/// إنشاء انتقال
pub fn transition() -> Transition {
    Transition::new()
}

/// إنشاء رسوم متحركة مخصصة
pub fn create_animation(name: &str) -> Animation {
    Animation::new(AnimationKind::Custom(name.to_string()))
}

/// تشغيل رسوم متحركة
pub fn play_animation(manager: &mut AnimationManager, id: &str) -> Result<(), String> {
    manager.play(id)
}

fn uuid() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}
