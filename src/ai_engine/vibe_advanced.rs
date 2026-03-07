// ═══════════════════════════════════════════════════════════════════════════════
// نظام Vibe Coding المتقدم - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// نظام متكامل لتحويل النص العربي الطبيعي إلى كود تنفيذي
// يتضمن:
// - فهم دلالي عميق (Semantic Understanding)
// - كشف النوايا المتعددة (Multi-Intent Detection)
// - التعلم من أنماط المستخدم (Learning & Adaptation)
// - السياق الذكي (Context Awareness)
// - الشرح التلقائي (Auto Explanation)
// - التصحيح الذكي (Smart Correction)
// - المكالمة المحادثية (Conversational Mode)
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::{HashMap, VecDeque};
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
// الثوابت والتعدادات
// ═══════════════════════════════════════════════════════════════════════════════

/// الحد الأقصى لتاريخ المحادثة
const MAX_CONVERSATION_HISTORY: usize = 50;
/// الحد الأقصى للأنماط المحفوظة
const MAX_PATTERNS: usize = 1000;
/// عتبة الثقة للكشف
const CONFIDENCE_THRESHOLD: f64 = 0.6;

/// نوع النية (Intent Type)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IntentType {
    /// إنشاء متغير
    CreateVariable,
    /// إنشاء دالة
    CreateFunction,
    /// إنشاء شرط
    CreateCondition,
    /// إنشاء حلقة
    CreateLoop,
    /// إنشاء طباعة
    CreatePrint,
    /// إنشاء قائمة/مصفوفة
    CreateList,
    /// إنشاء كائن/هيكل
    CreateStruct,
    /// إنشاء فئة
    CreateClass,
    /// عملية حسابية
    Arithmetic,
    /// عملية مقارنة
    Comparison,
    /// استدعاء دالة
    FunctionCall,
    /// إرجاع قيمة
    Return,
    /// استيراد مكتبة
    Import,
    /// تصدير برنامج
    ExportProgram,
    /// معالجة أخطاء
    ErrorHandling,
    /// تعليق
    Comment,
    /// نية مركبة (متعددة)
    Compound(Vec<IntentType>),
    /// غير معروف
    Unknown,
}

/// مستوى الثقة
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ConfidenceLevel {
    /// ثقة عالية (90%+)
    High,
    /// ثقة متوسطة (70-90%)
    Medium,
    /// ثقة منخفضة (50-70%)
    Low,
    /// غير واثق (أقل من 50%)
    Uncertain,
}

/// نوع السياق
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContextType {
    /// داخل دالة
    InsideFunction,
    /// داخل حلقة
    InsideLoop,
    /// داخل شرط
    InsideCondition,
    /// في النطاق العام
    GlobalScope,
    /// في تعريف متغير
    VariableDefinition,
    /// في استدعاء دالة
    FunctionCall,
}

/// نوع التعلم
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LearningType {
    /// نمط كتابة
    WritingPattern,
    /// تفضيل تسمية
    NamingPreference,
    /// نمط هيكلة
    StructuringPattern,
    /// اختصار شائع
    CommonAbbreviation,
}

// ═══════════════════════════════════════════════════════════════════════════════
// هياكل البيانات الأساسية
// ═══════════════════════════════════════════════════════════════════════════════

/// نية مُكتشفة مع تفاصيلها
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedIntent {
    /// نوع النية
    pub intent_type: IntentType,
    /// مستوى الثقة
    pub confidence: f64,
    /// المعاملات المستخرجة
    pub params: HashMap<String, String>,
    /// الكود المُنتج
    pub generated_code: Option<String>,
    /// الشرح بالعربية
    pub explanation: Option<String>,
    /// البدائل المقترحة
    pub alternatives: Vec<DetectedIntent>,
}

impl DetectedIntent {
    pub fn new(intent_type: IntentType, confidence: f64) -> Self {
        Self {
            intent_type,
            confidence,
            params: HashMap::new(),
            generated_code: None,
            explanation: None,
            alternatives: Vec::new(),
        }
    }

    pub fn with_param(mut self, key: &str, value: &str) -> Self {
        self.params.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_code(mut self, code: &str) -> Self {
        self.generated_code = Some(code.to_string());
        self
    }

    pub fn with_explanation(mut self, explanation: &str) -> Self {
        self.explanation = Some(explanation.to_string());
        self
    }

    pub fn confidence_level(&self) -> ConfidenceLevel {
        if self.confidence >= 0.9 {
            ConfidenceLevel::High
        } else if self.confidence >= 0.7 {
            ConfidenceLevel::Medium
        } else if self.confidence >= 0.5 {
            ConfidenceLevel::Low
        } else {
            ConfidenceLevel::Uncertain
        }
    }
}

/// عنصر في تاريخ المحادثة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationItem {
    /// نص المستخدم
    pub user_input: String,
    /// النوايا المُكتشفة
    pub intents: Vec<DetectedIntent>,
    /// الكود المُنتج
    pub generated_code: String,
    /// الطابع الزمني
    pub timestamp: u64,
    /// تم القبول أم لا
    pub accepted: Option<bool>,
}

/// نمط تعلم من المستخدم
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPattern {
    /// النص الأصلي
    pub input_pattern: String,
    /// الكود المقابل
    pub code_pattern: String,
    /// عدد مرات الاستخدام
    pub usage_count: u32,
    /// نوع التعلم
    pub learning_type: LearningType,
    /// آخر استخدام
    pub last_used: u64,
}

/// سياق التنفيذ الحالي
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    /// المتغيرات المعرفة
    pub variables: HashMap<String, String>,
    /// الدوال المعرفة
    pub functions: HashMap<String, Vec<String>>,
    /// النطاق الحالي
    pub current_scope: ContextType,
    /// اسم الدالة الحالية إن وجد
    pub current_function: Option<String>,
    /// مستوى التداخل
    pub nesting_level: usize,
    /// آخر نية
    pub last_intent: Option<IntentType>,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            current_scope: ContextType::GlobalScope,
            current_function: None,
            nesting_level: 0,
            last_intent: None,
        }
    }
}

/// نتيجة التحليل الدلالي
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticAnalysis {
    /// الكيانات المستخرجة
    pub entities: Vec<SemanticEntity>,
    /// العلاقات بين الكيانات
    pub relations: Vec<SemanticRelation>,
    /// البنية العامة
    pub structure: SentenceStructure,
    /// المعنى المستنتج
    pub meaning: String,
    /// مستوى التعقيد
    pub complexity: ComplexityLevel,
}

/// كيان دلالي
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticEntity {
    /// نوع الكيان
    pub entity_type: SemanticEntityType,
    /// القيمة
    pub value: String,
    /// الموقع في النص
    pub position: (usize, usize),
    /// الثقة
    pub confidence: f64,
}

/// نوع الكيان الدلالي
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SemanticEntityType {
    /// اسم متغير
    VariableName,
    /// اسم دالة
    FunctionName,
    /// قيمة رقمية
    NumericValue,
    /// قيمة نصية
    StringValue,
    /// عملية
    Operation,
    /// شرط
    Condition,
    /// معامل
    Parameter,
    /// نوع بيانات
    DataType,
    /// كلمة ربط
    Connector,
    /// كلمة مفتاحية
    Keyword,
}

/// علاقة دلالية
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticRelation {
    /// نوع العلاقة
    pub relation_type: RelationType,
    /// الكيان المصدر
    pub source: usize,
    /// الكيان الهدف
    pub target: usize,
}

/// نوع العلاقة
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RelationType {
    /// يساوي
    Equals,
    /// يحتوي
    Contains,
    /// يستدعي
    Calls,
    /// يتبع
    Follows,
    /// شرط لـ
    ConditionFor,
    /// معامل لـ
    ParameterOf,
    /// قيمة لـ
    ValueOf,
}

/// مستوى التعقيد
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ComplexityLevel {
    /// بسيط
    Simple,
    /// متوسط
    Medium,
    /// معقد
    Complex,
    /// متعدد المكونات
    MultiPart,
}

