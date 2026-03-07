// ═══════════════════════════════════════════════════════════════════════════════
// Pipeline Module - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

pub mod engine;

pub use engine::{
    PipelineEngine, PipelineResult, Intent,
    run_pipeline, run_example, parse_intent, generate_code,
};
