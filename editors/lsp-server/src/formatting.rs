//! Formatting - التنسيق

use lsp_types::*;

/// تنسيق المستند
pub fn format_document(content: &str, options: &FormattingOptions) -> Vec<TextEdit> {
    let mut edits = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut formatted_lines = Vec::new();
    
    let indent_str = if options.insert_spaces {
        " ".repeat(options.tab_size as usize)
    } else {
        "\t".to_string()
    };
    
    let mut indent_level = 0;
    let mut in_string = false;
    let mut string_char = ' ';
    
    for line in &lines {
        let trimmed = line.trim();
        
        // تقليل المسافة البادئة للأقواس المغلقة
        if trimmed.starts_with('}') || trimmed.ends_with('}') {
            indent_level = indent_level.saturating_sub(1);
        }
        
        // حساب المسافة البادئة
        let formatted_line = if trimmed.is_empty() {
            String::new()
        } else {
            format!("{}{}", indent_str.repeat(indent_level), trimmed)
        };
        
        formatted_lines.push(formatted_line);
        
        // زيادة المسافة البادئة للأقواس المفتوحة
        for c in trimmed.chars() {
            if !in_string {
                if c == '{' {
                    indent_level += 1;
                } else if c == '}' {
                    indent_level = indent_level.saturating_sub(1);
                } else if c == '"' || c == '\'' {
                    in_string = true;
                    string_char = c;
                }
            } else if c == string_char {
                in_string = false;
            }
        }
    }
    
    let formatted = formatted_lines.join("\n");
    
    if formatted != content {
        edits.push(TextEdit {
            range: Range::new(
                Position::new(0, 0),
                Position::new(lines.len() as u32, 0),
            ),
            new_text: formatted,
        });
    }
    
    edits
}
