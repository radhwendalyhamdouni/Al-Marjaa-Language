pub fn format_source(source: &str) -> String {
    let mut output = String::new();
    let mut indent_level = 0usize;
    let mut previous_was_blank = false;

    for raw_line in source.lines() {
        let trimmed = raw_line.trim();

        if trimmed.is_empty() {
            if !previous_was_blank && !output.is_empty() {
                output.push('\n');
            }
            previous_was_blank = true;
            continue;
        }

        let starts_with_closing = trimmed.starts_with('}');
        if starts_with_closing {
            indent_level = indent_level.saturating_sub(1);
        }

        let normalized = format_line(trimmed);
        let wrapped_lines = wrap_line(&normalized, indent_level);

        for wrapped in wrapped_lines {
            output.push_str(&wrapped);
            output.push('\n');
        }

        if should_increase_indent(&normalized) {
            indent_level += 1;
        }

        previous_was_blank = false;
    }

    while output.ends_with(
        "

",
    ) {
        output.pop();
    }

    output
}

const INDENT: &str = "    ";
const MAX_LINE_WIDTH: usize = 88;

fn format_line(line: &str) -> String {
    if let Some((comment_start, marker_len)) = find_comment_start(line) {
        let code_part = line[..comment_start].trim();
        let comment_part = &line[comment_start + marker_len..];
        let marker = if marker_len == 2 { "//" } else { "#" };
        let comment = format_comment_body(comment_part);

        if code_part.is_empty() {
            return format_comment(marker, &comment);
        }

        return format!(
            "{} {}",
            normalize_whitespace(code_part),
            format_comment(marker, &comment)
        );
    }

    normalize_whitespace(line)
}

fn format_comment_body(comment_body: &str) -> String {
    let normalized = normalize_whitespace(comment_body.trim());
    normalized.trim_start_matches(['#', '/']).trim().to_string()
}

fn format_comment(marker: &str, comment: &str) -> String {
    if comment.is_empty() {
        marker.to_string()
    } else {
        format!("{} {}", marker, comment)
    }
}

fn find_comment_start(line: &str) -> Option<(usize, usize)> {
    let mut in_string = false;
    let chars: Vec<(usize, char)> = line.char_indices().collect();
    let mut i = 0usize;

    while i < chars.len() {
        let (idx, ch) = chars[i];

        if ch == '"' {
            in_string = !in_string;
            i += 1;
            continue;
        }

        if !in_string {
            if ch == '#' {
                return Some((idx, 1));
            }
            if ch == '/' && i + 1 < chars.len() && chars[i + 1].1 == '/' {
                return Some((idx, 2));
            }
        }

        i += 1;
    }

    None
}

fn wrap_line(line: &str, indent_level: usize) -> Vec<String> {
    let base_indent = INDENT.repeat(indent_level);
    let continuation_indent = format!("{}{}", base_indent, INDENT);
    let max_content_width = MAX_LINE_WIDTH.saturating_sub(base_indent.chars().count());

    if line.chars().count() <= max_content_width {
        return vec![format!("{}{}", base_indent, line)];
    }

    let chunks = split_by_space_outside_string(line);
    if chunks.len() <= 1 {
        return vec![format!("{}{}", base_indent, line)];
    }

    let mut result = Vec::new();
    let mut current = String::new();

    for chunk in chunks {
        let candidate = if current.is_empty() {
            chunk.clone()
        } else {
            format!("{} {}", current, chunk)
        };

        if candidate.chars().count() <= max_content_width {
            current = candidate;
            continue;
        }

        if !current.is_empty() {
            let indent = if result.is_empty() {
                &base_indent
            } else {
                &continuation_indent
            };
            result.push(format!("{}{}", indent, current));
        }

        current = chunk;
    }

    if !current.is_empty() {
        let indent = if result.is_empty() {
            &base_indent
        } else {
            &continuation_indent
        };
        result.push(format!("{}{}", indent, current));
    }

    result
}

fn split_by_space_outside_string(line: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_string = false;

    for ch in line.chars() {
        if ch == '"' {
            in_string = !in_string;
            current.push(ch);
            continue;
        }

        if ch.is_whitespace() && !in_string {
            if !current.is_empty() {
                result.push(std::mem::take(&mut current));
            }
            continue;
        }

        current.push(ch);
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

fn should_increase_indent(line: &str) -> bool {
    line.ends_with('{')
}

fn normalize_whitespace(line: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut previous_space = false;

    for ch in line.chars() {
        if ch == '"' {
            in_string = !in_string;
            result.push(ch);
            previous_space = false;
            continue;
        }

        if in_string {
            result.push(ch);
            continue;
        }

        if ch.is_whitespace() {
            if !previous_space {
                result.push(' ');
                previous_space = true;
            }
            continue;
        }

        previous_space = false;

        if is_tight_punctuation(ch) && result.ends_with(' ') {
            result.pop();
        }

        result.push(ch);
    }

    result.trim().to_string()
}

fn is_tight_punctuation(ch: char) -> bool {
    matches!(ch, '؛' | ',' | '،' | ')' | ']' | '}')
}

#[cfg(test)]
mod tests {
    use super::format_source;

    #[test]
    fn test_indent_blocks_and_trim_whitespace() {
        let source = r#"
        إذا   صحيح   {
        اطبع("مرحبا")   ؛
        إذا خطأ {
        اطبع("لن تظهر")؛
        }
        }
        "#;

        let formatted = format_source(source);

        assert_eq!(
            formatted,
            "إذا صحيح {\n    اطبع(\"مرحبا\")؛\n    إذا خطأ {\n        اطبع(\"لن تظهر\")؛\n    }\n}\n"
        );
    }

    #[test]
    fn test_collapses_extra_blank_lines() {
        let source = "متغير س = ١؛\n\n\nاطبع(س)؛\n";
        let formatted = format_source(source);

        assert_eq!(formatted, "متغير س = ١؛\n\nاطبع(س)؛\n");
    }

    #[test]
    fn test_formats_comment_spacing() {
        let source = "//   تعليق   مع  مسافات\nمتغير س=١؛    #  تعليق  اخر\n";
        let formatted = format_source(source);

        assert_eq!(formatted, "// تعليق مع مسافات\nمتغير س=١؛ # تعليق اخر\n");
    }

    #[test]
    fn test_wraps_long_lines() {
        let source = "متغير نتيجة = قيمة_اولى + قيمة_ثانية + قيمة_ثالثة + قيمة_رابعة + قيمة_خامسة + قيمة_سادسة؛\n";
        let formatted = format_source(source);

        assert_eq!(
            formatted,
            "متغير نتيجة = قيمة_اولى + قيمة_ثانية + قيمة_ثالثة + قيمة_رابعة + قيمة_خامسة +
    قيمة_سادسة؛\n"
        );
    }
}