/// بنية الجملة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceStructure {
    /// نوع الجملة
    pub sentence_type: SentenceType,
    /// الفعل الرئيسي
    pub main_action: Option<String>,
    /// الفاعل
    pub subject: Option<String>,
    /// المفعول
    pub object: Option<String>,
    /// التفاصيل الإضافية
    pub details: Vec<String>,
}

/// نوع الجملة
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SentenceType {
    /// أمر
    Imperative,
    /// شرطية
    Conditional,
    /// استفهامية
    Interrogative,
    /// خبرية
    Declarative,
    /// تكرارية
    Iterative,
    /// مركبة
    Compound,
}

/// قالب كود ذكي
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeTemplate {
    /// اسم القالب
    pub name: String,
    /// الوصف
    pub description: String,
    /// الكود
    pub template: String,
    /// المعاملات المطلوبة
    pub required_params: Vec<String>,
    /// المعاملات الاختيارية
    pub optional_params: Vec<String>,
    /// الكلمات المفتاحية للتفعيل
    pub trigger_keywords: Vec<String>,
    /// الأولوية
    pub priority: u8,
}

/// نتيجة Vibe Coding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibeResult {
    /// النص الأصلي
    pub input: String,
    /// النوايا المُكتشفة
    pub intents: Vec<DetectedIntent>,
    /// الكود المُنتج
    pub code: String,
    /// الشرح
    pub explanation: String,
    /// اقتراحات التحسين
    pub improvements: Vec<String>,
    /// الثقة العامة
    pub overall_confidence: f64,
    /// نجح أم لا
    pub success: bool,
}

// ═══════════════════════════════════════════════════════════════════════════════
// المحرك الرئيسي - VibeCodingEngine
// ═══════════════════════════════════════════════════════════════════════════════

/// محرك Vibe Coding المتقدم
pub struct VibeCodingEngine {
    /// تاريخ المحادثة
    conversation_history: VecDeque<ConversationItem>,
    /// أنماط المستخدم
    user_patterns: HashMap<String, UserPattern>,
    /// سياق التنفيذ
    context: ExecutionContext,
    /// قوالب الكود
    templates: HashMap<String, CodeTemplate>,
    /// الكلمات المفتاحية
    keywords: HashMap<String, (IntentType, f64)>,
    /// المرادفات
    synonyms: HashMap<String, Vec<String>>,
    /// إحصائيات
    stats: EngineStats,
}

/// إحصائيات المحرك
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EngineStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub average_confidence: f64,
    pub patterns_learned: usize,
    pub cache_hits: u64,
}

impl VibeCodingEngine {
    /// إنشاء محرك جديد
    pub fn new() -> Self {
        let mut engine = VibeCodingEngine {
            conversation_history: VecDeque::with_capacity(MAX_CONVERSATION_HISTORY),
            user_patterns: HashMap::with_capacity(MAX_PATTERNS),
            context: ExecutionContext::default(),
            templates: HashMap::new(),
            keywords: HashMap::new(),
            synonyms: HashMap::new(),
            stats: EngineStats::default(),
        };
        engine.initialize();
        engine
    }

    /// تهيئة المحرك
    fn initialize(&mut self) {
        self.load_keywords();
        self.load_synonyms();
        self.load_templates();
    }

    /// تحميل الكلمات المفتاحية
    fn load_keywords(&mut self) {
        // كلمات إنشاء المتغيرات
        self.add_keyword("أنشئ متغير", IntentType::CreateVariable, 0.95);
        self.add_keyword("عرف متغير", IntentType::CreateVariable, 0.95);
        self.add_keyword("متغير جديد", IntentType::CreateVariable, 0.90);
        self.add_keyword("أضف متغير", IntentType::CreateVariable, 0.85);

        // كلمات إنشاء الدوال
        self.add_keyword("أنشئ دالة", IntentType::CreateFunction, 0.95);
        self.add_keyword("عرف دالة", IntentType::CreateFunction, 0.95);
        self.add_keyword("دالة جديدة", IntentType::CreateFunction, 0.90);
        self.add_keyword("وظيفة", IntentType::CreateFunction, 0.80);

        // كلمات الشرط
        self.add_keyword("إذا", IntentType::CreateCondition, 0.95);
        self.add_keyword("لو", IntentType::CreateCondition, 0.90);
        self.add_keyword("في حالة", IntentType::CreateCondition, 0.85);
        self.add_keyword("شرط", IntentType::CreateCondition, 0.80);

        // كلمات الحلقات
        self.add_keyword("كرر", IntentType::CreateLoop, 0.95);
        self.add_keyword("طالما", IntentType::CreateLoop, 0.95);
        self.add_keyword("لكل", IntentType::CreateLoop, 0.90);
        self.add_keyword("حلقة", IntentType::CreateLoop, 0.85);

        // كلمات الطباعة
        self.add_keyword("اطبع", IntentType::CreatePrint, 0.95);
        self.add_keyword("اعرض", IntentType::CreatePrint, 0.90);
        self.add_keyword("اكتب", IntentType::CreatePrint, 0.85);
        self.add_keyword("أظهر", IntentType::CreatePrint, 0.80);

        // كلمات القوائم
        self.add_keyword("قائمة", IntentType::CreateList, 0.90);
        self.add_keyword("مصفوفة", IntentType::CreateList, 0.90);
        self.add_keyword("قائمة جديدة", IntentType::CreateList, 0.95);

        // كلمات العمليات
        self.add_keyword("اجمع", IntentType::Arithmetic, 0.90);
        self.add_keyword("اطرح", IntentType::Arithmetic, 0.90);
        self.add_keyword("اضرب", IntentType::Arithmetic, 0.90);
        self.add_keyword("اقسم", IntentType::Arithmetic, 0.90);

        // كلمات الإرجاع
        self.add_keyword("أعطِ", IntentType::Return, 0.95);
        self.add_keyword("أرجع", IntentType::Return, 0.90);
        self.add_keyword("عد", IntentType::Return, 0.85);

        // كلمات التصدير
        self.add_keyword("صدر", IntentType::ExportProgram, 0.95);
        self.add_keyword("بنِ", IntentType::ExportProgram, 0.85);
        self.add_keyword("حوّل", IntentType::ExportProgram, 0.80);
    }

    /// إضافة كلمة مفتاحية
    fn add_keyword(&mut self, keyword: &str, intent: IntentType, confidence: f64) {
        self.keywords.insert(keyword.to_string(), (intent, confidence));
    }

    /// تحميل المرادفات
    fn load_synonyms(&mut self) {
        // مرادفات الإنشاء
        self.synonyms.insert("أنشئ".to_string(), vec!["عرف".to_string(), "أضف".to_string(), "خلق".to_string()]);
        
        // مرادفات المتغير
        self.synonyms.insert("متغير".to_string(), vec!["خانة".to_string(), "عنصر".to_string(), "حاوية".to_string()]);
        
        // مرادفات الدالة
        self.synonyms.insert("دالة".to_string(), vec!["وظيفة".to_string(), "إجراء".to_string(), "طريقة".to_string()]);
        
        // مرادفات الطباعة
        self.synonyms.insert("اطبع".to_string(), vec!["اعرض".to_string(), "اكتب".to_string(), "أظهر".to_string()]);
        
        // مرادفات الشرط
        self.synonyms.insert("إذا".to_string(), vec!["لو".to_string(), "في حالة".to_string(), "عندما".to_string()]);
        
        // مرادفات التكرار
        self.synonyms.insert("كرر".to_string(), vec!["ردد".to_string(), "أعد".to_string(), "حلّق".to_string()]);
    }

