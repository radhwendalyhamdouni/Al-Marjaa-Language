// src/stdlib/crypto/jwt.rs
// JWT Tokens
// JSON Web Tokens

use super::{CryptoAlgorithm, Key};
use std::collections::HashMap;

/// JWT Header
#[derive(Debug, Clone)]
pub struct JwtHeader {
    pub alg: String,
    pub typ: String,
    pub kid: Option<String>,
}

impl Default for JwtHeader {
    fn default() -> Self {
        Self {
            alg: "HS256".to_string(),
            typ: "JWT".to_string(),
            kid: None,
        }
    }
}

/// JWT Payload
#[derive(Debug, Clone, Default)]
pub struct JwtPayload {
    pub claims: HashMap<String, serde_json::Value>,
}

impl JwtPayload {
    pub fn new() -> Self {
        Self {
            claims: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, key: &str, value: serde_json::Value) {
        self.claims.insert(key.to_string(), value);
    }
    
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.claims.get(key)
    }
    
    pub fn iss(&self) -> Option<&str> {
        self.get("iss").and_then(|v| v.as_str())
    }
    
    pub fn sub(&self) -> Option<&str> {
        self.get("sub").and_then(|v| v.as_str())
    }
    
    pub fn aud(&self) -> Option<&str> {
        self.get("aud").and_then(|v| v.as_str())
    }
    
    pub fn exp(&self) -> Option<i64> {
        self.get("exp").and_then(|v| v.as_i64())
    }
    
    pub fn nbf(&self) -> Option<i64> {
        self.get("nbf").and_then(|v| v.as_i64())
    }
    
    pub fn iat(&self) -> Option<i64> {
        self.get("iat").and_then(|v| v.as_i64())
    }
    
    pub fn jti(&self) -> Option<&str> {
        self.get("jti").and_then(|v| v.as_str())
    }
}

/// JWT Token
#[derive(Debug, Clone)]
pub struct JwtToken {
    pub header: JwtHeader,
    pub payload: JwtPayload,
    pub signature: Vec<u8>,
}

impl JwtToken {
    /// إنشاء JWT جديد
    pub fn new(payload: JwtPayload) -> Self {
        Self {
            header: JwtHeader::default(),
            payload,
            signature: Vec::new(),
        }
    }
    
    /// توقيع JWT
    pub fn sign(&mut self, secret: &[u8]) -> String {
        let header_json = serde_json::to_string(&serde_json::json!({
            "alg": self.header.alg,
            "typ": self.header.typ
        })).unwrap_or_default();
        
        let payload_json = serde_json::to_string(&serde_json::to_value(&self.payload.claims).unwrap_or_default()).unwrap_or_default();
        
        let header_b64 = base64::encode_config(header_json, base64::URL_SAFE_NO_PAD);
        let payload_b64 = base64::encode_config(payload_json, base64::URL_SAFE_NO_PAD);
        
        let message = format!("{}.{}", header_b64, payload_b64);
        
        self.signature = super::hash::hmac_sha256(secret, message.as_bytes());
        
        let signature_b64 = base64::encode_config(&self.signature, base64::URL_SAFE_NO_PAD);
        
        format!("{}.{}.{}", header_b64, payload_b64, signature_b64)
    }
}

/// تحليل JWT
pub fn parse_jwt(token: &str) -> Result<JwtToken, String> {
    let parts: Vec<&str> = token.split('.').collect();
    
    if parts.len() != 3 {
        return Err("صيغة JWT غير صالحة".to_string());
    }
    
    let header_json = base64::decode_config(parts[0], base64::URL_SAFE_NO_PAD)
        .map_err(|e| format!("خطأ في فك ترميز الرأس: {}", e))?;
    
    let payload_json = base64::decode_config(parts[1], base64::URL_SAFE_NO_PAD)
        .map_err(|e| format!("خطأ في فك ترميز المحتوى: {}", e))?;
    
    let signature = base64::decode_config(parts[2], base64::URL_SAFE_NO_PAD)
        .map_err(|e| format!("خطأ في فك ترميز التوقيع: {}", e))?;
    
    let header: serde_json::Value = serde_json::from_slice(&header_json)
        .map_err(|e| format!("خطأ في تحليل الرأس: {}", e))?;
    
    let claims: HashMap<String, serde_json::Value> = serde_json::from_slice(&payload_json)
        .map_err(|e| format!("خطأ في تحليل المحتوى: {}", e))?;
    
    Ok(JwtToken {
        header: JwtHeader {
            alg: header["alg"].as_str().unwrap_or("HS256").to_string(),
            typ: header["typ"].as_str().unwrap_or("JWT").to_string(),
            kid: header["kid"].as_str().map(|s| s.to_string()),
        },
        payload: JwtPayload { claims },
        signature,
    })
}

/// التحقق من JWT
pub fn verify_jwt(token: &str, secret: &[u8]) -> Result<JwtPayload, String> {
    let parts: Vec<&str> = token.split('.').collect();
    
    if parts.len() != 3 {
        return Err("صيغة JWT غير صالحة".to_string());
    }
    
    let message = format!("{}.{}", parts[0], parts[1]);
    let expected_signature = super::hash::hmac_sha256(secret, message.as_bytes());
    let provided_signature = base64::decode_config(parts[2], base64::URL_SAFE_NO_PAD)
        .map_err(|e| format!("خطأ في فك ترميز التوقيع: {}", e))?;
    
    if expected_signature != provided_signature {
        return Err("التوقيع غير صالح".to_string());
    }
    
    let jwt = parse_jwt(token)?;
    
    // التحقق من انتهاء الصلاحية
    if let Some(exp) = jwt.payload.exp() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        if now > exp {
            return Err("الرمز منتهي الصلاحية".to_string());
        }
    }
    
    Ok(jwt.payload)
}

// ===== دوال عربية =====

/// إنشاء JWT
pub fn أنشئ_jwt(payload: JwtPayload, secret: &[u8]) -> String {
    let mut token = JwtToken::new(payload);
    token.sign(secret)
}

/// تحقق من JWT
pub fn تحقق_jwt(token: &str, secret: &[u8]) -> Result<JwtPayload, String> {
    verify_jwt(token, secret)
}
