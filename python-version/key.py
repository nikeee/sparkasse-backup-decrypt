#!/usr/bin/env python3

import struct
import base64
import hashlib
from typing import Optional, Union, BinaryIO
from Crypto.Cipher import AES

# sqlcipher supports keys and even re-keying. It also does HMAC internally.
# Don't know why they roll their own thing. Maybe this is a compat-thing or they don't want to rely on sqlcipher so heavily.

def extract_key_from_shared_prefs(contents: bytes, password: str) -> Optional[str]:

    sp = SharedPreferences(contents)

    sf1 = sp.read_str(b'sf1')  # sf1 holds the salt of PBKDF2 (also used as IV in AES encryption)
    assert len(sf1) == 0x18
    sf3 = sp.read_str(b'sf3')  # sf3 holds the encrypted database key
    assert len(sf3) == 0x40
    # sf2, sf4, sf5 and sf6 are used for different purposes
    # They are all byte[] (except sf5, which is an int)

    # byte[] are saved as base64 strings
    salt_and_iv = base64.b64decode(sf1)
    encrypted_database_key = base64.b64decode(sf3)

    database_key = decrypt_key(encrypted_database_key, password, salt_and_iv)
    return database_key[:32]


def decrypt_key(payload: bytes, password: str, salt_and_iv: bytes) -> Optional[str]:
    """
    The database is encrypted with a 32-byte key (k_0).
    The user's password is used to derive a key (k_d) that is used to encrypt k_0.
    This way, the user can change his password and k_0 can remain unchanged.
    It only has to get re-encrypted with a newly derived key.

    This function returns k_0.
    """

    k_d = derive_key(password, salt_and_iv)
    assert len(k_d) == 32

    # AES parameters according to source: AES/CBC/PKCS5Padding
    cipher = AES.new(
        k_d,
        mode=AES.MODE_CBC,
        IV=salt_and_iv,
    )

    k_0_bytes = cipher.decrypt(payload)
    assert len(payload) == len(k_0_bytes)

    try:
        return k_0_bytes.decode('utf-8')
    except UnicodeDecodeError:
        return None


def derive_key(password: str, salt: bytes) -> bytes:
    """
    Besides it is called 'NativeTools.encode', it actually derives a HMAC-SHA1 key with PBKDF2 (using mbedtls).

    This function returns k_d.
    """

    encoded_password = password.encode('utf-8')

    # hopefully performs the same operations as `mbedtls_pkcs5_pbkdf2_hmac`
    return hashlib.pbkdf2_hmac(
        'sha1',
        encoded_password,
        salt,
        100001,
        32,
    )


class SharedPreferences:
    def __init__(self, contents: bytes):
        self.prefs = contents

    def read_str(self, entry: bytes) -> Optional[str]:

        entry_start = self.prefs.index(entry)
        if entry_start < 0:
            return None

        # Starting from entry_start, we have this data:
        # <entry name> <1 byte for whatever usage> <2 byte content length> <<content-length> bytes content>

        # <entry name> <1 byte for whatever usage>
        content_length_start = entry_start + len(entry) + 1

        # https://stackoverflow.com/q/14215715
        content_length, = struct.unpack(
            '>H',
            self.prefs[content_length_start:content_length_start + 2]
        )

        content_start = content_length_start + 2

        content = self.prefs[content_start:content_start + content_length]

        try:
            return content.decode('utf-8')
        except UnicodeDecodeError:
            return None