    /// تحميل القوالب
    fn load_templates(&mut self) {
        // قالب المتغير
        self.add_template(CodeTemplate {
            name: "variable_basic".to_string(),
            description: "إنشاء متغير بسيط".to_string(),
            template: "متغير {{name}} = {{value}}؛".to_string(),
            required_params: vec!["name".to_string(), "value".to_string()],
            optional_params: vec!["type".to_string()],
            trigger_keywords: vec!["متغير".to_string(), "أنشئ متغير".to_string()],
            priority: 10,
        });

        // قالب الدالة
        self.add_template(CodeTemplate {
            name: "function_basic".to_string(),
            description: "إنشاء دالة بسيطة".to_string(),
            template: "دالة {{name}}({{params}}) {\n    {{body}}\n}".to_string(),
            required_params: vec!["name".to_string()],
            optional_params: vec!["params".to_string(), "body".to_string()],
            trigger_keywords: vec!["دالة".to_string(), "أنشئ دالة".to_string()],
            priority: 10,
        });

        // قالب الشرط
        self.add_template(CodeTemplate {
            name: "condition_basic".to_string(),
            description: "إنشاء شرط بسيط".to_string(),
            template: "إذا {{condition}} {\n    {{body}}\n}".to_string(),
            required_params: vec!["condition".to_string()],
            optional_params: vec!["else_body".to_string()],
            trigger_keywords: vec!["إذا".to_string(), "لو".to_string()],
            priority: 10,
        });

        // قالب الحلقة
        self.add_template(CodeTemplate {
            name: "loop_while".to_string(),
            description: "حلقة طالما".to_string(),
            template: "طالما {{condition}} {\n    {{body}}\n}".to_string(),
            required_params: vec!["condition".to_string()],
            optional_params: vec!["body".to_string()],
            trigger_keywords: vec!["طالما".to_string(), "حلقة".to_string()],
            priority: 10,
        });

        // قالب الطباعة
        self.add_template(CodeTemplate {
            name: "print_basic".to_string(),
            description: "طباعة نص".to_string(),
            template: "اطبع(\"{{text}}\")؛".to_string(),
            required_params: vec!["text".to_string()],
            optional_params: vec![],
            trigger_keywords: vec!["اطبع".to_string(), "اعرض".to_string()],
            priority: 10,
        });

        // قوالب متقدمة
        self.add_template(CodeTemplate {
            name: "function_add".to_string(),
            description: "دالة جمع رقمين".to_string(),
            template: "دالة اجمع(أ، ب) {\n    أعطِ أ + ب؛\n}".to_string(),
            required_params: vec![],
            optional_params: vec![],
            trigger_keywords: vec!["دالة جمع".to_string(), "تضيف رقمين".to_string()],
            priority: 15,
        });

        self.add_template(CodeTemplate {
            name: "function_multiply".to_string(),
            description: "دالة ضرب رقمين".to_string(),
            template: "دالة اضرب(أ، ب) {\n    أعطِ أ * ب؛\n}".to_string(),
            required_params: vec![],
            optional_params: vec![],
            trigger_keywords: vec!["دالة ضرب".to_string(), "تضرب رقمين".to_string()],
            priority: 15,
        });

        // قالب حلقة التكرار مع عداد
        self.add_template(CodeTemplate {
            name: "loop_repeat".to_string(),
            description: "تكرار عدد محدد من المرات".to_string(),
            template: "متغير {{counter}} = 0؛\nطالما {{counter}} < {{count}} {\n    {{body}}\n    {{counter}} = {{counter}} + 1؛\n}".to_string(),
            required_params: vec!["count".to_string()],
            optional_params: vec!["counter".to_string(), "body".to_string()],
            trigger_keywords: vec!["كرر".to_string(), "تكرار".to_string()],
            priority: 12,
        });

        // قالب التصدير
        self.add_template(CodeTemplate {
            name: "export_program".to_string(),
            description: "تصدير البرنامج".to_string(),
            template: "صدر البرنامج \"{{name}}\" على {{platform}}؛".to_string(),
            required_params: vec!["name".to_string()],
            optional_params: vec!["platform".to_string()],
            trigger_keywords: vec!["صدر".to_string(), "بنِ".to_string()],
            priority: 10,
        });

        // قالب القائمة
        self.add_template(CodeTemplate {
            name: "list_basic".to_string(),
            description: "إنشاء قائمة".to_string(),
            template: "قائمة {{name}} = [{{items}}]؛".to_string(),
            required_params: vec!["name".to_string()],
            optional_params: vec!["items".to_string()],
            trigger_keywords: vec!["قائمة".to_string(), "مصفوفة".to_string()],
            priority: 10,
        });

        // قالب معالجة الأخطاء
        self.add_template(CodeTemplate {
            name: "try_catch".to_string(),
            description: "معالجة الأخطاء".to_string(),
            template: "حاول {\n    {{try_body}}\n} أمسك {{error}} {\n    {{catch_body}}\n}".to_string(),
            required_params: vec!["try_body".to_string()],
            optional_params: vec!["error".to_string(), "catch_body".to_string()],
            trigger_keywords: vec!["حاول".to_string(), "معالجة خطأ".to_string()],
            priority: 10,
        });
    }

    /// إضافة قالب
    fn add_template(&mut self, template: CodeTemplate) {
        self.templates.insert(template.name.clone(), template);
    }

    // ═══════════════════════════════════════════════════════════════
    // التحليل الدلالي
    // ═══════════════════════════════════════════════════════════════

    /// تحليل دلالي للنص
    pub fn analyze_semantics(&self, text: &str) -> SemanticAnalysis {
        // استخراج الكيانات
        let entities = self.extract_entities(text);
        
        // تحليل العلاقات
        let relations = self.analyze_relations(&entities);
        
        // تحليل البنية
        let structure = self.analyze_sentence_structure(text, &entities);
        
        // استنتاج المعنى
        let meaning = self.infer_meaning(&entities, &relations, &structure);
        
        // تحديد التعقيد
        let complexity = self.determine_complexity(&entities, &relations);

        SemanticAnalysis {
            entities,
            relations,
            structure,
            meaning,
            complexity,
        }
    }

    /// استخراج الكيانات
    fn extract_entities(&self, text: &str) -> Vec<SemanticEntity> {
        let mut entities = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut position = 0;

        for (i, word) in words.iter().enumerate() {
            let word_lower = word.to_lowercase();
            let start = position;
            let end = position + word.len();
            position = end + 1;

            // تحقق من الكلمات المفتاحية
            if self.keywords.contains_key(&word_lower) || 
               self.keywords.keys().any(|k| word_lower.contains(k)) {
                entities.push(SemanticEntity {
                    entity_type: SemanticEntityType::Keyword,
                    value: word.to_string(),
                    position: (start, end),
                    confidence: 0.95,
                });
                continue;
            }

            // تحقق من الأرقام
            if let Ok(num) = word.parse::<f64>() {
                entities.push(SemanticEntity {
                    entity_type: SemanticEntityType::NumericValue,
                    value: num.to_string(),
                    position: (start, end),
                    confidence: 1.0,
                });
                continue;
            }

            // تحقق من الأرقام العربية
            if let Ok(num) = self.parse_arabic_number(word) {
                entities.push(SemanticEntity {
                    entity_type: SemanticEntityType::NumericValue,
                    value: num.to_string(),
                    position: (start, end),
                    confidence: 1.0,
                });
                continue;
            }

            // تحقق من النصوص بين علامات تنصيص
            if word.starts_with('"') || word.starts_with('\'') || 
               word.starts_with('"') || word.starts_with('"') {
                entities.push(SemanticEntity {
                    entity_type: SemanticEntityType::StringValue,
                    value: word.trim_matches(|c| c == '"' || c == '\'' || c == '"' || c == '"').to_string(),
                    position: (start, end),
                    confidence: 0.95,
                });
                continue;
            }

            // تحقق من العمليات
            if ["+", "-", "*", "/", "=", ">", "<", "أكبر", "أصغر", "يساوي"].contains(&word_lower.as_str()) {
                entities.push(SemanticEntity {
                    entity_type: SemanticEntityType::Operation,
                    value: word.to_string(),
                    position: (start, end),
                    confidence: 0.9,
                });
                continue;
            }

            // كلمات الربط
            if ["و", "ثم", "بعد", "قبل", "مع", "من", "إلى", "في", "على"].contains(&word_lower.as_str()) {
                entities.push(SemanticEntity {
                    entity_type: SemanticEntityType::Connector,
                    value: word.to_string(),
                    position: (start, end),
                    confidence: 0.95,
                });
                continue;
            }

            // باقي الكلمات - قد تكون أسماء متغيرات أو دوال
            let entity_type = self.infer_entity_type_from_context(&words, i);
            entities.push(SemanticEntity {
                entity_type,
                value: word.to_string(),
                position: (start, end),
                confidence: 0.6,
            });
        }

        entities
    }

