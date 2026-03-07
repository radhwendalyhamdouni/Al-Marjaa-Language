// ═══════════════════════════════════════════════════════════════════════════════
// نظام توقيع الحزم - لغة المرجع
// Package Signing System
// ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};

/// مفتاح التوقيع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningKey {
    /// معرف المفتاح
    pub key_id: String,
    /// المفتاح العام (Base64)
    pub public_key: String,
    /// المفتاح الخاص (Base64, مشفر)
    pub private_key: Option<String>,
    /// تاريخ الإنشاء
    pub created_at: u64,
    /// تاريخ الانتهاء
    pub expires_at: Option<u64>,
    /// المالك
    pub owner: String,
    /// البريد الإلكتروني
    pub email: String,
}

impl SigningKey {
    /// إنشاء مفتاح جديد
    pub fn new(owner: &str, email: &str) -> Self {
        use rand::Rng;
        
        let key_id: String = (0..16)
            .map(|_| rand::thread_rng().gen_range(0..16).to_string())
            .collect();
        
        let public_key = base64::encode(&rand::thread_rng().gen::<[u8; 32]>());
        let private_key = Some(base64::encode(&rand::thread_rng().gen::<[u8; 64]>()));
        
        Self {
            key_id,
            public_key,
            private_key,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            expires_at: None,
            owner: owner.to_string(),
            email: email.to_string(),
        }
    }
    
    /// الحصول على معرف مختصر
    pub fn short_id(&self) -> &str {
        &self.key_id[..8]
    }
    
    /// هل منتهي الصلاحية؟
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            return now > expires;
        }
        false
    }
}

/// توقيع حزمة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSignature {
    /// معرف الحزمة
    pub package_id: String,
    /// الإصدار
    pub version: String,
    /// التوقيع (Base64)
    pub signature: String,
    /// معرف المفتاح
    pub key_id: String,
    /// تاريخ التوقيع
    pub signed_at: u64,
    /// التجزئة الموقّعة
    pub content_hash: String,
    /// خوارزمية التوقيع
    pub algorithm: SignatureAlgorithm,
}

/// خوارزمية التوقيع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignatureAlgorithm {
    Ed25519,
    RSA2048,
    RSA4096,
}

impl std::fmt::Display for SignatureAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ed25519 => write!(f, "Ed25519"),
            Self::RSA2048 => write!(f, "RSA-2048"),
            Self::RSA4096 => write!(f, "RSA-4096"),
        }
    }
}

impl PackageSignature {
    /// إنشاء توقيع جديد
    pub fn new(package_id: &str, version: &str, key: &SigningKey, content: &[u8]) -> Self {
        let content_hash = Self::hash_content(content);
        
        // في التنفيذ الحقيقي، نستخدم Ed25519 أو RSA
        // هذا توقيع وهمي للعرض
        let signature = base64::encode(&format!("{}:{}", key.key_id, content_hash));
        
        Self {
            package_id: package_id.to_string(),
            version: version.to_string(),
            signature,
            key_id: key.key_id.clone(),
            signed_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            content_hash,
            algorithm: SignatureAlgorithm::Ed25519,
        }
    }
    
    /// حساب تجزئة المحتوى
    fn hash_content(content: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }
    
    /// التحقق من التوقيع
    pub fn verify(&self, key: &SigningKey, content: &[u8]) -> bool {
        // التحقق من معرف المفتاح
        if self.key_id != key.key_id {
            return false;
        }
        
        // التحقق من التجزئة
        let expected_hash = Self::hash_content(content);
        if self.content_hash != expected_hash {
            return false;
        }
        
        // في التنفيذ الحقيقي، نستخدم التحقق بالتشفير
        // هذا تنفيذ مبسط
        
        let expected_sig = base64::encode(&format!("{}:{}", key.key_id, self.content_hash));
        self.signature == expected_sig
    }
}

/// سجل المفاتيح
#[derive(Debug, Clone, Default)]
pub struct KeyRegistry {
    /// المفاتيح المعروفة
    keys: HashMap<String, SigningKey>,
    /// المفاتيح الموثوقة
    trusted_keys: Vec<String>,
}

