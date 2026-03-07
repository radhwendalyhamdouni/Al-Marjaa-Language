//! ═══════════════════════════════════════════════════════════════════════════════
//! اختبارات المحلل المعجمي الشاملة
//! Comprehensive Lexer Tests for Al-Marjaa Language
//! ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::Lexer;
use almarjaa::lexer::tokens::TokenType;

#[cfg(test)]
mod lexer_comprehensive_tests {
    use super::*;

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات الأرقام
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_arabic_numbers() {
        let mut lexer = Lexer::new("١٢٣ ٤٥.٦٧ ٠.٥");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 4); // 3 numbers + EOF
        assert!(matches!(tokens[0].token_type, TokenType::Number(123.0)));
        assert!(matches!(tokens[1].token_type, TokenType::Number(45.67)));
        assert!(matches!(tokens[2].token_type, TokenType::Number(0.5)));
    }

    #[test]
    fn test_ascii_numbers() {
        let mut lexer = Lexer::new("123 45.67 0.5");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0].token_type, TokenType::Number(123.0)));
        assert!(matches!(tokens[1].token_type, TokenType::Number(45.67)));
        assert!(matches!(tokens[2].token_type, TokenType::Number(0.5)));
    }

    #[test]
    fn test_hexadecimal_numbers() {
        let mut lexer = Lexer::new("0xFF 0x1A 0xABCD");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::Number(255.0)));
        assert!(matches!(tokens[1].token_type, TokenType::Number(26.0)));
        assert!(matches!(tokens[2].token_type, TokenType::Number(43981.0)));
    }

    #[test]
    fn test_binary_numbers() {
        let mut lexer = Lexer::new("0b1010 0b1111 0b0");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::Number(10.0)));
        assert!(matches!(tokens[1].token_type, TokenType::Number(15.0)));
        assert!(matches!(tokens[2].token_type, TokenType::Number(0.0)));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات النصوص
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_arabic_strings() {
        let mut lexer = Lexer::new("\"مرحباً بالعالم\" 'نص عربي'");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::String(_)));
        assert!(matches!(tokens[1].token_type, TokenType::String(_)));
    }

    #[test]
    fn test_escape_sequences() {
        let mut lexer = Lexer::new("\"نص\\nجديد\" \"تاب\\t\\\"اقتباس\\\"\"");
        let tokens = lexer.tokenize().unwrap();
        
        if let TokenType::String(s) = &tokens[0].token_type {
            assert!(s.contains('\n'));
        }
    }

    #[test]
    fn test_multiline_string() {
        let mut lexer = Lexer::new("`سطر أول\nسطر ثاني\nسطر ثالث`");
        let tokens = lexer.tokenize().unwrap();
        
        if let TokenType::String(s) = &tokens[0].token_type {
            assert!(s.contains('\n'));
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات الكلمات المفتاحية العربية
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_arabic_keywords() {
        let mut lexer = Lexer::new("متغير ثابت دالة إذا وإلا طالما لكل");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::Let));
        assert!(matches!(tokens[1].token_type, TokenType::Const));
        assert!(matches!(tokens[2].token_type, TokenType::Function));
        assert!(matches!(tokens[3].token_type, TokenType::If));
        assert!(matches!(tokens[4].token_type, TokenType::Else));
        assert!(matches!(tokens[5].token_type, TokenType::While));
        assert!(matches!(tokens[6].token_type, TokenType::For));
    }

    #[test]
    fn test_short_keywords() {
        let mut lexer = Lexer::new("م س = ١٠؛ ث ص = صح؛");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::Let));
        assert!(matches!(tokens[3].token_type, TokenType::Const));
    }

    #[test]
    fn test_control_flow_keywords() {
        let mut lexer = Lexer::new("أرجع توقف أكمل حاول امسك ألقِ");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::Return));
        assert!(matches!(tokens[1].token_type, TokenType::Break));
        assert!(matches!(tokens[2].token_type, TokenType::Continue));
        assert!(matches!(tokens[3].token_type, TokenType::Try));
        assert!(matches!(tokens[4].token_type, TokenType::Catch));
        assert!(matches!(tokens[5].token_type, TokenType::Throw));
    }

    #[test]
    fn test_boolean_keywords() {
        let mut lexer = Lexer::new("صح خطأ لا_شيء");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::True));
        assert!(matches!(tokens[1].token_type, TokenType::False));
        assert!(matches!(tokens[2].token_type, TokenType::NullKeyword));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات المعاملات
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_arithmetic_operators() {
        let mut lexer = Lexer::new("+ - * / % ^ //");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::Plus));
        assert!(matches!(tokens[1].token_type, TokenType::Minus));
        assert!(matches!(tokens[2].token_type, TokenType::Multiply));
        assert!(matches!(tokens[3].token_type, TokenType::Divide));
        assert!(matches!(tokens[4].token_type, TokenType::Modulo));
        assert!(matches!(tokens[5].token_type, TokenType::Power));
        assert!(matches!(tokens[6].token_type, TokenType::FloorDiv));
    }

    #[test]
    fn test_comparison_operators() {
        let mut lexer = Lexer::new("== != < > <= >=");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::Equal));
        assert!(matches!(tokens[1].token_type, TokenType::NotEqual));
        assert!(matches!(tokens[2].token_type, TokenType::Less));
        assert!(matches!(tokens[3].token_type, TokenType::Greater));
        assert!(matches!(tokens[4].token_type, TokenType::LessEqual));
        assert!(matches!(tokens[5].token_type, TokenType::GreaterEqual));
    }

    #[test]
    fn test_assignment_operators() {
        let mut lexer = Lexer::new("= += -= *= /= %= ^=");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::Assign));
        assert!(matches!(tokens[1].token_type, TokenType::PlusAssign));
        assert!(matches!(tokens[2].token_type, TokenType::MinusAssign));
        assert!(matches!(tokens[3].token_type, TokenType::MultAssign));
        assert!(matches!(tokens[4].token_type, TokenType::DivAssign));
        assert!(matches!(tokens[5].token_type, TokenType::ModAssign));
        assert!(matches!(tokens[6].token_type, TokenType::PowAssign));
    }

    #[test]
    fn test_logical_operators() {
        let mut lexer = Lexer::new("و أو ليس && || !");
        let tokens = lexer.tokenize().unwrap();
        
        // Arabic keywords
        assert!(matches!(tokens[0].token_type, TokenType::And));
        assert!(matches!(tokens[1].token_type, TokenType::Or));
        assert!(matches!(tokens[2].token_type, TokenType::Not));
        // Symbolic operators
        assert!(matches!(tokens[3].token_type, TokenType::And));
        assert!(matches!(tokens[4].token_type, TokenType::Or));
        assert!(matches!(tokens[5].token_type, TokenType::Not));
    }

    #[test]
    fn test_increment_decrement() {
        let mut lexer = Lexer::new("++ --");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::Increment));
        assert!(matches!(tokens[1].token_type, TokenType::Decrement));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات الفواصل والأقواس
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_arabic_punctuation() {
        let mut lexer = Lexer::new("؛ ،");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::Semicolon));
        assert!(matches!(tokens[1].token_type, TokenType::Comma));
    }

    #[test]
    fn test_brackets() {
        let mut lexer = Lexer::new("( ) { } [ ]");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::LParen));
        assert!(matches!(tokens[1].token_type, TokenType::RParen));
        assert!(matches!(tokens[2].token_type, TokenType::LBrace));
        assert!(matches!(tokens[3].token_type, TokenType::RBrace));
        assert!(matches!(tokens[4].token_type, TokenType::LBracket));
        assert!(matches!(tokens[5].token_type, TokenType::RBracket));
    }

    #[test]
    fn test_special_operators() {
        let mut lexer = Lexer::new("-> => |> .. ... ? ?. ?? @");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::Arrow));
        assert!(matches!(tokens[1].token_type, TokenType::FatArrow));
        assert!(matches!(tokens[2].token_type, TokenType::Pipe));
        assert!(matches!(tokens[3].token_type, TokenType::DotDot));
        assert!(matches!(tokens[4].token_type, TokenType::DotDotDot));
        assert!(matches!(tokens[5].token_type, TokenType::Question));
        assert!(matches!(tokens[6].token_type, TokenType::QuestionDot));
        assert!(matches!(tokens[7].token_type, TokenType::QuestionQuestion));
        assert!(matches!(tokens[8].token_type, TokenType::At));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات التعليقات
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_single_line_comments() {
        let mut lexer = Lexer::new("# هذا تعليق\nمتغير س = ١؛");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::Comment(_)));
        assert!(matches!(tokens[1].token_type, TokenType::Let));
    }

    #[test]
    fn test_double_slash_comments() {
        let mut lexer = Lexer::new("// تعليق\nمتغير");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::Comment(_)));
        assert!(matches!(tokens[1].token_type, TokenType::Let));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات المعرفات العربية
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_arabic_identifiers() {
        let mut lexer = Lexer::new("اسم عمر_محمد درجة_الحرارة");
        let tokens = lexer.tokenize().unwrap();
        
        if let TokenType::Identifier(s) = &tokens[0].token_type {
            assert_eq!(s, "اسم");
        }
        if let TokenType::Identifier(s) = &tokens[1].token_type {
            assert_eq!(s, "عمر_محمد");
        }
        if let TokenType::Identifier(s) = &tokens[2].token_type {
            assert_eq!(s, "درجة_الحرارة");
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات الأداء
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_lexer_performance() {
        // إنشاء كود كبير للاختبار
        let large_code: String = (0..1000)
            .map(|i| format!("متغير متغير{} = {}؛\n", i, i))
            .collect();
        
        let start = std::time::Instant::now();
        let mut lexer = Lexer::new(&large_code);
        let tokens = lexer.tokenize().unwrap();
        let duration = start.elapsed();
        
        println!("✓ Lexed {} tokens in {:?}", tokens.len(), duration);
        
        // يجب أن يكون أقل من 100ms لـ 1000 سطر
        assert!(duration.as_millis() < 100);
        assert!(tokens.len() > 5000);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات الموقع
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_token_positions() {
        let mut lexer = Lexer::new("متغير\nس = ١٠؛");
        let tokens = lexer.tokenize().unwrap();
        
        // السطر الأول
        assert_eq!(tokens[0].line, 1);
        
        // السطر الثاني
        assert_eq!(tokens[1].line, 2);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات شاملة للكود العربي
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_complete_arabic_code() {
        let code = r#"
            متغير اسم = "أحمد"؛
            ثابت عمر = ٢٥؛
            
            دالة ترحيب(شخص) {
                اطبع("مرحباً " + شخص)؛
            }
            
            إذا عمر > ١٨ {
                ترحيب(اسم)؛
            } وإلا {
                اطبع("قاصر")؛
            }
        "#;
        
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        
        // التحقق من عدم وجود أخطاء
        assert!(tokens.len() > 20);
        
        // التحقق من الكلمات المفتاحية
        let keywords: Vec<&TokenType> = tokens.iter()
            .map(|t| &t.token_type)
            .filter(|t| matches!(t, 
                TokenType::Let | TokenType::Const | TokenType::Function | 
                TokenType::If | TokenType::Else | TokenType::Print
            ))
            .collect();
        
        assert!(keywords.len() >= 5);
    }
}