    /// تحليل الرقم العربي
    fn parse_arabic_number(&self, s: &str) -> Result<f64, ()> {
        let converted: String = s.chars().map(|c| {
            match c {
                '٠' => '0', '١' => '1', '٢' => '2', '٣' => '3', '٤' => '4',
                '٥' => '5', '٦' => '6', '٧' => '7', '٨' => '8', '٩' => '9',
                '۰' => '0', '۱' => '1', '۲' => '2', '۳' => '3', '۴' => '4',
                '۵' => '5', '۶' => '6', '۷' => '7', '۸' => '8', '۹' => '9',
                c => c,
            }
        }).collect();

        // الكلمات الرقمية
        let word_numbers = [
            ("صفر", 0.0), ("واحد", 1.0), ("اثنان", 2.0), ("اثنين", 2.0),
            ("ثلاثة", 3.0), ("ثلاث", 3.0), ("أربعة", 4.0), ("أربع", 4.0),
            ("خمسة", 5.0), ("خمس", 5.0), ("ستة", 6.0), ("ست", 6.0),
            ("سبعة", 7.0), ("سبع", 7.0), ("ثمانية", 8.0), ("ثمان", 8.0),
            ("تسعة", 9.0), ("تسع", 9.0), ("عشرة", 10.0), ("عشر", 10.0),
            ("مئة", 100.0), ("مائة", 100.0), ("ألف", 1000.0),
        ];

        for (word, num) in word_numbers {
            if converted == word {
                return Ok(num);
            }
        }

        converted.parse().map_err(|_| ())
    }

    /// استنتاج نوع الكيان من السياق
    fn infer_entity_type_from_context(&self, words: &[&str], index: usize) -> SemanticEntityType {
        // الكلمة السابقة
        if index > 0 {
            let prev = words[index - 1].to_lowercase();
            if ["متغير", "ثابت", "عرف", "أنشئ"].contains(&prev.as_str()) {
                return SemanticEntityType::VariableName;
            }
            if ["دالة", "وظيفة"].contains(&prev.as_str()) {
                return SemanticEntityType::FunctionName;
            }
            if ["يساوي", "=", "بقيمة"].contains(&prev.as_str()) {
                return SemanticEntityType::NumericValue;
            }
        }

        // الكلمة التالية
        if index + 1 < words.len() {
            let next = words[index + 1].to_lowercase();
            if ["يساوي", "="].contains(&next.as_str()) {
                return SemanticEntityType::VariableName;
            }
            if next == "(" {
                return SemanticEntityType::FunctionName;
            }
        }

        SemanticEntityType::VariableName
    }

