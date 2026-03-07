// src/stdlib/crypto/hash.rs
// دوال الهاش
// Hash Functions

use super::{CryptoAlgorithm, HashResult};

/// حساب الهاش
pub fn hash(data: &[u8], algorithm: CryptoAlgorithm) -> HashResult {
    let hash = match algorithm {
        CryptoAlgorithm::Md5 => md5_hash(data),
        CryptoAlgorithm::Sha1 => sha1_hash(data),
        CryptoAlgorithm::Sha256 => sha256_hash(data),
        CryptoAlgorithm::Sha384 => sha384_hash(data),
        CryptoAlgorithm::Sha512 => sha512_hash(data),
        _ => vec![],
    };
    
    HashResult { hash, algorithm }
}

/// حساب الهاش من نص
pub fn hash_string(text: &str, algorithm: CryptoAlgorithm) -> HashResult {
    hash(text.as_bytes(), algorithm)
}

/// MD5 (غير آمن للاستخدام الأمني!)
pub fn md5_hash(data: &[u8]) -> Vec<u8> {
    use std::io::Write;
    let mut hasher = md5::Md5::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// SHA-1 (غير آمن للاستخدام الأمني!)
pub fn sha1_hash(data: &[u8]) -> Vec<u8> {
    use sha1::{Sha1, Digest};
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// SHA-256
pub fn sha256_hash(data: &[u8]) -> Vec<u8> {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// SHA-384
pub fn sha384_hash(data: &[u8]) -> Vec<u8> {
    use sha2::{Sha384, Digest};
    let mut hasher = Sha384::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// SHA-512
pub fn sha512_hash(data: &[u8]) -> Vec<u8> {
    use sha2::{Sha512, Digest};
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// HMAC-SHA256
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    
    type HmacSha256 = Hmac<Sha256>;
    
    let mut mac = HmacSha256::new_from_slice(key)
        .expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

/// HMAC-SHA512
pub fn hmac_sha512(key: &[u8], data: &[u8]) -> Vec<u8> {
    use hmac::{Hmac, Mac};
    use sha2::Sha512;
    
    type HmacSha512 = Hmac<Sha512>;
    
    let mut mac = HmacSha512::new_from_slice(key)
        .expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

/// التحقق من HMAC
pub fn verify_hmac_sha256(key: &[u8], data: &[u8], expected: &[u8]) -> bool {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    
    type HmacSha256 = Hmac<Sha256>;
    
    let mut mac = HmacSha256::new_from_slice(key)
        .expect("HMAC can take key of any size");
    mac.update(data);
    
    mac.verify_slice(expected).is_ok()
}

// ===== دوال عربية =====

/// حساب الهاش
pub fn هاش(text: &str, algorithm: &str) -> HashResult {
    let algo = CryptoAlgorithm::from_arabic(algorithm)
        .unwrap_or(CryptoAlgorithm::Sha256);
    hash_string(text, algo)
}

/// SHA-256
pub fn شا256(text: &str) -> String {
    sha256_hash(text.as_bytes()).iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

/// SHA-512
pub fn شا512(text: &str) -> String {
    sha512_hash(text.as_bytes()).iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

/// MD5 (غير آمن!)
pub fn إم_دي5(text: &str) -> String {
    md5_hash(text.as_bytes()).iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}
