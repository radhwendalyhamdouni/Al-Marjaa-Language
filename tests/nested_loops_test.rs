// اختبار الحلقات المتداخلة

#[test]
fn test_nested_loops_interpreter() {
    let code = r#"
        متغير مجموع = 0؛
        متغير أ = 0؛
        متغير ب = 0؛
        طالما أ < 5 {
            ب = 0؛
            طالما ب < 5 {
                مجموع = مجموع + أ * ب؛
                ب = ب + 1؛
            }
            أ = أ + 1؛
        }
    "#;
    
    println!("Testing nested loops with interpreter...");
    let mut interp = almarjaa::interpreter::Interpreter::new();
    let start = std::time::Instant::now();
    let result = interp.run(code);
    let duration = start.elapsed();
    
    println!("Result: {:?}", result);
    println!("Duration: {:?}", duration);
    
    assert!(result.is_ok(), "Interpreter failed: {:?}", result);
}

#[test]
fn test_nested_loops_bytecode() {
    let code = r#"
        متغير مجموع = 0؛
        متغير أ = 0؛
        متغير ب = 0؛
        طالما أ < 5 {
            ب = 0؛
            طالما ب < 5 {
                مجموع = مجموع + أ * ب؛
                ب = ب + 1؛
            }
            أ = أ + 1؛
        }
    "#;
    
    println!("Testing nested loops with bytecode...");
    let start = std::time::Instant::now();
    let result = almarjaa::bytecode::run_bytecode(code);
    let duration = start.elapsed();
    
    println!("Result: {:?}", result);
    println!("Duration: {:?}", duration);
    
    assert!(result.is_ok(), "Bytecode failed: {:?}", result);
}
