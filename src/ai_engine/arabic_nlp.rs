// ═══════════════════════════════════════════════════════════════════════════════
// معالجة اللغة العربية الطبيعية (Arabic NLP) - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════
// نظام متقدم لفهم وتحليل النصوص العربية البرمجية
// يتضمن:
// - تحليل الرموز (Tokenization)
// - استخراج الجذور (Stemming)
// - تحليل نحوي مبسط
// - التعرف على الكيانات (NER)
// - فهم السياق (Context Understanding)
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;

/// الرمز (Token)
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// كلمة عربية
    Word(String),
    /// رقم
    Number(f64),
    /// معامل
    Operator(String),
    /// فاصلة
    Comma,
    /// نقطة
    Dot,
    /// نقطتان
    Colon,
    /// قوس فتح
    OpenParen,
    /// قوس إغلاق
    CloseParen,
    /// قوس مجعد فتح
    OpenBrace,
    /// قوس مجعد إغلاق
    CloseBrace,
    /// نص بين علامات تنصيص
    String(String),
    /// نهاية الجملة
    EndOfSentence,
}

/// نوع الكيان
#[derive(Debug, Clone, PartialEq)]
pub enum EntityType {
    /// اسم متغير
    VariableName,
    /// اسم دالة
    FunctionName,
    /// قيمة رقمية
    NumberValue,
    /// نص
    StringValue,
    /// اسم معامل
    ParameterName,
    /// شرط
    Condition,
    /// عملية
    Operation,
}

/// كيان مستخرج
#[derive(Debug, Clone)]
pub struct Entity {
    pub entity_type: EntityType,
    pub value: String,
    pub position: usize,
}

/// نتيجة التحليل النحوي
#[derive(Debug, Clone)]
pub struct ParseResult {
    /// الرموز
    pub tokens: Vec<Token>,
    /// الكيانات
    pub entities: Vec<Entity>,
    /// البنية
    pub structure: SentenceStructure,
    /// الثقة
    pub confidence: f64,
}

/// بنية الجملة
#[derive(Debug, Clone)]
pub struct SentenceStructure {
    /// نوع الجملة
    pub sentence_type: SentenceType,
    /// الفعل إن وجد
    pub verb: Option<String>,
    /// الفاعل إن وجد
    pub subject: Option<String>,
    /// المفعول به إن وجد
    pub object: Option<String>,
    /// المعاملات الإضافية
    pub modifiers: Vec<String>,
}

/// نوع الجملة العربية
#[derive(Debug, Clone, PartialEq)]
pub enum SentenceType {
    /// جملة أمر (أنشئ، اطبع، احسب)
    Imperative,
    /// جملة شرطية (إذا، لو)
    Conditional,
    /// جملة استفهام (هل، ما)
    Interrogative,
    /// جملة خبرية (س يساوي 5)
    Declarative,
    /// جملة تكرار (كرر، طالما)
    Iterative,
    /// غير معروف
    Unknown,
}

/// معالج اللغة العربية
pub struct ArabicNlp {
    /// الكلمات المفتاحية البرمجية
    keywords: HashMap<String, KeywordInfo>,
    /// المرادفات
    synonyms: HashMap<String, Vec<String>>,
    /// الجذور
    stems: HashMap<String, String>,
}

/// معلومات الكلمة المفتاحية
#[derive(Debug, Clone)]
struct KeywordInfo {
    category: String,
    action: String,
    /// الأولوية (محجوز للاستخدام المستقبلي)
    _priority: u8,
}

impl ArabicNlp {
    /// إنشاء معالج جديد
    pub fn new() -> Self {
        let mut nlp = ArabicNlp {
            keywords: HashMap::new(),
            synonyms: HashMap::new(),
            stems: HashMap::new(),
        };
        nlp.initialize_keywords();
        nlp.initialize_synonyms();
        nlp.initialize_stems();
        nlp
    }
    
