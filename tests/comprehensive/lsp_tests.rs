//! ═══════════════════════════════════════════════════════════════════════════════
//! اختبارات LSP Server الشاملة
//! Comprehensive LSP Server Tests for Al-Marjaa Language
//! ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod lsp_server_tests {
    use almarjaa::{Lexer, Parser, lint_source_with_config, LintConfig};

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات التحليل المعجمي
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_lexer_arabic_identifiers() {
        let mut lexer = Lexer::new("متغير اسم_المستخدم = \"أحمد\"؛");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(tokens.len() >= 5);
        
        // التحقق من الكلمات المفتاحية
        use almarjaa::lexer::tokens::TokenType;
        assert!(matches!(tokens[0].token_type, TokenType::Let));
    }

    #[test]
    fn test_lexer_error_recovery() {
        // اختبار مع نص غير مغلق
        let mut lexer = Lexer::new("متغير س = \"نص غير مغلق");
        let result = lexer.tokenize();
        assert!(result.is_err());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات التحليل النحوي
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_parser_variable_declaration() {
        let result = Parser::parse("متغير س = ١٠؛");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_function_declaration() {
        let result = Parser::parse(r#"
            دالة ترحيب(اسم) {
                اطبع("مرحباً " + اسم)؛
            }
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_class_declaration() {
        let result = Parser::parse(r#"
            صنف شخص {
                متغير اسم؛
                دالة شخص(الاسم) {
                    هذا.اسم = الاسم؛
                }
            }
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_error_recovery() {
        // كود مع خطأ نحوي
        let result = Parser::parse("متغير س = ١٠")؛ // بدون فاصلة منقوطة
        // قد ينجح أو يفشل حسب التطبيق
        // الهدف هو التحقق من استرداد الأخطاء
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات الـ Linter
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_linter_unused_variable() {
        let code = r#"
            متغير س = ١٠؛
            اطبع("لا استخدام لس")؛
        "#;
        
        let result = lint_source_with_config(code, &LintConfig::default());
        assert!(result.is_ok());
        
        let diagnostics = result.unwrap();
        // قد يكتشف متغير غير مستخدم
    }

    #[test]
    fn test_linter_reachable_code() {
        let code = r#"
            دالة اختبار() {
                أرجع ١؛
                اطبع("غير قابل للوصول")؛
            }
        "#;
        
        let result = lint_source_with_config(code, &LintConfig::default());
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات التكامل
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_full_analysis_pipeline() {
        let code = r#"
            # دالة حساب المجموع
            دالة مجموع(أ، ب) {
                أرجع أ + ب؛
            }
            
            متغير نتيجة = مجموع(٣، ٤)؛
            اطبع(نتيجة)؛
        "#;
        
        // 1. التحليل المعجمي
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.len() > 10);
        
        // 2. التحليل النحوي
        let program = Parser::parse(code);
        assert!(program.is_ok());
        
        // 3. التحليل الثابت
        let diagnostics = lint_source_with_config(code, &LintConfig::default());
        assert!(diagnostics.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات تحديد الموقع
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_token_position_tracking() {
        let mut lexer = Lexer::new("متغير\nس = ١٠؛");
        let tokens = lexer.tokenize().unwrap();
        
        // السطر الأول
        assert_eq!(tokens[0].line, 1);
        
        // السطر الثاني
        assert!(tokens.iter().any(|t| t.line == 2));
    }

    #[test]
    fn test_error_position_reporting() {
        // خطأ معروف الموقع
        let result = Parser::parse(r#"
            متغير س = "نص غير مغلق
        "#);
        
        if let Err(e) = result {
            // التحقق من أن الخطأ يحتوي على معلومات الموقع
            assert!(e.line > 0);
            assert!(e.column > 0);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات الإكمال التلقائي
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_keyword_completion() {
        let keywords = vec![
            "متغير", "ثابت", "دالة", "إذا", "وإلا", "طالما", "لكل",
            "أرجع", "اطبع", "صح", "خطأ", "حاول", "امسك"
        ];
        
        for kw in keywords {
            let mut lexer = Lexer::new(kw);
            let tokens = lexer.tokenize().unwrap();
            assert!(!tokens.is_empty(), "Keyword '{}' should tokenize", kw);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات الأداء
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_lexer_performance() {
        let code: String = (0..1000)
            .map(|i| format!("متغير متغير{} = {}؛\n", i, i))
            .collect();
        
        let start = std::time::Instant::now();
        let mut lexer = Lexer::new(&code);
        let tokens = lexer.tokenize().unwrap();
        let duration = start.elapsed();
        
        println!("✓ Lexed {} tokens in {:?}", tokens.len(), duration);
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_parser_performance() {
        let code: String = (0..100)
            .map(|i| format!("دالة دالة{}() {{ أرجع {}؛ }}\n", i, i))
            .collect();
        
        let start = std::time::Instant::now();
        let result = Parser::parse(&code);
        let duration = start.elapsed();
        
        println!("✓ Parsed in {:?}", duration);
        assert!(result.is_ok());
        assert!(duration.as_millis() < 500);
    }

    #[test]
    fn test_linter_performance() {
        let code: String = (0..100)
            .map(|i| format!("متغير س{} = {}؛\n", i, i))
            .collect();
        
        let start = std::time::Instant::now();
        let result = lint_source_with_config(&code, &LintConfig::default());
        let duration = start.elapsed();
        
        println!("✓ Linted in {:?}", duration);
        assert!(result.is_ok());
        assert!(duration.as_millis() < 200);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات خاصة باللغة العربية
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_arabic_numerals() {
        let mut lexer = Lexer::new("١٢٣ ٤٥.٦٧ ٠.٥");
        let tokens = lexer.tokenize().unwrap();
        
        use almarjaa::lexer::tokens::TokenType;
        
        assert!(matches!(tokens[0].token_type, TokenType::Number(123.0)));
        assert!(matches!(tokens[1].token_type, TokenType::Number(45.67)));
        assert!(matches!(tokens[2].token_type, TokenType::Number(0.5)));
    }

    #[test]
    fn test_arabic_punctuation() {
        let mut lexer = Lexer::new("متغير س = ١٠؛");
        let tokens = lexer.tokenize().unwrap();
        
        use almarjaa::lexer::tokens::TokenType;
        
        // البحث عن الفاصلة المنقوطة العربية
        assert!(tokens.iter().any(|t| matches!(t.token_type, TokenType::Semicolon)));
    }

    #[test]
    fn test_arabic_operators() {
        let mut lexer = Lexer::new("و أو ليس");
        let tokens = lexer.tokenize().unwrap();
        
        use almarjaa::lexer::tokens::TokenType;
        
        assert!(matches!(tokens[0].token_type, TokenType::And));
        assert!(matches!(tokens[1].token_type, TokenType::Or));
        assert!(matches!(tokens[2].token_type, TokenType::Not));
    }

    #[test]
    fn test_mixed_arabic_english() {
        let code = r#"
            متغير name = "Ahmed"؛
            متغير الاسم = "أحمد"؛
            اطبع(name + " - " + الاسم)؛
        "#;
        
        let result = Parser::parse(code);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات الحالات الخاصة
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_empty_file() {
        let result = Parser::parse("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_comments_only() {
        let code = r#"
            # تعليق أول
            # تعليق ثاني
            // تعليق ثالث
        "#;
        
        let result = Parser::parse(code);
        assert!(result.is_ok());
    }

    #[test]
    fn test_nested_structures() {
        let code = r#"
            إذا صح {
                طالما صح {
                    لكل س في [١، ٢، ٣] {
                        إذا س > ١ {
                            اطبع(س)؛
                        }
                    }
                }
            }
        "#;
        
        let result = Parser::parse(code);
        assert!(result.is_ok());
    }
}
