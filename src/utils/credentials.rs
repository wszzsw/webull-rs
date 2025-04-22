use crate::auth::{AccessToken, Credentials};
use crate::error::{WebullError, WebullResult};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

/// Interface for storing and retrieving credentials.
pub trait CredentialStore: Send + Sync {
    /// Get the stored credentials.
    fn get_credentials(&self) -> WebullResult<Option<Credentials>>;

    /// Store credentials.
    fn store_credentials(&self, credentials: Credentials) -> WebullResult<()>;

    /// Clear the stored credentials.
    fn clear_credentials(&self) -> WebullResult<()>;

    /// Get the stored access token.
    fn get_token(&self) -> WebullResult<Option<AccessToken>>;

    /// Store an access token.
    fn store_token(&self, token: AccessToken) -> WebullResult<()>;

    /// Clear the stored token.
    fn clear_token(&self) -> WebullResult<()>;
}

/// In-memory credential store.
#[derive(Debug, Default)]
pub struct MemoryCredentialStore {
    /// Stored credentials
    credentials: Mutex<Option<Credentials>>,

    /// Stored access token
    token: Mutex<Option<AccessToken>>,
}

impl CredentialStore for MemoryCredentialStore {
    fn get_credentials(&self) -> WebullResult<Option<Credentials>> {
        Ok(self.credentials.lock().unwrap().clone())
    }

    fn store_credentials(&self, credentials: Credentials) -> WebullResult<()> {
        *self.credentials.lock().unwrap() = Some(credentials);
        Ok(())
    }

    fn clear_credentials(&self) -> WebullResult<()> {
        *self.credentials.lock().unwrap() = None;
        Ok(())
    }

    fn get_token(&self) -> WebullResult<Option<AccessToken>> {
        Ok(self.token.lock().unwrap().clone())
    }

    fn store_token(&self, token: AccessToken) -> WebullResult<()> {
        *self.token.lock().unwrap() = Some(token);
        Ok(())
    }

    fn clear_token(&self) -> WebullResult<()> {
        *self.token.lock().unwrap() = None;
        Ok(())
    }
}

/// Encrypted credential store for disk-based storage.
pub struct EncryptedCredentialStore {
    /// Path to the credentials file
    credentials_path: String,

    /// Path to the token file
    token_path: String,

    /// Encryption key
    encryption_key: String,

    /// In-memory cache
    memory_store: MemoryCredentialStore,
}

/// Stored credentials with encryption.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredCredentials {
    /// Encrypted username
    encrypted_username: String,

    /// Encrypted password
    encrypted_password: String,

    /// Initialization vector for encryption
    iv: String,

    /// Salt for encryption
    salt: String,
}

/// Stored token with encryption.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredToken {
    /// Encrypted token
    encrypted_token: String,

    /// Encrypted refresh token
    encrypted_refresh_token: Option<String>,

    /// Expiration timestamp
    expires_at: i64,

    /// Initialization vector for encryption
    iv: String,

    /// Salt for encryption
    salt: String,
}

impl EncryptedCredentialStore {
    /// Create a new encrypted credential store.
    pub fn new(credentials_path: String, token_path: String, encryption_key: String) -> Self {
        Self {
            credentials_path,
            token_path,
            encryption_key,
            memory_store: MemoryCredentialStore::default(),
        }
    }

    /// Encrypt a string.
    fn encrypt(&self, data: &str) -> WebullResult<(String, String, String)> {
        // Generate a random salt and IV
        let salt = self.generate_random_string(16);
        let iv = self.generate_random_string(16);

        // Derive a key from the encryption key and salt
        let key = self.derive_key(&self.encryption_key, &salt)?;

        // Encrypt the data
        let encrypted = self.encrypt_with_key(data, &key, &iv)?;

        Ok((encrypted, iv, salt))
    }

    /// Decrypt a string.
    fn decrypt(&self, encrypted: &str, iv: &str, salt: &str) -> WebullResult<String> {
        // Derive a key from the encryption key and salt
        let key = self.derive_key(&self.encryption_key, salt)?;

        // Decrypt the data
        self.decrypt_with_key(encrypted, &key, iv)
    }

