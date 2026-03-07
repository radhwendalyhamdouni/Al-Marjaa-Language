// ═══════════════════════════════════════════════════════════════════════════════
// محرك الذكاء الاصطناعي - لغة المرجع
// ═══════════════════════════════════════════════════════════════════════════════

pub mod pipeline;
pub mod inference;
pub mod gguf_inference;
pub mod arabic_nlp;
pub mod real_inference;
pub mod vibe_advanced;
pub mod unified_vibe;

pub use pipeline::engine::{
    PipelineEngine, PipelineResult, Intent,
    run_pipeline, run_example, parse_intent, generate_code,
};

pub use inference::{
    AIEngine, ModelConfig, ModelType, InferenceResult,
    InferenceCache, CacheStats,
    create_engine, text_to_code, text_to_intent_json,
};

pub use gguf_inference::{
    GGUFEngine, GGUFConfig, GGUFResult,
};

pub use arabic_nlp::{
    ArabicNlp, Token, Entity, EntityType,
    ParseResult, SentenceStructure, SentenceType,
};

pub use real_inference::{
    RealAIEngine, RealModelConfig, RealInferenceResult,
    text_to_code_real,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصديرات Vibe Coding المتقدم
// ═══════════════════════════════════════════════════════════════════════════════

pub use vibe_advanced::{
    // المحرك الرئيسي
    VibeCodingEngine, VibeResult, EngineStats,
    // النوايا
    IntentType, DetectedIntent, ConfidenceLevel,
    // التحليل الدلالي
    SemanticAnalysis, SemanticEntity, SemanticEntityType,
    SemanticRelation, RelationType, ComplexityLevel,
    // ملاحظة: SentenceStructure و SentenceType موجودة في arabic_nlp
    // استخدام vibe_advanced::SentenceStructure as VibeSentenceStructure
    // السياق والتعلم
    ExecutionContext, ContextType, UserPattern, LearningType,
    ConversationItem, CodeTemplate,
    // الدوال السهلة
    vibe_process, vibe_detect_intents, vibe_explain_code,
    vibe_suggest_fix, vibe_smart_completion,
};

// ═══════════════════════════════════════════════════════════════════════════════
// تصديرات المحرك الموحد (Vibe + GGUF)
// ═══════════════════════════════════════════════════════════════════════════════

pub use unified_vibe::{
    // المحرك الموحد
    UnifiedVibeEngine, UnifiedVibeResult, UnifiedVibeConfig,
    UnifiedStats, EngineState, EngineType,
    // الدوال السهلة
    unified_vibe_process, unified_text_to_code, unified_process_batch,
};
