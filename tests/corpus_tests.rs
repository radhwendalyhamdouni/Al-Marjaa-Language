use std::fs;
use std::path::Path;

use almarjaa::{Interpreter, Parser};

#[test]
fn test_corpus_files_are_available() {
    let corpus_dir = Path::new("tests/corpus");
    let entries = fs::read_dir(corpus_dir).expect("تعذر قراءة مجلد corpus");
    let files: Vec<_> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|s| s.to_str()) == Some("mrj"))
        .collect();

    assert!(
        files.len() >= 10,
        "يجب أن يحتوي test corpus على 10 ملفات .mrj على الأقل"
    );
}

#[test]
fn test_corpus_programs_execute_without_runtime_errors() {
    let corpus_dir = Path::new("tests/corpus");
    let entries = fs::read_dir(corpus_dir).expect("تعذر قراءة مجلد corpus");
    let mut files: Vec<_> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|s| s.to_str()) == Some("mrj"))
        .collect();

    files.sort();

    for path in files {
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("تعذر قراءة الملف: {}", path.display()));

        let parse_result = Parser::parse(&source);
        assert!(
            parse_result.is_ok(),
            "فشل التحليل (compile) لملف corpus: {} بسبب: {:?}",
            path.display(),
            parse_result.err()
        );

        let mut interpreter = Interpreter::new();
        let run_result = interpreter.run(&source);

        assert!(
            run_result.is_ok(),
            "فشل التنفيذ لملف corpus: {} بسبب: {:?}",
            path.display(),
            run_result.err()
        );
    }
}
