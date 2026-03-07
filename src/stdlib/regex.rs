// ═══════════════════════════════════════════════════════════════════════════════
// وحدة التعبيرات النمطية المتقدمة
// Advanced Regular Expressions Module
// ═══════════════════════════════════════════════════════════════════════════════
// © 2026 رضوان دالي حمدوني | RADHWEN DALY HAMDOUNI
// جميع الحقوق محفوظة | All Rights Reserved
// ═══════════════════════════════════════════════════════════════════════════════

//! # وحدة التعبيرات النمطية المتقدمة
//!
//! توفر هذه الوحدة:
//! - Pattern matching
//! - Capture groups
//! - Named groups
//! - Unicode support (Arabic especially)
//! - Replace operations
//! - Split operations
//! - Compile & cache patterns

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// ═══════════════════════════════════════════════════════════════════════════════
// الأخطاء
// ═══════════════════════════════════════════════════════════════════════════════

/// خطأ Regex
#[derive(Debug, Clone)]
pub enum RegexError {
    /// خطأ في التجميع
    CompileError(String),
    /// خطأ في التنفيذ
    ExecutionError(String),
    /// خطأ في النمط
    PatternError(String),
    /// خطأ في الاستبدال
    ReplaceError(String),
    /// خطأ في التحويل
    ConversionError(String),
    /// خطأ عام
    GenericError(String),
}

impl std::fmt::Display for RegexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegexError::CompileError(msg) => write!(f, "خطأ في تجميع النمط: {}", msg),
            RegexError::ExecutionError(msg) => write!(f, "خطأ في التنفيذ: {}", msg),
            RegexError::PatternError(msg) => write!(f, "خطأ في النمط: {}", msg),
            RegexError::ReplaceError(msg) => write!(f, "خطأ في الاستبدال: {}", msg),
            RegexError::ConversionError(msg) => write!(f, "خطأ في التحويل: {}", msg),
            RegexError::GenericError(msg) => write!(f, "خطأ: {}", msg),
        }
    }
}

impl std::error::Error for RegexError {}

/// نتيجة Regex
pub type RegexResult<T> = Result<T, RegexError>;

// ═══════════════════════════════════════════════════════════════════════════════
// خيارات التجميع
// ═══════════════════════════════════════════════════════════════════════════════

/// خيارات تجميع النمط
#[derive(Debug, Clone, Default)]
pub struct RegexCompileOptions {
    /// تجاهل حالة الأحرف
    pub case_insensitive: bool,
    /// متعدد الأسطر
    pub multi_line: bool,
    /// نقطة تطابق كل شيء
    pub dot_matches_newline: bool,
    /// تعليقات
    pub ignore_whitespace: bool,
    /// Unicode
    pub unicode: bool,
    /// دعم العربية
    pub arabic_support: bool,
}

impl RegexCompileOptions {
    /// إنشاء خيارات جديدة
    pub fn new() -> Self {
        Self {
            case_insensitive: false,
            multi_line: false,
            dot_matches_newline: false,
            ignore_whitespace: false,
            unicode: true, // Unicode مفعّل افتراضياً
            arabic_support: true, // دعم العربية مفعّل افتراضياً
        }
    }

    /// تجاهل حالة الأحرف
    pub fn case_insensitive(mut self) -> Self {
        self.case_insensitive = true;
        self
    }

    /// متعدد الأسطر
    pub fn multi_line(mut self) -> Self {
        self.multi_line = true;
        self
    }

    /// نقطة تطابق كل شيء
    pub fn dot_all(mut self) -> Self {
        self.dot_matches_newline = true;
        self
    }

    /// تجاهل المسافات البيضاء
    pub fn ignore_whitespace(mut self) -> Self {
        self.ignore_whitespace = true;
        self
    }

    /// تفعيل Unicode
    pub fn unicode(mut self, enabled: bool) -> Self {
        self.unicode = enabled;
        self
    }

    /// تفعيل دعم العربية
    pub fn arabic_support(mut self, enabled: bool) -> Self {
        self.arabic_support = enabled;
        self
    }

