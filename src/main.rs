use std::{fs::{read, File}, path::Path, io::{BufReader, Read}};

mod key;
mod prefs;

// TODO: Error handling
fn main() {

    let backup_path = Path::new("./backup.zip");
    let backup_file = File::open(&backup_path).unwrap();
    let backup_reader = BufReader::new(backup_file);

    let mut zip_file = zip::ZipArchive::new(backup_reader).unwrap();

    let mut sp = zip_file.by_name("StarMoneyPrefs").unwrap();
    let mut shared_prefs = Vec::new();
    sp.read_to_end(&mut shared_prefs).unwrap();

    let key_params = prefs::read_key_params_from_shared_preferences(&shared_prefs).unwrap();
    let sqlcipher_key = key::decrypt_key(&key_params, &"abc").unwrap();
}
