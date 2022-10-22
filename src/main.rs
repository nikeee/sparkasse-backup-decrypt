use std::{fs::File, io::BufReader, path::Path};

mod decryption;
mod key;
mod prefs;

const DATABASE_PATH: &str =
    "/data/user/0/com.starfinanz.smob.android.sfinanzstatus/databases/data.db";

// TODO: Error handling
fn main() {
    let password = "TODO";
    let in_file = "TODO";
    let out_file = "TODO.sqlite";

    let backup_path = Path::new(in_file);
    let backup_file = File::open(&backup_path).unwrap();
    let backup_reader = BufReader::new(backup_file);

    let mut zip_file = zip::ZipArchive::new(backup_reader).unwrap();

    let key = match zip_file.by_name("StarMoneyPrefs") {
        Err(_) => panic!("Could not find StarMoneyPrefs in backup. Most likely, the provided ZIP file is not a valid backup."),
        Ok(mut sp) => decryption::decrypt_sqlcipher_key(&mut sp, password).unwrap_or_else(|e| panic!("{}", e.message))
    };

    let mut encrypted_database = zip_file.by_name(DATABASE_PATH).unwrap_or_else(|_| panic!("Could not find StarMoneyPrefs in backup. Most likely, the provided ZIP file is not a valid backup."));

    decryption::decrypt_database_file_to(&mut encrypted_database, &key, &out_file)
        .unwrap_or_else(|e| panic!("{}", e.message))
}
