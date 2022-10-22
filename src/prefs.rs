pub struct KeyParams {
    pub salt_and_iv: [u8; 16],
    pub encrypted_database_key: Vec<u8>,
}

pub fn read_key_params_from_shared_preferences(shared_preferences: &[u8]) -> Option<KeyParams> {
    // sf1 holds the salt of PBKDF2 (also used as IV in AES encryption)
    let sf1 = read_shared_preferences_entry(shared_preferences, b"sf1")?;
    if sf1.len() != 0x18 {
        return None;
    }

    // sf3 holds the encrypted database key
    let sf3 = read_shared_preferences_entry(shared_preferences, b"sf3")?;
    if sf3.len() != 0x40 {
        return None;
    }
    // sf2, sf4, sf5 and sf6 are used for different purposes
    // They are all byte[] (except sf5, which is an int)

    // byte[] are saved as base64 strings
    let salt_and_iv = base64::decode(sf1).ok()?;
    let encrypted_database_key = base64::decode(sf3).ok()?;

    Some(KeyParams {
        salt_and_iv: salt_and_iv.try_into().ok()?,
        encrypted_database_key,
    })
}

fn read_shared_preferences_entry<'a>(
    shared_preferences: &'a [u8],
    entry: &'_ [u8],
) -> Option<&'a str> {
    let entry_start = shared_preferences
        .windows(entry.len())
        .enumerate()
        .find(|(_, s)| *s == entry)
        .map(|(index, _)| index)?;

    // Starting from entry_start, we have this data:
    // <entry name> <1 byte for whatever usage> <2 byte content length> <<content-length> bytes content>

    let content_length_start = entry_start + entry.len() + 1;

    let content_length_bytes = &shared_preferences[content_length_start..content_length_start + 2]
        .try_into()
        .ok()?;

    let content_length = u16::from_be_bytes(*content_length_bytes) as usize;
    let content_start = content_length_start + 2;

    let content = &shared_preferences[content_start..content_start + content_length];
    std::str::from_utf8(content).ok()
}