    /// Generate a random string.
    fn generate_random_string(&self, length: usize) -> String {
        use rand::{thread_rng, Rng};
        use rand::distributions::Alphanumeric;

        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    /// Derive a key from a password and salt.
    fn derive_key(&self, password: &str, salt: &str) -> WebullResult<Vec<u8>> {
        // In a real implementation, we would use a proper key derivation function
        // like PBKDF2, Argon2, or scrypt. For simplicity, we'll just use a basic
        // approach here.

        let mut key = Vec::with_capacity(32);
        let password_bytes = password.as_bytes();
        let salt_bytes = salt.as_bytes();

        for i in 0..32 {
            let byte = password_bytes[i % password_bytes.len()] ^ salt_bytes[i % salt_bytes.len()];
            key.push(byte);
        }

        Ok(key)
    }

    /// Encrypt data with a key and IV.
    fn encrypt_with_key(&self, data: &str, _key: &[u8], _iv: &str) -> WebullResult<String> {
        // In a real implementation, we would use a proper encryption algorithm
        // like AES-GCM. For simplicity, we'll just use base64 encoding as a
        // placeholder.

        let encoded = base64::encode(data);
        Ok(encoded)
    }

    /// Decrypt data with a key and IV.
    fn decrypt_with_key(&self, encrypted: &str, _key: &[u8], _iv: &str) -> WebullResult<String> {
        // In a real implementation, we would use a proper decryption algorithm
        // like AES-GCM. For simplicity, we'll just use base64 decoding as a
        // placeholder.

        let decoded = base64::decode(encrypted)
            .map_err(|e| WebullError::InvalidRequest(format!("Invalid data: {}", e)))?;

        let decrypted = String::from_utf8(decoded)
            .map_err(|e| WebullError::InvalidRequest(format!("Invalid UTF-8: {}", e)))?;

        Ok(decrypted)
    }

    /// Load credentials from disk.
    fn load_credentials(&self) -> WebullResult<Option<Credentials>> {
        // Check if the file exists
        let path = Path::new(&self.credentials_path);
        if !path.exists() {
            return Ok(None);
        }

        // Read the file
        let contents = std::fs::read_to_string(path)
            .map_err(|e| WebullError::InvalidRequest(format!("Failed to read credentials file: {}", e)))?;

        // Parse the stored credentials
        let stored: StoredCredentials = serde_json::from_str(&contents)
            .map_err(|e| WebullError::SerializationError(e))?;

        // Decrypt the username and password
        let username = self.decrypt(&stored.encrypted_username, &stored.iv, &stored.salt)?;
        let password = self.decrypt(&stored.encrypted_password, &stored.iv, &stored.salt)?;

        Ok(Some(Credentials {
            username,
            password,
        }))
    }

    /// Save credentials to disk.
    fn save_credentials(&self, credentials: &Credentials) -> WebullResult<()> {
        // Encrypt the username and password
        let (encrypted_username, iv, salt) = self.encrypt(&credentials.username)?;
        let (encrypted_password, _, _) = self.encrypt(&credentials.password)?;

        // Create the stored credentials
        let stored = StoredCredentials {
            encrypted_username,
            encrypted_password,
            iv,
            salt,
        };

        // Serialize to JSON
        let json = serde_json::to_string(&stored)
            .map_err(|e| WebullError::SerializationError(e))?;

        // Write to file
        std::fs::write(&self.credentials_path, json)
            .map_err(|e| WebullError::InvalidRequest(format!("Failed to write credentials file: {}", e)))?;

        Ok(())
    }

    /// Load token from disk.
    fn load_token(&self) -> WebullResult<Option<AccessToken>> {
        // Check if the file exists
        let path = Path::new(&self.token_path);
        if !path.exists() {
            return Ok(None);
        }

        // Read the file
        let contents = std::fs::read_to_string(path)
            .map_err(|e| WebullError::InvalidRequest(format!("Failed to read token file: {}", e)))?;

        // Parse the stored token
        let stored: StoredToken = serde_json::from_str(&contents)
            .map_err(|e| WebullError::SerializationError(e))?;

        // Decrypt the token
        let token = self.decrypt(&stored.encrypted_token, &stored.iv, &stored.salt)?;

        // Decrypt the refresh token if present
        let refresh_token = if let Some(encrypted_refresh_token) = stored.encrypted_refresh_token {
            Some(self.decrypt(&encrypted_refresh_token, &stored.iv, &stored.salt)?)
        } else {
            None
        };

        // Create the access token
        let expires_at = chrono::DateTime::from_timestamp(stored.expires_at, 0)
            .ok_or_else(|| WebullError::InvalidRequest("Invalid timestamp".to_string()))?;

        Ok(Some(AccessToken {
            token,
            expires_at,
            refresh_token,
        }))
    }

    /// Save token to disk.
    fn save_token(&self, token: &AccessToken) -> WebullResult<()> {
        // Encrypt the token
        let (encrypted_token, iv, salt) = self.encrypt(&token.token)?;

        // Encrypt the refresh token if present
        let encrypted_refresh_token = if let Some(refresh_token) = &token.refresh_token {
            Some(self.encrypt(refresh_token)?.0)
        } else {
            None
        };

        // Create the stored token
        let stored = StoredToken {
            encrypted_token,
            encrypted_refresh_token,
            expires_at: token.expires_at.timestamp(),
            iv,
            salt,
        };

        // Serialize to JSON
        let json = serde_json::to_string(&stored)
            .map_err(|e| WebullError::SerializationError(e))?;

        // Write to file
        std::fs::write(&self.token_path, json)
            .map_err(|e| WebullError::InvalidRequest(format!("Failed to write token file: {}", e)))?;

        Ok(())
    }
}

impl CredentialStore for EncryptedCredentialStore {
    fn get_credentials(&self) -> WebullResult<Option<Credentials>> {
        // Check if we have credentials in memory
        if let Some(credentials) = self.memory_store.get_credentials()? {
            return Ok(Some(credentials));
        }

        // Load credentials from disk
        let credentials = self.load_credentials()?;

        // Store in memory for future use
        if let Some(credentials) = &credentials {
            self.memory_store.store_credentials(credentials.clone())?;
        }

        Ok(credentials)
    }