    /// تهيئة الكلمات المفتاحية
    fn initialize_keywords(&mut self) {
        // أوامر إنشاء
        self.add_keyword("أنشئ", "creation", "create", 10);
        self.add_keyword("عرّف", "creation", "define", 10);
        self.add_keyword("عرف", "creation", "define", 10);
        self.add_keyword("أضف", "creation", "add", 9);
        
        // أوامر المتغيرات
        self.add_keyword("متغير", "variable", "var", 10);
        self.add_keyword("ثابت", "variable", "const", 10);
        self.add_keyword("متجر", "variable", "store", 10);
        
        // أوامر الإخراج
        self.add_keyword("اطبع", "output", "print", 10);
        self.add_keyword("اعرض", "output", "display", 9);
        self.add_keyword("اكتب", "output", "write", 9);
        self.add_keyword("أظهر", "output", "show", 8);
        
        // أوامر الشرط
        self.add_keyword("إذا", "condition", "if", 10);
        self.add_keyword("لو", "condition", "if", 9);
        self.add_keyword("في_حالة", "condition", "if", 8);
        self.add_keyword("إلا", "condition", "else", 9);
        self.add_keyword("وإلا", "condition", "else", 9);
        
        // أوامر التكرار
        self.add_keyword("كرر", "iteration", "repeat", 10);
        self.add_keyword("طالما", "iteration", "while", 10);
        self.add_keyword("لكل", "iteration", "for", 10);
        self.add_keyword("من", "iteration", "from", 8);
        self.add_keyword("إلى", "iteration", "to", 8);
        
        // أوامر الدوال
        self.add_keyword("دالة", "function", "func", 10);
        self.add_keyword("وظيفة", "function", "func", 9);
        self.add_keyword("أعطِ", "function", "return", 10);
        self.add_keyword("أرجع", "function", "return", 9);
        
        // عمليات المقارنة
        self.add_keyword("أكبر", "comparison", "greater", 10);
        self.add_keyword("أصغر", "comparison", "less", 10);
        self.add_keyword("يساوي", "comparison", "equal", 10);
        self.add_keyword("يساوي_لا", "comparison", "not_equal", 9);
        
        // عمليات حسابية
        self.add_keyword("اجمع", "arithmetic", "add", 9);
        self.add_keyword("اطرح", "arithmetic", "subtract", 9);
        self.add_keyword("اضرب", "arithmetic", "multiply", 9);
        self.add_keyword("اقسم", "arithmetic", "divide", 9);
        
        // قيم
        self.add_keyword("صح", "value", "true", 10);
        self.add_keyword("خطأ", "value", "false", 10);
        self.add_keyword("لا_شيء", "value", "null", 10);
    }
    
    /// تهيئة المرادفات
    fn initialize_synonyms(&mut self) {
        // مرادفات الطباعة
        self.add_synonyms("اطبع", &["اعرض", "اكتب", "أظهر", "طبّع"]);
        
        // مرادفات الإنشاء
        self.add_synonyms("أنشئ", &["عرّف", "عرف", "أنشء", "خلق"]);
        
        // مرادفات المتغير
        self.add_synonyms("متغير", &["متغير", "خانة", "عنصر"]);
        
        // مرادفات الدالة
        self.add_synonyms("دالة", &["وظيفة", "إجراء", "روتين"]);
        
        // مرادفات الشرط
        self.add_synonyms("إذا", &["لو", "في_حالة", "عندما"]);
        
        // مرادفات التكرار
        self.add_synonyms("كرر", &["ردد", "أعد", "حلّق"]);
    }
    
    /// تهيئة الجذور
    fn initialize_stems(&mut self) {
        // جذور الأفعال الشائعة
        self.stems.insert("أنشئ".to_string(), "أنشأ".to_string());
        self.stems.insert("أنشء".to_string(), "أنشأ".to_string());
        self.stems.insert("أنشأن".to_string(), "أنشأ".to_string());
        
        self.stems.insert("اطبع".to_string(), "طبع".to_string());
        self.stems.insert("طبّع".to_string(), "طبع".to_string());
        self.stems.insert("يطبع".to_string(), "طبع".to_string());
        
        self.stems.insert("اعرض".to_string(), "عرض".to_string());
        self.stems.insert("يعرض".to_string(), "عرض".to_string());
        
        self.stems.insert("اكتب".to_string(), "كتب".to_string());
        self.stems.insert("يكتب".to_string(), "كتب".to_string());
        
        self.stems.insert("اجمع".to_string(), "جمع".to_string());
        self.stems.insert("يجمع".to_string(), "جمع".to_string());
        
        self.stems.insert("احسب".to_string(), "حسب".to_string());
        self.stems.insert("يحسب".to_string(), "حسب".to_string());
    }
    