    /// تحويل إلى أعلام regex
    pub fn to_flags(&self) -> String {
        let mut flags = String::new();
        if self.case_insensitive {
            flags.push('i');
        }
        if self.multi_line {
            flags.push('m');
        }
        if self.dot_matches_newline {
            flags.push('s');
        }
        if self.ignore_whitespace {
            flags.push('x');
        }
        if self.unicode {
            flags.push('u');
        }
        flags
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// نتيجة المطابقة
// ═══════════════════════════════════════════════════════════════════════════════

/// نتيجة المطابقة
#[derive(Debug, Clone)]
pub struct RegexMatch {
    /// النص المتطابق
    pub text: String,
    /// موضع البداية
    pub start: usize,
    /// موضع النهاية
    pub end: usize,
    /// المجموعات الملتقطة
    pub groups: Vec<Option<String>>,
    /// المجموعات المسماة
    pub named_groups: HashMap<String, String>,
}

impl RegexMatch {
    /// إنشاء نتيجة جديدة
    pub fn new(text: String, start: usize, end: usize) -> Self {
        Self {
            text,
            start,
            end,
            groups: Vec::new(),
            named_groups: HashMap::new(),
        }
    }

    /// هل هناك تطابق
    pub fn is_match(&self) -> bool {
        !self.text.is_empty()
    }

    /// الحصول على مجموعة حسب الفهرس
    pub fn group(&self, index: usize) -> Option<&String> {
        self.groups.get(index).and_then(|g| g.as_ref())
    }

    /// الحصول على مجموعة مسماة
    pub fn named(&self, name: &str) -> Option<&String> {
        self.named_groups.get(name)
    }

    /// عدد المجموعات
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    /// طول التطابق
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// هل فارغ
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// النمط
// ═══════════════════════════════════════════════════════════════════════════════

/// نمط Regex
#[derive(Debug)]
pub struct RegexPattern {
    /// النمط الأصلي
    pattern: String,
    /// النمط المترجم
    compiled: regex::Regex,
    /// الخيارات
    options: RegexCompileOptions,
}

impl RegexPattern {
    /// إنشاء نمط جديد
    pub fn new(pattern: &str) -> RegexResult<Self> {
        Self::with_options(pattern, RegexCompileOptions::new())
    }

    /// إنشاء نمط مع خيارات
    pub fn with_options(pattern: &str, options: RegexCompileOptions) -> RegexResult<Self> {
        let mut builder = regex::RegexBuilder::new(pattern);
        
        builder
            .case_insensitive(options.case_insensitive)
            .multi_line(options.multi_line)
            .dot_matches_new_line(options.dot_matches_newline)
            .ignore_whitespace(options.ignore_whitespace)
            .unicode(options.unicode);
        
        let compiled = builder
            .build()
            .map_err(|e| RegexError::CompileError(e.to_string()))?;
        
        Ok(Self {
            pattern: pattern.to_string(),
            compiled,
            options,
        })
    }

    /// البحث عن تطابق
    pub fn find(&self, text: &str) -> Option<RegexMatch> {
        self.compiled.find(text).map(|m| {
            let mut match_result = RegexMatch::new(
                m.as_str().to_string(),
                m.start(),
                m.end(),
            );
            
            // استخراج المجموعات
            let captures = self.compiled.captures(text);
            if let Some(caps) = captures {
                for (i, cap) in caps.iter().enumerate() {
                    match_result.groups.push(cap.map(|c| c.as_str().to_string()));
                }
                
                // استخراج المجموعات المسماة
                for name in self.compiled.capture_names().flatten() {
                    if let Some(cap) = caps.name(name) {
                        match_result.named_groups.insert(name.to_string(), cap.as_str().to_string());
                    }
                }
            }
            
            match_result
        })
    }

    /// البحث عن جميع التطابقات
    pub fn find_all(&self, text: &str) -> Vec<RegexMatch> {
        self.compiled
            .captures_iter(text)
            .map(|caps| {
                let full_match = caps.get(0).unwrap();
                let mut match_result = RegexMatch::new(
                    full_match.as_str().to_string(),
                    full_match.start(),
                    full_match.end(),
                );
                
                for (i, cap) in caps.iter().enumerate() {
                    match_result.groups.push(cap.map(|c| c.as_str().to_string()));
                }
                
                for name in self.compiled.capture_names().flatten() {
                    if let Some(cap) = caps.name(name) {
                        match_result.named_groups.insert(name.to_string(), cap.as_str().to_string());
                    }
                }
                
                match_result
            })
            .collect()
    }

    /// التحقق من التطابق
    pub fn is_match(&self, text: &str) -> bool {
        self.compiled.is_match(text)
    }

    /// المطابقة من البداية
    pub fn matches(&self, text: &str) -> Option<RegexMatch> {
        let captures = self.compiled.captures(text)?;
        let full_match = captures.get(0)?;
        
        // التحقق من أن التطابق من البداية
        if full_match.start() != 0 {
            return None;
        }
        
        let mut match_result = RegexMatch::new(
            full_match.as_str().to_string(),
            full_match.start(),
            full_match.end(),
        );
        
        for (i, cap) in captures.iter().enumerate() {
            match_result.groups.push(cap.map(|c| c.as_str().to_string()));
        }
        
        for name in self.compiled.capture_names().flatten() {
            if let Some(cap) = captures.name(name) {
                match_result.named_groups.insert(name.to_string(), cap.as_str().to_string());
            }
        }
        
        Some(match_result)
    }

    /// الاستبدال
    pub fn replace(&self, text: &str, replacement: &str) -> String {
        self.compiled.replace_all(text, replacement).to_string()
    }

    /// الاستبدال بدالة
    pub fn replace_with<F>(&self, text: &str, mut replacer: F) -> String
    where
        F: FnMut(&RegexMatch) -> String,
    {
        let mut result = String::new();
        let mut last_end = 0;
        
        for caps in self.compiled.captures_iter(text) {
            let m = caps.get(0).unwrap();
            result.push_str(&text[last_end..m.start()]);
            
            let mut match_result = RegexMatch::new(
                m.as_str().to_string(),
                m.start(),
                m.end(),
            );
            
            for (i, cap) in caps.iter().enumerate() {
                match_result.groups.push(cap.map(|c| c.as_str().to_string()));
            }
            
            for name in self.compiled.capture_names().flatten() {
                if let Some(cap) = caps.name(name) {
                    match_result.named_groups.insert(name.to_string(), cap.as_str().to_string());
                }
            }
            
            result.push_str(&replacer(&match_result));
            last_end = m.end();
        }
        
        result.push_str(&text[last_end..]);
        result
    }

    /// التقسيم
    pub fn split(&self, text: &str) -> Vec<String> {
        self.compiled.split(text).map(|s| s.to_string()).collect()
    }

    /// التقسيم مع الحد
    pub fn splitn(&self, text: &str, limit: usize) -> Vec<String> {
        self.compiled.splitn(text, limit).map(|s| s.to_string()).collect()
    }

    /// عدد التطابقات
    pub fn count(&self, text: &str) -> usize {
        self.compiled.find_iter(text).count()
    }

    /// النمط الأصلي
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// الخيارات
    pub fn options(&self) -> &RegexCompileOptions {
        &self.options
    }
}

impl Clone for RegexPattern {
    fn clone(&self) -> Self {
        Self::with_options(&self.pattern, self.options.clone()).unwrap()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ذاكرة التخزين المؤقت
// ═══════════════════════════════════════════════════════════════════════════════

/// ذاكرة تخزين مؤقت للأنماط
#[derive(Debug)]
pub struct RegexCache {
    /// الأنماط المخزنة
    patterns: RwLock<HashMap<String, Arc<RegexPattern>>>,
    /// الحد الأقصى للحجم
    max_size: usize,
}

impl RegexCache {
    /// إنشاء ذاكرة جديدة
    pub fn new() -> Self {
        Self {
            patterns: RwLock::new(HashMap::new()),
            max_size: 100,
        }
    }

    /// إنشاء ذاكرة بحجم محدد
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            patterns: RwLock::new(HashMap::new()),
            max_size,
        }
    }

    /// الحصول على نمط أو إنشاؤه
    pub fn get_or_compile(&self, pattern: &str) -> RegexResult<Arc<RegexPattern>> {
        // محاولة الحصول من الذاكرة
        {
            let patterns = self.patterns.read().unwrap();
            if let Some(cached) = patterns.get(pattern) {
                return Ok(Arc::clone(cached));
            }
        }
        
        // تجميع النمط
        let compiled = RegexPattern::new(pattern)?;
        let cached = Arc::new(compiled);
        
        // إضافة إلى الذاكرة
        {
            let mut patterns = self.patterns.write().unwrap();
            
            // التحقق من الحجم
            if patterns.len() >= self.max_size {
                // إزالة أول عنصر (LRU بسيط)
                if let Some(first_key) = patterns.keys().next().cloned() {
                    patterns.remove(&first_key);
                }
            }
            
            patterns.insert(pattern.to_string(), Arc::clone(&cached));
        }
        
        Ok(cached)
    }

    /// الحصول على نمط مع خيارات
    pub fn get_or_compile_with_options(
        &self,
        pattern: &str,
        options: &RegexCompileOptions,
    ) -> RegexResult<Arc<RegexPattern>> {
        // مفتاح فريد يحتوي على النمط والخيارات
        let key = format!("{}|{}", pattern, options.to_flags());
        
        // محاولة الحصول من الذاكرة
        {
            let patterns = self.patterns.read().unwrap();
            if let Some(cached) = patterns.get(&key) {
                return Ok(Arc::clone(cached));
            }
        }
        
        // تجميع النمط
        let compiled = RegexPattern::with_options(pattern, options.clone())?;
        let cached = Arc::new(compiled);
        
        // إضافة إلى الذاكرة
        {
            let mut patterns = self.patterns.write().unwrap();
            
            if patterns.len() >= self.max_size {
                if let Some(first_key) = patterns.keys().next().cloned() {
                    patterns.remove(&first_key);
                }
            }
            
            patterns.insert(key, Arc::clone(&cached));
        }
        
        Ok(cached)
    }

    /// إزالة نمط من الذاكرة
    pub fn remove(&self, pattern: &str) {
        self.patterns.write().unwrap().remove(pattern);
    }

    /// مسح الذاكرة
    pub fn clear(&self) {
        self.patterns.write().unwrap().clear();
    }

    /// حجم الذاكرة
    pub fn len(&self) -> usize {
        self.patterns.read().unwrap().len()
    }

    /// هل الذاكرة فارغة
    pub fn is_empty(&self) -> bool {
        self.patterns.read().unwrap().is_empty()
    }
}

impl Default for RegexCache {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// أنماط عربية جاهزة
// ═══════════════════════════════════════════════════════════════════════════════

/// أنماط Regex العربية
pub struct ArabicPatterns;

impl ArabicPatterns {
    /// حرف عربي
    pub fn arabic_letter() -> &'static str {
        r"[\u0600-\u06FF]"
    }

    /// كلمة عربية
    pub fn arabic_word() -> &'static str {
        r"[\u0600-\u06FF]+"
    }

    /// رقم عربي (هندي)
    pub fn arabic_digit() -> &'static str {
        r"[٠-٩]"
    }

    /// أرقام عربية (هندي)
    pub fn arabic_digits() -> &'static str {
        r"[٠-٩]+"
    }

