use std::fmt;

/// التعليقات التوضيحية للأنواع
#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    /// نوع بسيط: رقم، نص، منطقي
    Simple(String),
    /// نوع قابل للاختيار: نوع؟
    Optional(Box<TypeAnnotation>),
    /// نوع قائمة: [نوع]
    List(Box<TypeAnnotation>),
    /// نوع قاموس: {مفتاح: قيمة}
    Dict(Box<TypeAnnotation>, Box<TypeAnnotation>),
    /// نوع دالة: (معاملات) -> نتيجة
    Function(Vec<TypeAnnotation>, Box<TypeAnnotation>),
    /// نوع موحد (Union): نوع أ | نوع ب
    Union(Vec<TypeAnnotation>),
}

/// نمط التفكيك للمتغيرات
#[derive(Debug, Clone, PartialEq)]
pub enum DestructuringPattern {
    /// تفكيك قائمة: [أ، ب، ج]
    List(Vec<String>),
    /// تفكيك كائن: {اسم، عمر}
    Object(Vec<(String, Option<String>)>), // (الخاصية، الاسم المستعار)
    /// متغير بسيط
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Identifier(String),

    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },

    Logical {
        left: Box<Expr>,
        op: LogicalOp,
        right: Box<Expr>,
    },

    Comparison {
        left: Box<Expr>,
        op: ComparisonOp,
        right: Box<Expr>,
    },

    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    Property {
        object: Box<Expr>,
        property: String,
    },

    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },

    List(Vec<Expr>),

    Dictionary(Vec<(Expr, Expr)>),

    Assignment {
        target: Box<Expr>,
        value: Box<Expr>,
    },

    CompoundAssignment {
        name: String,
        op: BinaryOp,
        value: Box<Expr>,
    },

    Lambda {
        params: Vec<String>,
        body: Box<Expr>,
    },

    Ternary {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },

    Increment {
        name: String,
        is_prefix: bool,
        delta: f64,
    },

    Range {
        start: Box<Expr>,
        end: Box<Expr>,
        step: Option<Box<Expr>>,
    },

    // List Comprehension: [تعبير لكل عنصر في قابل_التكرار إذا شرط]
    ListComprehension {
        element: Box<Expr>,
        variable: String,
        iterable: Box<Expr>,
        condition: Option<Box<Expr>>,
    },

    // Dictionary Comprehension: {مفتاح: قيمة لكل عنصر في قابل_التكرار إذا شرط}
    DictComprehension {
        key: Box<Expr>,
        value: Box<Expr>,
        variable: String,
        iterable: Box<Expr>,
        condition: Option<Box<Expr>>,
    },

    // Spread Operator: ...قائمة
    Spread(Box<Expr>),

    // Null Coalescing: أ ؟؟ ب
    NullCoalescing {
        left: Box<Expr>,
        right: Box<Expr>,
    },

    // Optional Chaining: كائن؟.خاصية
    OptionalProperty {
        object: Box<Expr>,
        property: String,
    },

    // Optional Index: كائن؟?[فهرس]
    OptionalIndex {
        object: Box<Expr>,
        index: Box<Expr>,
    },

    // Optional Call: دالة؟?(معاملات)
    OptionalCall {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    // Pipe Operator: بيانات |> دالة
    Pipe {
        value: Box<Expr>,
        function: Box<Expr>,
    },

    Await(Box<Expr>),

    FormatString(Vec<FormatPart>),

    /// تعبير Yield للمولدات: أعطِ قيمة
    Yield(Box<Expr>),

    /// تعبير مولد: دالة ترجع عدة قيم
    Generator {
        body: Box<Stmt>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormatPart {
    Literal(String),
    Expression(Expr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    FloorDiv,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
    BitNot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    VariableDecl {
        name: String,
        value: Expr,
        is_const: bool,
    },

    /// تعريف متغير مع تفكيك: متغير [أ، ب، ج] = قائمة؛
    DestructuringDecl {
        pattern: DestructuringPattern,
        value: Expr,
        is_const: bool,
    },

    MultiVarDecl {
        names: Vec<String>,
        values: Vec<Expr>,
        is_const: bool,
    },

    Expression(Expr),

    Block(Vec<Stmt>),

    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_if_branches: Vec<(Expr, Box<Stmt>)>,
        else_branch: Option<Box<Stmt>>,
    },

    While {
        condition: Expr,
        body: Box<Stmt>,
    },

    For {
        variable: String,
        iterable: Expr,
        body: Box<Stmt>,
    },

    ForRange {
        variable: String,
        start: Expr,
        end: Expr,
        step: Option<Expr>,
        body: Box<Stmt>,
    },

    Repeat {
        count: Expr,
        body: Box<Stmt>,
    },

    /// تعريف دالة مع دعم التعليقات التوضيحية للأنواع
    FunctionDecl {
        name: String,
        params: Vec<(String, Option<Expr>, Option<TypeAnnotation>)>, // (الاسم، القيمة الافتراضية، نوع)
        body: Box<Stmt>,
        is_async: bool,
        return_type: Option<TypeAnnotation>,
    },

    /// إرجاع قيمة من المولد: أعطِ قيمة؛
    Yield(Expr),

    Return(Option<Expr>),

    Break,

    Continue,

    Print(Vec<Expr>),

    Input(String),

    TryCatch {
        try_block: Box<Stmt>,
        catch_var: Option<String>,
        catch_block: Box<Stmt>,
        finally_block: Option<Box<Stmt>>,
    },

    Throw(Expr),

    Match {
        value: Expr,
        cases: Vec<(Vec<Expr>, Box<Stmt>)>,
        default: Option<Box<Stmt>>,
    },

    ClassDecl {
        name: String,
        parent: Option<String>,
        methods: Vec<Stmt>,
        fields: Vec<(String, Option<Expr>)>,
    },

    Import {
        path: String,
        alias: Option<String>,
        items: Vec<String>,
    },

    Assert {
        condition: Expr,
        message: Option<Expr>,
    },

    Delete(String),

    UiComponentDecl {
        name: String,
        params: Vec<String>,
        body: Box<Stmt>,
    },

    StateDecl {
        name: String,
        value: Expr,
    },

    ThemeDecl {
        name: String,
        value: Expr,
    },

    RouteDecl {
        name: String,
        value: Expr,
    },

    EventHandlerDecl {
        name: String,
        params: Vec<String>,
        body: Box<Stmt>,
    },

    // Context Manager: مع مورد كـ اسم { ... }
    With {
        resource: Expr,
        alias: Option<String>,
        body: Box<Stmt>,
    },

    // Data Class: بيانات اسم { حقل: نوع، ... }
    DataClassDecl {
        name: String,
        fields: Vec<(String, Option<Expr>)>,
    },

    // Enum: تعداد اسم { قيمة، ... }
    EnumDecl {
        name: String,
        variants: Vec<(String, Option<Expr>)>,
    },

    // Decorator application
    Decorated {
        decorator: Expr,
        target: Box<Stmt>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Program { statements }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Mod => write!(f, "%"),
            BinaryOp::Pow => write!(f, "^"),
            BinaryOp::FloorDiv => write!(f, "//"),
            BinaryOp::BitAnd => write!(f, "&"),
            BinaryOp::BitOr => write!(f, "|"),
            BinaryOp::BitXor => write!(f, "~~"),
            BinaryOp::ShiftLeft => write!(f, "<<"),
            BinaryOp::ShiftRight => write!(f, ">>"),
        }
    }
}

impl fmt::Display for LogicalOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogicalOp::And => write!(f, "و"),
            LogicalOp::Or => write!(f, "أو"),
        }
    }
}

