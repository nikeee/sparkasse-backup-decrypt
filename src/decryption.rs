use std::{io::Read, path::Path};

use rusqlite::Connection;
use tempfile::NamedTempFile;

use crate::{key, prefs};

pub fn decrypt_sqlcipher_key<T: Read>(
    shared_prefs: &mut T,
    password: &str,
) -> Result<String, DecryptionError> {
    let mut shared_prefs_buf = Vec::new();
    shared_prefs
        .read_to_end(&mut shared_prefs_buf)
        .map_err(|_| "Unable read shared prefs")?;

    let key_params = prefs::read_key_params_from_shared_preferences(&shared_prefs_buf)
        .ok_or_else(|| DecryptionError::new("Unable read shared prefs"))?;

    key::decrypt_key(&key_params, password)
        .ok_or_else(|| DecryptionError::new("Unable to decrypt SQLcipher key."))
}

pub fn decrypt_database_file_to<T: Read>(
    encrypted_database: &mut T,
    key: &str,
    out_file: &Path,
) -> Result<(), DecryptionError> {
    let mut file = NamedTempFile::new().map_err(|_| "Unable to create temporary file")?;

    std::io::copy(encrypted_database, &mut file)
        .map_err(|_| "Unable to copy encrypted data to temporary file")?;

    let conn = Connection::open(file.path())
        .map_err(|_| "Unable to open database connection to encrypted temporary file")?;

    conn.pragma_update(None, "key", key)
        .map_err(|_| "Could not set key for sqlcipher.")?;

    // https://stackoverflow.com/a/32571540
    // Empty key will disable encryption
    conn.execute(
        "ATTACH DATABASE ? AS plaintext KEY ''",
        &[out_file.to_str().unwrap()],
    )
    .map_err(|_| "Unable to attach output database")?;

    conn.query_row("SELECT sqlcipher_export('plaintext')", [], |_| Ok(()))
        .map_err(|_| "Unable to export plaintext database")?;

    conn.execute("DETACH DATABASE plaintext", []).map_err(|_| {
        "Could not DETACH plaintext database. Plaintext data may be written to disk regardless."
    })?;

    file.close()
        .map_err(|_| "Unable to close temporary file.")?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct DecryptionError {
    pub message: String,
}

impl DecryptionError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl From<String> for DecryptionError {
    fn from(str: String) -> Self {
        Self::new(str)
    }
}

impl From<&str> for DecryptionError {
    fn from(str: &str) -> Self {
        Self::new(str)
    }
}