    /// رقم عربي أو غربي
    pub fn any_digit() -> &'static str {
        r"[0-9٠-٩]"
    }

    /// أرقام عربية أو غربية
    pub fn any_digits() -> &'static str {
        r"[0-9٠-٩]+"
    }

    /// نص عربي (كلمات ومسافات)
    pub fn arabic_text() -> &'static str {
        r"[\u0600-\u06FF\s]+"
    }

    /// جملة عربية
    pub fn arabic_sentence() -> &'static str {
        r"[^.!؟。\u0600-\u06FF]+[.!؟。]"
    }

    /// تشكيل
    pub fn diacritics() -> &'static str {
        r"[\u064B-\u065F\u0670]"
    }

    /// حرف بدون تشكيل
    pub fn letter_without_diacritics() -> &'static str {
        r"[\u0600-\u06FF](?![\u064B-\u065F])"
    }

    /// اسم عربي (بدون أرقام)
    pub fn arabic_name() -> &'static str {
        r"^[\u0600-\u06FF\s]+$"
    }

    /// تاريخ عربي
    pub fn arabic_date() -> &'static str {
        r"[٠-٩]{1,4}[\/\-][٠-٩]{1,2}[\/\-][٠-٩]{1,4}"
    }

    /// وقت عربي
    pub fn arabic_time() -> &'static str {
        r"[٠-٩]{1,2}:[٠-٩]{2}(:[٠-٩]{2})?\s*(ص|م)?"
    }

    /// بريد إلكتروني (يدعم العربية في النص)
    pub fn email() -> &'static str {
        r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}"
    }

    /// رقم هاتف (يدعم الأرقام العربية)
    pub fn phone() -> &'static str {
        r"[\+]?[(]?[0-9٠-٩]{1,3}[)]?[-\s\.]?[0-9٠-٩]{1,4}[-\s\.]?[0-9٠-٩]{1,4}[-\s\.]?[0-9٠-٩]{1,9}"
    }

    /// رابط URL
    pub fn url() -> &'static str {
        r"https?://[^\s]+"
    }

    /// رقم هاتف سعودي
    pub fn saudi_phone() -> &'static str {
        r"(\+966|966|0)?5[0-9٠-٩]{8}"
    }

    /// رقم هاتف مصري
    pub fn egyptian_phone() -> &'static str {
        r"(\+20|20|0)?1[0-9٠-٩]{9}"
    }

    /// رقم هاتف إماراتي
    pub fn uae_phone() -> &'static str {
        r"(\+971|971|0)?[0-9٠-٩]{9}"
    }

    /// رقم هوية سعودية
    pub fn saudi_id() -> &'static str {
        r"[1-2][0-9]{9}"
    }

    /// رقم الحساب البنكي
    pub fn bank_account() -> &'static str {
        r"[0-9٠-٩]{10,24}"
    }

    /// بطاقة ائتمان
    pub fn credit_card() -> &'static str {
        r"[0-9]{4}[-\s]?[0-9]{4}[-\s]?[0-9]{4}[-\s]?[0-9]{4}"
    }

    /// IPv4
    pub fn ipv4() -> &'static str {
        r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b"
    }

    /// IPv6
    pub fn ipv6() -> &'static str {
        r"([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}"
    }

    /// hexadecimal
    pub fn hex_color() -> &'static str {
        r"#[0-9a-fA-F]{6}|#[0-9a-fA-F]{3}"
    }

    /// JSON
    pub fn json_string() -> &'static str {
        r#""(?:[^"\\]|\\.)*""#
    }

    /// HTML tag
    pub fn html_tag() -> &'static str {
        r"<[^>]+>"
    }

    /// Markdown header
    pub fn markdown_header() -> &'static str {
        r"^#{1,6}\s+.+$"
    }

    /// Markdown link
    pub fn markdown_link() -> &'static str {
        r"\[([^\]]+)\]\(([^)]+)\)"
    }

    /// Markdown image
    pub fn markdown_image() -> &'static str {
        r"!\[([^\]]*)\]\(([^)]+)\)"
    }

    /// قائمة Markdown
    pub fn markdown_list() -> &'static str {
        r"^[\*\-\+]\s+.+$|^\d+\.\s+.+$"
    }

    /// YAML front matter
    pub fn yaml_front_matter() -> &'static str {
        r"^---\n([\s\S]*?)\n---"
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال مساعدة
// ═══════════════════════════════════════════════════════════════════════════════

