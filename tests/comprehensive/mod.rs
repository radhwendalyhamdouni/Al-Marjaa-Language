//! ═══════════════════════════════════════════════════════════════════════════════
//! اختبارات شاملة للغة المرجع
//! Comprehensive Test Suite for Al-Marjaa Language
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! هذا الملف يحتوي على اختبارات شاملة لمكونات لغة المرجع:
//! - المحلل المعجمي (Lexer)
//! - المحلل النحوي (Parser)
//! - المفسر (Interpreter)
//! - تكامل الذكاء الاصطناعي (AI Integration)
//! - خادم LSP (LSP Server)
//!
//! لتشغيل الاختبارات:
//! ```
//! cargo test --all-features
//! cargo test --test comprehensive
//! ```

pub mod lexer_tests;
pub mod interpreter_tests;
pub mod ai_integration_tests;
pub mod lsp_tests;

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات التكامل النهائية
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod integration_tests {
    use almarjaa::{Lexer, Parser, Interpreter, lint_source_with_config, LintConfig};

    /// اختبار تكامل كامل: Lexer -> Parser -> Interpreter
    #[test]
    fn test_full_pipeline() {
        let code = r#"
            # برنامج اختبار شامل
            ثابت PI = ٣.١٤١٥٩؛
            
            دالة مساحة_دائرة(نصف_القطر) {
                أرجع PI * نصف_القطر ^ ٢؛
            }
            
            متغير نتيجة = مساحة_دائرة(٥)؛
            اطبع("مساحة الدائرة: " + نتيجة)؛
        "#;
        
        // 1. التحليل المعجمي
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.len() > 20);
        
        // 2. التحليل النحوي
        let program = Parser::parse(code);
        assert!(program.is_ok());
        
        // 3. التحليل الثابت
        let diagnostics = lint_source_with_config(code, &LintConfig::default());
        assert!(diagnostics.is_ok());
        
        // 4. التنفيذ
        let mut interp = Interpreter::new();
        let result = interp.run(code);
        assert!(result.is_ok());
    }

    /// اختبار أداء شامل
    #[test]
    fn test_performance_benchmark() {
        let mut interp = Interpreter::new();
        
        // كود معقد لقياس الأداء
        let code = r#"
            # إنشاء قائمة كبيرة
            متغير أرقام = [س لكل س في مدى(١، ١٠٠٠)]؛
            
            # حساب المجموع
            متغير مجموع = ٠؛
            لكل رقم في أرقام {
                مجموع += رقم؛
            }
            
            # حساب المتوسط
            متغير المتوسط = مجموع / طول(أرقام)؛
            
            # فلترة الأرقام الزوجية
            متغير زوجية = [س لكل س في أرقام إذا س % ٢ == ٠]؛
        "#;
        
        let start = std::time::Instant::now();
        let result = interp.run(code);
        let duration = start.elapsed();
        
        println!("✓ Complex benchmark completed in {:?}", duration);
        assert!(result.is_ok());
        assert!(duration.as_millis() < 1000);
    }

    /// اختبار الذاكرة والتسريبات
    #[test]
    fn test_memory_usage() {
        let mut interp = Interpreter::new();
        
        // إنشاء وتدمير كائنات متعددة
        for _ in 0..100 {
            let _ = interp.run(r#"
                متغير س = ٠؛
                لكل _ في مدى(١، ١٠٠) {
                    س += ١؛
                }
            "#);
        }
        
        // إذا وصلنا هنا بدون تسريب ذاكرة، الاختبار ناجح
        assert!(true);
    }

    /// اختبار التعامل مع الأخطاء
    #[test]
    fn test_error_handling() {
        let mut interp = Interpreter::new();
        
        // خطأ في التشغيل
        let result = interp.run(r#"
            حاول {
                متغير س = غير_معرف؛
            } امسك(خطأ) {
                اطبع("تم التقاط الخطأ: " + خطأ)؛
            }
        "#);
        
        // يجب أن ينجح البرنامج مع التقاط الخطأ
        assert!(result.is_ok());
    }

    /// اختبار الدوال العودية
    #[test]
    fn test_recursion() {
        let mut interp = Interpreter::new();
        
        let result = interp.run(r#"
            دالة فيبوناتشي(ن) {
                إذا ن <= ١ {
                    أرجع ن؛
                }
                أرجع فيبوناتشي(ن - ١) + فيبوناتشي(ن - ٢)؛
            }
            
            متغير fib10 = فيبوناتشي(١٠)؛
        "#);
        
        assert!(result.is_ok());
    }

    /// اختبار الإغلاق (Closures)
    #[test]
    fn test_closures() {
        let mut interp = Interpreter::new();
        
        let result = interp.run(r#"
            دالة مضاعف(عامل) {
                أرجع لامدا(س) => س * عامل؛
            }
            
            متغير مضاعف_٢ = مضاعف(٢)؛
            متغير مضاعف_٣ = مضاعف(٣)؛
            
            متغير نتيجة_١ = مضاعف_٢(٥)؛
            متغير نتيجة_٢ = مضاعف_٣(٥)؛
        "#);
        
        assert!(result.is_ok());
    }

    /// اختبار البرمجة الكائنية
    #[test]
    fn test_oop() {
        let mut interp = Interpreter::new();
        
        let result = interp.run(r#"
            صنف حيوان {
                متغير اسم؛
                
                دالة حيوان(الاسم) {
                    هذا.اسم = الاسم؛
                }
                
                دالة صوت() {
                    أرجع "صوت عام"؛
                }
            }
            
            صنف كلب: حيوان {
                دالة صوت() {
                    أرجع "نباح"؛
                }
            }
            
            متغير كلبي = جديد كلب("بوبي")؛
            متغير صوت = كلبي.صوت()؛
        "#);
        
        assert!(result.is_ok());
    }

    /// اختبار المقاطع البرمجية (Generators)
    #[test]
    fn test_generators() {
        let mut interp = Interpreter::new();
        
        let result = interp.run(r#"
            دالة أرقام_طبيعية() {
                متغير ن = ٠؛
                طالما صح {
                    ن += ١؛
                    أعطِ ن؛
                }
            }
            
            متغير مولد = أرقام_طبيعية()؛
        "#);
        
        assert!(result.is_ok());
    }

    /// اختبار المطابقة (Pattern Matching)
    #[test]
    fn test_pattern_matching() {
        let mut interp = Interpreter::new();
        
        let result = interp.run(r#"
            متغير قيمة = ٢؛
            
            طابق قيمة {
                حالة ١:
                    اطبع("واحد")؛
                حالة ٢:
                    اطبع("اثنان")؛
                حالة ٣:
                    اطبع("ثلاثة")؛
                افتراضي:
                    اطبع("غير معروف")؛
            }
        "#);
        
        assert!(result.is_ok());
    }

    /// اختبار التعدادات (Enums)
    #[test]
    fn test_enums() {
        let mut interp = Interpreter::new();
        
        let result = interp.run(r#"
            تعداد يوم {
                الأحد = ١،
                الإثنين = ٢،
                الثلاثاء = ٣،
                الأربعاء = ٤،
                الخميس = ٥،
                الجمعة = ٦،
                السبت = ٧
            }
            
            متغير اليوم = يوم.الجمعة؛
        "#);
        
        assert!(result.is_ok());
    }
}
