// tests/cli_smoke_tests.rs
// اختبارات دخان لمسار التنفيذ الأساسي

use almarjaa::Interpreter;

#[test]
fn test_hello_world() {
    let mut interpreter = Interpreter::new();
    let code = r#"اطبع("مرحباً بالعالم!")؛"#;
    let result = interpreter.run(code);
    assert!(result.is_ok());
}

#[test]
fn test_variables() {
    let mut interpreter = Interpreter::new();
    interpreter.run("متغير س = ١٠؛").unwrap();
    let result = interpreter.run("س").unwrap();
    assert_eq!(result.borrow().to_string(), "10");
}

#[test]
fn test_arithmetic() {
    let mut interpreter = Interpreter::new();

    assert_eq!(interpreter.run("٢ + ٣").unwrap().borrow().to_string(), "5");
    assert_eq!(interpreter.run("١٠ - ٤").unwrap().borrow().to_string(), "6");
    assert_eq!(interpreter.run("٣ * ٤").unwrap().borrow().to_string(), "12");
    assert_eq!(interpreter.run("٢٠ / ٤").unwrap().borrow().to_string(), "5");
}