/// التحقق من نص عربي
pub fn is_arabic(text: &str) -> bool {
    let pattern = RegexPattern::new(ArabicPatterns::arabic_text()).unwrap();
    pattern.is_match(text)
}

/// استخراج الكلمات العربية
pub fn extract_arabic_words(text: &str) -> Vec<String> {
    let pattern = RegexPattern::new(ArabicPatterns::arabic_word()).unwrap();
    pattern.find_all(text).iter().map(|m| m.text.clone()).collect()
}

/// استخراج الأرقام العربية
pub fn extract_arabic_digits(text: &str) -> Vec<String> {
    let pattern = RegexPattern::new(ArabicPatterns::arabic_digits()).unwrap();
    pattern.find_all(text).iter().map(|m| m.text.clone()).collect()
}

/// تحويل الأرقام العربية إلى غربية
pub fn arabic_to_western_digits(text: &str) -> String {
    let arabic_numerals = ['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'];
    let western_numerals = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    
    let mut result = text.to_string();
    for (ar, en) in arabic_numerals.iter().zip(western_numerals.iter()) {
        result = result.replace(*ar, &en.to_string());
    }
    result
}

/// تحويل الأرقام الغربية إلى عربية
pub fn western_to_arabic_digits(text: &str) -> String {
    let western_numerals = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let arabic_numerals = ['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'];
    
    let mut result = text.to_string();
    for (en, ar) in western_numerals.iter().zip(arabic_numerals.iter()) {
        result = result.replace(*en, &ar.to_string());
    }
    result
}

