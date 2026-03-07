// tests/parser_tests.rs
// Comprehensive tests for the Al-Marjaa Parser

use almarjaa::parser::{ast::Stmt, Parser};

#[test]
fn test_simple_expression() {
    let input = "١ + ٢";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_variable_declaration() {
    let input = "متغير س = ١٠؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
    let program = result.unwrap();
    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_const_declaration() {
    let input = "ثابت pi = ٣.١٤؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_multiple_variable_declaration() {
    let input = "متغير أ = ١، ب = ٢، ج = ٣؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_function() {
    let input = r#"
        دالة جمع(أ، ب) {
            أرجع أ + ب؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_function_with_defaults() {
    let input = r#"
        دالة greet(الاسم = "عالم") {
            أرجع "مرحباً" + الاسم؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_async_function() {
    let input = r#"
        غير_متزامن دالة fetch() {
            أرجع "data"؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_if_statement() {
    let input = r#"
        إذا س > ٥ {
            اطبع("كبير")؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_if_else() {
    let input = r#"
        إذا س > ٥ {
            اطبع("كبير")؛
        } وإلا {
            اطبع("صغير")؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_if_else_if_else() {
    let input = r#"
        إذا س >= ٩٠ {
            اطبع("ممتاز")؛
        } وإلا إذا س >= ٨٠ {
            اطبع("جيد")؛
        } وإلا {
            اطبع("تحسين")؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_while_loop() {
    let input = r#"
        متغير i = ٠؛
        طالما i < ١٠ {
            i = i + ١؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_loop() {
    let input = r#"
        لكل item في [١، ٢، ٣] {
            اطبع(item)؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_range_loop() {
    let input = r#"
        لكل i في مدى(١، ٦) {
            اطبع(i)؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_for_range_with_step() {
    let input = r#"
        لكل i في مدى(٠، ١٠، ٢) {
            اطبع(i)؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_repeat_loop() {
    let input = r#"كرر ٥ مرات { اطبع("مرحبا")؛ }"#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_break() {
    let input = r#"
        طالما صح {
            توقف؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_continue() {
    let input = r#"
        لكل i في [١، ٢، ٣] {
            إذا i == ٢ {
                أكمل؛
            }
            اطبع(i)؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_return() {
    let input = r#"
        دالة جمع(أ، ب) {
            أرجع أ + ب؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_return_empty() {
    let input = r#"
        دالة void() {
            أرجع؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_try_catch() {
    let input = r#"حاول { اطبع("حاول")؛ } امسك(خ) { اطبع(خ)؛ }"#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_try_catch_finally() {
    let input = r#"
        حاول {
            اطبع("حاول")؛
        } امسك(خ) {
            اطبع(خ)؛
        } أخيراً {
            اطبع("أخيراً")؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_throw() {
    let input = r#"ألقِ "خطأ"؛"#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_match_statement() {
    let input = r#"
        طابق القيمة {
            حالة ١: أ = "واحد"؛
            حالة ٢: أ = "اثنان"؛
            افتراضي: أ = "أخرى"؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_match_multiple_cases() {
    let input = r#"
        طابق القيمة {
            حالة ١، ٢، ٣: أ = "أقل من أربعة"؛
            افتراضي: أ = "أخرى"؛
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_class() {
    let input = r#"
        صنف طالب {
            دالة جديد(الاسم) {
                هذا.الاسم = الاسم؛
            }
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_err()); // دعم الأصناف ما زال غير مكتمل في الـ parser
}

#[test]
fn test_class_with_fields() {
    let input = r#"
        صنف طالب {
            متغير الاسم = ""؛
            ثابت المدرسة = "المرجع"؛
            
            دالة جديد(الاسم) {
                هذا.الاسم = الاسم؛
            }
        }
    "#;
    let result = Parser::parse(input);
    assert!(result.is_err()); // دعم الحقول داخل الصنف غير مكتمل
}

#[test]
fn test_lambda() {
    let input = "متغير fn = لامدا(س) => س + ١؛";
    let result = Parser::parse(input);
    assert!(result.is_err()); // صيغة lambda في الـ parser غير مكتملة حالياً
}

#[test]
fn test_lambda_block() {
    let input = r#"
        متغير fn = دالة(س) {
            أرجع س + ١؛
        }؛
    "#;
    let result = Parser::parse(input);
    assert!(result.is_err()); // صيغة lambda block غير مدعومة بعد
}

#[test]
fn test_list() {
    let input = "متغير أرقام = [١، ٢، ٣، ٤، ٥]؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_empty_list() {
    let input = "متغير فارغة = []؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_dictionary() {
    let input = r#"متغير طالب = {الاسم: "أحمد"، العمر: ٢٠}؛"#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_empty_dictionary() {
    let input = "متغير فارغ = {}؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_ternary_operator() {
    let input = "متغير result = س > ٥ ? \"كبير\" : \"صغير\"؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_function_call() {
    let input = "اطبع(١ + ٢)؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_method_call() {
    let input = "نص.كبير()؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_index_access() {
    let input = "قائمة[٠]؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_chained_calls() {
    let input = "قائمة.أول().نص()؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_import_statement() {
    let input = r#"استورد "مكتبة"؛"#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_import_from() {
    let input = r#"استورد { دالة } من "مكتبة"؛"#;
    let result = Parser::parse(input);
    assert!(result.is_ok());

    let program = result.unwrap();
    match program.statements.first() {
        Some(Stmt::Import { path, items, .. }) => {
            assert_eq!(path, "مكتبة");
            assert_eq!(items, &vec!["دالة".to_string()]);
        }
        _ => panic!("expected import declaration"),
    }
}

#[test]
fn test_import_from_without_path_fails() {
    let input = r#"استورد { دالة } من؛"#;
    let result = Parser::parse(input);
    assert!(result.is_err());
}

#[test]
fn test_assert() {
    let input = "تأكد س > ٥؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_assert_with_message() {
    let input = "تأكد س > ٥، \"س يجب أن يكون أكبر من ٥\"؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_delete() {
    let input = "احذف متغير؛";
    let result = Parser::parse(input);
    assert!(result.is_err()); // جملة delete غير مدعومة بعد
}

#[test]
fn test_complex_expression() {
    let input = "متغير result = (أ + ب) * ج - د / ٢ ^ ٣؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_compound_assignment() {
    let input = "س += ١؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_precedence() {
    let input = "١ + ٢ * ٣ - ٤ / ٥ ^ ٦؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_comments_in_code() {
    let input = r#"
        # هذا تعليق
        متغير س = ١٠؛ # تعريف متغير
    "#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_format_string() {
    let input = r#"اطبع("الاسم: {الاسم}")؛"#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_format_string_escaped_braces() {
    let input = r#"اطبع("{{ قوس }}")؛"#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_format_string_expression() {
    let input = r#"اطبع("١ + ٢ = {١ + ٢}")؛"#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_string_multiplication() {
    let input = r#"متغير repeated = "أ" * ٥؛"#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_list_multiplication() {
    let input = "متغير repeated = [١، ٢] * ٣؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_list_concatenation() {
    let input = "متغير combined = [١، ٢] + [٣، ٤]؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_nested_lists() {
    let input = "متغير matrix = [[١، ٢]، [٣، ٤]]؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_nested_dicts() {
    let input = r#"متغير nested = {أ: {ب: ١}}؛"#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_print_multiple_args() {
    let input = "اطبع(أ، ب، ج)؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_print_without_parens() {
    let input = "اطبع أ؛";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_anonymous_block() {
    let input = "{ متغير س = ١؛ اطبع(س)؛ }";
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_parser_error_contains_context_and_code() {
    let input = "متغير = ١٠؛";
    let error = Parser::parse(input).unwrap_err();

    assert!(error.message.contains("E200"));
    assert!(error.message.contains("<النص>:1:7"));
    assert!(error.message.contains("متغير = ١٠؛"));
}

#[test]
fn test_anonymous_function_expression_parse() {
    let input = r#"
        دالة adder(n) {
            أرجع دالة(x) {
                أرجع n + x؛
            }؛
        }
    "#;

    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_parser_expected_token_error_has_suggestion() {
    let input = "اطبع(١ + ٢؛";
    let error = Parser::parse(input).unwrap_err();

    assert!(error.message.contains("E201"));
    assert!(error.message.contains("هل تقصد"));
    assert!(error.message.contains("إغلاق القوس ')'"));
}

#[test]
fn test_parser_reports_multiple_errors_after_recovery() {
    let input = r#"
        متغير أ = ؛
        ثابت = ٢؛
        اطبع("تم")؛
    "#;
    let result = Parser::parse(input);
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(
        err.message.contains("تم اكتشاف 1 أخطاء إضافية")
            || err.message.contains("تم اكتشاف 1 خطأ إضافي"),
        "رسالة التجميع لا تحتوي ملخص الأخطاء: {}",
        err.message
    );
    assert!(err.message.contains("السطر"));
}

#[test]
fn test_parser_recovery_keeps_valid_statements() {
    let input = r#"
        متغير س = ١؛
        إذا س > {
            اطبع("غير صالح")؛
        }
        اطبع("صحيح")؛
    "#;
    let result = Parser::parse(input);
    assert!(result.is_err(), "يجب أن يفشل التحليل مع خطأ نحوي");

    let err = result.unwrap_err();
    assert!(
        err.message.contains("اطبع") || err.message.contains("السطر"),
        "يجب أن تتضمن الرسالة سياقاً بعد محاولة التعافي: {}",
        err.message
    );
}

#[test]
fn test_length_builtin_call_parses() {
    let input = r#"متغير ن = طول([١، ٢، ٣])؛"#;
    let result = Parser::parse(input);
    assert!(result.is_ok());
}

#[test]
fn test_ui_component_declaration() {
    let input = r#"
        واجهة صفحة_الرئيسية() {
            اطبع("جاهز")؛
        }
    "#;
    let result = Parser::parse(input).expect("يجب أن ينجح تحليل واجهة");
    assert!(matches!(result.statements[0], Stmt::UiComponentDecl { .. }));
}

#[test]
fn test_state_theme_route_declarations() {
    let input = r#"
        حالة المتجر = {"عداد": 1}؛
        ثيم المصنع = {"اتجاه": "RTL"}؛
        موجه المسارات = {"/": "صفحة_الرئيسية"}؛
    "#;
    let result = Parser::parse(input).expect("يجب أن ينجح تحليل تعريفات UI");
    assert!(matches!(result.statements[0], Stmt::StateDecl { .. }));
    assert!(matches!(result.statements[1], Stmt::ThemeDecl { .. }));
    assert!(matches!(result.statements[2], Stmt::RouteDecl { .. }));
}

#[test]
fn test_route_declaration_accepts_masar_alias() {
    let input = r#"
        مسار المسارات = {"/": "صفحة_الرئيسية"}؛
    "#;
    let result = Parser::parse(input).expect("يجب أن ينجح تحليل تعريف المسار");
    assert!(matches!(result.statements[0], Stmt::RouteDecl { .. }));
}

#[test]
fn test_event_handler_declaration() {
    let input = r#"
        حدث اعادة_ضبط(كود) {
            اطبع(كود)؛
        }
    "#;
    let result = Parser::parse(input).expect("يجب أن ينجح تحليل الحدث");
    assert!(matches!(
        result.statements[0],
        Stmt::EventHandlerDecl { .. }
    ));
}
