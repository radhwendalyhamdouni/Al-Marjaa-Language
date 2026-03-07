//! ═══════════════════════════════════════════════════════════════════════════════
//! Code Actions - إجراءات الكود
//! ═══════════════════════════════════════════════════════════════════════════════

use lsp_types::*;

use crate::state::AnalysisResult;

/// الحصول على Code Actions
pub fn get_code_actions(
    uri: &Url,
    range: &Range,
    analysis: &AnalysisResult,
    context: &CodeActionContext,
) -> Vec<CodeActionOrCommand> {
    let mut actions = Vec::new();
    
    // إجراءات الإصلاح السريع
    for diagnostic in &context.diagnostics {
        if let Some(fixes) = get_quick_fixes(uri, diagnostic) {
            actions.extend(fixes);
        }
    }
    
    // إجراءات إعادة البناء
    actions.extend(get_refactor_actions(uri, range, analysis));
    
    // إجراءات المصدر
    actions.extend(get_source_actions(uri));
    
    actions
}

/// الحصول على الإصلاحات السريعة
fn get_quick_fixes(uri: &Url, diagnostic: &Diagnostic) -> Option<Vec<CodeActionOrCommand>> {
    let mut actions = Vec::new();
    
    // إصلاح متغير غير مستخدم
    if diagnostic.code == Some(NumberOrString::String("L001".to_string())) {
        actions.push(CodeActionOrCommand::CodeAction(CodeAction {
            title: "احذف المتغير غير المستخدم".to_string(),
            kind: Some(CodeActionKind::QUICKFIX),
            diagnostics: Some(vec![diagnostic.clone()]),
            edit: Some(WorkspaceEdit {
                changes: Some({
                    let mut map = std::collections::HashMap::new();
                    map.insert(uri.clone(), vec![
                        TextEdit {
                            range: diagnostic.range,
                            new_text: String::new(),
                        }
                    ]);
                    map
                }),
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
    
    // إصلاح كتلة catch فارغة
    if diagnostic.code == Some(NumberOrString::String("L003".to_string())) {
        actions.push(CodeActionOrCommand::CodeAction(CodeAction {
            title: "أضف تعليق TODO للكتلة الفارغة".to_string(),
            kind: Some(CodeActionKind::QUICKFIX),
            diagnostics: Some(vec![diagnostic.clone()]),
            edit: Some(WorkspaceEdit {
                changes: Some({
                    let mut map = std::collections::HashMap::new();
                    map.insert(uri.clone(), vec![
                        TextEdit {
                            range: diagnostic.range,
                            new_text: "// TODO: أضف معالجة للخطأ".to_string(),
                        }
                    ]);
                    map
                }),
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
    
    if actions.is_empty() {
        None
    } else {
        Some(actions)
    }
}

/// الحصول على إجراءات إعادة البناء
fn get_refactor_actions(uri: &Url, range: &Range, _analysis: &AnalysisResult) -> Vec<CodeActionOrCommand> {
    let mut actions = Vec::new();
    
    // استخراج دالة
    actions.push(CodeActionOrCommand::CodeAction(CodeAction {
        title: "استخراج كدالة".to_string(),
        kind: Some(CodeActionKind::REFACTOR_EXTRACT),
        command: Some(Command {
            title: "استخراج كدالة".to_string(),
            command: "almarjaa.extractFunction".to_string(),
            arguments: Some(vec![
                serde_json::to_value(uri).unwrap(),
                serde_json::to_value(range).unwrap(),
            ]),
        }),
        ..Default::default()
    }));
    
    // استخراج متغير
    actions.push(CodeActionOrCommand::CodeAction(CodeAction {
        title: "استخراج كمتغير".to_string(),
        kind: Some(CodeActionKind::REFACTOR_EXTRACT),
        command: Some(Command {
            title: "استخراج كمتغير".to_string(),
            command: "almarjaa.extractVariable".to_string(),
            arguments: Some(vec![
                serde_json::to_value(uri).unwrap(),
                serde_json::to_value(range).unwrap(),
            ]),
        }),
        ..Default::default()
    }));
    
    // إضافة توثيق
    actions.push(CodeActionOrCommand::CodeAction(CodeAction {
        title: "أضف توثيق الدالة".to_string(),
        kind: Some(CodeActionKind::REFACTOR_REWRITE),
        edit: Some(WorkspaceEdit {
            changes: Some({
                let mut map = std::collections::HashMap::new();
                map.insert(uri.clone(), vec![
                    TextEdit {
                        range: Range::new(
                            Position::new(range.start.line, 0),
                            Position::new(range.start.line, 0),
                        ),
                        new_text: "// وصف الدالة\n".to_string(),
                    }
                ]);
                map
            }),
            ..Default::default()
        }),
        ..Default::default()
    }));
    
    actions
}

/// الحصول على إجراءات المصدر
fn get_source_actions(uri: &Url) -> Vec<CodeActionOrCommand> {
    vec![
        CodeActionOrCommand::CodeAction(CodeAction {
            title: "تنظيم الاستيرادات".to_string(),
            kind: Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
            command: Some(Command {
                title: "تنظيم الاستيرادات".to_string(),
                command: "almarjaa.organizeImports".to_string(),
                arguments: Some(vec![serde_json::to_value(uri).unwrap()]),
            }),
            ..Default::default()
        }),
        CodeActionOrCommand::CodeAction(CodeAction {
            title: "إصلاح الكل".to_string(),
            kind: Some(CodeActionKind::SOURCE),
            command: Some(Command {
                title: "إصلاح الكل".to_string(),
                command: "almarjaa.fixAll".to_string(),
                arguments: Some(vec![serde_json::to_value(uri).unwrap()]),
            }),
            ..Default::default()
        }),
    ]
}
