use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut};
use cbc::cipher::KeyIvInit;
use hmac::Hmac;

use crate::prefs::KeyParams;

/// The database is encrypted with a 32-byte key (k_0).
/// The user's password is used to derive a key (k_d) that is used to encrypt/decrypt k_0.
/// This way, the user can change his password and k_0 can remain unchanged.
/// It only has to get re-encrypted with a newly derived key.
///
/// This function returns k_0.
///
/// sqlcipher supports keys and even re-keying. It also does HMAC internally.
/// Don't know why they roll their own thing. Maybe this is a compat-thing or they don't want to rely on sqlcipher so heavily.
pub fn decrypt_key(params: &KeyParams, password: &str) -> Option<String> {
    let k_d = derive_internal_key(&params.salt_and_iv, password);

    // AES parameters according to source: AES/CBC/PKCS5Padding
    // We use PKCS#7 because PKCS#5 is a subset of PKCS#7
    let decryptor = cbc::Decryptor::<aes::Aes256>::new(&k_d.into(), &params.salt_and_iv.into());

    let mut k_0_buf = [0u8; 0x40];
    decryptor.decrypt_padded_mut::<Pkcs7>(&mut k_0_buf).ok()?;

    let k_0 = std::str::from_utf8(&k_0_buf).ok()?;

    Some(k_0.into())
}

/// This function returns k_d
fn derive_internal_key(salt: &[u8], password: &str) -> [u8; 32] {
    // hopefully performs the same operations as `mbedtls_pkcs5_pbkdf2_hmac`
    let mut res = [0u8; 32];
    pbkdf2::pbkdf2::<Hmac<sha1::Sha1>>(password.as_bytes(), salt, 100001, &mut res);
    res
}
