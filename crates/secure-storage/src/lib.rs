use serde::{Deserialize, Serialize};

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
    DatabaseOpenFailed(String),
    MigrationFailed(String),
}

pub trait KeyProtector {
    fn wrap_key(&self, key: &[u8]) -> Result<Vec<u8>, SecureStorageError>;
    fn unwrap_key(&self, wrapped_key: &[u8]) -> Result<Vec<u8>, SecureStorageError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlCipherOpenPlan {
    pub database_path: String,
    pub key_bytes: Vec<u8>,
    pub pragmas: Vec<String>,
}

impl SqlCipherOpenPlan {
    pub fn new(database_path: impl Into<String>, key_bytes: Vec<u8>) -> Self {
        Self {
            database_path: database_path.into(),
            key_bytes,
            pragmas: vec![
                "PRAGMA cipher_page_size = 4096".into(),
                "PRAGMA kdf_iter = 256000".into(),
                "PRAGMA foreign_keys = ON".into(),
                "PRAGMA journal_mode = WAL".into(),
            ],
        }
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
        let plan = SqlCipherOpenPlan::new("app.db", vec![1; DB_KEY_BYTES]);

        assert!(plan
            .pragmas
            .iter()
            .any(|pragma| pragma.contains("kdf_iter")));
        assert!(plan.pragmas.iter().any(|pragma| pragma.contains("WAL")));
    }
}
