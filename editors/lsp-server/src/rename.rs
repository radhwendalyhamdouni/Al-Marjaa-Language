//! Rename - إعادة التسمية

use lsp_types::*;

use crate::state::{AnalysisResult, Document};

/// تجهيز إعادة التسمية
pub fn prepare_rename(doc: &Document, analysis: &AnalysisResult, position: Position) -> Option<PrepareRenameResponse> {
    let line = doc.get_line(position.line as usize)?;
    let word = find_word_at_position(line, position.character as usize)?;
    
    // التحقق من وجود الرمز
    if analysis.definitions.contains_key(&word) || analysis.references.contains_key(&word) {
        return Some(PrepareRenameResponse::RangeWithPlaceholder {
            range: Range::new(
                Position::new(position.line, 0),
                Position::new(position.line, word.chars().count() as u32),
            ),
            placeholder: word,
        });
    }
    
    None
}

/// إعادة تسمية الرمز
pub fn rename_symbol(doc: &Document, analysis: &AnalysisResult, position: Position, new_name: &str) -> Option<WorkspaceEdit> {
    let line = doc.get_line(position.line as usize)?;
    let word = find_word_at_position(line, position.character as usize)?;
    
    // الحصول على جميع المراجع
    let references = analysis.references.get(&word)?;
    
    let mut changes = std::collections::HashMap::new();
    let mut edits = Vec::new();
    
    for r in references {
        edits.push(TextEdit {
            range: Range::new(
                Position::new(r.line.saturating_sub(1) as u32, r.column as u32),
                Position::new(r.line.saturating_sub(1) as u32, r.end_column as u32),
            ),
            new_text: new_name.to_string(),
        });
    }
    
    // إضافة تعريف
    if let Some(def) = analysis.definitions.get(&word) {
        edits.push(TextEdit {
            range: Range::new(
                Position::new(def.line.saturating_sub(1) as u32, def.column as u32),
                Position::new(def.line.saturating_sub(1) as u32, def.end_column as u32),
            ),
            new_text: new_name.to_string(),
        });
    }
    
    // ترتيب التعديلات
    edits.sort_by(|a, b| b.range.start.cmp(&a.range.start));
    
    // TODO: استخدام الـ URI الصحيح
    // changes.insert(uri, edits);
    
    Some(WorkspaceEdit {
        changes: Some(changes),
        ..Default::default()
    })
}

fn find_word_at_position(line: &str, column: usize) -> Option<String> {
    let chars: Vec<char> = line.chars().collect();
    if chars.is_empty() {
        return None;
    }
    
    let column = column.min(chars.len().saturating_sub(1));
    
    let mut start = column;
    while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
        start -= 1;
    }
    
    let mut end = column;
    while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
        end += 1;
    }
    
    if start == end {
        return None;
    }
    
    Some(chars[start..end].iter().collect())
}
