//! Inlay Hints - التلميحات المضمنة

use lsp_types::*;

use crate::state::{AnalysisResult, Document, DefinitionKind};

/// الحصول على Inlay Hints
pub fn get_inlay_hints(doc: &Document, analysis: &AnalysisResult, range: Range) -> Vec<InlayHint> {
    let mut hints = Vec::new();
    
    // تلميحات الأنواع للمتغيرات
    for (name, def) in &analysis.definitions {
        if def.line < range.start.line as usize + 1 || def.line > range.end.line as usize + 1 {
            continue;
        }
        
        // إظهار نوع المتغير إذا كان معروفاً
        if let Some(type_ann) = &def.type_annotation {
            let line = doc.get_line(def.line.saturating_sub(1)).unwrap_or("");
            let end_col = line.chars().count();
            
            hints.push(InlayHint {
                position: Position::new(def.line.saturating_sub(1) as u32, end_col as u32),
                label: InlayHintLabel::String(format!(": {}", type_ann)),
                kind: Some(InlayHintKind::TYPE),
                text_edits: None,
                tooltip: Some(InlayHintTooltip::String(format!("النوع: {}", type_ann))),
                padding_left: Some(true),
                padding_right: Some(false),
                data: None,
            });
        }
        
        // إظهار أسماء المعاملات
        if matches!(def.kind, DefinitionKind::Parameter) {
            if let Some(refs) = analysis.references.get(name) {
                for r in refs {
                    if !r.is_definition {
                        hints.push(InlayHint {
                            position: Position::new(r.line.saturating_sub(1) as u32, r.column as u32),
                            label: InlayHintLabel::String(format!("{}: ", name)),
                            kind: Some(InlayHintKind::PARAMETER),
                            text_edits: None,
                            tooltip: Some(InlayHintTooltip::String(format!("المعامل: {}", name))),
                            padding_left: Some(false),
                            padding_right: Some(false),
                            data: None,
                        });
                    }
                }
            }
        }
    }
    
    hints
}