impl fmt::Display for ComparisonOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComparisonOp::Equal => write!(f, "=="),
            ComparisonOp::NotEqual => write!(f, "!="),
            ComparisonOp::Less => write!(f, "<"),
            ComparisonOp::Greater => write!(f, ">"),
            ComparisonOp::LessEqual => write!(f, "<="),
            ComparisonOp::GreaterEqual => write!(f, ">="),
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Not => write!(f, "ليس"),
            UnaryOp::BitNot => write!(f, "~"),
        }
    }
}

impl fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeAnnotation::Simple(name) => write!(f, "{}", name),
            TypeAnnotation::Optional(inner) => write!(f, "{}؟", inner),
            TypeAnnotation::List(inner) => write!(f, "[{}]", inner),
            TypeAnnotation::Dict(key, val) => write!(f, "{{{}: {}}}", key, val),
            TypeAnnotation::Function(params, ret) => {
                let params_str: Vec<String> = params.iter().map(|p| p.to_string()).collect();
                write!(f, "({}) -> {}", params_str.join("، "), ret)
            }
            TypeAnnotation::Union(types) => {
                let types_str: Vec<String> = types.iter().map(|t| t.to_string()).collect();
                write!(f, "{}", types_str.join(" | "))
            }
        }
    }
}

impl fmt::Display for DestructuringPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DestructuringPattern::List(names) => {
                write!(f, "[{}]", names.join("، "))
            }
            DestructuringPattern::Object(fields) => {
                let fields_str: Vec<String> = fields
                    .iter()
                    .map(|(prop, alias)| {
                        match alias {
                            Some(a) => format!("{}: {}", prop, a),
                            None => prop.clone(),
                        }
                    })
                    .collect();
                write!(f, "{{{}}}", fields_str.join("، "))
            }
            DestructuringPattern::Identifier(name) => write!(f, "{}", name),
        }
    }
}
