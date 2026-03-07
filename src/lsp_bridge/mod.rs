use crate::lexer::{tokens::TokenType, Lexer};
use crate::linter::{lint_source_with_config, LintConfig};
use crate::parser::Parser;
use serde::Serialize;
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, Serialize)]
pub struct JsonDiagnostic {
    pub source: String,
    pub severity: String,
    pub line: usize,
    pub column: usize,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HoverInfo {
    pub symbol: String,
    pub message: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DefinitionInfo {
    pub symbol: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReferenceInfo {
    pub symbol: String,
    pub line: usize,
    pub column: usize,
    pub is_definition: bool,
}

pub fn build_diagnostics_json(content: &str) -> Vec<JsonDiagnostic> {
    let mut diagnostics = Vec::new();

    if let Err(err) = Parser::parse(content) {
        diagnostics.push(JsonDiagnostic {
            source: "parser".to_string(),
            severity: "error".to_string(),
            line: err.line,
            column: err.column,
            code: "E200".to_string(),
            message: err.message,
        });
    }

    if let Ok(lints) = lint_source_with_config(content, &LintConfig::default()) {
        for lint in lints {
            diagnostics.push(JsonDiagnostic {
                source: "linter".to_string(),
                severity: "warning".to_string(),
                line: 1,
                column: 1,
                code: lint.code.to_string(),
                message: lint.message,
            });
        }
    }

    diagnostics
}

pub fn build_completions(
    content: &str,
    prefix: Option<&str>,
) -> Result<Vec<CompletionItem>, String> {
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize()?;

    let mut items = BTreeSet::new();
    for kw in [
        "متغير",
        "ثابت",
        "دالة",
        "إذا",
        "وإلا",
        "طالما",
        "لكل",
        "ارجع",
        "اطبع",
        "تأكد",
    ] {
        items.insert((kw.to_string(), "keyword".to_string()));
    }

    for token in tokens {
        if let TokenType::Identifier(name) = token.token_type {
            items.insert((name, "identifier".to_string()));
        }
    }

    let filter = prefix.unwrap_or("");
    let mut out: Vec<CompletionItem> = items
        .into_iter()
        .filter(|(label, _)| matches_completion_prefix(label, filter))
        .map(|(label, kind)| CompletionItem { label, kind })
        .collect();
    out.sort_by(|a, b| a.label.cmp(&b.label));
    Ok(out)
}

fn matches_completion_prefix(label: &str, prefix: &str) -> bool {
    if prefix.is_empty() {
        return true;
    }

    label.starts_with(prefix)
        || label
            .strip_prefix("ال")
            .is_some_and(|stripped| stripped.starts_with(prefix))
}

pub fn find_hover(content: &str, line: usize, column: usize) -> Result<Option<HoverInfo>, String> {
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize()?;

    let symbol_table = collect_definitions(content)?;

    for token in tokens {
        if token.line == line
            && token.column <= column
            && matches!(token.token_type, TokenType::Identifier(_))
        {
            if let TokenType::Identifier(name) = token.token_type {
                let message = if let Some((def_line, def_col)) = symbol_table.get(&name) {
                    format!(
                        "المعرف '{}' مُعرّف عند السطر {}، العمود {}",
                        name, def_line, def_col
                    )
                } else {
                    format!("المعرف '{}' مستخدم في هذا الموضع", name)
                };

                return Ok(Some(HoverInfo {
                    symbol: name,
                    message,
                    line: token.line,
                    column: token.column,
                }));
            }
        }
    }

    Ok(None)
}

pub fn find_definition(
    content: &str,
    line: usize,
    column: usize,
) -> Result<Option<DefinitionInfo>, String> {
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize()?;
    let definitions = collect_definitions(content)?;

    for token in tokens {
        if token.line == line
            && token.column <= column
            && matches!(token.token_type, TokenType::Identifier(_))
        {
            if let TokenType::Identifier(name) = token.token_type {
                if let Some((def_line, def_col)) = definitions.get(&name) {
                    return Ok(Some(DefinitionInfo {
                        symbol: name,
                        line: *def_line,
                        column: *def_col,
                    }));
                }
            }
        }
    }

    Ok(None)
}

pub fn find_references(
    content: &str,
    line: usize,
    column: usize,
) -> Result<Vec<ReferenceInfo>, String> {
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize()?;
    let definitions = collect_definitions(content)?;

    let mut selected_symbol: Option<String> = None;
    for token in &tokens {
        if token.line == line
            && token.column <= column
            && matches!(token.token_type, TokenType::Identifier(_))
        {
            if let TokenType::Identifier(name) = &token.token_type {
                selected_symbol = Some(name.clone());
                break;
            }
        }
    }

    let symbol = match selected_symbol {
        Some(name) => name,
        None => return Ok(Vec::new()),
    };

    let def_pos = definitions.get(&symbol).copied();
    let mut refs = Vec::new();
    for token in tokens {
        if let TokenType::Identifier(name) = token.token_type {
            if name == symbol {
                refs.push(ReferenceInfo {
                    symbol: name,
                    line: token.line,
                    column: token.column,
                    is_definition: def_pos == Some((token.line, token.column)),
                });
            }
        }
    }

    Ok(refs)
}

fn collect_definitions(content: &str) -> Result<HashMap<String, (usize, usize)>, String> {
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize()?;
    let mut defs = HashMap::new();

    let mut idx = 0;
    while idx + 1 < tokens.len() {
        let token = &tokens[idx];
        let next = &tokens[idx + 1];

        let is_decl = matches!(
            token.token_type,
            TokenType::Let | TokenType::Const | TokenType::Function
        );

        if is_decl {
            if let TokenType::Identifier(name) = &next.token_type {
                defs.entry(name.clone()).or_insert((next.line, next.column));
            }
        }

        idx += 1;
    }

    Ok(defs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completion_returns_keywords_and_symbols() {
        let src = "متغير المستخدم = ١؛\n";
        let items = build_completions(src, Some("م")).expect("completion should work");
        assert!(items.iter().any(|i| i.label == "متغير"));
        assert!(items.iter().any(|i| i.label == "المستخدم"));
    }

    #[test]
    fn definition_finds_variable() {
        let src = "متغير س = ١؛\nاطبع(س)؛\n";
        let def = find_definition(src, 2, 6)
            .expect("definition should work")
            .expect("definition should exist");
        assert_eq!(def.symbol, "س");
        assert_eq!(def.line, 1);
    }

    #[test]
    fn references_include_definition_and_usages() {
        let src = "متغير س = ١؛\nاطبع(س)؛\nس = س + ١؛\n";
        let refs = find_references(src, 2, 6).expect("references should work");

        assert_eq!(refs.len(), 4);
        assert!(refs.iter().any(|r| r.is_definition && r.line == 1));
        assert!(refs.iter().any(|r| !r.is_definition && r.line == 2));
        assert_eq!(refs.iter().filter(|r| !r.is_definition).count(), 3);
    }
}