    /// تحليل العلاقات
    fn analyze_relations(&self, entities: &[SemanticEntity]) -> Vec<SemanticRelation> {
        let mut relations = Vec::new();

        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let e1 = &entities[i];
                let e2 = &entities[j];

                // تحقق من علاقة "يساوي"
                if e1.entity_type == SemanticEntityType::VariableName &&
                   e2.entity_type == SemanticEntityType::NumericValue {
                    relations.push(SemanticRelation {
                        relation_type: RelationType::ValueOf,
                        source: i,
                        target: j,
                    });
                }

                // تحقق من علاقة "معامل لـ"
                if e1.entity_type == SemanticEntityType::Parameter &&
                   e2.entity_type == SemanticEntityType::FunctionName {
                    relations.push(SemanticRelation {
                        relation_type: RelationType::ParameterOf,
                        source: i,
                        target: j,
                    });
                }
            }
        }

        relations
    }

    /// تحليل بنية الجملة
    fn analyze_sentence_structure(&self, text: &str, entities: &[SemanticEntity]) -> SentenceStructure {
        let text_lower = text.to_lowercase();

        // تحديد نوع الجملة
        let sentence_type = if text_lower.contains("إذا") || text_lower.contains("لو") {
            SentenceType::Conditional
        } else if text_lower.contains("كرر") || text_lower.contains("طالما") || text_lower.contains("لكل") {
            SentenceType::Iterative
        } else if text_lower.contains("هل") || text_lower.contains("ما") || text_lower.contains("كم") {
            SentenceType::Interrogative
        } else if text_lower.contains("و") || text_lower.contains("ثم") {
            SentenceType::Compound
        } else if text_lower.starts_with("أنشئ") || text_lower.starts_with("عرف") || 
                  text_lower.starts_with("اطبع") || text_lower.starts_with("اعرض") {
            SentenceType::Imperative
        } else {
            SentenceType::Declarative
        };

        // استخراج الفعل الرئيسي
        let main_action = entities.iter()
            .find(|e| e.entity_type == SemanticEntityType::Keyword)
            .map(|e| e.value.clone());

        // استخراج الفاعل (عادة اسم المتغير أو الدالة)
        let subject = entities.iter()
            .find(|e| e.entity_type == SemanticEntityType::VariableName ||
                       e.entity_type == SemanticEntityType::FunctionName)
            .map(|e| e.value.clone());

        // استخراج المفعول (القيمة)
        let object = entities.iter()
            .find(|e| e.entity_type == SemanticEntityType::NumericValue ||
                       e.entity_type == SemanticEntityType::StringValue)
            .map(|e| e.value.clone());

        // استخراج التفاصيل
        let details: Vec<String> = entities.iter()
            .filter(|e| e.entity_type == SemanticEntityType::Operation ||
                       e.entity_type == SemanticEntityType::Condition)
            .map(|e| e.value.clone())
            .collect();

        SentenceStructure {
            sentence_type,
            main_action,
            subject,
            object,
            details,
        }
    }

    /// استنتاج المعنى
    fn infer_meaning(&self, entities: &[SemanticEntity], relations: &[SemanticRelation], structure: &SentenceStructure) -> String {
        let mut meaning_parts = Vec::new();

        // الفعل الرئيسي
        if let Some(action) = &structure.main_action {
            let action_meaning = match action.as_str() {
                "أنشئ" | "عرف" => "إنشاء",
                "اطبع" | "اعرض" => "طباعة",
                "إذا" | "لو" => "شرط",
                "كرر" | "طالما" => "تكرار",
                "دالة" => "تعريف دالة",
                _ => action,
            };
            meaning_parts.push(action_meaning.to_string());
        }

        // الفاعل
        if let Some(subject) = &structure.subject {
            meaning_parts.push(format!("المسمى '{}'", subject));
        }

        // المفعول
        if let Some(obj) = &structure.object {
            meaning_parts.push(format!("بقيمة {}", obj));
        }

        // العلاقات
        for rel in relations {
            if rel.relation_type == RelationType::ValueOf {
                let source = entities.get(rel.source).map(|e| e.value.as_str()).unwrap_or("");
                let target = entities.get(rel.target).map(|e| e.value.as_str()).unwrap_or("");
                meaning_parts.push(format!("{} يساوي {}", source, target));
            }
        }

        if meaning_parts.is_empty() {
            "لم يتم فهم المعنى".to_string()
        } else {
            meaning_parts.join(" ")
        }
    }

    /// تحديد التعقيد
    fn determine_complexity(&self, entities: &[SemanticEntity], relations: &[SemanticRelation]) -> ComplexityLevel {
        let entity_count = entities.len();
        let relation_count = relations.len();

        if entity_count <= 3 && relation_count == 0 {
            ComplexityLevel::Simple
        } else if entity_count <= 6 && relation_count <= 2 {
            ComplexityLevel::Medium
        } else if entity_count <= 10 {
            ComplexityLevel::Complex
        } else {
            ComplexityLevel::MultiPart
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // كشف النوايا
    // ═══════════════════════════════════════════════════════════════

    /// كشف النوايا من النص
    pub fn detect_intents(&self, text: &str) -> Vec<DetectedIntent> {
        let mut intents = Vec::new();
        let text_lower = text.to_lowercase();

        // 1. البحث عن أنماط مركبة أولاً
        let compound_intents = self.detect_compound_intents(text);
        if !compound_intents.is_empty() {
            return compound_intents;
        }

        // 2. البحث في الأنماط المحفوظة للمستخدم
        if let Some(pattern) = self.find_user_pattern(text) {
            let mut intent = DetectedIntent::new(IntentType::Unknown, 0.95);
            intent.generated_code = Some(pattern.code_pattern.clone());
            intent.explanation = Some(format!("استخدام نمط محفوظ: {}", pattern.input_pattern));
            intents.push(intent);
            return intents;
        }

        // 3. البحث عن قوالب مطابقة
        for template in self.templates.values() {
            if template.trigger_keywords.iter().any(|k| text_lower.contains(k)) {
                if let Some(intent) = self.match_template(text, template) {
                    intents.push(intent);
                }
            }
        }

        // 4. البحث عن كلمات مفتاحية
        for (keyword, (intent_type, confidence)) in &self.keywords {
            if text_lower.contains(keyword) {
                let mut intent = DetectedIntent::new(intent_type.clone(), *confidence);
                self.extract_intent_params(text, &mut intent);
                intents.push(intent);
            }
        }

        // 5. إزالة التكرارات وترتيب حسب الثقة
        intents.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        intents.dedup_by(|a, b| a.intent_type == b.intent_type);

        // 6. توليد الكود للنوايا
        for intent in &mut intents {
            if intent.generated_code.is_none() {
                intent.generated_code = Some(self.generate_code_for_intent(intent));
            }
            if intent.explanation.is_none() {
                intent.explanation = Some(self.explain_intent(intent));
            }
        }

        intents
    }

    /// كشف النوايا المركبة
    fn detect_compound_intents(&self, text: &str) -> Vec<DetectedIntent> {
        let mut intents = Vec::new();

        // تقسيم النص بواسطة " و " (مع مسافات) لتجنب تقسيم الكلمات التي تحتوي على حرف الواو
        // أو بواسطة الفاصلة
        let mut parts = Vec::new();
        let mut current_part = String::new();
        let chars: Vec<char> = text.chars().collect();
        
        for i in 0..chars.len() {
            let ch = chars[i];
            
            // تحقق من " و " (واو مع مسافات حولها)
            if ch == 'و' {
                let prev_is_space = i > 0 && chars[i - 1] == ' ';
                let next_is_space = i + 1 < chars.len() && chars[i + 1] == ' ';
                
                if prev_is_space && next_is_space {
                    // هذا واو عطف - نقسم هنا
                    if !current_part.trim().is_empty() {
                        parts.push(current_part.trim().to_string());
                    }
                    current_part = String::new();
                    continue;
                }
            }
            
            // تحقق من الفاصلة
            if ch == ',' {
                if !current_part.trim().is_empty() {
                    parts.push(current_part.trim().to_string());
                }
                current_part = String::new();
                continue;
            }
            
            current_part.push(ch);
        }
        
        // إضافة الجزء الأخير
        if !current_part.trim().is_empty() {
            parts.push(current_part.trim().to_string());
        }

        if parts.len() > 1 {
            let mut sub_intents = Vec::new();
            for part in &parts {
                let part_intents = self.detect_intents(part);
                sub_intents.extend(part_intents);
            }

            if !sub_intents.is_empty() {
                let mut compound_intent = DetectedIntent::new(
                    IntentType::Compound(sub_intents.iter().map(|i| i.intent_type.clone()).collect()),
                    0.85
                );
                
                let code: String = sub_intents.iter()
                    .filter_map(|i| i.generated_code.as_ref())
                    .cloned()
                    .collect::<Vec<_>>()
                    .join("\n");
                
                compound_intent.generated_code = Some(code);
                compound_intent.explanation = Some(format!("تنفيذ {} عمليات متتالية", sub_intents.len()));
                compound_intent.alternatives = sub_intents;
                
                intents.push(compound_intent);
            }
        }

        intents
    }

    /// البحث عن نمط مستخدم
    fn find_user_pattern(&self, text: &str) -> Option<UserPattern> {
        // البحث عن تطابق جزئي
        for (pattern, user_pattern) in &self.user_patterns {
            if text.contains(pattern) || pattern.contains(&text.to_lowercase()) {
                return Some(user_pattern.clone());
            }
        }
        None
    }

    /// مطابقة قالب
    fn match_template(&self, text: &str, template: &CodeTemplate) -> Option<DetectedIntent> {
        let mut intent = DetectedIntent::new(
            self.template_to_intent_type(template),
            0.9
        );

        // استخراج المعاملات من النص
        for param in &template.required_params {
            if let Some(value) = self.extract_param(text, param) {
                intent.params.insert(param.clone(), value);
            }
        }

        for param in &template.optional_params {
            if let Some(value) = self.extract_param(text, param) {
                intent.params.insert(param.clone(), value);
            }
        }

        // تطبيق القالب
        let code = self.apply_template(template, &intent.params);
        intent.generated_code = Some(code);

        Some(intent)
    }

    /// تحويل القالب إلى نوع النية
    fn template_to_intent_type(&self, template: &CodeTemplate) -> IntentType {
        match template.name.as_str() {
            "variable_basic" => IntentType::CreateVariable,
            "function_basic" | "function_add" | "function_multiply" => IntentType::CreateFunction,
            "condition_basic" => IntentType::CreateCondition,
            "loop_while" | "loop_repeat" => IntentType::CreateLoop,
            "print_basic" => IntentType::CreatePrint,
            "list_basic" => IntentType::CreateList,
            "export_program" => IntentType::ExportProgram,
            "try_catch" => IntentType::ErrorHandling,
            _ => IntentType::Unknown,
        }
    }

    /// استخراج معامل من النص
    fn extract_param(&self, text: &str, param: &str) -> Option<String> {
        let text_lower = text.to_lowercase();

        match param {
            "name" => {
                // استخراج الاسم
                let words: Vec<&str> = text.split_whitespace().collect();
                for (i, word) in words.iter().enumerate() {
                    let word_lower = word.to_lowercase();
                    if ["متغير", "دالة", "قائمة"].contains(&word_lower.as_str()) {
                        if let Some(name) = words.get(i + 1) {
                            if !["يساوي", "بقيمة", "التي", "تقوم"].contains(name) {
                                return Some(name.to_string());
                            }
                        }
                    }
                }
                Some("س".to_string()) // اسم افتراضي
            }
            "value" => {
                // استخراج القيمة
                if let Some(pos) = text_lower.find("يساوي") {
                    let after = &text[pos + "يساوي".len()..].trim();
                    if let Some(word) = after.split_whitespace().next() {
                        return Some(word.to_string());
                    }
                }
                Some("0".to_string())
            }
            "text" => {
                // استخراج النص للطباعة
                let text = text
                    .replace("اطبع", "")
                    .replace("اعرض", "")
                    .replace("اكتب", "")
                    .replace("رسالة", "")
                    .trim()
                    .to_string();
                Some(text)
            }
            "condition" => {
                // استخراج الشرط
                if text_lower.contains("أكبر من") {
                    let num = self.extract_number(text);
                    return Some(format!("س > {}", num));
                } else if text_lower.contains("أصغر من") {
                    let num = self.extract_number(text);
                    return Some(format!("س < {}", num));
                } else if text_lower.contains("يساوي") {
                    let num = self.extract_number(text);
                    return Some(format!("س == {}", num));
                }
                Some("صحيح".to_string())
            }
            "count" => {
                // استخراج عدد التكرارات
                Some(self.extract_number(text).to_string())
            }
            "params" => Some("".to_string()),
            "body" => Some("اطبع(\"تم\")؛".to_string()),
            "platform" => {
                if text_lower.contains("ويندوز") || text_lower.contains("windows") {
                    Some("windows".to_string())
                } else if text_lower.contains("لينكس") || text_lower.contains("linux") {
                    Some("linux".to_string())
                } else if text_lower.contains("ويب") || text_lower.contains("web") {
                    Some("web".to_string())
                } else {
                    Some("windows".to_string())
                }
            }
            "items" => Some("1، 2، 3".to_string()),
            _ => None,
        }
    }

    /// استخراج رقم من النص
    fn extract_number(&self, text: &str) -> i64 {
        let text_lower = text.to_lowercase();

        // الأرقام بالكلمات
        if text_lower.contains("واحد") || text_lower.contains("مرة واحدة") { return 1; }
        if text_lower.contains("اثنين") || text_lower.contains("مرتين") || text_lower.contains("2") { return 2; }
        if text_lower.contains("ثلاث") || text_lower.contains("3") { return 3; }
        if text_lower.contains("أربع") || text_lower.contains("4") { return 4; }
        if text_lower.contains("خمس") || text_lower.contains("5") { return 5; }
        if text_lower.contains("ست") || text_lower.contains("6") { return 6; }
        if text_lower.contains("سبع") || text_lower.contains("7") { return 7; }
        if text_lower.contains("ثمان") || text_lower.contains("8") { return 8; }
        if text_lower.contains("تسع") || text_lower.contains("9") { return 9; }
        if text_lower.contains("عشر") || text_lower.contains("10") { return 10; }

        // استخراج الرقم مباشرة
        text.chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap_or(1)
    }

    /// تطبيق القالب
    fn apply_template(&self, template: &CodeTemplate, params: &HashMap<String, String>) -> String {
        let mut result = template.template.clone();
        
        for (key, value) in params {
            result = result.replace(&format!("{{{{{}}}}}", key), value);
        }

        // معاملات افتراضية
        result = result.replace("{{counter}}", "ع");
        result = result.replace("{{body}}", "اطبع(\"تكرار\")؛");

        result
    }

    /// استخراج معاملات النية
    fn extract_intent_params(&self, text: &str, intent: &mut DetectedIntent) {
        let params = self.extract_all_params(text);
        intent.params = params;
    }

    /// استخراج جميع المعاملات
    fn extract_all_params(&self, text: &str) -> HashMap<String, String> {
        let mut params = HashMap::new();
        
        // استخراج الاسم
        if let Some(name) = self.extract_param(text, "name") {
            params.insert("name".to_string(), name);
        }
        
        // استخراج القيمة
        if let Some(value) = self.extract_param(text, "value") {
            params.insert("value".to_string(), value);
        }

        params
    }

    /// توليد الكود للنية
    fn generate_code_for_intent(&self, intent: &DetectedIntent) -> String {
        match &intent.intent_type {
            IntentType::CreateVariable => {
                let name = intent.params.get("name").map(|s| s.as_str()).unwrap_or("س");
                let value = intent.params.get("value").map(|s| s.as_str()).unwrap_or("0");
                format!("متغير {} = {}؛", name, value)
            }
            IntentType::CreateFunction => {
                let name = intent.params.get("name").map(|s| s.as_str()).unwrap_or("دالة_جديدة");
                let params = intent.params.get("params").map(|s| s.as_str()).unwrap_or("");
                let body = intent.params.get("body").map(|s| s.as_str()).unwrap_or("أعطِ لا_شيء؛");
                format!("دالة {}({}) {{\n    {}\n}}", name, params, body)
            }
            IntentType::CreateCondition => {
                let condition = intent.params.get("condition").map(|s| s.as_str()).unwrap_or("صحيح");
                let body = intent.params.get("body").map(|s| s.as_str()).unwrap_or("اطبع(\"تم\")؛");
                format!("إذا {} {{\n    {}\n}}", condition, body)
            }
            IntentType::CreateLoop => {
                let count = intent.params.get("count").map(|s| s.as_str()).unwrap_or("5");
                let body = intent.params.get("body").map(|s| s.as_str()).unwrap_or("اطبع(\"تكرار\")؛");
                format!("متغير ع = 0؛\nطالما ع < {} {{\n    {}\n    ع = ع + 1؛\n}}", count, body)
            }
            IntentType::CreatePrint => {
                let text = intent.params.get("text").map(|s| s.as_str()).unwrap_or("مرحبا");
                format!("اطبع(\"{}\")؛", text)
            }
            IntentType::Return => {
                let value = intent.params.get("value").map(|s| s.as_str()).unwrap_or("لا_شيء");
                format!("أعطِ {}؛", value)
            }
            IntentType::ExportProgram => {
                let name = intent.params.get("name").map(|s| s.as_str()).unwrap_or("myapp");
                let platform = intent.params.get("platform").map(|s| s.as_str()).unwrap_or("windows");
                format!("صدر البرنامج \"{}\" على {}؛", name, platform)
            }
            IntentType::CreateList => {
                let name = intent.params.get("name").map(|s| s.as_str()).unwrap_or("قائمة");
                let items = intent.params.get("items").map(|s| s.as_str()).unwrap_or("1، 2، 3");
                format!("قائمة {} = [{}]؛", name, items)
            }
            IntentType::Arithmetic => {
                let op = intent.params.get("operation").map(|s| s.as_str()).unwrap_or("+");
                format!("متغير نتيجة = أ {} ب؛", op)
            }
            IntentType::Compound(sub_intents) => {
                sub_intents.iter()
                    .map(|i| self.generate_code_for_intent(&DetectedIntent::new(i.clone(), 0.9)))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            _ => format!("// لم يتم فهم: {:?}", intent.intent_type),
        }
    }

    /// شرح النية
    fn explain_intent(&self, intent: &DetectedIntent) -> String {
        match &intent.intent_type {
            IntentType::CreateVariable => {
                let name = intent.params.get("name").map(|s| s.as_str()).unwrap_or("س");
                let value = intent.params.get("value").map(|s| s.as_str()).unwrap_or("0");
                format!("إنشاء متغير '{}' بقيمة {}", name, value)
            }
            IntentType::CreateFunction => {
                let name = intent.params.get("name").map(|s| s.as_str()).unwrap_or("دالة");
                format!("تعريف دالة '{}'", name)
            }
            IntentType::CreateCondition => {
                let condition = intent.params.get("condition").map(|s| s.as_str()).unwrap_or("صحيح");
                format!("تنفيذ شرط: {}", condition)
            }
            IntentType::CreateLoop => {
                let count = intent.params.get("count").map(|s| s.as_str()).unwrap_or("5");
                format!("تكرار {} مرات", count)
            }
            IntentType::CreatePrint => {
                let text = intent.params.get("text").map(|s| s.as_str()).unwrap_or("");
                format!("طباعة: {}", text)
            }
            IntentType::ExportProgram => {
                let name = intent.params.get("name").map(|s| s.as_str()).unwrap_or("myapp");
                format!("تصدير البرنامج: {}", name)
            }
            IntentType::Compound(_) => "تنفيذ عمليات متعددة".to_string(),
            _ => "نية غير معروفة".to_string(),
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // معالجة الطلب الرئيسي
    // ═══════════════════════════════════════════════════════════════

    /// معالجة نص Vibe Coding
    pub fn process(&mut self, text: &str) -> VibeResult {
        self.stats.total_requests += 1;

        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║          🚀 Vibe Coding المتقدم - لغة المرجع                 ║");
        println!("╚══════════════════════════════════════════════════════════════╝");

        // المرحلة 1: التحليل الدلالي
        println!("\n📝 المرحلة 1: التحليل الدلالي...");
        let analysis = self.analyze_semantics(text);
        println!("   المعنى: {}", analysis.meaning);
        println!("   التعقيد: {:?}", analysis.complexity);

        // المرحلة 2: كشف النوايا
        println!("\n🧠 المرحلة 2: كشف النوايا...");
        let intents = self.detect_intents(text);
        for (i, intent) in intents.iter().enumerate() {
            println!("   النية {}: {:?} (ثقة: {:.0}%)", i + 1, intent.intent_type, intent.confidence * 100.0);
        }

        // المرحلة 3: توليد الكود
        println!("\n⚙️ المرحلة 3: توليد الكود...");
        let code = self.generate_code(&intents);
        for line in code.lines() {
            println!("   │ {}", line);
        }

        // المرحلة 4: توليد الشرح
        println!("\n📚 المرحلة 4: توليد الشرح...");
        let explanation = self.generate_explanation(&intents, &analysis);
        println!("   {}", explanation);

        // المرحلة 5: اقتراحات التحسين
        println!("\n💡 المرحلة 5: اقتراحات التحسين...");
        let improvements = self.suggest_improvements(&code, &analysis);
        for imp in &improvements {
            println!("   • {}", imp);
        }

        // حساب الثقة العامة
        let overall_confidence = if intents.is_empty() {
            0.0
        } else {
            intents.iter().map(|i| i.confidence).sum::<f64>() / intents.len() as f64
        };

        let success = overall_confidence >= CONFIDENCE_THRESHOLD;

        // تحديث الإحصائيات
        if success {
            self.stats.successful_requests += 1;
        }
        self.stats.average_confidence = 
            (self.stats.average_confidence * (self.stats.total_requests - 1) as f64 + overall_confidence) 
            / self.stats.total_requests as f64;

        // إضافة للمحادثة
        self.add_to_conversation(text, &intents, &code);

        // التعلم من النمط
        self.learn_pattern(text, &code);

        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║     ✨ اكتملت المعالجة (ثقة: {:.0}%) ✨", overall_confidence * 100.0);
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        VibeResult {
            input: text.to_string(),
            intents,
            code,
            explanation,
            improvements,
            overall_confidence,
            success,
        }
    }

    /// توليد الكود من النوايا
    fn generate_code(&self, intents: &[DetectedIntent]) -> String {
        if intents.is_empty() {
            return "// لم يتم التعرف على أي نية".to_string();
        }

        intents.iter()
            .filter_map(|i| i.generated_code.as_ref())
            .cloned()
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    /// توليد الشرح
    fn generate_explanation(&self, intents: &[DetectedIntent], analysis: &SemanticAnalysis) -> String {
        let mut parts = Vec::new();

        parts.push(format!("📋 المعنى: {}", analysis.meaning));

        if !intents.is_empty() {
            parts.push("\n📝 العمليات:".to_string());
            for (i, intent) in intents.iter().enumerate() {
                if let Some(expl) = &intent.explanation {
                    parts.push(format!("   {}. {}", i + 1, expl));
                }
            }
        }

        parts.join("\n")
    }

    /// اقتراحات التحسين
    fn suggest_improvements(&self, code: &str, analysis: &SemanticAnalysis) -> Vec<String> {
        let mut suggestions = Vec::new();

        // اقتراحات بناءً على التعقيد
        match analysis.complexity {
            ComplexityLevel::Simple => {
                suggestions.push("يمكن إضافة تعليقات لتوضيح الكود".to_string());
            }
            ComplexityLevel::Medium => {
                suggestions.push("يمكن تقسيم الكود إلى دوال أصغر".to_string());
            }
            ComplexityLevel::Complex => {
                suggestions.push("يُنصح بإعادة هيكلة الكود إلى وحدات مستقلة".to_string());
            }
            ComplexityLevel::MultiPart => {
                suggestions.push("الكود معقد، يُنصح بتبسيطه أو تقسيمه".to_string());
            }
        }

        // اقتراحات بناءً على السياق
        if code.contains("طالما") && !code.contains("ع = ع + 1") {
            suggestions.push("تأكد من وجود عداد للحلقة لتجنب الحلقة اللانهائية".to_string());
        }

        if code.contains("متغير") && !code.contains("؛") {
            suggestions.push("أضف فاصلة منقوطة '؛' في نهاية كل جملة".to_string());
        }

        suggestions
    }

    /// إضافة للمحادثة
    fn add_to_conversation(&mut self, input: &str, intents: &[DetectedIntent], code: &str) {
        let item = ConversationItem {
            user_input: input.to_string(),
            intents: intents.to_vec(),
            generated_code: code.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            accepted: None,
        };

        if self.conversation_history.len() >= MAX_CONVERSATION_HISTORY {
            self.conversation_history.pop_front();
        }
        self.conversation_history.push_back(item);
    }

    /// التعلم من النمط
    fn learn_pattern(&mut self, input: &str, code: &str) {
        if self.user_patterns.len() >= MAX_PATTERNS {
            // إزالة الأقل استخداماً
            let mut entries: Vec<_> = self.user_patterns.iter()
                .map(|(k, v)| (k.clone(), v.usage_count))
                .collect();
            entries.sort_by_key(|(_, c)| *c);
            
            if let Some((key, _)) = entries.first() {
                self.user_patterns.remove(key);
            }
        }

        let key = input.to_lowercase();
        if let Some(pattern) = self.user_patterns.get_mut(&key) {
            pattern.usage_count += 1;
            pattern.last_used = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        } else {
            self.user_patterns.insert(key, UserPattern {
                input_pattern: input.to_string(),
                code_pattern: code.to_string(),
                usage_count: 1,
                learning_type: LearningType::WritingPattern,
                last_used: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            });
            self.stats.patterns_learned = self.user_patterns.len();
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // دوال إضافية
    // ═══════════════════════════════════════════════════════════════

    /// الحصول على سياق التنفيذ
    pub fn get_context(&self) -> &ExecutionContext {
        &self.context
    }

    /// تحديث السياق
    pub fn update_context(&mut self, context: ExecutionContext) {
        self.context = context;
    }

    /// الحصول على تاريخ المحادثة
    pub fn get_conversation_history(&self) -> &VecDeque<ConversationItem> {
        &self.conversation_history
    }

    /// مسح تاريخ المحادثة
    pub fn clear_conversation(&mut self) {
        self.conversation_history.clear();
    }

    /// الحصول على الأنماط المحفوظة
    pub fn get_user_patterns(&self) -> &HashMap<String, UserPattern> {
        &self.user_patterns
    }

    /// الحصول على الإحصائيات
    pub fn get_stats(&self) -> &EngineStats {
        &self.stats
    }

    /// شرح كود موجود
    pub fn explain_code(&self, code: &str) -> String {
        let mut explanations = Vec::new();

        // تحليل الكود سطراً بسطر
        for line in code.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            if line.starts_with("متغير") {
                if let Some(name) = line.split_whitespace().nth(1) {
                    explanations.push(format!("• '{}': تعريف متغير", name));
                }
            } else if line.starts_with("دالة") {
                if let Some(name) = line.split_whitespace().nth(1) {
                    explanations.push(format!("• '{}': تعريف دالة", name.split('(').next().unwrap_or(name)));
                }
            } else if line.starts_with("إذا") {
                explanations.push("• 'إذا': شرط - ينفذ الكود إذا تحقق الشرط".to_string());
            } else if line.starts_with("طالما") {
                explanations.push("• 'طالما': حلقة - تكرر الكود طالما الشرط صحيح".to_string());
            } else if line.starts_with("اطبع") {
                explanations.push("• 'اطبع': طباعة - تعرض النص في الخرج".to_string());
            } else if line.starts_with("أعطِ") {
                explanations.push("• 'أعطِ': إرجاع - تعيد قيمة من الدالة".to_string());
            } else if line.starts_with("صدر") {
                explanations.push("• 'صدر': تصدير - تصدر البرنامج للمنصة المحددة".to_string());
            }
        }

        if explanations.is_empty() {
            "لم يتم العثور على كود قابل للشرح".to_string()
        } else {
            explanations.join("\n")
        }
    }

    /// اقتراح إصلاح للخطأ
    pub fn suggest_fix(&self, error_code: &str, error_message: &str) -> Vec<String> {
        let mut fixes = Vec::new();

        let error_lower = error_message.to_lowercase();

        // أخطاء شائعة
        if error_lower.contains("متغير غير معرف") || error_lower.contains("undefined") {
            fixes.push("تأكد من تعريف المتغير قبل استخدامه".to_string());
            fixes.push("تحقق من تهجئة اسم المتغير".to_string());
        }

        if error_lower.contains("خطأ في الصيغة") || error_lower.contains("syntax") {
            fixes.push("تأكد من إغلاق جميع الأقواس".to_string());
            fixes.push("تأكد من وجود الفاصلة المنقوطة '؛' في نهاية كل جملة".to_string());
        }

        if error_lower.contains("نوع") || error_lower.contains("type") {
            fixes.push("تأكد من تطابق أنواع البيانات".to_string());
            fixes.push("قد تحتاج إلى تحويل النوع".to_string());
        }

        if error_lower.contains("دالة") || error_lower.contains("function") {
            fixes.push("تأكد من تعريف الدالة قبل استدعائها".to_string());
            fixes.push("تحقق من عدد المعاملات الممررة".to_string());
        }

        // اقتراحات خاصة بالكود
        if error_code.contains("طالما") && !error_code.contains("ع + 1") {
            fixes.push("أضف عداد للحلقة: ع = ع + 1؛ لتجنب الحلقة اللانهائية".to_string());
        }

        if fixes.is_empty() {
            fixes.push("حاول إعادة كتابة الكود بصيغة مختلفة".to_string());
        }

        fixes
    }

    /// الحصول على تتمة ذكية
    pub fn get_smart_completion(&self, partial_code: &str) -> Vec<String> {
        let mut completions = Vec::new();
        let partial_lower = partial_code.to_lowercase();

        // إكمالات بناءً على السياق
        if partial_lower.starts_with("متغير") {
            completions.push("متغير اسم = 0؛".to_string());
            completions.push("متغير نص = \"مرحبا\"؛".to_string());
        }

        if partial_lower.starts_with("دالة") {
            completions.push("دالة اسم_دالة() {\n    أعطِ لا_شيء؛\n}".to_string());
            completions.push("دالة اجمع(أ، ب) {\n    أعطِ أ + ب؛\n}".to_string());
        }

        if partial_lower.starts_with("إذا") {
            completions.push("إذا شرط {\n    // الكود هنا\n}".to_string());
            completions.push("إذا س > 10 {\n    اطبع(\"كبير\")؛\n}".to_string());
        }

        if partial_lower.starts_with("كرر") {
            completions.push("متغير ع = 0؛\nطالما ع < 5 {\n    // الكود هنا\n    ع = ع + 1؛\n}".to_string());
        }

        if partial_lower.starts_with("اطبع") {
            completions.push("اطبع(\"نص\");".to_string());
        }

        // إكمالات من الأنماط المحفوظة
        for pattern in self.user_patterns.values() {
            if pattern.input_pattern.to_lowercase().starts_with(&partial_lower) {
                completions.push(pattern.code_pattern.clone());
            }
        }

        completions.truncate(5); // أكبر 5 اقتراحات
        completions
    }
}

impl Default for VibeCodingEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال سهلة الاستخدام
// ═══════════════════════════════════════════════════════════════════════════════

/// معالجة نص Vibe Coding
pub fn vibe_process(text: &str) -> VibeResult {
    let mut engine = VibeCodingEngine::new();
    engine.process(text)
}

/// كشف النوايا من نص
pub fn vibe_detect_intents(text: &str) -> Vec<DetectedIntent> {
    let engine = VibeCodingEngine::new();
    engine.detect_intents(text)
}

/// شرح كود
pub fn vibe_explain_code(code: &str) -> String {
    let engine = VibeCodingEngine::new();
    engine.explain_code(code)
}

/// اقتراح إصلاح
pub fn vibe_suggest_fix(error_code: &str, error_message: &str) -> Vec<String> {
    let engine = VibeCodingEngine::new();
    engine.suggest_fix(error_code, error_message)
}

/// الحصول على تتمة ذكية
pub fn vibe_smart_completion(partial_code: &str) -> Vec<String> {
    let engine = VibeCodingEngine::new();
    engine.get_smart_completion(partial_code)
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = VibeCodingEngine::new();
        assert!(!engine.keywords.is_empty());
        assert!(!engine.templates.is_empty());
    }

    #[test]
    fn test_detect_variable_intent() {
        let engine = VibeCodingEngine::new();
        let intents = engine.detect_intents("أنشئ متغير س يساوي 5");
        
        // طباعة للتشخيص
        for intent in &intents {
            println!("Intent: {:?}, confidence: {}", intent.intent_type, intent.confidence);
        }
        
        assert!(!intents.is_empty());
        // قد تكون النية مركبة أو مباشرة
        let has_variable = intents.iter().any(|i| {
            match &i.intent_type {
                IntentType::CreateVariable => true,
                IntentType::Compound(types) => types.contains(&IntentType::CreateVariable),
                _ => false
            }
        });
        assert!(has_variable, "Expected CreateVariable intent, got: {:?}", intents.iter().map(|i| &i.intent_type).collect::<Vec<_>>());
    }

    #[test]
    fn test_detect_function_intent() {
        let engine = VibeCodingEngine::new();
        let intents = engine.detect_intents("أنشئ دالة تجمع رقمين");
        
        assert!(!intents.is_empty());
        assert!(intents.iter().any(|i| i.intent_type == IntentType::CreateFunction));
    }

    #[test]
    fn test_detect_loop_intent() {
        let engine = VibeCodingEngine::new();
        let intents = engine.detect_intents("كرر 5 مرات اطبع مرحبا");
        
        assert!(!intents.is_empty());
        assert!(intents.iter().any(|i| i.intent_type == IntentType::CreateLoop));
    }

    #[test]
    fn test_semantic_analysis() {
        let engine = VibeCodingEngine::new();
        let analysis = engine.analyze_semantics("أنشئ متغير اسم يساوي 10");
        
        assert!(!analysis.entities.is_empty());
        assert!(!analysis.meaning.is_empty());
    }

    #[test]
    fn test_process_vibe() {
        let mut engine = VibeCodingEngine::new();
        let result = engine.process("اطبع مرحبا بالعالم");
        
        assert!(result.success);
        assert!(result.code.contains("اطبع"));
    }

    #[test]
    fn test_learning() {
        let mut engine = VibeCodingEngine::new();
        engine.process("أنشئ متغير س يساوي 5");
        
        assert!(!engine.user_patterns.is_empty());
    }

    #[test]
    fn test_explain_code() {
        let engine = VibeCodingEngine::new();
        let explanation = engine.explain_code("متغير س = 5؛\nاطبع(س)؛");
        
        assert!(explanation.contains("متغير"));
    }

    #[test]
    fn test_smart_completion() {
        let engine = VibeCodingEngine::new();
        let completions = engine.get_smart_completion("متغير");
        
        assert!(!completions.is_empty());
    }

    #[test]
    fn test_suggest_fix() {
        let engine = VibeCodingEngine::new();
        let fixes = engine.suggest_fix("اطبع(س)", "متغير غير معرف");
        
        assert!(!fixes.is_empty());
    }

    #[test]
    fn test_compound_intent() {
        let engine = VibeCodingEngine::new();
        let intents = engine.detect_intents("أنشئ متغير س يساوي 5 واطبعه");
        
        // يجب كشف نية مركبة أو نيتين منفصلتين
        assert!(!intents.is_empty());
    }
}
