// tests/interpreter_tests.rs
// Comprehensive tests for the Al-Marjaa Interpreter

use almarjaa::interpreter::value::Value;
use almarjaa::interpreter::Interpreter;

fn run(code: &str) -> Value {
    let mut interpreter = Interpreter::new();
    interpreter.run(code).unwrap().borrow().clone()
}

fn run_result(code: &str) -> Result<Value, String> {
    let mut interpreter = Interpreter::new();
    match interpreter.run(code) {
        Ok(result) => Ok(result.borrow().clone()),
        Err(e) => Err(e.message),
    }
}

#[test]
fn test_arithmetic_add() {
    let result = run("١ + ٢");
    assert!(matches!(result, Value::Number(n) if (n - 3.0).abs() < 0.001));
}

#[test]
fn test_arithmetic_subtract() {
    let result = run("٥ - ٣");
    assert!(matches!(result, Value::Number(n) if (n - 2.0).abs() < 0.001));
}

#[test]
fn test_arithmetic_multiply() {
    let result = run("٤ * ٣");
    assert!(matches!(result, Value::Number(n) if (n - 12.0).abs() < 0.001));
}

#[test]
fn test_arithmetic_divide() {
    let result = run("١٠ / ٢");
    assert!(matches!(result, Value::Number(n) if (n - 5.0).abs() < 0.001));
}

#[test]
fn test_arithmetic_modulo() {
    let result = run("١٠ % ٣");
    assert!(matches!(result, Value::Number(n) if (n - 1.0).abs() < 0.001));
}

#[test]
fn test_arithmetic_power() {
    let result = run("٢ ^ ٣");
    assert!(matches!(result, Value::Number(n) if (n - 8.0).abs() < 0.001));
}

#[test]
fn test_arithmetic_precedence() {
    let result = run("٢ + ٣ * ٤");
    assert!(matches!(result, Value::Number(n) if (n - 14.0).abs() < 0.001));
}

#[test]
fn test_arithmetic_parentheses() {
    let result = run("(٢ + ٣) * ٤");
    assert!(matches!(result, Value::Number(n) if (n - 20.0).abs() < 0.001));
}

#[test]
fn test_unary_minus() {
    let result = run("-٥");
    assert!(matches!(result, Value::Number(n) if (n - (-5.0)).abs() < 0.001));
}

