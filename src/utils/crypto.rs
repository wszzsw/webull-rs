use crate::error::{WebullError, WebullResult};
use base64::{decode, encode};
use hmac::{Hmac, Mac, NewMac};
use rand::{thread_rng, Rng};
use sha2::Sha256;

/// Generate a random device ID.
pub fn generate_device_id() -> String {
    let mut rng = thread_rng();
    let random_bytes: [u8; 16] = rng.gen();
    encode(&random_bytes)
}

/// Generate an HMAC-SHA256 signature.
pub fn generate_signature(key: &str, message: &str) -> WebullResult<String> {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key.as_bytes())
        .map_err(|e| WebullError::InvalidRequest(format!("Invalid key: {}", e)))?;

    mac.update(message.as_bytes());
    let result = mac.finalize();
    let signature = encode(result.into_bytes());

    Ok(signature)
}

/// Encrypt a password using the Webull algorithm.
pub fn encrypt_password(password: &str, _key: &str) -> WebullResult<String> {
    // This is a simplified version - in a real implementation,
    // we would use the actual encryption algorithm used by Webull

    // For now, we'll just use base64 encoding as a placeholder
    let encrypted = encode(password.as_bytes());

    Ok(encrypted)
}

/// Decrypt data using the Webull algorithm.
pub fn decrypt_data(data: &str, _key: &str) -> WebullResult<String> {
    // This is a simplified version - in a real implementation,
    // we would use the actual decryption algorithm used by Webull

    // For now, we'll just use base64 decoding as a placeholder
    let decoded =
        decode(data).map_err(|e| WebullError::InvalidRequest(format!("Invalid data: {}", e)))?;

    let decrypted = String::from_utf8(decoded)
        .map_err(|e| WebullError::InvalidRequest(format!("Invalid UTF-8: {}", e)))?;

    Ok(decrypted)
}

/// Generate a timestamp for API requests.
pub fn generate_timestamp() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    now.to_string()
}
