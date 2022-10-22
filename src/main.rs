use std::fs::read;

mod key;
mod prefs;

// TODO: Error handling
fn main() {
    let shared_prefs = read("").unwrap();
    let key_params = prefs::read_key_params_from_shared_preferences(&shared_prefs).unwrap();
    let sqlcipher_key = key::decrypt_key(&key_params, &"abc").unwrap();
}