    /// إضافة كلمة مفتاحية
    fn add_keyword(&mut self, word: &str, category: &str, action: &str, priority: u8) {
        self.keywords.insert(word.to_string(), KeywordInfo {
            category: category.to_string(),
            action: action.to_string(),
            _priority: priority,
        });
    }
    
    /// إضافة مرادفات
    fn add_synonyms(&mut self, main: &str, synonyms: &[&str]) {
        self.synonyms.insert(
            main.to_string(),
            synonyms.iter().map(|s| s.to_string()).collect()
        );
    }
    
    // ═══════════════════════════════════════════════════════════════
    // التحليل الرئيسي
    // ═══════════════════════════════════════════════════════════════
    
    /// تحليل النص العربي
    pub fn analyze(&self, text: &str) -> ParseResult {
        // المرحلة 1: الترميز
        let tokens = self.tokenize(text);
        
        // المرحلة 2: استخراج الكيانات
        let entities = self.extract_entities(&tokens, text);
        
        // المرحلة 3: تحليل البنية
        let structure = self.analyze_structure(&tokens);
        
        // المرحلة 4: حساب الثقة
        let confidence = self.calculate_confidence(&tokens, &entities);
        
        ParseResult {
            tokens,
            entities,
            structure,
            confidence,
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // الترميز (Tokenization)
    // ═══════════════════════════════════════════════════════════════
    
    /// تحويل النص إلى رموز
    pub fn tokenize(&self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current_word = String::new();
        let mut in_string = false;
        let mut string_delimiter = ' ';
        let mut current_string = String::new();
        
        for ch in text.chars() {
            if in_string {
                if ch == string_delimiter {
                    tokens.push(Token::String(current_string.clone()));
                    current_string.clear();
                    in_string = false;
                } else {
                    current_string.push(ch);
                }
                continue;
            }
            
            match ch {
                ' ' | '\t' | '\n' => {
                    if !current_word.is_empty() {
                        if let Some(token) = self.word_to_token(&current_word) {
                            tokens.push(token);
                        }
                        current_word.clear();
                    }
                }
                '"' | '\'' => {
                    if !current_word.is_empty() {
                        if let Some(token) = self.word_to_token(&current_word) {
                            tokens.push(token);
                        }
                        current_word.clear();
                    }
                    in_string = true;
                    string_delimiter = ch;
                }
                '(' => {
                    if !current_word.is_empty() {
                        if let Some(token) = self.word_to_token(&current_word) {
                            tokens.push(token);
                        }
                        current_word.clear();
                    }
                    tokens.push(Token::OpenParen);
                }
                ')' => {
                    if !current_word.is_empty() {
                        if let Some(token) = self.word_to_token(&current_word) {
                            tokens.push(token);
                        }
                        current_word.clear();
                    }
                    tokens.push(Token::CloseParen);
                }
                '{' => {
                    tokens.push(Token::OpenBrace);
                }
                '}' => {
                    tokens.push(Token::CloseBrace);
                }
                ',' => {
                    if !current_word.is_empty() {
                        if let Some(token) = self.word_to_token(&current_word) {
                            tokens.push(token);
                        }
                        current_word.clear();
                    }
                    tokens.push(Token::Comma);
                }
                '.' => {
                    if !current_word.is_empty() {
                        // قد يكون رقماً عشرياً
                        if current_word.chars().all(|c| c.is_numeric() || c == '٫') {
                            current_word.push('.');
                            continue;
                        }
                        if let Some(token) = self.word_to_token(&current_word) {
                            tokens.push(token);
                        }
                        current_word.clear();
                    }
                    tokens.push(Token::Dot);
                }
                ':' => {
                    tokens.push(Token::Colon);
                }
                '؛' | ';' => {
                    if !current_word.is_empty() {
                        if let Some(token) = self.word_to_token(&current_word) {
                            tokens.push(token);
                        }
                        current_word.clear();
                    }
                    tokens.push(Token::EndOfSentence);
                }
                '+' | '-' | '*' | '/' | '%' | '^' | '=' | '<' | '>' => {
                    if !current_word.is_empty() {
                        if let Some(token) = self.word_to_token(&current_word) {
                            tokens.push(token);
                        }
                        current_word.clear();
                    }
                    tokens.push(Token::Operator(ch.to_string()));
                }
                _ => {
                    current_word.push(ch);
                }
            }
        }
        
        // معالجة الكلمة الأخيرة
        if !current_word.is_empty() {
            if let Some(token) = self.word_to_token(&current_word) {
                tokens.push(token);
            }
        }
        
        tokens
    }
    
    /// تحويل كلمة إلى رمز
    fn word_to_token(&self, word: &str) -> Option<Token> {
        // تحقق من الرقم
        if let Ok(num) = self.parse_arabic_number(word) {
            return Some(Token::Number(num));
        }
        
        // تحقق من المعامل
        let operators = ["+", "-", "*", "/", "%", "^", "=", "==", "!=", "<", ">", "<=", ">="];
        if operators.contains(&word) {
            return Some(Token::Operator(word.to_string()));
        }
        
        // كلمة عادية
        Some(Token::Word(word.to_string()))
    }
    
    /// تحليل الرقم العربي
    fn parse_arabic_number(&self, s: &str) -> Result<f64, ()> {
        // تحويل الأرقام العربية والفارسية
        let converted: String = s.chars().map(|c| {
            match c {
                // الأرقام العربية الشرقية (٠-٩)
                '٠' => '0', '١' => '1', '٢' => '2', '٣' => '3', '٤' => '4',
                '٥' => '5', '٦' => '6', '٧' => '7', '٨' => '8', '٩' => '9',
                // الأرقام الفارسية (۰-۹)
                '۰' => '0', '۱' => '1', '۲' => '2', '۳' => '3', '۴' => '4',
                '۵' => '5', '۶' => '6', '۷' => '7', '۸' => '8', '۹' => '9',
                '٫' => '.',
                c => c,
            }
        }).collect();
        
        // محاولة تحويل الكلمات إلى أرقام
        let word_numbers = [
            ("صفر", 0.0), ("واحد", 1.0), ("اثنان", 2.0), ("اثنين", 2.0),
            ("ثلاثة", 3.0), ("ثلاث", 3.0), ("أربعة", 4.0), ("أربع", 4.0),
            ("خمسة", 5.0), ("خمس", 5.0), ("ستة", 6.0), ("ست", 6.0),
            ("سبعة", 7.0), ("سبع", 7.0), ("ثمانية", 8.0), ("ثمان", 8.0),
            ("تسعة", 9.0), ("تسع", 9.0), ("عشرة", 10.0), ("عشر", 10.0),
            ("مئة", 100.0), ("مائة", 100.0), ("ألف", 1000.0),
            ("مليون", 1_000_000.0),
        ];
        
        for (word, num) in word_numbers {
            if converted == word {
                return Ok(num);
            }
        }
        
        converted.parse().map_err(|_| ())
    }
    
    // ═══════════════════════════════════════════════════════════════
    // استخراج الكيانات (Named Entity Recognition)
    // ═══════════════════════════════════════════════════════════════
    
    /// استخراج الكيانات من الرموز
    fn extract_entities(&self, tokens: &[Token], _original_text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();
        let mut position = 0;
        
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Word(word) => {
                    // تحقق من الكلمات المفتاحية
                    if self.keywords.contains_key(word) {
                        entities.push(Entity {
                            entity_type: EntityType::Operation,
                            value: word.clone(),
                            position,
                        });
                    } else {
                        // قد يكون اسم متغير أو دالة
                        // نستخدم السياق لتحديد النوع
                        let entity_type = self.infer_entity_type(tokens, i);
                        entities.push(Entity {
                            entity_type,
                            value: word.clone(),
                            position,
                        });
                    }
                }
                Token::Number(n) => {
                    entities.push(Entity {
                        entity_type: EntityType::NumberValue,
                        value: n.to_string(),
                        position,
                    });
                }
                Token::String(s) => {
                    entities.push(Entity {
                        entity_type: EntityType::StringValue,
                        value: s.clone(),
                        position,
                    });
                }
                _ => {}
            }
            position += 1;
        }
        
        entities
    }
    
