// src/stdlib/crypto/mod.rs
// وحدة التشفير الشاملة
// Comprehensive Cryptography Module

pub mod hash;
pub mod symmetric;
pub mod asymmetric;
pub mod random;
pub mod jwt;
pub mod password;

pub use hash::*;
pub use symmetric::*;
pub use asymmetric::*;
pub use random::*;
pub use jwt::*;
pub use password::*;

use std::collections::HashMap;

/// خوارزمية التشفير
#[derive(Debug, Clone, PartialEq)]
pub enum CryptoAlgorithm {
    // Hash
    Md5,
    Sha1,
    Sha256,
    Sha384,
    Sha512,
    Blake2b,
    Blake2s,
    
    // Symmetric
    Aes128Gcm,
    Aes256Gcm,
    Aes128Cbc,
    Aes256Cbc,
    ChaCha20Poly1305,
    
    // Asymmetric
    Rsa2048,
    Rsa4096,
    Ed25519,
    
    // Password Hashing
    Bcrypt,
    Scrypt,
    Pbkdf2,
    
    // HMAC
    HmacSha256,
    HmacSha512,
}

impl CryptoAlgorithm {
    /// من الاسم العربي
    pub fn from_arabic(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "md5" | "إم دي 5" => Some(Self::Md5),
            "sha1" | "شا 1" => Some(Self::Sha1),
            "sha256" | "شا 256" => Some(Self::Sha256),
            "sha384" | "شا 384" => Some(Self::Sha384),
            "sha512" | "شا 512" => Some(Self::Sha512),
            "aes" | "إيه إي إس" => Some(Self::Aes256Gcm),
            "rsa" | "آر إس إيه" => Some(Self::Rsa2048),
            "bcrypt" | "بي كريبت" => Some(Self::Bcrypt),
            _ => None,
        }
    }
    
    /// طول المفتاح بالبايت
    pub fn key_length(&self) -> usize {
        match self {
            Self::Aes128Gcm | Self::Aes128Cbc => 16,
            Self::Aes256Gcm | Self::Aes256Cbc => 32,
            Self::ChaCha20Poly1305 => 32,
            Self::Rsa2048 => 256,
            Self::Rsa4096 => 512,
            Self::Ed25519 => 32,
            _ => 0,
        }
    }
    
    /// طول التهيئة (IV/Nonce)
    pub fn iv_length(&self) -> usize {
        match self {
            Self::Aes128Gcm | Self::Aes256Gcm => 12,
            Self::Aes128Cbc | Self::Aes256Cbc => 16,
            Self::ChaCha20Poly1305 => 12,
            _ => 0,
        }
    }
}

/// نتيجة التشفير
#[derive(Debug, Clone)]
pub struct EncryptionResult {
    /// النص المشفر
    pub ciphertext: Vec<u8>,
    /// متجه التهيئة
    pub iv: Vec<u8>,
    /// العلامة (لـ GCM)
    pub tag: Option<Vec<u8>>,
    /// الخوارزمية المستخدمة
    pub algorithm: CryptoAlgorithm,
}

impl EncryptionResult {
    /// تحويل إلى Base64
    pub fn to_base64(&self) -> String {
        base64::encode(&self.ciphertext)
    }
    
    /// تحويل إلى سداسي عشري
    pub fn to_hex(&self) -> String {
        hex::encode(&self.ciphertext)
    }
}

/// نتيجة الهاش
#[derive(Debug, Clone)]
pub struct HashResult {
    /// الهاش
    pub hash: Vec<u8>,
    /// الخوارزمية
    pub algorithm: CryptoAlgorithm,
}

impl HashResult {
    /// تحويل إلى سداسي عشري
    pub fn to_hex(&self) -> String {
        hex::encode(&self.hash)
    }
    
    /// تحويل إلى Base64
    pub fn to_base64(&self) -> String {
        base64::encode(&self.hash)
    }
    
    /// طول الهاش
    pub fn len(&self) -> usize {
        self.hash.len()
    }
    
    /// هل فارغ؟
    pub fn is_empty(&self) -> bool {
        self.hash.is_empty()
    }
}

/// مفتاح
#[derive(Debug, Clone)]
pub struct Key {
    /// البيانات
    pub data: Vec<u8>,
    /// الخوارزمية
    pub algorithm: CryptoAlgorithm,
}

impl Key {
    /// إنشاء مفتاح جديد
    pub fn new(data: Vec<u8>, algorithm: CryptoAlgorithm) -> Self {
        Self { data, algorithm }
    }
    
    /// إنشاء من Base64
    pub fn from_base64(encoded: &str, algorithm: CryptoAlgorithm) -> Result<Self, String> {
        let data = base64::decode(encoded)
            .map_err(|e| format!("خطأ في فك ترميز Base64: {}", e))?;
        Ok(Self { data, algorithm })
    }
    
    /// تحويل إلى Base64
    pub fn to_base64(&self) -> String {
        base64::encode(&self.data)
    }
    
    /// الطول
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// هل فارغ؟
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// زوج المفاتيح
#[derive(Debug, Clone)]
pub struct KeyPair {
    /// المفتاح العام
    pub public_key: Key,
    /// المفتاح الخاص
    pub private_key: Key,
}

impl KeyPair {
    /// إنشاء زوج جديد
    pub fn new(public: Key, private: Key) -> Self {
        Self {
            public_key: public,
            private_key: private,
        }
    }
}