/// إزالة التشكيل
pub fn remove_diacritics(text: &str) -> String {
    let pattern = RegexPattern::new(ArabicPatterns::diacritics()).unwrap();
    pattern.replace(text, "")
}

/// تطبيع النص العربي
pub fn normalize_arabic(text: &str) -> String {
    let mut result = text.to_string();
    
    // توحيد الألف
    result = result.replace('أ', "ا");
    result = result.replace('إ', "ا");
    result = result.replace('آ', "ا");
    
    // توحيد الياء
    result = result.replace('ى', "ي");
    
    // توحيد الهاء والتاء المربوطة
    result = result.replace('ة', 'ه');
    
    // إزالة التشكيل
    result = remove_diacritics(&result);
    
    // تحويل الأرقام إلى غربية
    result = arabic_to_western_digits(&result);
    
    result
}

/// التحقق من بريد إلكتروني
pub fn is_valid_email(email: &str) -> bool {
    let pattern = RegexPattern::new(ArabicPatterns::email()).unwrap();
    pattern.is_match(email)
}

/// التحقق من رقم هاتف
pub fn is_valid_phone(phone: &str) -> bool {
    let pattern = RegexPattern::new(ArabicPatterns::phone()).unwrap();
    pattern.is_match(phone)
}

/// التحقق من URL
pub fn is_valid_url(url: &str) -> bool {
    let pattern = RegexPattern::new(ArabicPatterns::url()).unwrap();
    pattern.is_match(url)
}

