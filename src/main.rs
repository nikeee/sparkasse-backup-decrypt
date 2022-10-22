use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

mod key;
mod prefs;
mod decryption;

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

    let sqlcipher_key = {
        let mut sp = zip_file.by_name("StarMoneyPrefs").unwrap();
        let mut shared_prefs = Vec::new();
        sp.read_to_end(&mut shared_prefs).unwrap();

        let key_params = prefs::read_key_params_from_shared_preferences(&shared_prefs).unwrap();
        println!("{:?}", key_params);

        key::decrypt_key(&key_params, &password).unwrap()
    };

    println!("{:?}", sqlcipher_key);
    let mut encrypted_database = zip_file.by_name(DATABASE_PATH).unwrap();
    decryption::decrypt_database_file(&mut encrypted_database, &sqlcipher_key, &out_file);
}