#[test]
fn test_comparison_equal() {
    let result = run("٥ == ٥");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_comparison_not_equal() {
    let result = run("٥ != ٣");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_comparison_less() {
    let result = run("٣ < ٥");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_comparison_greater() {
    let result = run("٥ > ٣");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_comparison_less_equal() {
    let result = run("٥ <= ٥");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_comparison_greater_equal() {
    let result = run("٥ >= ٥");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_logical_and() {
    let result = run("صح و صح");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_logical_and_false() {
    let result = run("صح و خطأ");
    assert!(matches!(result, Value::Boolean(false)));
}

#[test]
fn test_logical_or() {
    let result = run("خطأ أو صح");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_logical_or_both_false() {
    let result = run("خطأ أو خطأ");
    assert!(matches!(result, Value::Boolean(false)));
}

#[test]
fn test_logical_not() {
    let result = run("ليس خطأ");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_short_circuit_and() {
    let result = run("خطأ و (١ / ٠ > ٠)");
    assert!(matches!(result, Value::Boolean(false)));
}

#[test]
fn test_short_circuit_or() {
    let result = run("صح أو (١ / ٠ > ٠)");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_variable_declaration() {
    let mut interp = Interpreter::new();
    interp.run("متغير س = ١٠؛").unwrap();
    let result = interp.run("س").unwrap();
    assert!(matches!(*result.borrow(), Value::Number(n) if (n - 10.0).abs() < 0.001));
}

#[test]
fn test_variable_reassignment() {
    let result = run("متغير س = ١؛ س = ٥؛ س");
    assert!(matches!(result, Value::Number(n) if (n - 5.0).abs() < 0.001));
}

#[test]
fn test_constant_declaration() {
    let result = run("ثابت pi = ٣.١٤؛ pi");
    assert!(matches!(result, Value::Number(n) if (n - (157.0 / 50.0)).abs() < 0.001));
}

#[test]
fn test_string_concatenation() {
    let result = run("\"مرحبا\" + \" \" + \"عالم\"");
    assert!(matches!(result, Value::String(s) if s == "مرحبا عالم"));
}

#[test]
fn test_string_with_number() {
    let result = run("\"العدد: \" + ٤٢");
    assert!(matches!(result, Value::String(s) if s == "العدد: 42"));
}

#[test]
fn test_string_multiplication() {
    let result = run("\"أ\" * ٣");
    assert!(matches!(result, Value::String(s) if s == "أأأ"));
}

#[test]
fn test_list_creation() {
    let result = run("[١، ٢، ٣]");
    assert!(matches!(result, Value::List(_)));
}

#[test]
fn test_list_length() {
    let result = run("طول([١، ٢، ٣])");
    assert!(matches!(result, Value::Number(n) if n == 3.0));
}

#[test]
fn test_list_index() {
    let result = run("[١، ٢، ٣][٠]");
    assert!(matches!(result, Value::Number(n) if n == 1.0));
}

#[test]
fn test_list_negative_index() {
    let result = run("[١، ٢، ٣][-١]");
    assert!(matches!(result, Value::Number(n) if n == 3.0));
}

#[test]
fn test_list_concatenation() {
    let result = run("[١، ٢] + [٣، ٤]");
    if let Value::List(l) = result {
        assert_eq!(l.len(), 4);
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_list_list_multiplication() {
    let result = run("[١] * ٣");
    if let Value::List(l) = result {
        assert_eq!(l.len(), 3);
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_dictionary_creation() {
    let result = run("متغير d = {الاسم: \"أحمد\"، العمر: ٢٠}؛ d");
    assert!(matches!(result, Value::Dictionary(_)));
}

#[test]
fn test_dictionary_access() {
    let result = run("متغير d = {الاسم: \"أحمد\"}؛ d.الاسم");
    assert!(matches!(result, Value::String(s) if s == "أحمد"));
}

#[test]
fn test_dictionary_index() {
    let result = run("متغير d = {الاسم: \"أحمد\"}؛ d[\"الاسم\"]");
    assert!(matches!(result, Value::String(s) if s == "أحمد"));
}

#[test]
fn test_function_call() {
    let code = r#"
        دالة جمع(أ، ب) {
            أرجع أ + ب؛
        }
        جمع(٣، ٤)
    "#;
    let result = run(code);
    assert!(matches!(result, Value::Number(n) if (n - 7.0).abs() < 0.001));
}

#[test]
fn test_function_no_return() {
    let code = r#"
        دالة greet() {
            اطبع("مرحبا")؛
        }
        greet()
    "#;
    let result = run(code);
    assert!(matches!(result, Value::Null));
}

#[test]
fn test_function_default_params() {
    let code = r#"
        دالة greet(الاسم = "عالم") {
            أرجع "مرحبا " + الاسم؛
        }
        greet()
    "#;
    let result = run(code);
    assert!(matches!(result, Value::String(s) if s == "مرحبا عالم"));
}

#[test]
fn test_function_with_default_override() {
    let code = r#"
        دالة greet(الاسم = "عالم") {
            أرجع "مرحبا " + الاسم؛
        }
        greet("أحمد")
    "#;
    let result = run(code);
    assert!(matches!(result, Value::String(s) if s == "مرحبا أحمد"));
}

#[test]
fn test_closure() {
    let code = r#"
        دالة adder(n) {
            أرجع دالة(x) {
                أرجع n + x؛
            }؛
        }
        متغير add5 = adder(٥)؛
        add5(٣)
    "#;
    let result = run(code);
    assert!(matches!(result, Value::Number(n) if (n - 8.0).abs() < 0.001));
}

#[test]
fn test_recursive_function() {
    let code = r#"
        دالة factorial(n) {
            إذا n <= ١ {
                أرجع ١؛
            }
            أرجع n * factorial(n - ١)؛
        }
        factorial(٥)
    "#;
    let result = run(code);
    assert!(matches!(result, Value::Number(n) if (n - 120.0).abs() < 0.001));
}

#[test]
fn test_while_loop() {
    let code = r#"
        متغير i = ٠؛
        متغير sum = ٠؛
        طالما i < ١٠ {
            sum = sum + i؛
            i = i + ١؛
        }
        sum
    "#;
    let result = run(code);
    assert!(matches!(result, Value::Number(n) if (n - 45.0).abs() < 0.001));
}

#[test]
fn test_for_loop() {
    let code = r#"
        متغير sum = ٠؛
        لكل x في [١، ٢، ٣، ٤، ٥] {
            sum = sum + x؛
        }
        sum
    "#;
    let result = run(code);
    assert!(matches!(result, Value::Number(n) if (n - 15.0).abs() < 0.001));
}

#[test]
fn test_for_range() {
    let code = r#"
        متغير sum = ٠؛
        لكل i في مدى(١، ٦) {
            sum = sum + i؛
        }
        sum
    "#;
    let result = run(code);
    assert!(matches!(result, Value::Number(n) if (n - 15.0).abs() < 0.001));
}

#[test]
fn test_for_range_with_step() {
    let code = r#"
        متغير sum = ٠؛
        لكل i في مدى(٠، ١٠، ٢) {
            sum = sum + i؛
        }
        sum
    "#;
    let result = run(code);
    assert!(matches!(result, Value::Number(n) if (n - 20.0).abs() < 0.001));
}

#[test]
fn test_repeat_loop() {
    let code = r#"
        متغير sum = ٠؛
        كرر ٥ مرات {
            sum = sum + ١؛
        }
        sum
    "#;
    let result = run(code);
    assert!(matches!(result, Value::Number(n) if (n - 5.0).abs() < 0.001));
}

#[test]
fn test_break() {
    let code = r#"
        متغير sum = ٠؛
        طالما صح {
            sum = sum + ١؛
            إذا sum == ٥ {
                توقف؛
            }
        }
        sum
    "#;
    let result = run(code);
    assert!(matches!(result, Value::Number(n) if (n - 5.0).abs() < 0.001));
}

#[test]
fn test_continue() {
    let code = r#"
        متغير sum = ٠؛
        لكل i في [١، ٢، ٣، ٤، ٥] {
            إذا i == ٣ {
                أكمل؛
            }
            sum = sum + i؛
        }
        sum
    "#;
    let result = run(code);
    assert!(matches!(result, Value::Number(n) if (n - 12.0).abs() < 0.001));
}

#[test]
fn test_ternary() {
    let result = run("٥ > ٣ ? \"كبير\" : \"صغير\"");
    assert!(matches!(result, Value::String(s) if s == "كبير"));
}

#[test]
fn test_try_catch() {
    let code = r#"
        متغير result = "default"؛
        حاول {
            ألقِ "error"؛
        } امسك(e) {
            result = "caught"؛
        }
        result
    "#;
    let result = run(code);
    assert!(matches!(result, Value::String(s) if s == "caught"));
}

#[test]
fn test_try_catch_finally() {
    let code = r#"
        متغير result = ""؛
        حاول {
            result = result + "try"؛
        } امسك(e) {
            result = result + "catch"؛
        } أخيراً {
            result = result + "finally"؛
        }
        result
    "#;
    let result = run(code);
    assert!(matches!(result, Value::String(s) if s == "tryfinally"));
}

#[test]
fn test_match() {
    let code = r#"
        متغير result = ""؛
        طابق ٢ {
            حالة ١: result = "one"؛
            حالة ٢: result = "two"؛
            افتراضي: result = "other"؛
        }
        result
    "#;
    let result = run(code);
    assert!(matches!(result, Value::String(s) if s == "two"));
}

#[test]
fn test_match_default() {
    let code = r#"
        متغير result = ""؛
        طابق ١٠ {
            حالة ١: result = "one"؛
            حالة ٢: result = "two"؛
            افتراضي: result = "other"؛
        }
        result
    "#;
    let result = run(code);
    assert!(matches!(result, Value::String(s) if s == "other"));
}

#[test]
fn test_null_falsy() {
    let result = run(
        "متغير نتيجة = \"\"؛ إذا لا_شيء { نتيجة = \"truthy\"؛ } وإلا { نتيجة = \"falsy\"؛ } نتيجة",
    );
    assert!(matches!(result, Value::String(s) if s == "falsy"));
}

#[test]
fn test_null_equality() {
    let result = run("لا_شيء == لا_شيء");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_null_not_equal_to_number() {
    let result = run("لا_شيء == ٠");
    assert!(matches!(result, Value::Boolean(false)));
}

#[test]
fn test_string_equality() {
    let result = run("\"hello\" == \"hello\"");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_string_inequality() {
    let result = run("\"hello\" != \"world\"");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_string_less_than() {
    let result = run("\"a\" < \"b\"");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_method_on_string() {
    let result = run("\"hello\".كبير");
    assert!(matches!(result, Value::String(s) if s == "HELLO"));
}

#[test]
fn test_method_on_string_small() {
    let result = run("\"HELLO\".صغير");
    assert!(matches!(result, Value::String(s) if s == "hello"));
}

#[test]
fn test_method_on_list_first() {
    let result = run("[١، ٢، ٣].أول");
    assert!(matches!(result, Value::Number(n) if n == 1.0));
}

#[test]
fn test_method_on_list_last() {
    let result = run("[١، ٢، ٣].آخر");
    assert!(matches!(result, Value::Number(n) if n == 3.0));
}

#[test]
fn test_method_on_list_empty() {
    let result = run("[].فارغة");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_divide_by_zero() {
    let result = run_result("١ / ٠");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("القسمة على صفر"));
}

#[test]
fn test_index_out_of_bounds() {
    let result = run_result("[١، ٢][١٠]");
    assert!(result.is_err());
}

#[test]
fn test_undefined_variable() {
    let result = run_result("س");
    assert!(result.is_err());
}

#[test]
fn test_assignment_to_undefined_creates() {
    let result = run("undefined_var = ١٠؛ undefined_var");
    assert!(matches!(result, Value::Number(n) if n == 10.0));
}

#[test]
fn test_compound_assignment() {
    let result = run("متغير س = ٥؛ س += ٣؛ س");
    assert!(matches!(result, Value::Number(n) if n == 8.0));
}

#[test]
fn test_increment() {
    let result = run("متغير س = ٥؛ ++س؛ س");
    assert!(matches!(result, Value::Number(n) if n == 6.0));
}

#[test]
fn test_decrement() {
    let result = run("متغير س = ٥؛ --س؛ س");
    assert!(matches!(result, Value::Number(n) if n == 4.0));
}

#[test]
fn test_increment_prefix() {
    let result = run("متغير س = ٥؛ ++س");
    assert!(matches!(result, Value::Number(n) if n == 6.0));
}

#[test]
fn test_increment_postfix() {
    let result = run("متغير س = ٥؛ س++");
    assert!(matches!(result, Value::Number(n) if n == 5.0));
}

#[test]
fn test_format_string() {
    let code = r#"
        متغير name = "أحمد"؛
        "الاسم: {name}"
    "#;
    let result = run(code);
    assert!(matches!(result, Value::String(s) if s == "الاسم: أحمد"));
}

#[test]
fn test_format_string_expression() {
    let result = run("\"٢ + ٣ = {٢ + ٣}\"");
    assert!(matches!(result, Value::String(s) if s == "٢ + ٣ = 5"));
}

#[test]
fn test_fibonacci() {
    let code = r#"
        دالة fib(n) {
            إذا n <= ١ {
                أرجع n؛
            }
            أرجع fib(n - ١) + fib(n - ٢)؛
        }
        fib(١٠)
    "#;
    let result = run(code);
    assert!(matches!(result, Value::Number(n) if (n - 55.0).abs() < 0.001));
}

#[test]
fn test_hardware_requires_arguments() {
    let result = run_result("gpio_اكتب()").expect_err("expected arity error for gpio_اكتب");
    assert!(result.contains("gpio_اكتب يتطلب 2 معاملات على الأقل"));
}

#[test]
fn test_cnc_status_returns_dictionary() {
    let result = run("cnc_حالة()");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("الحالة"));
            assert!(dict.contains_key("المحاور"));
            assert!(dict.contains_key("الإنذار"));
        }
        _ => panic!("Expected dictionary from cnc_حالة"),
    }
}

#[test]
fn test_esp_wifi_returns_connection_info() {
    let result = run("esp_اتصل_واي_فاي(\"FactoryNet\")");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("ssid"));
            assert!(dict.contains_key("متصل"));
            assert!(dict.contains_key("ip"));
        }
        _ => panic!("Expected dictionary from esp_اتصل_واي_فاي"),
    }
}

#[test]
fn test_hmi_display_returns_true() {
    let result = run("hmi_اعرض(\"الرئيسية\"، \"جاهز\")");
    assert!(matches!(result, Value::Boolean(true)));
}

#[test]
fn test_secure_http_blocks_private_hosts_by_default() {
    let result = run_result("http_أحضر_آمن(\"http://localhost:8080\")")
        .expect_err("expected private-network block");
    assert!(result.contains("تم حظر الوصول إلى المضيف الداخلي"));
}

#[test]
fn test_secure_http_rejects_invalid_options_type() {
    let result = run_result("http_أحضر_آمن(\"https://example.com\"، \"خطأ\")")
        .expect_err("expected options type validation");
    assert!(result.contains("خيارات الشبكة يجب أن تكون قاموساً"));
}

#[test]
fn test_secure_http_rejects_invalid_timeout_range() {
    let result = run_result("http_أحضر_آمن(\"https://example.com\"، {مهلة: ٠})")
        .expect_err("expected timeout range validation");
    assert!(result.contains("قيمة 'مهلة' يجب أن تكون بين 1 و300 ثانية"));
}

#[test]
fn test_list_negative_index_out_of_bounds() {
    let result = run_result("[١، ٢، ٣][-٤]").expect_err("expected out of bounds error");
    assert!(result.contains("خارج النطاق"));
}

#[test]
fn test_dictionary_missing_key_returns_null() {
    let result = run(r#"متغير d = {الاسم: "أحمد"}؛ d["العمر"]"#);
    assert!(matches!(result, Value::Null));
}

#[test]
fn test_const_reassignment_reports_error() {
    let result =
        run_result("ثابت الحد = ١٠؛ الحد = ٢٠؛").expect_err("expected constant reassignment");
    assert!(result.contains("ثابت") || result.contains("إعادة"));
}

#[test]
fn test_invalid_for_in_target_reports_error() {
    let result = run_result("لكل عنصر في ١٢٣ { اطبع(عنصر)؛ }");
    assert!(result.is_err());
}

// ═══════════════════════════════════════════════════════════════
// اختبارات دوال الذكاء الاصطناعي
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_sigmoid_function() {
    let result = run("سيجمويد(٠)");
    assert!(matches!(result, Value::Number(n) if (n - 0.5).abs() < 0.001));
}

#[test]
fn test_sigmoid_high_value() {
    let result = run("سيجمويد(١٠)");
    assert!(matches!(result, Value::Number(n) if n > 0.99));
}

#[test]
fn test_sigmoid_low_value() {
    let result = run("سيجمويد(-١٠)");
    assert!(matches!(result, Value::Number(n) if n < 0.01));
}

#[test]
fn test_relu_positive() {
    let result = run("ريلو(٥)");
    assert!(matches!(result, Value::Number(n) if n == 5.0));
}

#[test]
fn test_relu_negative() {
    let result = run("ريلو(-٥)");
    assert!(matches!(result, Value::Number(n) if n == 0.0));
}

#[test]
fn test_tanh_function() {
    let result = run("تانه(٠)");
    assert!(matches!(result, Value::Number(n) if n.abs() < 0.001));
}

#[test]
fn test_leaky_relu_positive() {
    let result = run("ليكي_ريلو(٥)");
    assert!(matches!(result, Value::Number(n) if n == 5.0));
}

#[test]
fn test_leaky_relu_negative() {
    let result = run("ليكي_ريلو(-٥)");
    assert!(matches!(result, Value::Number(n) if n < 0.0 && n > -1.0));
}

#[test]
fn test_softmax_returns_list() {
    let result = run("سوفتماكس([١، ٢، ٣])");
    assert!(matches!(result, Value::List(_)));
}

#[test]
fn test_softmax_sums_to_one() {
    let result = run("سوفتماكس([١، ٢، ٣])");
    if let Value::List(l) = result {
        let sum: f64 = l.iter()
            .map(|v| match &*v.borrow() {
                Value::Number(n) => *n,
                _ => 0.0
            })
            .sum();
        assert!((sum - 1.0).abs() < 0.001);
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_mse_loss() {
    let result = run("خطأ_مربع(٣، ٥)");
    assert!(matches!(result, Value::Number(n) if (n - 4.0).abs() < 0.001));
}

#[test]
fn test_cross_entropy_loss() {
    let result = run("خطأ_تقاطع(٠.٩، ١)");
    assert!(matches!(result, Value::Number(n) if n > 0.0));
}

// ═══════════════════════════════════════════════════════════════
// اختبارات المحسنات (Optimizers)
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_sgd_optimizer() {
    let result = run("sgd(١.٠، ٠.١، ٠.١)");
    assert!(matches!(result, Value::Number(n) if (n - 0.99).abs() < 0.001));
}

#[test]
fn test_sgd_momentum_returns_dict() {
    let result = run("sgd_زخم(١.٠، ٠.١، ٠.١، ٠.٩، ٠.٠)");
    assert!(matches!(result, Value::Dictionary(_)));
}

#[test]
fn test_adam_optimizer_returns_dict() {
    let result = run("آدم(١.٠، ٠.١، ٠.٠٠١، ٠.٩، ٠.٩٩٩، ٠.٠، ٠.٠، ١)");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("وزن"));
            assert!(dict.contains_key("م"));
            assert!(dict.contains_key("ف"));
        }
        _ => panic!("Expected Dictionary from Adam optimizer"),
    }
}

#[test]
fn test_rmsprop_optimizer() {
    let result = run("rmsprop(١.٠، ٠.١، ٠.٠٠١، ٠.٩، ٠.٠)");
    assert!(matches!(result, Value::Dictionary(_)));
}

#[test]
fn test_adagrad_optimizer() {
    let result = run("adagrad(١.٠، ٠.١، ٠.٠١، ٠.٠)");
    assert!(matches!(result, Value::Dictionary(_)));
}

// ═══════════════════════════════════════════════════════════════
// اختبارات المتجهات (Tensors)
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_tensor_creation() {
    let result = run("متجه([١، ٢، ٣])");
    assert!(matches!(result, Value::Tensor { .. }));
}

#[test]
fn test_zeros_tensor() {
    let result = run("أصفار(٥)");
    if let Value::Tensor { data, shape } = result {
        assert_eq!(data.len(), 5);
        assert_eq!(shape, vec![5]);
        assert!(data.iter().all(|&x| x == 0.0));
    } else {
        panic!("Expected Tensor");
    }
}

#[test]
fn test_ones_tensor() {
    let result = run("آحاد(٥)");
    if let Value::Tensor { data, shape } = result {
        assert_eq!(data.len(), 5);
        assert_eq!(shape, vec![5]);
        assert!(data.iter().all(|&x| x == 1.0));
    } else {
        panic!("Expected Tensor");
    }
}

#[test]
fn test_tensor_size() {
    let result = run("حجم_متجه(متجه([١، ٢، ٣، ٤، ٥]))");
    assert!(matches!(result, Value::Number(n) if n == 5.0));
}

#[test]
fn test_tensor_to_list() {
    let result = run("إلى_قائمة(متجه([١، ٢، ٣]))");
    assert!(matches!(result, Value::List(_)));
}

// ═══════════════════════════════════════════════════════════════
// اختبارات الشبكات العصبية
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_neural_network_creation() {
    let result = run("شبكة(\"اختبار\"، [])");
    assert!(matches!(result, Value::NeuralNetwork { .. }));
}

#[test]
fn test_mlp_network() {
    let result = run("شبكة_متعددة(٧٨٤، [١٢٨، ٦٤]، ١٠)");
    if let Value::NeuralNetwork { name, layers } = result {
        assert_eq!(name, "MLP");
        assert!(layers.len() > 0);
    } else {
        panic!("Expected NeuralNetwork");
    }
}

#[test]
fn test_cnn_network() {
    let result = run("شبكة_التفاف(٣، ٣٢، ١٠)");
    if let Value::NeuralNetwork { name, layers } = result {
        assert_eq!(name, "CNN");
        assert!(layers.len() > 0);
    } else {
        panic!("Expected NeuralNetwork");
    }
}

#[test]
fn test_rnn_network() {
    let result = run("شبكة_متكررة(١٠٠، ٥٠، ١٠)");
    if let Value::NeuralNetwork { name, layers } = result {
        assert_eq!(name, "RNN");
        assert!(layers.len() > 0);
    } else {
        panic!("Expected NeuralNetwork");
    }
}

#[test]
fn test_lstm_network() {
    let result = run("شبكة_lstm(١٠٠، ٥٠، ١٠)");
    if let Value::NeuralNetwork { name, layers } = result {
        assert_eq!(name, "LSTM");
        assert!(layers.len() > 0);
    } else {
        panic!("Expected NeuralNetwork");
    }
}

#[test]
fn test_gru_network() {
    let result = run("شبكة_gru(١٠٠، ٥٠، ١٠)");
    if let Value::NeuralNetwork { name, layers } = result {
        assert_eq!(name, "GRU");
        assert!(layers.len() > 0);
    } else {
        panic!("Expected NeuralNetwork");
    }
}

#[test]
fn test_transformer_network() {
    let result = run("محول(١٠٠٠، ١٠، ٤، ١٢٨)");
    if let Value::NeuralNetwork { name, layers } = result {
        assert_eq!(name, "Transformer");
        assert!(layers.len() > 0);
    } else {
        panic!("Expected NeuralNetwork");
    }
}

// ═══════════════════════════════════════════════════════════════
// اختبارات DataLoader
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_dataloader_creation() {
    let result = run("محمل_بيانات([١، ٢، ٣، ٤، ٥]، ٢)");
    assert!(matches!(result, Value::Dictionary(_)));
}

#[test]
fn test_batches_function() {
    let result = run("دفعات([١، ٢، ٣، ٤، ٥، ٦]، ٢)");
    if let Value::List(batches) = result {
        assert_eq!(batches.len(), 3);
    } else {
        panic!("Expected List of batches");
    }
}

#[test]
fn test_one_hot_encoding() {
    let result = run("واحد_ساخن(١، ٣)");
    if let Value::List(l) = result {
        assert_eq!(l.len(), 3);
        // يجب أن يكون [0, 1, 0]
        let values: Vec<f64> = l.iter()
            .map(|v| match &*v.borrow() {
                Value::Number(n) => *n,
                _ => -1.0
            })
            .collect();
        assert_eq!(values, vec![0.0, 1.0, 0.0]);
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_argmax_function() {
    let result = run("فهرس_أقصى([١، ٥، ٣، ٢])");
    assert!(matches!(result, Value::Number(n) if n == 1.0));
}

#[test]
fn test_argmin_function() {
    let result = run("فهرس_أدنى([٥، ١، ٣، ٢])");
    assert!(matches!(result, Value::Number(n) if n == 1.0));
}

// ═══════════════════════════════════════════════════════════════
// اختبارات Regularization
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_dropout_returns_list() {
    let result = run("تسرب([١، ٢، ٣، ٤، ٥]، ٠.٥)");
    assert!(matches!(result, Value::List(_)));
}

#[test]
fn test_batch_norm_returns_dict() {
    let result = run("تسوية_دفعة([١، ٢، ٣، ٤، ٥])");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("قيم"));
            assert!(dict.contains_key("متوسط"));
            assert!(dict.contains_key("تباين"));
        }
        _ => panic!("Expected Dictionary from batch norm"),
    }
}

#[test]
fn test_layer_norm_returns_list() {
    let result = run("تسوية_طبقة([١، ٢، ٣، ٤، ٥])");
    assert!(matches!(result, Value::List(_)));
}

// ═══════════════════════════════════════════════════════════════
// اختبارات تهيئة الأوزان
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_xavier_init() {
    let result = run("xavier(١٠٠، ٥٠)");
    if let Value::List(weights) = result {
        assert_eq!(weights.len(), 5000); // 100 * 50
    } else {
        panic!("Expected List of weights");
    }
}

#[test]
fn test_he_init() {
    let result = run("he(١٠٠، ٥٠)");
    if let Value::List(weights) = result {
        assert_eq!(weights.len(), 5000); // 100 * 50
    } else {
        panic!("Expected List of weights");
    }
}

// ═══════════════════════════════════════════════════════════════
// اختبارات دوال التدريب
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_accuracy_function() {
    let result = run("دقة([١، ٢، ٣]، [١، ٢، ٤])");
    assert!(matches!(result, Value::Number(n) if (n - 0.666).abs() < 0.01));
}

#[test]
fn test_accuracy_perfect() {
    let result = run("دقة([١، ٢، ٣]، [١، ٢، ٣])");
    assert!(matches!(result, Value::Number(n) if n == 1.0));
}

#[test]
fn test_accuracy_zero() {
    let result = run("دقة([١، ٢، ٣]، [٤، ٥، ٦])");
    assert!(matches!(result, Value::Number(n) if n == 0.0));
}

#[test]
fn test_early_stopping() {
    let result = run("توقف_مبكر(٥، ٣، ٠.٥، ٠.٦)");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("توقف"));
            assert!(dict.contains_key("انتظار"));
        }
        _ => panic!("Expected Dictionary from early stopping"),
    }
}

#[test]
fn test_lr_scheduler() {
    let result = run("قلّص_معدل(٠.٠٠١، ٠.٥، ٥، ٠.٥، ٠.٦، ٣)");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("معدل_تعلم"));
        }
        _ => panic!("Expected Dictionary from lr scheduler"),
    }
}

// ═══════════════════════════════════════════════════════════════
// اختبارات حفظ وتحميل النماذج
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_save_model() {
    let result = run("متغير نموذج = شبكة(\"test\"، [])؛ احفظ_نموذج(نموذج، \"model.json\")");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("نجاح"));
            assert!(dict.contains_key("مسار"));
        }
        _ => panic!("Expected Dictionary from save_model"),
    }
}

#[test]
fn test_load_model() {
    let result = run("حمّل_نموذج(\"model.json\")");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("نجاح"));
            assert!(dict.contains_key("مسار"));
        }
        _ => panic!("Expected Dictionary from load_model"),
    }
}