/// استخراج الروابط من نص
pub fn extract_urls(text: &str) -> Vec<String> {
    let pattern = RegexPattern::new(ArabicPatterns::url()).unwrap();
    pattern.find_all(text).iter().map(|m| m.text.clone()).collect()
}

/// استخراج البريد الإلكتروني من نص
pub fn extract_emails(text: &str) -> Vec<String> {
    let pattern = RegexPattern::new(ArabicPatterns::email()).unwrap();
    pattern.find_all(text).iter().map(|m| m.text.clone()).collect()
}

/// استخراج أرقام الهواتف من نص
pub fn extract_phones(text: &str) -> Vec<String> {
    let pattern = RegexPattern::new(ArabicPatterns::phone()).unwrap();
    pattern.find_all(text).iter().map(|m| m.text.clone()).collect()
}

/// التحقق من IP
pub fn is_valid_ipv4(ip: &str) -> bool {
    let pattern = RegexPattern::new(ArabicPatterns::ipv4()).unwrap();
    pattern.is_match(ip)
}

/// التحقق من IPv6
pub fn is_valid_ipv6(ip: &str) -> bool {
    let pattern = RegexPattern::new(ArabicPatterns::ipv6()).unwrap();
    pattern.is_match(ip)
}

/// التحقق من لون hex
pub fn is_valid_hex_color(color: &str) -> bool {
    let pattern = RegexPattern::new(ArabicPatterns::hex_color()).unwrap();
    pattern.is_match(color)
}

// ═══════════════════════════════════════════════════════════════════════════════
// باني الأنماط
// ═══════════════════════════════════════════════════════════════════════════════

/// منشئ أنماط Regex
#[derive(Debug, Clone)]
pub struct RegexBuilder {
    /// النمط
    pattern: String,
    /// الخيارات
    options: RegexCompileOptions,
}

impl RegexBuilder {
    /// إنشاء منشئ جديد
    pub fn new() -> Self {
        Self {
            pattern: String::new(),
            options: RegexCompileOptions::new(),
        }
    }

