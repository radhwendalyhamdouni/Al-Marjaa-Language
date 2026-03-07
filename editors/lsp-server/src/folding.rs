//! Folding Ranges - نطاقات الطي

use lsp_types::*;

use crate::state::{AnalysisResult, FoldingKind};

/// الحصول على نطاقات الطي
pub fn get_folding_ranges(analysis: &AnalysisResult) -> Vec<FoldingRange> {
    analysis.folding_ranges.iter()
        .map(|fr| FoldingRange {
            start_line: fr.start_line as u32,
            end_line: fr.end_line as u32,
            start_character: None,
            end_character: None,
            kind: Some(match fr.kind {
                FoldingKind::Function => FoldingRangeKind::Region,
                FoldingKind::Class => FoldingRangeKind::Region,
                FoldingKind::Block => FoldingRangeKind::Region,
                FoldingKind::Comment => FoldingRangeKind::Comment,
                FoldingKind::Region => FoldingRangeKind::Region,
            }),
            collapsed_text: None,
        })
        .collect()
}
