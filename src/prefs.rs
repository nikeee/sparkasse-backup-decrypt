use base64::prelude::*;
use jaded::{AnnotationIter, ConversionResult, FromJava};

#[derive(Debug, FromJava)]
struct EncryptionSettings {
    #[jaded(extract(extract_sf3))]
    /// sf3 holds the encrypted database key
    sf3: String,
    #[jaded(extract(extract_sf1))]
    /// sf1 holds the salt of PBKDF2 (also used as IV in AES encryption)
    sf1: String,
}

fn extract_sf3(annotations: &mut AnnotationIter) -> ConversionResult<String> {
    loop {
        match annotations.read_object_as::<String>() {
            Ok(label) => {
                if label == "sf3" {
                    let value: String = annotations.read_object_as().unwrap();
                    return ConversionResult::Ok(value);
                }
            }
            Err(err) => match err {
                jaded::ConversionError::UnexpectedBlockData(ref _vec) => {
                    let _x = annotations.read_u8();
                    continue;
                }
                jaded::ConversionError::NullPointerException => {
                    return ConversionResult::Err(jaded::ConversionError::FieldNotFound(
                        "sf1".to_string(),
                    ));
                }
                _ => {
                    continue;
                }
            },
        }
    }
}

fn extract_sf1(annotations: &mut AnnotationIter) -> ConversionResult<String> {
    loop {
        match annotations.read_object_as::<String>() {
            Ok(label) => {
                if label == "sf1" {
                    let value: String = annotations.read_object_as().unwrap();
                    return ConversionResult::Ok(value);
                }
            }
            Err(err) => match err {
                jaded::ConversionError::UnexpectedBlockData(ref _vec) => {
                    let _x = annotations.read_u8();
                    continue;
                }
                jaded::ConversionError::NullPointerException => {
                    return ConversionResult::Err(jaded::ConversionError::FieldNotFound(
                        "sf1".to_string(),
                    ));
                }
                _ => {
                    continue;
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct KeyParams {
    pub salt_and_iv: [u8; 16],
    pub encrypted_database_key: Vec<u8>,
}

pub fn read_key_params_from_shared_preferences(shared_preferences: &[u8]) -> Option<KeyParams> {
    let mut preference_praser =
        jaded::Parser::new(shared_preferences).expect("java data parser failure");

    let encryption_settings: EncryptionSettings = preference_praser
        .read_as()
        .expect("unable to find encryption settings in StarMoneyPrefs");

    // sf2, sf4, sf5 and sf6 are used for different purposes
    // They are all byte[] (except sf5, which is an int)

    // byte[] are saved as base64 strings
    let salt_and_iv = BASE64_STANDARD.decode(encryption_settings.sf1).ok()?;
    let encrypted_database_key = BASE64_STANDARD.decode(encryption_settings.sf3).ok()?;

    Some(KeyParams {
        salt_and_iv: salt_and_iv.try_into().ok()?,
        encrypted_database_key,
    })
}