#[test]
fn test_export_weights() {
    let result = run("صدّر_أوزان([٠.١، ٠.٢، ٠.٣])");
    assert!(matches!(result, Value::String(_)));
}

#[test]
fn test_import_weights() {
    let result = run("استورد_أوزان(\"٠.١، ٠.٢، ٠.٣\")");
    assert!(matches!(result, Value::List(_)));
}

// ═══════════════════════════════════════════════════════════════
// اختبارات المصفوفات
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_transpose() {
    let result = run("تبديل([[١، ٢]، [٣، ٤]])");
    if let Value::List(rows) = result {
        assert_eq!(rows.len(), 2);
    } else {
        panic!("Expected List (matrix)");
    }
}

#[test]
fn test_normalize() {
    let result = run("تطبيع([١، ٢، ٣، ٤، ٥])");
    if let Value::List(values) = result {
        // القيم يجب أن تكون بين 0 و 1
        for v in values {
            if let Value::Number(n) = &*v.borrow() {
                assert!(*n >= 0.0 && *n <= 1.0);
            }
        }
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_standardize() {
    let result = run("تسوية_قياسية([١، ٢، ٣، ٤، ٥])");
    assert!(matches!(result, Value::List(_)));
}

// ═══════════════════════════════════════════════════════════════
// اختبارات نظام الوحدات
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_module_creation() {
    let result = run("أنشئ_وحدة(\"math_utils\")");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("اسم"));
            assert!(dict.contains_key("صادرات"));
        }
        _ => panic!("Expected Dictionary (module)"),
    }
}

