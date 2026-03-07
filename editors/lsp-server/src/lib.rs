//! ═══════════════════════════════════════════════════════════════════════════════
//! Al-Marjaa LSP Library
//! ═══════════════════════════════════════════════════════════════════════════════

pub mod transport;
pub mod server;
pub mod handlers;
pub mod state;
pub mod capabilities;
pub mod semantic_tokens;
pub mod code_actions;
pub mod code_lens;
pub mod inlay_hints;
pub mod call_hierarchy;
pub mod formatting;
pub mod folding;
pub mod rename;
pub mod signature_help;
pub mod workspace_symbols;
pub mod diagnostics;
pub mod arabic_support;
pub mod type_inference;
pub mod cache;

pub use crate::state::{ServerState, ServerSettings, ServerStats};
pub use crate::handlers::RequestHandler;
pub use crate::server::LspServer;
pub use crate::transport::Transport;