    /// استنتاج نوع الكيان من السياق
    fn infer_entity_type(&self, tokens: &[Token], index: usize) -> EntityType {
        // تحقق من السياق السابق
        if index > 0 {
            if let Some(Token::Word(prev)) = tokens.get(index - 1) {
                if prev == "دالة" || prev == "وظيفة" {
                    return EntityType::FunctionName;
                }
                if prev == "متغير" || prev == "ثابت" {
                    return EntityType::VariableName;
                }
                if prev == "يساوي" || prev == "=" {
                    return EntityType::NumberValue;
                }
            }
        }
        
        // تحقق من السياق اللاحق
        if let Some(Token::Word(next)) = tokens.get(index + 1) {
            if next == "يساوي" || next == "=" {
                return EntityType::VariableName;
            }
            if next == "(" {
                return EntityType::FunctionName;
            }
        }
        
        EntityType::VariableName
    }
    
    // ═══════════════════════════════════════════════════════════════
    // تحليل البنية (Structure Analysis)
    // ═══════════════════════════════════════════════════════════════
    
    /// تحليل بنية الجملة
    fn analyze_structure(&self, tokens: &[Token]) -> SentenceStructure {
        let sentence_type = self.detect_sentence_type(tokens);
        let verb = self.extract_verb(tokens);
        let subject = self.extract_subject(tokens);
        let object = self.extract_object(tokens);
        let modifiers = self.extract_modifiers(tokens);
        
        SentenceStructure {
            sentence_type,
            verb,
            subject,
            object,
            modifiers,
        }
    }
    