impl KeyRegistry {
    /// إنشاء سجل جديد
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            trusted_keys: Vec::new(),
        }
    }
    
    /// إضافة مفتاح
    pub fn add_key(&mut self, key: SigningKey) {
        self.keys.insert(key.key_id.clone(), key);
    }
    
    /// إضافة مفتاح موثوق
    pub fn trust_key(&mut self, key_id: &str) -> bool {
        if self.keys.contains_key(key_id) {
            if !self.trusted_keys.contains(&key_id.to_string()) {
                self.trusted_keys.push(key_id.to_string());
            }
            true
        } else {
            false
        }
    }
    
    /// إزالة ثقة
    pub fn untrust_key(&mut self, key_id: &str) {
        self.trusted_keys.retain(|k| k != key_id);
    }
    
    /// الحصول على مفتاح
    pub fn get_key(&self, key_id: &str) -> Option<&SigningKey> {
        self.keys.get(key_id)
    }
    
    /// هل المفتاح موثوق؟
    pub fn is_trusted(&self, key_id: &str) -> bool {
        self.trusted_keys.contains(&key_id.to_string())
    }
    
    /// قائمة المفاتيح الموثوقة
    pub fn trusted_keys(&self) -> Vec<&SigningKey> {
        self.trusted_keys
            .iter()
            .filter_map(|id| self.keys.get(id))
            .collect()
    }
    
    /// حفظ السجل
    pub fn save(&self, path: &Path) -> Result<(), String> {
        let content = serde_json::to_string_pretty(&self.keys)
            .map_err(|e| format!("فشل تحويل المفاتيح: {}", e))?;
        
        std::fs::write(path, content)
            .map_err(|e| format!("فشل حفظ السجل: {}", e))
    }
    
    /// تحميل السجل
    pub fn load(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self::new());
        }
        
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("فشل قراءة السجل: {}", e))?;
        
        let keys: HashMap<String, SigningKey> = serde_json::from_str(&content)
            .map_err(|e| format!("فشل تحليل السجل: {}", e))?;
        
        Ok(Self {
            keys,
            trusted_keys: Vec::new(),
        })
    }
}

/// مدقق التوقيعات
pub struct SignatureVerifier {
    /// سجل المفاتيح
    registry: KeyRegistry,
    /// مستوى الصرامة
    strict_mode: bool,
}

impl SignatureVerifier {
    /// إنشاء مدقق جديد
    pub fn new() -> Self {
        Self {
            registry: KeyRegistry::new(),
            strict_mode: true,
        }
    }
    
    /// استخدام سجل مفاتيح
    pub fn with_registry(mut self, registry: KeyRegistry) -> Self {
        self.registry = registry;
        self
    }
    
    /// تعيين مستوى الصرامة
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }
    
    /// التحقق من توقيع
    pub fn verify(&self, signature: &PackageSignature, content: &[u8]) -> SignatureResult {
        // البحث عن المفتاح
        let key = match self.registry.get_key(&signature.key_id) {
            Some(k) => k,
            None => {
                return SignatureResult {
                    valid: false,
                    message: format!("المفتاح {} غير معروف", signature.key_id),
                    trusted: false,
                    key_owner: None,
                };
            }
        };
        
        // التحقق من انتهاء الصلاحية
        if key.is_expired() {
            return SignatureResult {
                valid: false,
                message: "المفتاح منتهي الصلاحية".to_string(),
                trusted: false,
                key_owner: Some(key.owner.clone()),
            };
        }
        
        // التحقق من التوقيع
        if !signature.verify(key, content) {
            return SignatureResult {
                valid: false,
                message: "التوقيع غير صالح".to_string(),
                trusted: false,
                key_owner: Some(key.owner.clone()),
            };
        }
        
        // التحقق من الثقة
        let trusted = self.registry.is_trusted(&signature.key_id);
        
        SignatureResult {
            valid: true,
            message: if trusted { "التوقيع صالح وموثوق" } else { "التوقيع صالح لكن غير موثوق" }.to_string(),
            trusted,
            key_owner: Some(key.owner.clone()),
        }
    }
    
    /// التحقق من حزمة
    pub fn verify_package(
        &self,
        signature: &PackageSignature,
        content: &[u8],
    ) -> Result<(), String> {
        let result = self.verify(signature, content);
        
        if !result.valid {
            return Err(result.message);
        }
        
        if self.strict_mode && !result.trusted {
            return Err("الحزمة موقعة بمفتاح غير موثوق".to_string());
        }
        
        Ok(())
    }
    
    /// إضافة مفتاح موثوق
    pub fn trust_key(&mut self, key_id: &str) -> bool {
        self.registry.trust_key(key_id)
    }
    
    /// إضافة مفتاح
    pub fn add_key(&mut self, key: SigningKey) {
        self.registry.add_key(key);
    }
}

