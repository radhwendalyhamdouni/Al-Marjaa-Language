//! Code Lens - عدسات الكود

use lsp_types::*;

use crate::state::{AnalysisResult, Document, DefinitionKind};

/// الحصول على Code Lenses
pub fn get_code_lenses(uri: &Url, doc: &Document, analysis: &AnalysisResult) -> Vec<CodeLens> {
    let mut lenses = Vec::new();
    
    for (name, def) in &analysis.definitions {
        let line = def.line.saturating_sub(1) as u32;
        let range = Range::new(
            Position::new(line, 0),
            Position::new(line, 100),
        );
        
        match def.kind {
            DefinitionKind::Function | DefinitionKind::Method => {
                // تشغيل الدالة
                lenses.push(CodeLens {
                    range,
                    command: Some(Command {
                        title: "$(play) تشغيل".to_string(),
                        command: "almarjaa.runFunction".to_string(),
                        arguments: Some(vec![
                            serde_json::to_value(name).unwrap(),
                            serde_json::to_value(uri).unwrap(),
                        ]),
                    }),
                    data: None,
                });
                
                // اختبار الدالة
                lenses.push(CodeLens {
                    range,
                    command: Some(Command {
                        title: "$(beaker) اختبار".to_string(),
                        command: "almarjaa.testFunction".to_string(),
                        arguments: Some(vec![serde_json::to_value(name).unwrap()]),
                    }),
                    data: None,
                });
                
                // عدد المراجع
                if let Some(refs) = analysis.references.get(name) {
                    let count = refs.len();
                    lenses.push(CodeLens {
                        range,
                        command: Some(Command {
                            title: format!("$(references) {} مرجع", count),
                            command: "editor.action.showReferences".to_string(),
                            arguments: Some(vec![
                                serde_json::to_value(uri).unwrap(),
                                serde_json::to_value(Position::new(line, def.column as u32)).unwrap(),
                                serde_json::to_value(Vec::<Location>::new()).unwrap(),
                            ]),
                        }),
                        data: None,
                    });
                }
            }
            DefinitionKind::Variable | DefinitionKind::Constant => {
                // إظهار النوع
                if def.type_annotation.is_some() {
                    lenses.push(CodeLens {
                        range,
                        command: Some(Command {
                            title: format!("$(symbol-misc) {}", def.type_annotation.as_ref().unwrap()),
                            command: "".to_string(),
                            arguments: None,
                        }),
                        data: None,
                    });
                }
            }
            _ => {}
        }
    }
    
    lenses
}
