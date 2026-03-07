//! ═══════════════════════════════════════════════════════════════════════════════
//! اختبارات المفسر الشاملة
//! Comprehensive Interpreter Tests for Al-Marjaa Language
//! ═══════════════════════════════════════════════════════════════════════════════

use almarjaa::Interpreter;

#[cfg(test)]
mod interpreter_comprehensive_tests {
    use super::*;

    fn create_interpreter() -> Interpreter {
        Interpreter::new()
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات المتغيرات
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_variable_declaration() {
        let mut interp = create_interpreter();
        let result = interp.run("متغير س = ١٠؛");
        assert!(result.is_ok());
    }

    #[test]
    fn test_constant_declaration() {
        let mut interp = create_interpreter();
        let result = interp.run("ثابت PI = ٣.١٤؛");
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_variables() {
        let mut interp = create_interpreter();
        let result = interp.run("متغير أ = ١، ب = ٢، ج = ٣؛");
        assert!(result.is_ok());
    }

    #[test]
    fn test_variable_reassignment() {
        let mut interp = create_interpreter();
        interp.run("متغير س = ١٠؛").unwrap();
        let result = interp.run("س = ٢٠؛");
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات العمليات الحسابية
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_arithmetic_operations() {
        let mut interp = create_interpreter();
        
        // جمع
        let result = interp.run("متغير نتيجة = ٥ + ٣؛");
        assert!(result.is_ok());
        
        // طرح
        let result = interp.run("متغير نتيجة = ١٠ - ٤؛");
        assert!(result.is_ok());
        
        // ضرب
        let result = interp.run("متغير نتيجة = ٦ * ٧؛");
        assert!(result.is_ok());
        
        // قسمة
        let result = interp.run("متغير نتيجة = ٢٠ / ٤؛");
        assert!(result.is_ok());
        
        // باقي القسمة
        let result = interp.run("متغير نتيجة = ١٧ % ٥؛");
        assert!(result.is_ok());
        
        // أس
        let result = interp.run("متغير نتيجة = ٢ ^ ٣؛");
        assert!(result.is_ok());
    }

    #[test]
    fn test_compound_assignment() {
        let mut interp = create_interpreter();
        interp.run("متغير س = ١٠؛").unwrap();
        
        let result = interp.run("س += ٥؛");
        assert!(result.is_ok());
        
        let result = interp.run("س -= ٣؛");
        assert!(result.is_ok());
        
        let result = interp.run("س *= ٢؛");
        assert!(result.is_ok());
    }

    #[test]
    fn test_increment_decrement() {
        let mut interp = create_interpreter();
        interp.run("متغير س = ٥؛").unwrap();
        
        let result = interp.run("س++؛");
        assert!(result.is_ok());
        
        let result = interp.run("س--؛");
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات المقارنات
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_comparison_operations() {
        let mut interp = create_interpreter();
        
        // مساوٍ
        let result = interp.run("متغير نتيجة = ٥ == ٥؛");
        assert!(result.is_ok());
        
        // غير مساوٍ
        let result = interp.run("متغير نتيجة = ٥ != ٣؛");
        assert!(result.is_ok());
        
        // أكبر
        let result = interp.run("متغير نتيجة = ١٠ > ٥؛");
        assert!(result.is_ok());
        
        // أصغر
        let result = interp.run("متغير نتيجة = ٣ < ٧؛");
        assert!(result.is_ok());
        
        // أكبر أو يساوي
        let result = interp.run("متغير نتيجة = ٥ >= ٥؛");
        assert!(result.is_ok());
        
        // أصغر أو يساوي
        let result = interp.run("متغير نتيجة = ٣ <= ٣؛");
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات المنطق
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_logical_operations() {
        let mut interp = create_interpreter();
        
        // و
        let result = interp.run("متغير نتيجة = صح و صح؛");
        assert!(result.is_ok());
        
        // أو
        let result = interp.run("متغير نتيجة = خطأ أو صح؛");
        assert!(result.is_ok());
        
        // ليس
        let result = interp.run("متغير نتيجة = ليس خطأ؛");
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات الجمل الشرطية
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_if_statement() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير س = ١٠؛
            إذا س > ٥ {
                متغير نتيجة = "كبير"؛
            }
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_if_else_statement() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير س = ٣؛
            إذا س > ٥ {
                متغير نتيجة = "كبير"؛
            } وإلا {
                متغير نتيجة = "صغير"؛
            }
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_if_elseif_else_statement() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير درجة = ٧٥؛
            إذا درجة >= ٩٠ {
                متغير تقدير = "ممتاز"؛
            } وإذا درجة >= ٧٥ {
                متغير تقدير = "جيد جداً"؛
            } وإذا درجة >= ٦٠ {
                متغير تقدير = "مقبول"؛
            } وإلا {
                متغير تقدير = "راسب"؛
            }
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات الحلقات
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_while_loop() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير س = ٠؛
            طالما س < ٥ {
                س += ١؛
            }
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_for_loop() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير مجموع = ٠؛
            لكل عنصر في [١، ٢، ٣، ٤، ٥] {
                مجموع += عنصر؛
            }
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_for_range_loop() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير مجموع = ٠؛
            لكل س في مدى(١، ١٠) {
                مجموع += س؛
            }
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_repeat_loop() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير س = ٠؛
            كرر ٥ مرة {
                س += ١؛
            }
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_break_statement() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير س = ٠؛
            طالما صح {
                س += ١؛
                إذا س > ٥ {
                    توقف؛
                }
            }
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_continue_statement() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            لكل س في [١، ٢، ٣، ٤، ٥] {
                إذا س == ٣ {
                    أكمل؛
                }
            }
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات الدوال
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_function_definition() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            دالة ترحيب() {
                اطبع("مرحباً!")؛
            }
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_function_with_parameters() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            دالة مجموع(أ، ب) {
                أرجع أ + ب؛
            }
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_function_call() {
        let mut interp = create_interpreter();
        interp.run(r#"
            دالة مجموع(أ، ب) {
                أرجع أ + ب؛
            }
        "#).unwrap();
        
        let result = interp.run("متغير نتيجة = مجموع(٣، ٤)؛");
        assert!(result.is_ok());
    }

    #[test]
    fn test_recursive_function() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            دالة عاملي(ن) {
                إذا ن <= ١ {
                    أرجع ١؛
                }
                أرجع ن * عاملي(ن - ١)؛
            }
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lambda_function() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير مربع = لامدا(س) => س * س؛
            متغير نتيجة = مربع(٥)؛
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات القوائم
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_list_creation() {
        let mut interp = create_interpreter();
        let result = interp.run("متغير أرقام = [١، ٢، ٣، ٤، ٥]؛");
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_access() {
        let mut interp = create_interpreter();
        interp.run("متغير أرقام = [١، ٢، ٣]؛").unwrap();
        let result = interp.run("متغير أول = أرقام[٠]؛");
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_comprehension() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير مربعات = [س * س لكل س في [١، ٢، ٣، ٤، ٥]]؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_comprehension_with_condition() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير زوجية = [س لكل س في [١، ٢، ٣، ٤، ٥، ٦] إذا س % ٢ == ٠]؛
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات القواميس
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_dict_creation() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير شخص = {
                "اسم": "أحمد"،
                "عمر": ٢٥،
                "مدينة": "القاهرة"
            }؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_dict_access() {
        let mut interp = create_interpreter();
        interp.run(r#"
            متغير شخص = {"اسم": "أحمد"، "عمر": ٢٥}؛
        "#).unwrap();
        let result = interp.run(r#"متغير اسم = شخص["اسم"]؛"#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات الفئات
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_class_definition() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            صنف شخص {
                متغير اسم؛
                متغير عمر؛
                
                دالة شخص(الاسم، العمر) {
                    هذا.اسم = الاسم؛
                    هذا.عمر = العمر؛
                }
                
                دالة تعريف() {
                    أرجع هذا.اسم + " - " + هذا.عمر؛
                }
            }
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات معالجة الأخطاء
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_try_catch() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            حاول {
                متغير نتيجة = ١٠ / ٠؛
            } امسك(خطأ) {
                اطبع("خطأ: " + خطأ)؛
            }
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_throw_statement() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            دالة تحقق(قيمة) {
                إذا قيمة < ٠ {
                    ألقِ "القيمة سالبة!"؛
                }
            }
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات التفكيك
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_list_destructuring() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير [أ، ب، ج] = [١، ٢، ٣]؛
        "#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_object_destructuring() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            متغير شخص = {"اسم": "أحمد"، "عمر": ٢٥}؛
            متغير {اسم، عمر} = شخص؛
        "#);
        assert!(result.is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات الأداء
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_interpreter_performance() {
        let mut interp = create_interpreter();
        
        let code = r#"
            متغير مجموع = ٠؛
            لكل س في مدى(١، ١٠٠٠) {
                مجموع += س؛
            }
        "#;
        
        let start = std::time::Instant::now();
        let result = interp.run(code);
        let duration = start.elapsed();
        
        println!("✓ Loop execution time: {:?}", duration);
        assert!(result.is_ok());
        // يجب أن يكون أقل من 100ms
        assert!(duration.as_millis() < 100);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // اختبارات شاملة
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_complete_program() {
        let mut interp = create_interpreter();
        let result = interp.run(r#"
            # حساب المجموع والمتوسط
            دالة حساب_الإحصائيات(أرقام) {
                متغير مجموع = ٠؛
                لكل رقم في أرقام {
                    مجموع += رقم؛
                }
                متغير المتوسط = مجموع / طول(أرقام)؛
                أرجع [مجموع، المتوسط]؛
            }
            
            متغير بيانات = [١٠، ٢٠، ٣٠، ٤٠، ٥٠]؛
            متغير [المجموع، المتوسط] = حساب_الإحصائيات(بيانات)؛
            
            اطبع("المجموع: " + المجموع)؛
            اطبع("المتوسط: " + المتوسط)؛
        "#);
        assert!(result.is_ok());
    }
}
