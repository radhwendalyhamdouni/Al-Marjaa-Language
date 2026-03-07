// اختبار بسيط للـ bytecode

#[test]
fn test_simple_loop_bytecode() {
    let code = r#"
        متغير س = 0؛
        متغير مجموع = 0؛
        طالما س < 100 {
            مجموع = مجموع + س؛
            س = س + 1؛
        }
    "#;
    
    println!("Testing bytecode with 100 iterations...");
    let start = std::time::Instant::now();
    let result = almarjaa::bytecode::run_bytecode(code);
    let duration = start.elapsed();
    
    println!("Result: {:?}", result);
    println!("Duration: {:?}", duration);
    
    assert!(result.is_ok(), "Bytecode execution failed: {:?}", result);
}

#[test]
fn test_simple_loop_interpreter() {
    let code = r#"
        متغير س = 0؛
        متغير مجموع = 0؛
        طالما س < 100 {
            مجموع = مجموع + س؛
            س = س + 1؛
        }
    "#;
    
    println!("Testing interpreter with 100 iterations...");
    let mut interp = almarjaa::interpreter::Interpreter::new();
    let start = std::time::Instant::now();
    let result = interp.run(code);
    let duration = start.elapsed();
    
    println!("Result: {:?}", result);
    println!("Duration: {:?}", duration);
    
    assert!(result.is_ok(), "Interpreter execution failed: {:?}", result);
}
