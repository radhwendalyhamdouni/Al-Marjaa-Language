//! ═══════════════════════════════════════════════════════════════════════════════
//! Semantic Tokens - الرموز الدلالية
//! ═══════════════════════════════════════════════════════════════════════════════

use lsp_types::SemanticTokens;

use crate::state::SemanticTokenInfo;

/// بناء Semantic Tokens من المعلومات
pub fn build_semantic_tokens(tokens: &[SemanticTokenInfo]) -> SemanticTokens {
    let mut builder = SemanticTokensBuilder::new();
    
    let mut prev_line = 0;
    let mut prev_char = 0;
    
    for token in tokens {
        // حساب الفرق للترميز
        let delta_line = token.line as u32 - prev_line;
        let delta_start = if delta_line == 0 {
            token.column as u32 - prev_char
        } else {
            token.column as u32
        };
        
        builder.push(
            delta_line,
            delta_start,
            token.length as u32,
            token.token_type,
            token.token_modifiers,
        );
        
        prev_line = token.line as u32;
        prev_char = token.column as u32;
    }
    
    builder.build()
}

/// بناء Semantic Tokens
struct SemanticTokensBuilder {
    data: Vec<u32>,
}

impl SemanticTokensBuilder {
    fn new() -> Self {
        Self { data: Vec::new() }
    }
    
    fn push(&mut self, delta_line: u32, delta_start: u32, length: u32, token_type: u32, token_modifiers: u32) {
        self.data.push(delta_line);
        self.data.push(delta_start);
        self.data.push(length);
        self.data.push(token_type);
        self.data.push(token_modifiers);
    }
    
    fn build(self) -> SemanticTokens {
        SemanticTokens {
            result_id: None,
            data: Some(self.data),
        }
    }
}
