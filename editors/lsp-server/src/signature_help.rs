//! Signature Help - مساعدة التوقيع

use lsp_types::*;

use crate::state::{AnalysisResult, Document};

/// الحصول على مساعدة التوقيع
pub fn get_signature_help(doc: &Document, analysis: &AnalysisResult, position: Position) -> Option<SignatureHelp> {
    // البحث عن قوس فتح قبل الموضع
    let content = doc.content.clone();
    let offset = position_to_offset(&content, position);
    
    // البحث عن آخر قوس فتح
    let last_open_paren = content[..offset].rfind('(')?;
    
    // البحث عن اسم الدالة
    let before_paren = &content[..last_open_paren];
    let func_name = extract_function_name(before_paren)?;
    
    // الحصول على معلومات التوقيع
    let signature = analysis.signatures.get(&func_name)?;
    
    // تحديد المعامل الحالي
    let after_paren = &content[last_open_paren + 1..offset];
    let current_param = after_paren.matches('،').count() + after_paren.matches(',').count();
    
    let sig_info = SignatureInformation {
        label: signature.label.clone(),
        documentation: signature.documentation.clone().map(|d| Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: d,
        })),
        parameters: Some(signature.parameters.iter()
            .map(|p| ParameterInformation {
                label: ParameterLabel::Simple(format!("{}: {}", p.name, p.type_annotation.as_deref().unwrap_or(""))),
                documentation: p.documentation.clone().map(|d| Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: d,
                })),
            })
            .collect()),
        active_parameter: Some(current_param as u32),
    };
    
    Some(SignatureHelp {
        signatures: vec![sig_info],
        active_signature: Some(0),
        active_parameter: Some(current_param as u32),
    })
}

/// تحويل الموضع إلى إزاحة
fn position_to_offset(content: &str, position: Position) -> usize {
    let mut offset = 0;
    for (i, line) in content.lines().enumerate() {
        if i == position.line as usize {
            offset += position.character.min(line.len() as u32) as usize;
            break;
        }
        offset += line.len() + 1; // +1 للسطر الجديد
    }
    offset
}

/// استخراج اسم الدالة
fn extract_function_name(before_paren: &str) -> Option<String> {
    let chars: Vec<char> = before_paren.chars().collect();
    let mut end = chars.len();
    
    // تخطي المسافات
    while end > 0 && chars[end - 1].is_whitespace() {
        end -= 1;
    }
    
    // استخراج الاسم
    let mut start = end;
    while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
        start -= 1;
    }
    
    if start == end {
        None
    } else {
        Some(chars[start..end].iter().collect())
    }
}