impl Default for SignatureVerifier {
    fn default() -> Self {
        Self::new()
    }
}

/// نتيجة التحقق من التوقيع
#[derive(Debug, Clone)]
pub struct SignatureResult {
    /// هل التوقيع صالح؟
    pub valid: bool,
    /// الرسالة
    pub message: String,
    /// هل المفتاح موثوق؟
    pub trusted: bool,
    /// مالك المفتاح
    pub key_owner: Option<String>,
}

/// مُوقّع الحزم
pub struct PackageSigner {
    /// المفتاح
    key: SigningKey,
}

impl PackageSigner {
    /// إنشاء مُوقّع جديد
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }
    
    /// توقيع حزمة
    pub fn sign(&self, package_id: &str, version: &str, content: &[u8]) -> PackageSignature {
        PackageSignature::new(package_id, version, &self.key, content)
    }
    
    /// توقيع ملف
    pub fn sign_file(&self, package_id: &str, version: &str, path: &Path) -> Result<PackageSignature, String> {
        let content = std::fs::read(path)
            .map_err(|e| format!("فشل قراءة الملف: {}", e))?;
        
        Ok(self.sign(package_id, version, &content))
    }
    
    /// الحصول على المفتاح
    pub fn key(&self) -> &SigningKey {
        &self.key
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// دوال عربية
// ═══════════════════════════════════════════════════════════════════════════════

/// إنشاء مفتاح توقيع
pub fn أنشئ_مفتاح(المالك: &str, البريد: &str) -> SigningKey {
    SigningKey::new(المالك, البريد)
}

/// توقيع حزمة
pub fn وقّع_حزمة(الحزمة: &str, الإصدار: &str, المفتاح: &SigningKey, المحتوى: &[u8]) -> PackageSignature {
    PackageSignature::new(الحزمة, الإصدار, المفتاح, المحتوى)
}

/// التحقق من التوقيع
pub fn تحقق_من_التوقيع(التوقيع: &PackageSignature, المفتاح: &SigningKey, المحتوى: &[u8]) -> bool {
    التوقيع.verify(المفتاح, المحتوى)
}

// ═══════════════════════════════════════════════════════════════════════════════
// اختبارات
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_signing_key_creation() {
        let key = SigningKey::new("أحمد", "ahmed@example.com");
        
        assert_eq!(key.owner, "أحمد");
        assert_eq!(key.email, "ahmed@example.com");
        assert!(!key.is_expired());
    }
    
    #[test]
    fn test_package_signature() {
        let key = SigningKey::new("أحمد", "ahmed@example.com");
        let content = b"محتوى الحزمة";
        
        let sig = PackageSignature::new("my-package", "1.0.0", &key, content);
        
        assert_eq!(sig.package_id, "my-package");
        assert_eq!(sig.version, "1.0.0");
        assert_eq!(sig.key_id, key.key_id);
    }
    
    #[test]
    fn test_signature_verification() {
        let key = SigningKey::new("أحمد", "ahmed@example.com");
        let content = b"محتوى الحزمة";
        
        let sig = PackageSignature::new("my-package", "1.0.0", &key, content);
        
        assert!(sig.verify(&key, content));
        assert!(!sig.verify(&key, b"محتوى مختلف"));
    }
    
    #[test]
    fn test_key_registry() {
        let mut registry = KeyRegistry::new();
        let key = SigningKey::new("أحمد", "ahmed@example.com");
        let key_id = key.key_id.clone();
        
        registry.add_key(key);
        registry.trust_key(&key_id);
        
        assert!(registry.is_trusted(&key_id));
    }
    
    #[test]
    fn test_signature_verifier() {
        let key = SigningKey::new("أحمد", "ahmed@example.com");
        let content = b"محتوى الحزمة";
        
        let sig = PackageSignature::new("my-package", "1.0.0", &key, content);
        
        let mut registry = KeyRegistry::new();
        registry.add_key(key);
        
        let verifier = SignatureVerifier::new()
            .with_registry(registry)
            .strict(false);
        
        let result = verifier.verify(&sig, content);
        
        assert!(result.valid);
    }
}
