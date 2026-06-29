use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

pub const DB_KEY_BYTES: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecureStorageConfig {
    pub database_path: String,
    pub wrapped_key_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecureStorageError {
    KeyUnavailable,
    KeyUnwrapFailed,
    InvalidKeyLength { expected: usize, actual: usize },
    DatabaseOpenFailed(String),
    MigrationFailed(String),
}

impl std::fmt::Display for SecureStorageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecureStorageError::KeyUnavailable => {
                write!(formatter, "secure storage key is unavailable")
            }
            SecureStorageError::KeyUnwrapFailed => {
                write!(formatter, "secure storage key could not be unwrapped")
            }
            SecureStorageError::InvalidKeyLength { expected, actual } => write!(
                formatter,
                "invalid secure storage key length: expected {expected} bytes, got {actual}"
            ),
            SecureStorageError::DatabaseOpenFailed(message) => {
                write!(formatter, "database open failed: {message}")
            }
            SecureStorageError::MigrationFailed(message) => {
                write!(formatter, "database migration failed: {message}")
            }
        }
    }
}

impl std::error::Error for SecureStorageError {}

pub trait KeyProtector {
    fn wrap_key(&self, key: &[u8]) -> Result<Vec<u8>, SecureStorageError>;
    fn unwrap_key(&self, wrapped_key: &[u8]) -> Result<Vec<u8>, SecureStorageError>;
}

#[derive(Clone)]
pub struct SqlCipherOpenPlan {
    pub database_path: String,
    pub key_bytes: Zeroizing<Vec<u8>>,
    pub pragmas: Vec<String>,
}

impl std::fmt::Debug for SqlCipherOpenPlan {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SqlCipherOpenPlan")
            .field("database_path", &self.database_path)
            .field("key_bytes", &"[REDACTED]")
            .field("pragmas", &self.pragmas)
            .finish()
    }
}

impl SqlCipherOpenPlan {
    pub fn new(
        database_path: impl Into<String>,
        key_bytes: Vec<u8>,
    ) -> Result<Self, SecureStorageError> {
        if !is_valid_database_key(&key_bytes) {
            return Err(SecureStorageError::InvalidKeyLength {
                expected: DB_KEY_BYTES,
                actual: key_bytes.len(),
            });
        }

        Ok(Self {
            database_path: database_path.into(),
            key_bytes: Zeroizing::new(key_bytes),
            pragmas: vec![
                "PRAGMA cipher_page_size = 4096".into(),
                "PRAGMA kdf_iter = 256000".into(),
                "PRAGMA foreign_keys = ON".into(),
                "PRAGMA journal_mode = WAL".into(),
            ],
        })
    }
}

pub fn is_valid_database_key(key: &[u8]) -> bool {
    key.len() == DB_KEY_BYTES
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EchoProtector;

    impl KeyProtector for EchoProtector {
        fn wrap_key(&self, key: &[u8]) -> Result<Vec<u8>, SecureStorageError> {
            Ok(key.to_vec())
        }

        fn unwrap_key(&self, wrapped_key: &[u8]) -> Result<Vec<u8>, SecureStorageError> {
            Ok(wrapped_key.to_vec())
        }
    }

    #[test]
    fn validates_256_bit_database_key() {
        assert!(is_valid_database_key(&[7; DB_KEY_BYTES]));
        assert!(!is_valid_database_key(&[7; DB_KEY_BYTES - 1]));
    }

    #[test]
    fn key_protector_boundary_round_trips() {
        let key = [3_u8; DB_KEY_BYTES];
        let wrapped = EchoProtector.wrap_key(&key).unwrap();
        let unwrapped = EchoProtector.unwrap_key(&wrapped).unwrap();

        assert_eq!(unwrapped, key.to_vec());
    }

    #[test]
    fn sqlcipher_open_plan_sets_security_pragmas() {
        let plan = SqlCipherOpenPlan::new("app.db", vec![1; DB_KEY_BYTES]).unwrap();

        assert!(plan
            .pragmas
            .iter()
            .any(|pragma| pragma.contains("kdf_iter")));
        assert!(plan.pragmas.iter().any(|pragma| pragma.contains("WAL")));
    }

    #[test]
    fn sqlcipher_open_plan_debug_redacts_key_bytes() {
        let plan = SqlCipherOpenPlan::new("app.db", vec![1; DB_KEY_BYTES]).unwrap();
        let debug = format!("{plan:?}");

        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains("1, 1, 1"));
    }

    #[test]
    fn sqlcipher_open_plan_rejects_invalid_key_length() {
        let result = SqlCipherOpenPlan::new("app.db", vec![1; DB_KEY_BYTES - 1]);

        assert_eq!(
            result.unwrap_err(),
            SecureStorageError::InvalidKeyLength {
                expected: DB_KEY_BYTES,
                actual: DB_KEY_BYTES - 1
            }
        );
    }

    #[test]
    fn secure_storage_error_formats_for_callers() {
        let error = SecureStorageError::DatabaseOpenFailed("cipher mismatch".into());

        assert_eq!(error.to_string(), "database open failed: cipher mismatch");
    }
}
