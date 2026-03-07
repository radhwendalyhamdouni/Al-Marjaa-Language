//! Arabic Support - دعم اللغة العربية

use unicode_segmentation::UnicodeSegmentation;

/// معالجة النص العربي
pub struct ArabicTextProcessor {
    // الأحرف العربية
    arabic_chars: Vec<char>,
}

impl ArabicTextProcessor {
    pub fn new() -> Self {
        Self {
            arabic_chars: "ءآأؤإئابةتثجحخدذرزسشصضطظعغفقكلمنهوىي".chars().collect(),
        }
    }
    
    /// هل الحرف عربي؟
    pub fn is_arabic(&self, c: char) -> bool {
        self.arabic_chars.contains(&c)
    }
    
    /// هل الكلمة عربية؟
    pub fn is_arabic_word(&self, word: &str) -> bool {
        word.chars().all(|c| self.is_arabic(c) || c.is_numeric() || c == '_')
    }
    
    /// تحويل الأرقام العربية إلى هندية والعكس
    pub fn convert_digits(&self, text: &str, to_arabic: bool) -> String {
        let western = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let eastern = ['٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩'];
        
        text.chars().map(|c| {
            if to_arabic {
                if let Some(idx) = western.iter().position(|&d| d == c) {
                    eastern[idx]
                } else {
                    c
                }
            } else {
                if let Some(idx) = eastern.iter().position(|&d| d == c) {
                    western[idx]
                } else {
                    c
                }
            }
        }).collect()
    }
    
    /// تحسين العرض للنص العربي (RTL)
    pub fn rtl_display(&self, text: &str) -> String {
        // إضافة علامات RTL إذا لزم الأمر
        format!("\u{202B}{}\u{202C}", text)
    }
    
    /// الحصول على اتجاه الكتابة
    pub fn get_direction(&self, text: &str) -> TextDirection {
        let arabic_count = text.chars().filter(|&c| self.is_arabic(c)).count();
        let total_alpha: usize = text.chars().filter(|c| c.is_alphabetic()).count();
        
        if total_alpha == 0 {
            return TextDirection::Neutral;
        }
        
        if arabic_count as f32 / total_alpha as f32 > 0.3 {
            TextDirection::RightToLeft
        } else {
            TextDirection::LeftToRight
        }
    }
    
    /// تقسيم الكلمات العربية
    pub fn tokenize_arabic(&self, text: &str) -> Vec<String> {
        text.unicode_words()
            .map(|w| w.to_string())
            .collect()
    }
    
    /// التحقق من صحة الاسم العربي
    pub fn is_valid_identifier(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        
        let chars: Vec<char> = name.chars().collect();
        
        // الحرف الأول يجب أن يكون حرفاً أو _
        if !chars[0].is_alphabetic() && chars[0] != '_' {
            return false;
        }
        
        // باقي الأحرف
        chars.iter().all(|&c| c.is_alphanumeric() || c == '_')
    }
    
    /// اقتراحات التصحيح للأخطاء الإملائية العربية
    pub fn suggest_corrections(&self, word: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // الألف الممدودة
        if word.contains('ا') {
            suggestions.push(word.replace('ا', "أ"));
            suggestions.push(word.replace('ا', "إ"));
            suggestions.push(word.replace('ا', "آ"));
        }
        
        // الهاء والتاء المربوطة
        if word.ends_with('ه') {
            suggestions.push(format!("{}ة", &word[..word.len()-1]));
        }
        if word.ends_with('ة') {
            suggestions.push(format!("{}ه", &word[..word.len()-1]));
        }
        
        // الياء والألف المقصورة
        if word.ends_with('ي') {
            suggestions.push(format!("{}ى", &word[..word.len()-1]));
        }
        if word.ends_with('ى') {
            suggestions.push(format!("{}ي", &word[..word.len()-1]));
        }
        
        suggestions
    }
}

impl Default for ArabicTextProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// اتجاه النص
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
    Neutral,
}

/// الكلمات المفتاحية العربية بالتشكيل وبدون
pub const ARABIC_KEYWORDS: &[(&str, &str)] = &[
    ("متغير", "متغير"),
    ("ثابت", "ثابت"),
    ("دالة", "دالة"),
    ("إذا", "إذا"),
    ("وإلا", "وإلا"),
    ("طالما", "طالما"),
    ("لكل", "لكل"),
    ("أرجع", "أرجع"),
    ("اطبع", "اطبع"),
    ("صح", "صح"),
    ("خطأ", "خطأ"),
];

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_arabic() {
        let processor = ArabicTextProcessor::new();
        assert!(processor.is_arabic('أ'));
        assert!(processor.is_arabic('م'));
        assert!(!processor.is_arabic('a'));
    }
    
    #[test]
    fn test_convert_digits() {
        let processor = ArabicTextProcessor::new();
        assert_eq!(processor.convert_digits("123", true), "١٢٣");
        assert_eq!(processor.convert_digits("١٢٣", false), "123");
    }
    
    #[test]
    fn test_valid_identifier() {
        let processor = ArabicTextProcessor::new();
        assert!(processor.is_valid_identifier("متغير"));
        assert!(processor.is_valid_identifier("الاسم"));
        assert!(processor.is_valid_identifier("_test"));
        assert!(!processor.is_valid_identifier("123abc"));
    }
}
