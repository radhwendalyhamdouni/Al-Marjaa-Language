// src/stdlib/crypto/random.rs
// توليد أرقام عشوائية آمنة
// Secure Random Generation

use rand::{Rng, SeedableRng};
use rand::rngs::OsRng;
use rand::seq::SliceRandom;

/// توليد بايتات عشوائية آمنة
pub fn random_bytes(length: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; length];
    OsRng.fill_bytes(&mut bytes);
    bytes
}

/// توليد رقم عشوائي بين قيمتين
pub fn random_range(min: u64, max: u64) -> u64 {
    OsRng.gen_range(min..=max)
}

/// توليد رقم عشوائي
pub fn random_u64() -> u64 {
    OsRng.gen()
}

/// توليد سلسلة عشوائية
pub fn random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = OsRng;
    
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// توليد سلسلة أبجدية رقمية
pub fn random_alphanumeric(length: usize) -> String {
    random_string(length)
}

/// توليد سلسلة سداسية عشرية
pub fn random_hex(length: usize) -> String {
    let bytes = random_bytes(length / 2);
    hex::encode(bytes)
}

/// توليد UUID v4
pub fn random_uuid() -> String {
    let bytes = random_bytes(16);
    
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    )
}

/// اختيار عنصر عشوائي
pub fn random_choice<T: Clone>(items: &[T]) -> Option<T> {
    let mut rng = OsRng;
    items.choose(&mut rng).cloned()
}

/// خلط مصفوفة
pub fn shuffle<T>(items: &mut [T]) {
    let mut rng = OsRng;
    items.shuffle(&mut rng);
}

/// توليد كلمة مرور عشوائية
pub fn random_password(length: usize, include_special: bool) -> String {
    let charset = if include_special {
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:,.<>?"
    } else {
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    };
    
    let mut rng = OsRng;
    
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

/// توليد رمز OTP
pub fn generate_otp(length: usize) -> String {
    let mut rng = OsRng;
    
    (0..length)
        .map(|_| rng.gen_range(0..10).to_string().chars().next().unwrap())
        .collect()
}

// ===== دوال عربية =====

/// بايتات عشوائية
pub fn بايتات_عشوائية(length: usize) -> Vec<u8> {
    random_bytes(length)
}

/// نص عشوائي
pub fn نص_عشوائي(length: usize) -> String {
    random_string(length)
}

/// كلمة مرور عشوائية
pub fn كلمة_مرور_عشوائية(length: usize) -> String {
    random_password(length, true)
}

/// رقم عشوائي
pub fn رقم_عشوائي(min: u64, max: u64) -> u64 {
    random_range(min, max)
}

/// رمز تحقق
pub fn رمز_تحقق(length: usize) -> String {
    generate_otp(length)
}
