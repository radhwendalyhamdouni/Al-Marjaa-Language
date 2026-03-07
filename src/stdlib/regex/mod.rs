// src/stdlib/regex/mod.rs
// وحدة التعابير النمطية المتقدمة
// Advanced Regex Module

use std::collections::HashMap;

/// نمط التعابير النمطية
#[derive(Debug, Clone)]
pub struct RegexPattern {
    pattern: String,
    flags: RegexFlags,
    compiled: Option<regex::Regex>,
}

/// أعلام التعابير النمطية
#[derive(Debug, Clone, Default)]
pub struct RegexFlags {
    /// تجاهل حالة الأحرف
    pub ignore_case: bool,
    /// متعدد الأسطر
    pub multiline: bool,
    /// نقطة تطابق السطر الجديد
    pub dot_all: bool,
    /// Unicode
    pub unicode: bool,
    /// مطابقة الجشعية
    pub greedy: bool,
    /// مُطوَّل (تجاهل الفراغات)
    pub extended: bool,
}

impl RegexFlags {
    pub fn new() -> Self {
        Self {
            ignore_case: false,
            multiline: false,
            dot_all: false,
            unicode: true, // دعم Unicode افتراضياً للعربية
            greedy: true,
            extended: false,
        }
    }
    
    /// تجاهل الحالة
    pub fn ignore_case(mut self) -> Self {
        self.ignore_case = true;
        self
    }
    
    /// متعدد الأسطر
    pub fn multiline(mut self) -> Self {
        self.multiline = true;
        self
    }
    
    /// نقطة تطابق كل شيء
    pub fn dot_all(mut self) -> Self {
        self.dot_all = true;
        self
    }
    
    /// تحويل إلى نص الأعلام
    fn to_flags_string(&self) -> String {
        let mut flags = String::new();
        if self.ignore_case { flags.push('i'); }
        if self.multiline { flags.push('m'); }
        if self.dot_all { flags.push('s'); }
        if self.unicode { flags.push('u'); }
        if !self.greedy { flags.push('U'); }
        if self.extended { flags.push('x'); }
        flags
    }
}

impl RegexPattern {
    /// إنشاء نمط جديد
    pub fn new(pattern: &str) -> Result<Self, String> {
        Self::with_flags(pattern, RegexFlags::new())
    }
    
    /// إنشاء مع أعلام
    pub fn with_flags(pattern: &str, flags: RegexFlags) -> Result<Self, String> {
        let mut regex_flags = String::new();
        if flags.ignore_case { regex_flags.push('i'); }
        if flags.multiline { regex_flags.push('m'); }
        if flags.dot_all { regex_flags.push('s'); }
        if flags.unicode { regex_flags.push('u'); }
        
        let full_pattern = if regex_flags.is_empty() {
            pattern.to_string()
        } else {
            format!("(?{}){}", regex_flags, pattern)
        };
        
        let compiled = regex::Regex::new(&full_pattern)
            .map_err(|e| format!("خطأ في النمط: {}", e))?;
        
        Ok(Self {
            pattern: pattern.to_string(),
            flags,
            compiled: Some(compiled),
        })
    }
    
    /// هل يطابق؟
    pub fn is_match(&self, text: &str) -> bool {
        if let Some(ref re) = self.compiled {
            re.is_match(text)
        } else {
            false
        }
    }
    
    /// البحث عن أول تطابق
    pub fn find(&self, text: &str) -> Option<Match> {
        if let Some(ref re) = self.compiled {
            re.find(text).map(|m| Match {
                start: m.start(),
                end: m.end(),
                text: m.as_str().to_string(),
            })
        } else {
            None
        }
    }
    
