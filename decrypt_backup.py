#!/usr/bin/env python3

import zipfile
import argparse
import tempfile
from sqlcipher3 import dbapi2 as sqlite

from typing import Union, BinaryIO
from utils import print_opt
from key import extract_key_from_shared_prefs


def get_args():
    parser = argparse.ArgumentParser(
        description='CLI tool to extract and decrypt the SQLite database of a backup ZIP of the Sparkasse Android application',
    )
    parser.add_argument(
        'backup_file',
        metavar='<backup-file.zip>',
        type=str,
        help='The backup zip file created with the app. E. g. path/to/backup.zip',
    )
    parser.add_argument(
        'out_file',
        metavar='<out-file.zip>',
        type=str,
        help='The file to write the plaintext data to. E. g. path/to/plaintext_out.db',
    )

    return parser.parse_args()


def main():
    args = get_args()

    print_opt('Enter your app password (the input will be printed):')
    password = input()

    decrypt_backup(args.backup_file, password, args.out_file)


def decrypt_backup(backup_zip_file: Union[str, BinaryIO], password: str, out_file: str) -> None:

    with zipfile.ZipFile(backup_zip_file, 'r') as backup:
        sp = backup.read('StarMoneyPrefs')

        database_key = extract_key_from_shared_prefs(sp, password)
        print('Extracted database key')

        with tempfile.NamedTemporaryFile() as db_file:
            encrypted_db_file = backup.read('/data/user/0/com.starfinanz.smob.android.sfinanzstatus/databases/data.db')
            db_file.write(encrypted_db_file)
            db_file.flush()

            conn = sqlite.connect(db_file.name)

            # parameter binding does not work in this PRAGMA statement :(
            escaped_database_key = database_key.replace('"', '""')
            conn.execute(f'PRAGMA key = "{escaped_database_key}"')

            # Ref: https://stackoverflow.com/questions/32100227
            conn.execute(f'ATTACH DATABASE ? AS plaintext KEY ""', (out_file,))
            conn.execute(f'SELECT sqlcipher_export("plaintext")')
            conn.execute(f'DETACH DATABASE plaintext')

            print(f'Exported plaintext database to {out_file}')


if __name__ == '__main__':
    main()
