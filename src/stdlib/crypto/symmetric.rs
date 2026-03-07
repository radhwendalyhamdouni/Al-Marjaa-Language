// src/stdlib/crypto/symmetric.rs
// التشفير المتماثل (AES, ChaCha20)
// Symmetric Encryption

use super::{CryptoAlgorithm, EncryptionResult, Key};

/// تشفير AES-GCM
pub fn aes_gcm_encrypt(plaintext: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, String> {
    // تحقق من طول المفتاح
    if key.len() != 16 && key.len() != 32 {
        return Err("المفتاح يجب أن يكون 16 أو 32 بايت".to_string());
    }
    
    // تحقق من طول الـ nonce
    if nonce.len() != 12 {
        return Err("Nonce يجب أن يكون 12 بايت".to_string());
    }
    
    // في التنفيذ الحقيقي، نستخدم aes-gcm crate
    // هذا تنفيذ مبسط للعرض
    let mut ciphertext = plaintext.to_vec();
    
    // XOR بسيط للعرض (غير آمن!)
    for (i, byte) in ciphertext.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];
    }
    
    Ok(ciphertext)
}

/// فك تشفير AES-GCM
pub fn aes_gcm_decrypt(ciphertext: &[u8], key: &[u8], nonce: &[u8]) -> Result<Vec<u8>, String> {
    // نفس التشفير في XOR
    aes_gcm_encrypt(ciphertext, key, nonce)
}

/// تشفير AES-CBC
pub fn aes_cbc_encrypt(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
    if iv.len() != 16 {
        return Err("IV يجب أن يكون 16 بايت".to_string());
    }
    
    // Padding PKCS7
    let padding_len = 16 - (plaintext.len() % 16);
    let mut padded = plaintext.to_vec();
    for _ in 0..padding_len {
        padded.push(padding_len as u8);
    }
    
    // XOR بسيط للعرض
    let mut ciphertext = padded;
    for (i, byte) in ciphertext.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];
    }
    
    Ok(ciphertext)
}

/// فك تشفير AES-CBC
pub fn aes_cbc_decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
    // XOR عكسي
    let mut plaintext = ciphertext.to_vec();
    for (i, byte) in plaintext.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];
    }
    
    // إزالة Padding
    if !plaintext.is_empty() {
        let padding_len = plaintext[plaintext.len() - 1] as usize;
        if padding_len <= 16 && padding_len <= plaintext.len() {
            plaintext.truncate(plaintext.len() - padding_len);
        }
    }
    
    Ok(plaintext)
}

/// تشفير متماثل عام
pub fn encrypt(plaintext: &[u8], key: &Key) -> Result<EncryptionResult, String> {
    let iv = super::random::random_bytes(key.algorithm.iv_length());
    
    let ciphertext = match key.algorithm {
        CryptoAlgorithm::Aes128Gcm | CryptoAlgorithm::Aes256Gcm => {
            aes_gcm_encrypt(plaintext, &key.data, &iv)?
        }
        CryptoAlgorithm::Aes128Cbc | CryptoAlgorithm::Aes256Cbc => {
            aes_cbc_encrypt(plaintext, &key.data, &iv)?
        }
        _ => return Err("خوارزمية غير مدعومة".to_string()),
    };
    
    Ok(EncryptionResult {
        ciphertext,
        iv,
        tag: None,
        algorithm: key.algorithm.clone(),
    })
}

/// فك تشفير متماثل عام
pub fn decrypt(encrypted: &EncryptionResult, key: &Key) -> Result<Vec<u8>, String> {
    match key.algorithm {
        CryptoAlgorithm::Aes128Gcm | CryptoAlgorithm::Aes256Gcm => {
            aes_gcm_decrypt(&encrypted.ciphertext, &key.data, &encrypted.iv)
        }
        CryptoAlgorithm::Aes128Cbc | CryptoAlgorithm::Aes256Cbc => {
            aes_cbc_decrypt(&encrypted.ciphertext, &key.data, &encrypted.iv)
        }
        _ => Err("خوارزمية غير مدعومة".to_string()),
    }
}

// ===== دوال عربية =====

/// تشفير
pub fn شفر(plaintext: &[u8], key: &Key) -> Result<EncryptionResult, String> {
    encrypt(plaintext, key)
}

/// فك تشفير
pub fn فك_التشفير(encrypted: &EncryptionResult, key: &Key) -> Result<Vec<u8>, String> {
    decrypt(encrypted, key)
}
