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

    let key = {
        let mut sp = zip_file.by_name("StarMoneyPrefs").unwrap();
        decryption::decrypt_sqlcipher_key(&mut sp, password).unwrap()
    };

    let mut encrypted_database = zip_file.by_name(DATABASE_PATH).unwrap();
    decryption::decrypt_database_file_to(&mut encrypted_database, &key, &out_file);
}