    /// تحديد نوع الجملة
    fn detect_sentence_type(&self, tokens: &[Token]) -> SentenceType {
        for token in tokens {
            if let Token::Word(word) = token {
                if word == "إذا" || word == "لو" || word == "في_حالة" {
                    return SentenceType::Conditional;
                }
                if word == "هل" || word == "ما" || word == "كم" {
                    return SentenceType::Interrogative;
                }
                if word == "كرر" || word == "طالما" || word == "لكل" {
                    return SentenceType::Iterative;
                }
                if self.keywords.get(word).map(|k| k.category.as_str()) == Some("creation") {
                    return SentenceType::Imperative;
                }
            }
        }
        SentenceType::Declarative
    }
    
    /// استخراج الفعل
    fn extract_verb(&self, tokens: &[Token]) -> Option<String> {
        for token in tokens {
            if let Token::Word(word) = token {
                if let Some(info) = self.keywords.get(word) {
                    if info.category == "creation" || info.category == "output" {
                        return Some(word.clone());
                    }
                }
            }
        }
        None
    }
    
    /// استخراج الفاعل
    fn extract_subject(&self, tokens: &[Token]) -> Option<String> {
        for (i, token) in tokens.iter().enumerate() {
            if let Token::Word(word) = token {
                if word == "متغير" || word == "ثابت" || word == "دالة" {
                    if let Some(Token::Word(name)) = tokens.get(i + 1) {
                        return Some(name.clone());
                    }
                }
            }
        }
        None
    }
    
    /// استخراج المفعول به
    fn extract_object(&self, tokens: &[Token]) -> Option<String> {
        for (i, token) in tokens.iter().enumerate() {
            if let Token::Word(word) = token {
                if word == "يساوي" || word == "=" {
                    if let Some(next) = tokens.get(i + 1) {
                        return Some(self.token_to_string(next));
                    }
                }
            }
        }
        None
    }
    
    /// استخراج المعاملات
    fn extract_modifiers(&self, tokens: &[Token]) -> Vec<String> {
        let mut modifiers = Vec::new();
        for token in tokens {
            if let Token::Word(word) = token {
                if let Some(info) = self.keywords.get(word) {
                    if info.category == "comparison" || info.category == "arithmetic" {
                        modifiers.push(word.clone());
                    }
                }
            }
        }
        modifiers
    }
    
    // ═══════════════════════════════════════════════════════════════
    // دوال مساعدة
    // ═══════════════════════════════════════════════════════════════
    
