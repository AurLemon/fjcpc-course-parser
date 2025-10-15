use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::{anyhow, Result};
use sha2::{Digest, Sha256};

/// 使用 timestamp 作为密钥加密学号
/// 
/// 使用 AES-256-GCM 加密，密钥从 timestamp 派生
pub fn encrypt_student_id(student_id: &str, timestamp: i64) -> Result<String> {
    // 从 timestamp 派生 32 字节密钥
    let key = derive_key_from_timestamp(timestamp);
    
    // 创建加密器
    let cipher = Aes256Gcm::new(&key.into());
    
    // 使用 timestamp 的一部分作为 nonce（12 字节）
    let nonce_bytes = derive_nonce_from_timestamp(timestamp);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    // 加密
    let ciphertext = cipher
        .encrypt(nonce, student_id.as_bytes())
        .map_err(|e| anyhow!("Encryption failed: {}", e))?;
    
    // 转换为 hex 字符串
    Ok(hex::encode(ciphertext))
}

/// 解密学号（用于调试或必要时恢复）
pub fn decrypt_student_id(encrypted_hex: &str, timestamp: i64) -> Result<String> {
    // 从 hex 解码
    let ciphertext = hex::decode(encrypted_hex)
        .map_err(|e| anyhow!("Invalid hex string: {}", e))?;
    
    // 从 timestamp 派生密钥
    let key = derive_key_from_timestamp(timestamp);
    
    // 创建解密器
    let cipher = Aes256Gcm::new(&key.into());
    
    // 使用相同的 nonce
    let nonce_bytes = derive_nonce_from_timestamp(timestamp);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    // 解密
    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| anyhow!("Decryption failed: {}", e))?;
    
    String::from_utf8(plaintext)
        .map_err(|e| anyhow!("Invalid UTF-8: {}", e))
}

/// 从 timestamp 派生 32 字节密钥
fn derive_key_from_timestamp(timestamp: i64) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(timestamp.to_le_bytes());
    hasher.update(b"fjcpc-course-parser-secret-key");
    let result = hasher.finalize();
    result.into()
}

/// 从 timestamp 派生 12 字节 nonce
fn derive_nonce_from_timestamp(timestamp: i64) -> [u8; 12] {
    let mut hasher = Sha256::new();
    hasher.update(timestamp.to_le_bytes());
    hasher.update(b"fjcpc-nonce");
    let result = hasher.finalize();
    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&result[..12]);
    nonce
}

/// 计算 ucode 的哈希值（用于唯一用户统计）
pub fn hash_ucode(ucode: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(ucode.as_bytes());
    hex::encode(hasher.finalize())
}