    /// تعيين النمط
    pub fn pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = pattern.into();
        self
    }

    /// تجاهل حالة الأحرف
    pub fn case_insensitive(mut self) -> Self {
        self.options.case_insensitive = true;
        self
    }

    /// متعدد الأسطر
    pub fn multi_line(mut self) -> Self {
        self.options.multi_line = true;
        self
    }

    /// نقطة تطابق كل شيء
    pub fn dot_all(mut self) -> Self {
        self.options.dot_matches_newline = true;
        self
    }

    /// تجاهل المسافات
    pub fn ignore_whitespace(mut self) -> Self {
        self.options.ignore_whitespace = true;
        self
    }

    /// تفعيل Unicode
    pub fn unicode(mut self, enabled: bool) -> Self {
        self.options.unicode = enabled;
        self
    }

    /// تفعيل دعم العربية
    pub fn arabic_support(mut self, enabled: bool) -> Self {
        self.options.arabic_support = enabled;
        self
    }

    /// إضافة حرف عربي
    pub fn arabic_letter(self) -> Self {
        self.pattern(&format!("{}{}", self.pattern, ArabicPatterns::arabic_letter()))
    }

    /// إضافة كلمة عربية
    pub fn arabic_word(self) -> Self {
        self.pattern(&format!("{}{}", self.pattern, ArabicPatterns::arabic_word()))
    }

    /// إضافة رقم عربي
    pub fn arabic_digit(self) -> Self {
        self.pattern(&format!("{}{}", self.pattern, ArabicPatterns::arabic_digit()))
    }

    /// إضافة أرقام عربية
    pub fn arabic_digits(self) -> Self {
        self.pattern(&format!("{}{}", self.pattern, ArabicPatterns::arabic_digits()))
    }

    /// إضافة مجموعة
    pub fn group(self, content: &str) -> Self {
        self.pattern(&format!("{}({})", self.pattern, content))
    }

    /// إضافة مجموعة مسماة
    pub fn named_group(self, name: &str, content: &str) -> Self {
        self.pattern(&format!("{}(?P<{}>{})", self.pattern, name, content))
    }

    /// إضافة مجموعة غير ملتقطة
    pub fn non_capturing_group(self, content: &str) -> Self {
        self.pattern(&format!("{}(?:{})", self.pattern, content))
    }

    /// إضافة حرف اختياري
    pub fn optional(self, content: &str) -> Self {
        self.pattern(&format!("{}{}?", self.pattern, content))
    }

    /// إضافة تكرار 0 أو أكثر
    pub fn zero_or_more(self, content: &str) -> Self {
        self.pattern(&format!("{}{}*", self.pattern, content))
    }

    /// إضافة تكرار 1 أو أكثر
    pub fn one_or_more(self, content: &str) -> Self {
        self.pattern(&format!("{}{}+", self.pattern, content))
    }

    /// إضافة تكرار محدد
    pub fn repeat(self, content: &str, min: usize, max: Option<usize>) -> Self {
        let quantifier = match max {
            Some(max) => format!("{{{},{}}}", min, max),
            None => format!("{{{},}}", min),
        };
        self.pattern(&format!("{}{}{}", self.pattern, content, quantifier))
    }

    /// إضافة بداية سطر
    pub fn start_of_line(self) -> Self {
        self.pattern(&format!("{}^", self.pattern))
    }

    /// إضافة نهاية سطر
    pub fn end_of_line(self) -> Self {
        self.pattern(&format!("{}$", self.pattern))
    }

    /// إضافة حدود كلمة
    pub fn word_boundary(self) -> Self {
        self.pattern(&format!("{}\\b", self.pattern))
    }

    /// إضافة OR
    pub fn or(self, alternative: &str) -> Self {
        self.pattern(&format!("{}|{}", self.pattern, alternative))
    }

    /// إضافة فئة حرف
    pub fn char_class(self, chars: &str) -> Self {
        self.pattern(&format!("{}[{}]", self.pattern, chars))
    }

    /// إضافة فئة حرف منفية
    pub fn negated_char_class(self, chars: &str) -> Self {
        self.pattern(&format!("{}[^{}]", self.pattern, chars))
    }

    /// بناء النمط
    pub fn build(self) -> RegexResult<RegexPattern> {
        RegexPattern::with_options(&self.pattern, self.options)
    }
}

