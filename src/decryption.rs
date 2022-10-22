use std::io::Read;

use rusqlite::Connection;
use tempfile::NamedTempFile;


pub fn decrypt_database_file<T: Read>(encrypted_database: &mut T, key: &str, out_file: &str) -> Option<()> {
    let mut file = NamedTempFile::new().unwrap(); // TODO: Error handling

    std::io::copy(encrypted_database, &mut file).unwrap();

    let conn = Connection::open(file.path()).unwrap(); // TODO: Error handling

    conn.pragma_update(None, "key", key).unwrap(); // TODO: Error handling

    // https://stackoverflow.com/a/32571540
    // Empty key will disable encryption
    conn.execute("ATTACH DATABASE ? AS plaintext KEY ''", &[out_file]).unwrap(); // TODO: Error handling
    conn.query_row("SELECT sqlcipher_export('plaintext')", [], |_| Ok(())).unwrap(); // TODO: Error handling
    conn.execute("DETACH DATABASE plaintext", []).unwrap(); // TODO: Error handling

    file.close().unwrap(); // TODO: Error handling
    Some(())
}