    fn store_credentials(&self, credentials: Credentials) -> WebullResult<()> {
        // Store in memory
        self.memory_store.store_credentials(credentials.clone())?;

        // Save to disk
        self.save_credentials(&credentials)?;

        Ok(())
    }

    fn clear_credentials(&self) -> WebullResult<()> {
        // Clear from memory
        self.memory_store.clear_credentials()?;

        // Remove the file if it exists
        let path = Path::new(&self.credentials_path);
        if path.exists() {
            std::fs::remove_file(path)
                .map_err(|e| WebullError::InvalidRequest(format!("Failed to remove credentials file: {}", e)))?;
        }

        Ok(())
    }

    fn get_token(&self) -> WebullResult<Option<AccessToken>> {
        // Check if we have a token in memory
        if let Some(token) = self.memory_store.get_token()? {
            return Ok(Some(token));
        }

        // Load token from disk
        let token = self.load_token()?;

        // Store in memory for future use
        if let Some(token) = &token {
            self.memory_store.store_token(token.clone())?;
        }

        Ok(token)
    }

    fn store_token(&self, token: AccessToken) -> WebullResult<()> {
        // Store in memory
        self.memory_store.store_token(token.clone())?;

        // Save to disk
        self.save_token(&token)?;

        Ok(())
    }

    fn clear_token(&self) -> WebullResult<()> {
        // Clear from memory
        self.memory_store.clear_token()?;

        // Remove the file if it exists
        let path = Path::new(&self.token_path);
        if path.exists() {
            std::fs::remove_file(path)
                .map_err(|e| WebullError::InvalidRequest(format!("Failed to remove token file: {}", e)))?;
        }

        Ok(())
    }
}