impl Default for RegexBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// الاختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_match() {
        let pattern = RegexPattern::new(r"\d+").unwrap();
        let result = pattern.find("مرحبا 123 world");
        
        assert!(result.is_some());
        let m = result.unwrap();
        assert_eq!(m.text, "123");
    }

    #[test]
    fn test_arabic_word() {
        let pattern = RegexPattern::new(ArabicPatterns::arabic_word()).unwrap();
        let matches = pattern.find_all("مرحبا بالعالم");
        
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].text, "مرحبا");
        assert_eq!(matches[1].text, "بالعالم");
    }

    #[test]
    fn test_arabic_digits() {
        let pattern = RegexPattern::new(ArabicPatterns::arabic_digits()).unwrap();
        let matches = pattern.find_all("رقم ١٢٣ ورقم ٤٥٦");
        
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].text, "١٢٣");
    }

    #[test]
    fn test_capture_groups() {
        let pattern = RegexPattern::new(r"(\w+)@(\w+)\.(\w+)").unwrap();
        let result = pattern.find("test@example.com");
        
        assert!(result.is_some());
        let m = result.unwrap();
        assert_eq!(m.group_count(), 3);
        assert_eq!(m.group(1), Some(&"test".to_string()));
    }

    #[test]
    fn test_named_groups() {
        let pattern = RegexPattern::new(r"(?P<user>\w+)@(?P<domain>\w+)\.\w+").unwrap();
        let result = pattern.find("test@example.com");
        
        assert!(result.is_some());
        let m = result.unwrap();
        assert_eq!(m.named("user"), Some(&"test".to_string()));
        assert_eq!(m.named("domain"), Some(&"example".to_string()));
    }

    #[test]
    fn test_replace() {
        let pattern = RegexPattern::new(r"\d+").unwrap();
        let result = pattern.replace("مرحبا 123 world 456", "XXX");
        
        assert_eq!(result, "مرحبا XXX world XXX");
    }

    #[test]
    fn test_split() {
        let pattern = RegexPattern::new(r"\s+").unwrap();
        let result = pattern.split("مرحبا بالعالم");
        
        assert_eq!(result, vec!["مرحبا", "بالعالم"]);
    }

    #[test]
    fn test_regex_cache() {
        let cache = RegexCache::new();
        
        let p1 = cache.get_or_compile(r"\d+").unwrap();
        let p2 = cache.get_or_compile(r"\d+").unwrap();
        
        // يجب أن يكونا نفس الكائن
        assert!(Arc::ptr_eq(&p1, &p2));
    }

    #[test]
    fn test_arabic_to_western() {
        let result = arabic_to_western_digits("١٢٣٤٥");
        assert_eq!(result, "12345");
    }

    #[test]
    fn test_western_to_arabic() {
        let result = western_to_arabic_digits("12345");
        assert_eq!(result, "١٢٣٤٥");
    }

    #[test]
    fn test_remove_diacritics() {
        let result = remove_diacritics("مَرْحَباً");
        assert_eq!(result, "مرحبا");
    }

    #[test]
    fn test_normalize_arabic() {
        let result = normalize_arabic("أحمد إبراهيم الآلة");
        assert!(result.contains("احمد"));
    }

    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("test@example.com"));
        assert!(!is_valid_email("invalid-email"));
    }

    #[test]
    fn test_url_extraction() {
        let text = "زوروا https://example.com و http://test.org";
        let urls = extract_urls(text);
        
        assert_eq!(urls.len(), 2);
    }

    #[test]
    fn test_regex_builder() {
        let pattern = RegexBuilder::new()
            .start_of_line()
            .named_group("name", ArabicPatterns::arabic_word())
            .end_of_line()
            .build()
            .unwrap();
        
        let result = pattern.find("مرحبا");
        assert!(result.is_some());
        assert_eq!(result.unwrap().named("name"), Some(&"مرحبا".to_string()));
    }

    #[test]
    fn test_case_insensitive() {
        let pattern = RegexPattern::with_options(
            "hello",
            RegexCompileOptions::new().case_insensitive(),
        ).unwrap();
        
        assert!(pattern.is_match("HELLO"));
        assert!(pattern.is_match("Hello"));
    }
}
