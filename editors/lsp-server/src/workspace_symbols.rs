//! Workspace Symbols - رموز مساحة العمل

use std::sync::Arc;
use lsp_types::*;

use crate::state::ServerState;

/// البحث في رموز مساحة العمل
pub fn search_workspace_symbols(state: &Arc<ServerState>, query: &str) -> Vec<SymbolInformation> {
    let mut symbols = Vec::new();
    let query_lower = query.to_lowercase();
    
    // البحث في جميع الملفات المفتوحة
    for entry in state.analysis_cache.iter() {
        let uri = entry.key();
        let analysis = entry.value();
        
        for (name, def) in &analysis.definitions {
            if name.to_lowercase().contains(&query_lower) {
                symbols.push(SymbolInformation {
                    name: name.clone(),
                    kind: def.kind.to_symbol_kind(),
                    tags: None,
                    deprecated: None,
                    location: Location {
                        uri: uri.clone(),
                        range: Range::new(
                            Position::new(def.line.saturating_sub(1) as u32, def.column as u32),
                            Position::new(def.line.saturating_sub(1) as u32, def.end_column as u32),
                        ),
                    },
                    container_name: None,
                });
            }
        }
    }
    
    // ترتيب حسب الاسم
    symbols.sort_by(|a, b| a.name.cmp(&b.name));
    
    symbols
}
