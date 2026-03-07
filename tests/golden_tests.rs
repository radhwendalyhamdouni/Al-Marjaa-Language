use std::fs;
use std::path::Path;

use almarjaa::{format_source, lint_source};

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("تعذر قراءة {}", path.display()))
}

fn assert_golden_pairs<F>(dir: &str, transform: F)
where
    F: Fn(&str) -> String,
{
    let root = Path::new(dir);
    let mut inputs: Vec<_> = fs::read_dir(root)
        .expect("تعذر قراءة مجلد golden")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with(".input.mrj"))
        })
        .collect();

    inputs.sort();
    assert!(!inputs.is_empty(), "يجب توفير ملفات golden داخل {}", dir);

    for input_path in inputs {
        let expected_path = input_path
            .to_string_lossy()
            .replace(".input.mrj", ".golden.txt");
        let expected_path = Path::new(&expected_path).to_path_buf();

        assert!(
            expected_path.exists(),
            "ملف golden غير موجود: {}",
            expected_path.display()
        );

        let source = read(&input_path);
        let expected = read(&expected_path);
        let actual = transform(&source);

        assert_eq!(
            actual.trim_end(),
            expected.trim_end(),
            "اختلاف golden في {}",
            input_path.display()
        );
    }
}

#[test]
fn formatter_goldens_are_stable() {
    assert_golden_pairs("tests/golden/formatter", format_source);
}

#[test]
fn linter_goldens_are_stable() {
    assert_golden_pairs("tests/golden/linter", |source| {
        let diagnostics = lint_source(source).expect("يجب أن يمر parser في linter golden");
        if diagnostics.is_empty() {
            return "OK".to_string();
        }

        diagnostics
            .iter()
            .map(|d| format!("{}|{:?}|{}", d.code, d.level, d.message))
            .collect::<Vec<_>>()
            .join("\n")
    });
}