#[test]
fn test_export_function() {
    let result = run("صدّر(\"دالة_جمع\"، دالة(أ، ب) { أرجع أ + ب؛ })");
    assert!(matches!(result, Value::Dictionary(_)));
}

#[test]
fn test_module_path() {
    let result = run("مسار_وحدة(\"utils.math\")");
    assert!(matches!(result, Value::String(s) if s == "utils/math.mrj"));
}

// ═══════════════════════════════════════════════════════════════
// اختبارات نظام الاشتقاق التلقائي (AutoGrad)
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_autograd_tensor_creation() {
    let result = run("تدرج_متجه([١، ٢، ٣])");
    assert!(matches!(result, Value::AutoTensor { .. }));
}

#[test]
fn test_autograd_tensor_with_grad() {
    let result = run("تدرج_متجه([١، ٢، ٣]، صح)");
    match result {
        Value::AutoTensor { requires_grad, .. } => assert!(requires_grad),
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_autograd_zeros() {
    let result = run("تدرج_أصفار(٥)");
    match result {
        Value::AutoTensor { data, .. } => {
            assert_eq!(data.len(), 5);
            assert!(data.iter().all(|&x| x == 0.0));
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_autograd_ones() {
    let result = run("تدرج_آحاد(٥)");
    match result {
        Value::AutoTensor { data, .. } => {
            assert_eq!(data.len(), 5);
            assert!(data.iter().all(|&x| x == 1.0));
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_autograd_add() {
    let result = run("متغير أ = تدرج_آحاد(٣)؛ متغير ب = تدرج_آحاد(٣)؛ تدرج_جمع(أ، ب)");
    match result {
        Value::AutoTensor { data, .. } => {
            assert_eq!(data, vec![2.0, 2.0, 2.0]);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_autograd_sub() {
    let result = run("متغير أ = تدرج_متجه([٥، ٤، ٣])؛ متغير ب = تدرج_متجه([١، ٢، ٣])؛ تدرج_طرح(أ، ب)");
    match result {
        Value::AutoTensor { data, .. } => {
            assert_eq!(data, vec![4.0, 2.0, 0.0]);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_autograd_mul() {
    let result = run("متغير أ = تدرج_متجه([٢، ٣، ٤])؛ متغير ب = تدرج_متجه([٣، ٢، ١])؛ تدرج_ضرب(أ، ب)");
    match result {
        Value::AutoTensor { data, .. } => {
            assert_eq!(data, vec![6.0, 6.0, 4.0]);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_autograd_sigmoid() {
    let result = run("متغير أ = تدرج_متجه([٠])؛ تدرج_سيجمويد(أ)");
    match result {
        Value::AutoTensor { data, .. } => {
            assert!((data[0] - 0.5).abs() < 0.001);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_autograd_relu() {
    let result = run("متغير أ = تدرج_متجه([٥، -٣، ٠])؛ تدرج_ريلو(أ)");
    match result {
        Value::AutoTensor { data, .. } => {
            assert_eq!(data, vec![5.0, 0.0, 0.0]);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_autograd_tanh() {
    let result = run("متغير أ = تدرج_متجه([٠])؛ تدرج_تانه(أ)");
    match result {
        Value::AutoTensor { data, .. } => {
            assert!(data[0].abs() < 0.001);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_autograd_data_extraction() {
    let result = run("احصل_بيانات(تدرج_متجه([١، ٢، ٣]))");
    if let Value::List(l) = result {
        assert_eq!(l.len(), 3);
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_autograd_grad_extraction() {
    let result = run("تدرجات(تدرج_متجه([١، ٢، ٣]))");
    if let Value::List(l) = result {
        assert_eq!(l.len(), 3);
        // All gradients should be 0 initially
        let grads: Vec<f64> = l.iter().map(|v| match &*v.borrow() {
            Value::Number(n) => *n,
            _ => -1.0
        }).collect();
        assert!(grads.iter().all(|&g| g == 0.0));
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_autograd_zero_grads() {
    let result = run("
        متغير أ = تدرج_متجه([١، ٢، ٣])؛
        صفر_تدرجات(أ)؛
        تدرجات(أ)
    ");
    if let Value::List(l) = result {
        let grads: Vec<f64> = l.iter().map(|v| match &*v.borrow() {
            Value::Number(n) => *n,
            _ => -1.0
        }).collect();
        assert!(grads.iter().all(|&g| g == 0.0));
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_autograd_info() {
    let result = run("معلومات_تدرج(تدرج_متجه([١، ٢، ٣]))");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("معرف"));
            assert!(dict.contains_key("حجم"));
            assert!(dict.contains_key("أبعاد"));
        }
        _ => panic!("Expected Dictionary"),
    }
}

#[test]
fn test_autograd_mse_loss() {
    let result = run("
        متغير توقع = تدرج_متجه([١، ٢، ٣])؛
        متغير هدف = [١، ٢، ٤]؛
        تدرج_خطأ_مربع(توقع، هدف)
    ");
    match result {
        Value::AutoTensor { data, .. } => {
            // MSE = ((1-1)^2 + (2-2)^2 + (3-4)^2) / 3 = 0 + 0 + 1 / 3 = 0.333
            assert!((data[0] - 0.333).abs() < 0.1);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_autograd_chain_operations() {
    let result = run("
        متغير س = تدرج_متجه([١، ٢، ٣])؛
        متغير ب = تدرج_ضرب_عدد(س، ٢)؛
        احصل_بيانات(ب)
    ");
    if let Value::List(l) = result {
        let vals: Vec<f64> = l.iter().map(|v| match &*v.borrow() {
            Value::Number(n) => *n,
            _ => -1.0
        }).collect();
        assert_eq!(vals, vec![2.0, 4.0, 6.0]);
    } else {
        panic!("Expected List");
    }
}

// ═══════════════════════════════════════════════════════════════
// اختبارات AutoGrad الكامل
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_gradient_check_sigmoid() {
    let result = run("اختبر_تدرج(\"سيجمويد\"، ٠.٥)");
    match result {
        Value::Dictionary(dict) => {
            if let Some(passed) = dict.get("ناجح") {
                if let Value::Boolean(p) = &*passed.borrow() {
                    assert!(*p, "Sigmoid gradient check failed");
                }
            }
        }
        _ => panic!("Expected Dictionary"),
    }
}

#[test]
fn test_gradient_check_relu() {
    let result = run("اختبر_تدرج(\"ريلو\"، ١)");
    match result {
        Value::Dictionary(dict) => {
            if let Some(passed) = dict.get("ناجح") {
                if let Value::Boolean(p) = &*passed.borrow() {
                    assert!(*p, "ReLU gradient check failed");
                }
            }
        }
        _ => panic!("Expected Dictionary"),
    }
}

#[test]
fn test_gradient_check_tanh() {
    let result = run("اختبر_تدرج(\"تانه\"، ٠.٥)");
    match result {
        Value::Dictionary(dict) => {
            if let Some(passed) = dict.get("ناجح") {
                if let Value::Boolean(p) = &*passed.borrow() {
                    assert!(*p, "Tanh gradient check failed");
                }
            }
        }
        _ => panic!("Expected Dictionary"),
    }
}

#[test]
fn test_gradient_check_square() {
    let result = run("اختبر_تدرج(\"مربع\"، ٣)");
    match result {
        Value::Dictionary(dict) => {
            if let Some(passed) = dict.get("ناجح") {
                if let Value::Boolean(p) = &*passed.borrow() {
                    assert!(*p, "Square gradient check failed");
                }
            }
        }
        _ => panic!("Expected Dictionary"),
    }
}

#[test]
fn test_linear_training() {
    let result = run("
        متغير مدخلات = [١، ٢، ٣، ٤، ٥]؛
        متغير أهداف = [٢، ٤، ٦، ٨، ١٠]؛
        درّب_خطي(مدخلات، أهداف، ٠.٠١، ١٠٠)
    ");
    match result {
        Value::Dictionary(dict) => {
            // Check that weight is close to 2 (y = 2x)
            if let Some(weight) = dict.get("وزن") {
                if let Value::Number(w) = &*weight.borrow() {
                    assert!((w - 2.0).abs() < 0.5, "Weight should be close to 2");
                }
            }
        }
        _ => panic!("Expected Dictionary"),
    }
}

#[test]
fn test_mlp_training_xor() {
    let result = run("
        متغير طبقات = [٢، ٤، ١]؛
        متغير مدخلات = [[٠، ٠]، [٠، ١]، [١، ٠]، [١، ١]]؛
        متغير أهداف = [[٠]، [١]، [١]، [٠]]؛
        درّب_متعدد(طبقات، مدخلات، أهداف، ٠.٥، ٥٠)
    ");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("خسائر"));
            assert!(dict.contains_key("خسارة_أخيرة"));
        }
        _ => panic!("Expected Dictionary"),
    }
}

#[test]
fn test_prediction() {
    let result = run("
        متغير مدخلات = [١، ٢، ٣]؛
        متغير أهداف = [٢، ٤، ٦]؛
        متغير نموذج = درّب_خطي(مدخلات، أهداف، ٠.٠١، ٥٠)؛
        توقع(نموذج، [٤، ٥])
    ");
    match result {
        Value::List(l) => {
            assert_eq!(l.len(), 2);
        }
        _ => panic!("Expected List"),
    }
}

#[test]
fn test_chain_rule_gradient() {
    let result = run("
        متغير عمليات = [\"سيجمويد\"، \"ريلو\"، \"ضرب\"]؛
        متغير تدرج = [١]؛
        سلسلة_تدرج(عمليات، تدرج)
    ");
    match result {
        Value::List(l) => {
            // Should have some gradient value
            assert!(!l.is_empty());
        }
        _ => panic!("Expected List"),
    }
}

#[test]
fn test_autograd_tensor_creation_extended() {
    let result = run("تدرج_متجه([١، ٢، ٣، ٤، ٥])");
    match result {
        Value::AutoTensor { data, shape, requires_grad, .. } => {
            assert_eq!(data.len(), 5);
            assert_eq!(shape, vec![5]);
            assert!(requires_grad);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_autograd_random_tensor() {
    let result = run("تدرج_عشوائي(٣)");
    match result {
        Value::AutoTensor { data, shape, .. } => {
            assert_eq!(data.len(), 3);
            assert_eq!(shape, vec![3]);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_autograd_zeros_tensor() {
    let result = run("تدرج_أصفار(٤)");
    match result {
        Value::AutoTensor { data, shape, .. } => {
            assert_eq!(data.len(), 4);
            assert!(data.iter().all(|&x| x == 0.0));
            assert_eq!(shape, vec![4]);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_autograd_ones_tensor() {
    let result = run("تدرج_آحاد(٣)");
    match result {
        Value::AutoTensor { data, shape, .. } => {
            assert_eq!(data.len(), 3);
            assert!(data.iter().all(|&x| x == 1.0));
            assert_eq!(shape, vec![3]);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_autograd_addition() {
    let result = run("
        متغير أ = تدرج_متجه([١، ٢، ٣])؛
        متغير ب = تدرج_متجه([٤، ٥، ٦])؛
        احصل_بيانات(تدرج_جمع(أ، ب))
    ");
    if let Value::List(l) = result {
        let vals: Vec<f64> = l.iter().map(|v| match &*v.borrow() {
            Value::Number(n) => *n,
            _ => -1.0
        }).collect();
        assert_eq!(vals, vec![5.0, 7.0, 9.0]);
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_autograd_subtraction() {
    let result = run("
        متغير أ = تدرج_متجه([٤، ٥، ٦])؛
        متغير ب = تدرج_متجه([١، ٢، ٣])؛
        احصل_بيانات(تدرج_طرح(أ، ب))
    ");
    if let Value::List(l) = result {
        let vals: Vec<f64> = l.iter().map(|v| match &*v.borrow() {
            Value::Number(n) => *n,
            _ => -1.0
        }).collect();
        assert_eq!(vals, vec![3.0, 3.0, 3.0]);
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_autograd_multiplication() {
    let result = run("
        متغير أ = تدرج_متجه([٢، ٣، ٤])؛
        متغير ب = تدرج_متجه([٣، ٢، ١])؛
        احصل_بيانات(تدرج_ضرب(أ، ب))
    ");
    if let Value::List(l) = result {
        let vals: Vec<f64> = l.iter().map(|v| match &*v.borrow() {
            Value::Number(n) => *n,
            _ => -1.0
        }).collect();
        assert_eq!(vals, vec![6.0, 6.0, 4.0]);
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_autograd_division() {
    let result = run("
        متغير أ = تدرج_متجه([٦، ٨، ١٠])؛
        متغير ب = تدرج_متجه([٢، ٢، ٢])؛
        احصل_بيانات(تدرج_قسمة(أ، ب))
    ");
    if let Value::List(l) = result {
        let vals: Vec<f64> = l.iter().map(|v| match &*v.borrow() {
            Value::Number(n) => *n,
            _ => -1.0
        }).collect();
        // استخدام مقارنة تقريبية للفاصلة العائمة
        assert!((vals[0] - 3.0).abs() < 0.0001);
        assert!((vals[1] - 4.0).abs() < 0.0001);
        assert!((vals[2] - 5.0).abs() < 0.0001);
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_autograd_sigmoid_forward() {
    let result = run("
        متغير أ = تدرج_متجه([٠])؛
        احصل_بيانات(تدرج_سيجمويد(أ))
    ");
    if let Value::List(l) = result {
        if let Value::Number(n) = &*l[0].borrow() {
            // sigmoid(0) = 0.5
            assert!((n - 0.5).abs() < 0.001);
        }
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_autograd_relu_forward() {
    let result = run("
        متغير أ = تدرج_متجه([-١، ٠، ١، ٢])؛
        احصل_بيانات(تدرج_ريلو(أ))
    ");
    if let Value::List(l) = result {
        let vals: Vec<f64> = l.iter().map(|v| match &*v.borrow() {
            Value::Number(n) => *n,
            _ => -1.0
        }).collect();
        assert_eq!(vals, vec![0.0, 0.0, 1.0, 2.0]);
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_update_weights() {
    let result = run("
        متغير أوزان = [١، ٢، ٣]؛
        متغير تدرجات = [٠.١، ٠.٢، ٠.٣]؛
        تحديث_أوزان(أوزان، تدرجات، ٠.١)
    ");
    if let Value::List(l) = result {
        let vals: Vec<f64> = l.iter().map(|v| match &*v.borrow() {
            Value::Number(n) => *n,
            _ => -1.0
        }).collect();
        // w_new = w - lr * grad
        assert!((vals[0] - 0.99).abs() < 0.001);
        assert!((vals[1] - 1.98).abs() < 0.001);
        assert!((vals[2] - 2.97).abs() < 0.001);
    } else {
        panic!("Expected List");
    }
}

#[test]
fn test_training_step() {
    let result = run("
        متغير مدخلات = [١، ٢، ٣]؛
        متغير أهداف = [٢، ٤، ٦]؛
        خطوة_تدريب(لا_شيء، مدخلات، أهداف، ٠.٠١)
    ");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("خسارة"));
            assert!(dict.contains_key("تدرجات"));
        }
        _ => panic!("Expected Dictionary"),
    }
}

// ═══════════════════════════════════════════════════════════════
// اختبارات GPU
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_gpu_init() {
    let result = run("gpu_تهيئة()");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("متاح"));
            assert!(dict.contains_key("النوع"));
            assert!(dict.contains_key("الأنوية"));
        }
        _ => panic!("Expected Dictionary"),
    }
}

#[test]
fn test_gpu_tensor_creation() {
    let result = run("gpu_متجه([١، ٢، ٣، ٤، ٥])");
    match result {
        Value::AutoTensor { data, shape, .. } => {
            assert_eq!(data.len(), 5);
            assert_eq!(shape, vec![5]);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_gpu_matrix_creation() {
    let result = run("gpu_مصفوفة(٢، ٣)");
    match result {
        Value::AutoTensor { data, shape, .. } => {
            assert_eq!(data.len(), 6);
            assert_eq!(shape, vec![2, 3]);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_gpu_random_matrix() {
    let result = run("gpu_عشوائي(٣، ٣)");
    match result {
        Value::AutoTensor { data, shape, .. } => {
            assert_eq!(data.len(), 9);
            assert_eq!(shape, vec![3, 3]);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_gpu_matmul() {
    let result = run("
        متغير أ = gpu_مصفوفة(٢، ٢، [١، ٢، ٣، ٤])؛
        متغير ب = gpu_مصفوفة(٢، ٢، [٥، ٦، ٧، ٨])؛
        gpu_ضرب(أ، ب)
    ");
    match result {
        Value::AutoTensor { data, shape, .. } => {
            // [[1,2],[3,4]] × [[5,6],[7,8]] = [[19,22],[43,50]]
            assert_eq!(shape, vec![2, 2]);
            assert!((data[0] - 19.0).abs() < 0.001);
            assert!((data[3] - 50.0).abs() < 0.001);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_gpu_transpose() {
    let result = run("
        متغير مصفوف = gpu_مصفوفة(٢، ٣، [١، ٢، ٣، ٤، ٥، ٦])؛
        gpu_تبديل(مصفوف)
    ");
    match result {
        Value::AutoTensor { shape, .. } => {
            assert_eq!(shape, vec![3, 2]);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_gpu_sigmoid() {
    let result = run("
        متغير س = gpu_متجه([٠])؛
        gpu_سيجمويد(س)
    ");
    match result {
        Value::AutoTensor { data, .. } => {
            // sigmoid(0) = 0.5
            assert!((data[0] - 0.5).abs() < 0.001);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_gpu_relu() {
    let result = run("
        متغير س = gpu_متجه([-١، ٠، ١، ٢])؛
        gpu_ريلو(س)
    ");
    match result {
        Value::AutoTensor { data, .. } => {
            assert_eq!(data, vec![0.0, 0.0, 1.0, 2.0]);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_gpu_tanh() {
    let result = run("
        متغير س = gpu_متجه([٠])؛
        gpu_تانه(س)
    ");
    match result {
        Value::AutoTensor { data, .. } => {
            // tanh(0) = 0
            assert!(data[0].abs() < 0.001);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_gpu_softmax() {
    let result = run("
        متغير س = gpu_متجه([١، ٢، ٣])؛
        gpu_سوفتماكس(س)
    ");
    match result {
        Value::AutoTensor { data, .. } => {
            // sum of softmax should be 1
            let sum: f64 = data.iter().sum();
            assert!((sum - 1.0).abs() < 0.001);
        }
        _ => panic!("Expected AutoTensor"),
    }
}

#[test]
fn test_gpu_benchmark() {
    let result = run("gpu_قياس(٥)");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("وقت_مللي"));
            assert!(dict.contains_key("جيجا_فلوب"));
        }
        _ => panic!("Expected Dictionary"),
    }
}

#[test]
fn test_gpu_comparison() {
    let result = run("gpu_مقارنة(٣)");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("وقت_مللي"));
            assert!(dict.contains_key("الأنوية"));
        }
        _ => panic!("Expected Dictionary"),
    }
}

// ═══════════════════════════════════════════════════════════════
// اختبارات التجميع الفوري (JIT Compilation)
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_jit_enable() {
    let result = run("فعل_تجميع(صح)");
    assert!(matches!(result, Value::String(_)));
}

#[test]
fn test_jit_disable() {
    let result = run("فعل_تجميع(خطأ)");
    assert!(matches!(result, Value::String(_)));
}

#[test]
fn test_jit_compile_vector() {
    let result = run("جمّع_متجه(\"add_vec\"، \"جمع_متجه\"، [[٣]، [٣]]، [٣])");
    assert!(matches!(result, Value::String(_)));
}

#[test]
fn test_jit_compile_matrix() {
    let result = run("جمّع_مصفوفة(\"matmul\"، \"ضرب_مصفوفات\"، ٣، ٣، ٣)");
    assert!(matches!(result, Value::String(_)));
}

#[test]
fn test_jit_optimize() {
    let result = run("حسّن_مجمّع()");
    assert!(matches!(result, Value::String(_)));
}

#[test]
fn test_jit_report() {
    let result = run("تقرير_أداء()");
    assert!(matches!(result, Value::String(_)));
}

#[test]
fn test_jit_clear_cache() {
    let result = run("امسح_ذاكرة_مؤقتة()");
    assert!(matches!(result, Value::String(_)));
}

#[test]
fn test_vector_fast_zeros() {
    let result = run("متجه_سريع(\"أصفار\"، ٥)");
    if let Value::Tensor { data, shape } = result {
        assert_eq!(data.len(), 5);
        assert!(data.iter().all(|&x| x == 0.0));
        assert_eq!(shape, vec![5]);
    } else {
        panic!("Expected Tensor");
    }
}

#[test]
fn test_vector_fast_ones() {
    let result = run("متجه_سريع(\"آحاد\"، ٥)");
    if let Value::Tensor { data, shape } = result {
        assert_eq!(data.len(), 5);
        assert!(data.iter().all(|&x| x == 1.0));
        assert_eq!(shape, vec![5]);
    } else {
        panic!("Expected Tensor");
    }
}

#[test]
fn test_matrix_fast_identity() {
    let result = run("مصفوفة_سريعة(\"هوية\"، ٣، ٣)");
    if let Value::Tensor { data, shape } = result {
        assert_eq!(shape, vec![3, 3]);
        // التحقق من مصفوفة الهوية
        assert_eq!(data[0], 1.0);
        assert_eq!(data[4], 1.0);
        assert_eq!(data[8], 1.0);
    } else {
        panic!("Expected Tensor");
    }
}

#[test]
fn test_apply_vector_relu() {
    let result = run("طبّق_متجه(\"ريلو\"، متجه([١، -٢، ٣، -٤، ٥]))");
    if let Value::Tensor { data, .. } = result {
        assert_eq!(data, vec![1.0, 0.0, 3.0, 0.0, 5.0]);
    } else {
        panic!("Expected Tensor");
    }
}

#[test]
fn test_apply_vector_sigmoid() {
    let result = run("طبّق_متجه(\"سيجمويد\"، متجه([٠]))");
    if let Value::Tensor { data, .. } = result {
        assert!((data[0] - 0.5).abs() < 0.001);
    } else {
        panic!("Expected Tensor");
    }
}

#[test]
fn test_apply_vector_square() {
    let result = run("طبّق_متجه(\"مربع\"، متجه([١، ٢، ٣]))");
    if let Value::Tensor { data, .. } = result {
        assert_eq!(data, vec![1.0, 4.0, 9.0]);
    } else {
        panic!("Expected Tensor");
    }
}

// ═══════════════════════════════════════════════════════════════
// اختبارات تصدير/استيراد النماذج المتقدمة
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_export_onnx() {
    let result = run("متغير نموذج = شبكة_متعددة(٧٨٤، [١٢٨]، ١٠)؛ صدّر_أونكس(نموذج، \"model.onnx\")");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("نجاح"));
            assert!(dict.contains_key("صيغة"));
            assert!(dict.contains_key("بيانات"));
        }
        _ => panic!("Expected Dictionary from export_onnx"),
    }
}

#[test]
fn test_import_onnx() {
    let result = run("استورد_أونكس(\"model.onnx\")");
    assert!(matches!(result, Value::NeuralNetwork { .. }));
}

#[test]
fn test_export_pytorch() {
    let result = run("متغير نموذج = شبكة_متعددة(١٠٠، [٥٠]، ١٠)؛ صدّر_تورش(نموذج، \"model.pt\")");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("نجاح"));
            assert!(dict.contains_key("صيغة"));
            let format = dict.get("صيغة").unwrap().borrow();
            assert!(matches!(&*format, Value::String(s) if s == "PyTorch"));
        }
        _ => panic!("Expected Dictionary from export_pytorch"),
    }
}

#[test]
fn test_export_tensorflow() {
    let result = run("متغير نموذج = شبكة_متعددة(١٠٠، [٥٠]، ١٠)؛ صدّر_تنسورفلو(نموذج، \"model.pb\")");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("نجاح"));
            assert!(dict.contains_key("صيغة"));
        }
        _ => panic!("Expected Dictionary from export_tensorflow"),
    }
}

#[test]
fn test_export_json_model() {
    let result = run("متغير نموذج = شبكة_متعددة(١٠٠، [٥٠]، ١٠)؛ صدّر_جسون(نموذج)");
    assert!(matches!(result, Value::String(_)));
}

#[test]
fn test_import_json_model() {
    let result = run("استورد_جسون(\"{\\\"نوع\\\": \\\"شبكة_عصبية\\\"}\")");
    assert!(matches!(result, Value::NeuralNetwork { .. }));
}

#[test]
fn test_save_checkpoint() {
    let result = run("متغير نموذج = شبكة_متعددة(١٠٠، [٥٠]، ١٠)؛ احفظ_نقطة(نموذج، ٥)");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("epoch"));
            assert!(dict.contains_key("layers"));
            assert!(dict.contains_key("timestamp"));
        }
        _ => panic!("Expected Dictionary from save_checkpoint"),
    }
}

#[test]
fn test_load_checkpoint() {
    let result = run("متغير نقطة = احفظ_نقطة(شبكة_متعددة(١٠٠، [٥٠]، ١٠)، ٣)؛ حمّل_نقطة(نقطة)");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("epoch"));
            assert!(dict.contains_key("model"));
        }
        _ => panic!("Expected Dictionary from load_checkpoint"),
    }
}

#[test]
fn test_compress_model() {
    let result = run("متغير نموذج = شبكة_متعددة(١٠٠، [٥٠]، ١٠)؛ اضغط_نموذج(نموذج، ٠.٥)");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("model"));
            assert!(dict.contains_key("original_params"));
            assert!(dict.contains_key("compressed_params"));
            assert!(dict.contains_key("compression_percent"));
        }
        _ => panic!("Expected Dictionary from compress_model"),
    }
}

#[test]
fn test_quantize_model() {
    let result = run("متغير نموذج = شبكة_متعددة(١٠٠، [٥٠]، ١٠)؛ كمّم_نموذج(نموذج، ٨)");
    match result {
        Value::Dictionary(dict) => {
            assert!(dict.contains_key("model"));
            assert!(dict.contains_key("bits"));
            assert!(dict.contains_key("original_size_bytes"));
            assert!(dict.contains_key("quantized_size_bytes"));
            assert!(dict.contains_key("size_reduction_percent"));
        }
        _ => panic!("Expected Dictionary from quantize_model"),
    }
}

#[test]
fn test_quantize_model_4bit() {
    let result = run("متغير نموذج = شبكة_متعددة(١٠٠، [٥٠]، ١٠)؛ كمّم_نموذج(نموذج، ٤)");
    if let Value::Dictionary(dict) = result {
        let reduction = dict.get("size_reduction_percent").unwrap().borrow();
        if let Value::Number(n) = &*reduction {
            // 4-bit should give 87.5% reduction
            assert!((n - 87.5).abs() < 0.1);
        }
    }
}