    /// البحث عن جميع التطابقات
    pub fn find_all(&self, text: &str) -> Vec<Match> {
        if let Some(ref re) = self.compiled {
            re.find_iter(text)
                .map(|m| Match {
                    start: m.start(),
                    end: m.end(),
                    text: m.as_str().to_string(),
                })
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// استخراج المجموعات
    pub fn captures(&self, text: &str) -> Option<Captures> {
        if let Some(ref re) = self.compiled {
            re.captures(text).map(|caps| {
                let mut groups = Vec::new();
                let mut named = HashMap::new();
                
                for (i, cap) in caps.iter().enumerate() {
                    if let Some(m) = cap {
                        groups.push(Some(Match {
                            start: m.start(),
                            end: m.end(),
                            text: m.as_str().to_string(),
                        }));
                    } else {
                        groups.push(None);
                    }
                }
                
                // المجموعات المسماة
                for name in re.capture_names().flatten() {
                    if let Some(cap) = caps.name(name) {
                        named.insert(
                            name.to_string(),
                            Match {
                                start: cap.start(),
                                end: cap.end(),
                                text: cap.as_str().to_string(),
                            },
                        );
                    }
                }
                
                Captures { groups, named }
            })
        } else {
            None
        }
    }
    
    /// استبدال
    pub fn replace(&self, text: &str, replacement: &str) -> String {
        if let Some(ref re) = self.compiled {
            re.replace(text, replacement).to_string()
        } else {
            text.to_string()
        }
    }
    
    /// استبدال بكل التطابقات
    pub fn replace_all(&self, text: &str, replacement: &str) -> String {
        if let Some(ref re) = self.compiled {
            re.replace_all(text, replacement).to_string()
        } else {
            text.to_string()
        }
    }
    
    /// استبدال بدالة
    pub fn replace_with<F>(&self, text: &str, mut f: F) -> String
    where
        F: FnMut(&Match) -> String,
    {
        if let Some(ref re) = self.compiled {
            let mut result = String::new();
            let mut last_end = 0;
            
            for m in re.find_iter(text) {
                result.push_str(&text[last_end..m.start()]);
                let match_obj = Match {
                    start: m.start(),
                    end: m.end(),
                    text: m.as_str().to_string(),
                };
                result.push_str(&f(&match_obj));
                last_end = m.end();
            }
            
            result.push_str(&text[last_end..]);
            result
        } else {
            text.to_string()
        }
    }
    
    /// تقسيم النص
    pub fn split(&self, text: &str) -> Vec<String> {
        if let Some(ref re) = self.compiled {
            re.split(text).map(|s| s.to_string()).collect()
        } else {
            vec![text.to_string()]
        }
    }
    
    /// تقسيم مع حد
    pub fn splitn(&self, text: &str, limit: usize) -> Vec<String> {
        if let Some(ref re) = self.compiled {
            re.splitn(text, limit).map(|s| s.to_string()).collect()
        } else {
            vec![text.to_string()]
        }
    }
    
    /// النمط الأصلي
    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

/// تطابق
#[derive(Debug, Clone)]
pub struct Match {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

impl Match {
    /// طول التطابق
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    
    /// هل فارغ؟
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// مجموعات الالتقاط
#[derive(Debug, Clone)]
pub struct Captures {
    pub groups: Vec<Option<Match>>,
    pub named: HashMap<String, Match>,
}

impl Captures {
    /// الحصول على مجموعة بالرقم
    pub fn get(&self, index: usize) -> Option<&Match> {
        self.groups.get(index).and_then(|m| m.as_ref())
    }
    
    /// الحصول على مجموعة بالاسم
    pub fn name(&self, name: &str) -> Option<&Match> {
        self.named.get(name)
    }
    
    /// عدد المجموعات
    pub fn len(&self) -> usize {
        self.groups.len()
    }
    
    /// هل فارغ؟
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }
}

// ===== دوال عربية =====

/// إنشاء نمط جديد
pub fn نمط(pattern: &str) -> Result<RegexPattern, String> {
    RegexPattern::new(pattern)
}

/// هل يطابق؟
pub fn يطابق(pattern: &str, text: &str) -> bool {
    RegexPattern::new(pattern)
        .map(|p| p.is_match(text))
        .unwrap_or(false)
}

/// البحث
pub fn ابحث(pattern: &str, text: &str) -> Vec<Match> {
    RegexPattern::new(pattern)
        .map(|p| p.find_all(text))
        .unwrap_or_default()
}

/// استبدال
pub fn استبدل(pattern: &str, text: &str, replacement: &str) -> String {
    RegexPattern::new(pattern)
        .map(|p| p.replace_all(text, replacement))
        .unwrap_or_else(|_| text.to_string())
}

/// تقسيم
pub fn قسم(pattern: &str, text: &str) -> Vec<String> {
    RegexPattern::new(pattern)
        .map(|p| p.split(text))
        .unwrap_or_else(|_| vec![text.to_string()])
}

// ===== أنماط جاهزة للعربية =====

/// أنماط عربية جاهزة
pub mod arabic_patterns {
    use super::RegexPattern;
    
    /// الأحرف العربية
    pub fn arabic_letters() -> RegexPattern {
        RegexPattern::new(r"[\u0600-\u06FF]+").unwrap()
    }
    
    /// الكلمات العربية
    pub fn arabic_words() -> RegexPattern {
        RegexPattern::new(r"[\u0600-\u06FF]+(?:\s+[\u0600-\u06FF]+)*").unwrap()
    }
    
    /// الأرقام العربية
    pub fn arabic_numbers() -> RegexPattern {
        RegexPattern::new(r"[٠-٩]+").unwrap()
    }
    
    /// عناوين البريد الإلكتروني
    pub fn email() -> RegexPattern {
        RegexPattern::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap()
    }
    
    /// أرقام الهاتف
    pub fn phone() -> RegexPattern {
        RegexPattern::new(r"\+?[0-9]{1,3}[-.\s]?[0-9]{3,4}[-.\s]?[0-9]{4,}").unwrap()
    }
    
    /// التاريخ (YYYY-MM-DD)
    pub fn date() -> RegexPattern {
        RegexPattern::new(r"\d{4}-\d{2}-\d{2}").unwrap()
    }
    
    /// الوقت (HH:MM:SS)
    pub fn time() -> RegexPattern {
        RegexPattern::new(r"\d{2}:\d{2}(?::\d{2})?").unwrap()
    }
    
    /// URL
    pub fn url() -> RegexPattern {
        RegexPattern::new(r"https?://[^\s]+").unwrap()
    }
    
    /// IP Address
    pub fn ip_address() -> RegexPattern {
        RegexPattern::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap()
    }
}
