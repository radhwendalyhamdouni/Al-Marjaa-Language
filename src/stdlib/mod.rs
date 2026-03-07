// src/stdlib/mod.rs
// المكتبة القياسية المتقدمة للغة المرجع
// Advanced Standard Library for Al-Marjaa Language

pub mod http;
pub mod database;
pub mod regex;
pub mod crypto;

pub use http::*;
pub use database::*;
pub use regex::*;
pub use crypto::*;

/// معلومات المكتبة القياسية
pub fn stdlib_info() -> &'static str {
    "المكتبة القياسية للغة المرجع v3.2.0 - HTTP, Database, Regex, Crypto"
}
