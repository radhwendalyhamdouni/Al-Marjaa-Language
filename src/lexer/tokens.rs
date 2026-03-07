use std::fmt;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // أنواع البيانات
    Number(f64),
    String(String),
    Boolean(bool),
    Null,

    // المعرفات
    Identifier(String),

    // الكلمات المحجوزة الأساسية
    Function,    // دالة
    Return,      // أرجع
    If,          // إذا
    Else,        // وإلا
    ElseIf,      // وإذا
    While,       // طالما
    For,         // لكل
    In,          // في
    Break,       // توقف
    Continue,    // أكمل
    Let,         // متغير
    Const,       // ثابت
    True,        // صح
    False,       // خطأ
    NullKeyword, // لا_شيء
    Class,       // صنف
    This,        // هذا
    New,         // جديد
    Import,      // استورد
    Export,      // صدر
    Try,         // حاول
    Catch,       // امسك
    Finally,     // أخيراً
    Throw,       // ألقِ
    Print,       // اطبع
    Input,       // ادخل
    TypeOf,      // نوع
    Length,      // طول
    And,         // و
    Or,          // أو
    Not,         // ليس
    Delete,      // احذف
    Repeat,      // كرر
    Times,       // مرة
    Match,       // طابق
    Case,        // حالة
    Default,     // افتراضي
    Lambda,      // لامدا
    Async,       // غير_متزامن
    Await,       // انتظر
    Yield,       // أعطِ
    Module,      // وحدة
    Use,         // استخدم
    As,          // بوصف
    From,        // من
    Range,       // مدى
    Step,        // خطوة
    Assert,      // تأكد
    Typeof,      // نوع_من
    Instanceof,  // نوع_مثل
    Super,       // أصل

    // ميزات متقدمة على نمط بايثون
    With,        // مع (context manager)
    AsKeyword,   // كـ (as alias)
    Enum,        // تعداد
    Data,        // بيانات (data class)
    Pipe,        // |>
    MatchExpr,   // مطابقة تعبير

    // واجهات المستخدم
    Interface, // واجهة
    Theme,     // ثيم
    Router,    // موجه
    Event,     // حدث

    // ONNX والذكاء الاصطناعي المتقدم
    ONNX,       // أونكس
    Model,      // نموذج
    Load,       // حمّل
    Save,       // احفظ
    Infer,      // استدل
    Tensor,     // موتر
    Shape,      // شكل
    Output,     // مخرج
    Layer,      // طبقة
    Dense,      // كثيف
    Conv,       // التفاف
    Pool,       // تجميع
    Normalize,  // طبع
    Dropout,    // إسقاط
    Flatten,    // تسوية
    Reshape,    // إعادة_تشكيل
    Activation, // تنشيط
    Softmax,    // سوفت_ماكس
    Relu,       // ريلو
    Sigmoid,    // سيجمويد
    Batch,      // دفعة
    Train,      // درّب
    Predict,    // توقع
    Optimizer,  // محسّن
    Loss,       // خسارة

    // واجهات المستخدم المتقدمة
    // التخطيط
    Row,        // صف
    Column,     // عمود
    Grid,       // شبكة
    Flex,       // مرن
    Stack,      // كومة
    Wrap,       // التفاف
    Gap,        // فجوة
    Align,      // محاذاة
    Justify,    // تبرير
    Padding,    // حشو
    Margin,     // هامش
    
    // المكونات
    Button,     // زر
    Text,       // نص
    Input,      // إدخال
    Select,     // اختيار
    Checkbox,   // خانة
    Radio,      // راديو
    Slider,     // منزلق
    Progress,   // تقدم
    Spinner,    // مؤقت
    Card,       // بطاقة
    List,       // قائمة
    Table,      // جدول
    Form,       // نموذج
    Label,      // تسمية
    Image,      // صورة
    Icon,       // أيقونة
    Badge,      // شارة
    Avatar,     // صورة_شخصية
    Tooltip,    // تلميح
    Modal,      // نافذة
    Toast,      // تنبيه
    Popup,      // منبثق
    
    // الثيمات
    Color,      // لون
    Font,       // خط
    Size,       // حجم
    Width,      // عرض
    Height,     // ارتفاع
    Border,     // حدود
    Shadow,     // ظل
    Background, // خلفية
    
    // الرسوم البيانية
    Chart,      // رسم
    LineChart,  // رسم_خطي
    BarChart,   // رسم_أعمدة
    PieChart,   // رسم_دائري
    AreaChart,  // رسم_مساحي
    
    // الرسوم المتحركة
    Animate,    // حرك
    Transition, // انتقال
    Duration,   // مدة
    Delay,      // تأخير
    Easing,     // تخفيف
    
    // الأحداث
    Click,      // نقر
    Change,     // تغيير
    Submit,     // إرسال
    Focus,      // تركيز
    Blur,       // ضبابية
    Hover,      // تحويم
    Scroll,     // تمرير
    
    // الربط
    Bind,       // ربط
    Observe,    // راقب
    Computed,   // محسوب
    Watch,      // راقب

    // العمليات الحسابية
    Plus,     // +
    Minus,    // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %
    Power,    // ^
    FloorDiv, // //

    // العمليات المقارنة
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=

    // العمليات المنطقية والبت
    Assign,      // =
    PlusAssign,  // +=
    MinusAssign, // -=
    MultAssign,  // *=
    DivAssign,   // /=
    ModAssign,   // %=
    PowAssign,   // ^=
    AndAssign,   // &&=
    OrAssign,    // ||=
    BitAnd,      // &
    BitOr,       // |
    BitXor,      // ~~
    BitNot,      // ~
    ShiftLeft,   // <<
    ShiftRight,  // >>
    Increment,   // ++
    Decrement,   // --

    // الفواصل
    Semicolon,       // ؛ أو ;
    Comma,           // ، أو ,
    Dot,             // .
    Colon,           // :
    Arrow,           // ->
    FatArrow,        // =>
    DotDot,          // ..
    DotDotDot,       // ...
    Question,        // ?
    QuestionDot,     // ?.
    QuestionQuestion,// ??
    At,              // @

    // الأقواس
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]

    // التعليقات
    Comment(String),

    // نهاية الملف
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
    pub span_start: usize,
    pub span_end: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Token {
            token_type,
            line,
            column,
            span_start: 0,
            span_end: 0,
        }
    }

    pub fn with_span(mut self, start: usize, end: usize) -> Self {
        self.span_start = start;
        self.span_end = end;
        self
    }

    pub fn span(&self) -> (usize, usize) {
        (self.span_start, self.span_end)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Number(n) => write!(f, "رقم({})", n),
            TokenType::String(s) => write!(f, "نص({})", s),
            TokenType::Boolean(b) => write!(f, "منطقي({})", b),
            TokenType::Null => write!(f, "لا_شيء"),
            TokenType::Identifier(name) => write!(f, "معرف({})", name),
            TokenType::Function => write!(f, "دالة"),
            TokenType::Return => write!(f, "أرجع"),
            TokenType::If => write!(f, "إذا"),
            TokenType::Else => write!(f, "وإلا"),
            TokenType::ElseIf => write!(f, "وإذا"),
            TokenType::While => write!(f, "طالما"),
            TokenType::For => write!(f, "لكل"),
            TokenType::In => write!(f, "في"),
            TokenType::Break => write!(f, "توقف"),
            TokenType::Continue => write!(f, "أكمل"),
            TokenType::Let => write!(f, "متغير"),
            TokenType::Const => write!(f, "ثابت"),
            TokenType::True => write!(f, "صح"),
            TokenType::False => write!(f, "خطأ"),
            TokenType::NullKeyword => write!(f, "لا_شيء"),
            TokenType::Class => write!(f, "صنف"),
            TokenType::This => write!(f, "هذا"),
            TokenType::New => write!(f, "جديد"),
            TokenType::Import => write!(f, "استورد"),
            TokenType::Export => write!(f, "صدر"),
            TokenType::Try => write!(f, "حاول"),
            TokenType::Catch => write!(f, "امسك"),
            TokenType::Finally => write!(f, "أخيراً"),
            TokenType::Throw => write!(f, "ألقِ"),
            TokenType::Print => write!(f, "اطبع"),
            TokenType::Input => write!(f, "ادخل"),
            TokenType::TypeOf => write!(f, "نوع"),
            TokenType::Length => write!(f, "طول"),
            TokenType::And => write!(f, "و"),
            TokenType::Or => write!(f, "أو"),
            TokenType::Not => write!(f, "ليس"),
            TokenType::Delete => write!(f, "احذف"),
            TokenType::Repeat => write!(f, "كرر"),
            TokenType::Times => write!(f, "مرة"),
            TokenType::Match => write!(f, "طابق"),
            TokenType::Case => write!(f, "حالة"),
            TokenType::Default => write!(f, "افتراضي"),
            TokenType::Lambda => write!(f, "لامدا"),
            TokenType::Async => write!(f, "غير_متزامن"),
            TokenType::Await => write!(f, "انتظر"),
            TokenType::Yield => write!(f, "أعطِ"),
            TokenType::Module => write!(f, "وحدة"),
            TokenType::Use => write!(f, "استخدم"),
            TokenType::As => write!(f, "بوصف"),
            TokenType::From => write!(f, "من"),
            TokenType::Range => write!(f, "مدى"),
            TokenType::Step => write!(f, "خطوة"),
            TokenType::Assert => write!(f, "تأكد"),
            TokenType::Typeof => write!(f, "نوع_من"),
            TokenType::Instanceof => write!(f, "نوع_مثل"),
            TokenType::Super => write!(f, "أصل"),
            TokenType::With => write!(f, "مع"),
            TokenType::AsKeyword => write!(f, "كـ"),
            TokenType::Enum => write!(f, "تعداد"),
            TokenType::Data => write!(f, "بيانات"),
            TokenType::Pipe => write!(f, "|>"),
            TokenType::MatchExpr => write!(f, "مطابقة"),
            TokenType::Interface => write!(f, "واجهة"),
            TokenType::Theme => write!(f, "ثيم"),
            TokenType::Router => write!(f, "موجه"),
            TokenType::Event => write!(f, "حدث"),
            // ONNX والذكاء الاصطناعي
            TokenType::ONNX => write!(f, "أونكس"),
            TokenType::Model => write!(f, "نموذج"),
            TokenType::Load => write!(f, "حمّل"),
            TokenType::Save => write!(f, "احفظ"),
            TokenType::Infer => write!(f, "استدل"),
            TokenType::Tensor => write!(f, "موتر"),
            TokenType::Shape => write!(f, "شكل"),
            TokenType::Output => write!(f, "مخرج"),
            TokenType::Layer => write!(f, "طبقة"),
            TokenType::Dense => write!(f, "كثيف"),
            TokenType::Conv => write!(f, "التفاف"),
            TokenType::Pool => write!(f, "تجميع"),
            TokenType::Normalize => write!(f, "طبع"),
            TokenType::Dropout => write!(f, "إسقاط"),
            TokenType::Flatten => write!(f, "تسوية"),
            TokenType::Reshape => write!(f, "إعادة_تشكيل"),
            TokenType::Activation => write!(f, "تنشيط"),
            TokenType::Softmax => write!(f, "سوفت_ماكس"),
            TokenType::Relu => write!(f, "ريلو"),
            TokenType::Sigmoid => write!(f, "سيجمويد"),
            TokenType::Batch => write!(f, "دفعة"),
            TokenType::Train => write!(f, "درّب"),
            TokenType::Predict => write!(f, "توقع"),
            TokenType::Optimizer => write!(f, "محسّن"),
            TokenType::Loss => write!(f, "خسارة"),
            // واجهات المستخدم المتقدمة
            // التخطيط
            TokenType::Row => write!(f, "صف"),
            TokenType::Column => write!(f, "عمود"),
            TokenType::Grid => write!(f, "شبكة"),
            TokenType::Flex => write!(f, "مرن"),
            TokenType::Stack => write!(f, "كومة"),
            TokenType::Wrap => write!(f, "التفاف"),
            TokenType::Gap => write!(f, "فجوة"),
            TokenType::Align => write!(f, "محاذاة"),
            TokenType::Justify => write!(f, "تبرير"),
            TokenType::Padding => write!(f, "حشو"),
            TokenType::Margin => write!(f, "هامش"),
            // المكونات
            TokenType::Button => write!(f, "زر"),
            TokenType::Text => write!(f, "نص"),
            TokenType::Input => write!(f, "إدخال"),
            TokenType::Select => write!(f, "اختيار"),
            TokenType::Checkbox => write!(f, "خانة"),
            TokenType::Radio => write!(f, "راديو"),
            TokenType::Slider => write!(f, "منزلق"),
            TokenType::Progress => write!(f, "تقدم"),
            TokenType::Spinner => write!(f, "مؤقت"),
            TokenType::Card => write!(f, "بطاقة"),
            TokenType::List => write!(f, "قائمة"),
            TokenType::Table => write!(f, "جدول"),
            TokenType::Form => write!(f, "نموذج"),
            TokenType::Label => write!(f, "تسمية"),
            TokenType::Image => write!(f, "صورة"),
            TokenType::Icon => write!(f, "أيقونة"),
            TokenType::Badge => write!(f, "شارة"),
            TokenType::Avatar => write!(f, "صورة_شخصية"),
            TokenType::Tooltip => write!(f, "تلميح"),
            TokenType::Modal => write!(f, "نافذة"),
            TokenType::Toast => write!(f, "تنبيه"),
            TokenType::Popup => write!(f, "منبثق"),
            // الثيمات
            TokenType::Color => write!(f, "لون"),
            TokenType::Font => write!(f, "خط"),
            TokenType::Size => write!(f, "حجم"),
            TokenType::Width => write!(f, "عرض"),
            TokenType::Height => write!(f, "ارتفاع"),
            TokenType::Border => write!(f, "حدود"),
            TokenType::Shadow => write!(f, "ظل"),
            TokenType::Background => write!(f, "خلفية"),
            // الرسوم البيانية
            TokenType::Chart => write!(f, "رسم"),
            TokenType::LineChart => write!(f, "رسم_خطي"),
            TokenType::BarChart => write!(f, "رسم_أعمدة"),
            TokenType::PieChart => write!(f, "رسم_دائري"),
            TokenType::AreaChart => write!(f, "رسم_مساحي"),
            // الرسوم المتحركة
            TokenType::Animate => write!(f, "حرك"),
            TokenType::Transition => write!(f, "انتقال"),
            TokenType::Duration => write!(f, "مدة"),
            TokenType::Delay => write!(f, "تأخير"),
            TokenType::Easing => write!(f, "تخفيف"),
            // الأحداث
            TokenType::Click => write!(f, "نقر"),
            TokenType::Change => write!(f, "تغيير"),
            TokenType::Submit => write!(f, "إرسال"),
            TokenType::Focus => write!(f, "تركيز"),
            TokenType::Blur => write!(f, "ضبابية"),
            TokenType::Hover => write!(f, "تحويم"),
            TokenType::Scroll => write!(f, "تمرير"),
            // الربط
            TokenType::Bind => write!(f, "ربط"),
            TokenType::Observe => write!(f, "راقب"),
            TokenType::Computed => write!(f, "محسوب"),
            TokenType::Watch => write!(f, "راقب_التغييرات"),
            // العمليات
            TokenType::Minus => write!(f, "-"),
            TokenType::Multiply => write!(f, "*"),
            TokenType::Divide => write!(f, "/"),
            TokenType::Modulo => write!(f, "%"),
            TokenType::Power => write!(f, "^"),
            TokenType::FloorDiv => write!(f, "//"),
            TokenType::Equal => write!(f, "=="),
            TokenType::NotEqual => write!(f, "!="),
            TokenType::Less => write!(f, "<"),
            TokenType::Greater => write!(f, ">"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Assign => write!(f, "="),
            TokenType::PlusAssign => write!(f, "+="),
            TokenType::MinusAssign => write!(f, "-="),
            TokenType::MultAssign => write!(f, "*="),
            TokenType::DivAssign => write!(f, "/="),
            TokenType::ModAssign => write!(f, "%="),
            TokenType::PowAssign => write!(f, "^="),
            TokenType::AndAssign => write!(f, "&&="),
            TokenType::OrAssign => write!(f, "||="),
            TokenType::BitAnd => write!(f, "&"),
            TokenType::BitOr => write!(f, "|"),
            TokenType::BitXor => write!(f, "~~"),
            TokenType::BitNot => write!(f, "~"),
            TokenType::ShiftLeft => write!(f, "<<"),
            TokenType::ShiftRight => write!(f, ">>"),
            TokenType::Increment => write!(f, "++"),
            TokenType::Decrement => write!(f, "--"),
            TokenType::Semicolon => write!(f, "؛"),
            TokenType::Comma => write!(f, "،"),
            TokenType::Dot => write!(f, "."),
            TokenType::Colon => write!(f, ":"),
            TokenType::Arrow => write!(f, "->"),
            TokenType::FatArrow => write!(f, "=>"),
            TokenType::DotDot => write!(f, ".."),
            TokenType::DotDotDot => write!(f, "..."),
            TokenType::Question => write!(f, "?"),
            TokenType::QuestionDot => write!(f, "?."),
            TokenType::QuestionQuestion => write!(f, "??"),
            TokenType::At => write!(f, "@"),
            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LBrace => write!(f, "{{"),
            TokenType::RBrace => write!(f, "}}"),
            TokenType::LBracket => write!(f, "["),
            TokenType::RBracket => write!(f, "]"),
            TokenType::Comment(c) => write!(f, "تعليق({})", c),
            TokenType::EOF => write!(f, "نهاية_الملف"),
        }
    }
}