    /// حساب مستوى الثقة
    fn calculate_confidence(&self, tokens: &[Token], entities: &[Entity]) -> f64 {
        if tokens.is_empty() {
            return 0.0;
        }
        
        let recognized_count = entities.iter()
            .filter(|e| e.entity_type != EntityType::VariableName || 
                         self.keywords.contains_key(&e.value))
            .count();
        
        (recognized_count as f64 / tokens.len() as f64).min(1.0)
    }
    
    /// تحويل الرمز إلى نص
    fn token_to_string(&self, token: &Token) -> String {
        match token {
            Token::Word(s) => s.clone(),
            Token::Number(n) => n.to_string(),
            Token::String(s) => format!("\"{}\"", s),
            Token::Operator(s) => s.clone(),
            _ => String::new(),
        }
    }
    
    /// الحصول على مرادفات كلمة
    pub fn get_synonyms(&self, word: &str) -> Vec<String> {
        self.synonyms.get(word).cloned().unwrap_or_default()
    }
    
    /// الحصول على جذر الكلمة
    pub fn get_stem(&self, word: &str) -> Option<&str> {
        self.stems.get(word).map(|s| s.as_str())
    }
    
    /// التحقق من كلمة مفتاحية
    pub fn is_keyword(&self, word: &str) -> bool {
        self.keywords.contains_key(word)
    }
    
    /// الحصول على معلومات الكلمة المفتاحية
    pub fn get_keyword_info(&self, word: &str) -> Option<(&str, &str)> {
        self.keywords.get(word).map(|k| (k.category.as_str(), k.action.as_str()))
    }
    
    /// تطبيع النص العربي
    pub fn normalize(&self, text: &str) -> String {
        let mut normalized = String::new();
        
        for ch in text.chars() {
            match ch {
                'أ' | 'إ' | 'آ' => normalized.push('ا'),
                'ة' => normalized.push('ه'),
                'ى' => normalized.push('ي'),
                'ـ' => {}, // إزالة التطويل
                c => normalized.push(c),
            }
        }
        
        normalized
    }
    
    /// تصحيح التشكيل
    pub fn remove_diacritics(&self, text: &str) -> String {
        text.chars()
            .filter(|c| {
                !matches!(c, 
                    '\u{064B}'..='\u{065F}' | // التشكيل
                    '\u{0670}'               // الألف الصغيرة
                )
            })
            .collect()
    }
}

impl Default for ArabicNlp {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tokenize_simple() {
        let nlp = ArabicNlp::new();
        let tokens = nlp.tokenize("اطبع مرحبا");
        
        assert!(tokens.len() >= 2);
    }
    
    #[test]
    fn test_tokenize_number() {
        let nlp = ArabicNlp::new();
        let tokens = nlp.tokenize("متغير س = 42");
        
        assert!(tokens.iter().any(|t| matches!(t, Token::Number(42.0))));
    }
    
    #[test]
    fn test_analyze() {
        let nlp = ArabicNlp::new();
        let result = nlp.analyze("أنشئ متغير س يساوي 5");
        
        assert!(!result.tokens.is_empty());
        assert!(!result.entities.is_empty());
        assert!(result.confidence > 0.0);
    }
    
    #[test]
    fn test_sentence_type() {
        let nlp = ArabicNlp::new();
        
        let result = nlp.analyze("إذا س أكبر من 10");
        assert_eq!(result.structure.sentence_type, SentenceType::Conditional);
        
        let result = nlp.analyze("كرر 5 مرات");
        assert_eq!(result.structure.sentence_type, SentenceType::Iterative);
    }
    
    #[test]
    fn test_arabic_numbers() {
        let nlp = ArabicNlp::new();
        
        assert_eq!(nlp.parse_arabic_number("٥"), Ok(5.0));
        assert_eq!(nlp.parse_arabic_number("خمسة"), Ok(5.0));
        assert_eq!(nlp.parse_arabic_number("عشرة"), Ok(10.0));
    }
    
    #[test]
    fn test_normalize() {
        let nlp = ArabicNlp::new();
        
        assert_eq!(nlp.normalize("أحمد"), "احمد");
        assert_eq!(nlp.normalize("إبراهيم"), "ابراهيم");
    }
    
    #[test]
    fn test_synonyms() {
        let nlp = ArabicNlp::new();
        
        let synonyms = nlp.get_synonyms("اطبع");
        assert!(!synonyms.is_empty());
    }
}
