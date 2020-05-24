#!/usr/bin/env python3

from key import extract_key_from_shared_prefs
from utils import print_opt

# This script can be used to retrieve the key only (without decryption)
# You have to unzip the backup.zip and point this variable to the StarMoneyPrefs file:
shared_pref_path = 'path/to/StarMoneyPrefs'

def main():
    print_opt('Enter your app password (the input will be printed): ')
    password = input()

    with open(shared_pref_path, 'rb') as sp:
        database_key = extract_key_from_shared_prefs(sp, password)

        if database_key is None:
            print_opt('Could not decrypt database key.')
            print_opt('Probably you have entered a wrong password.')
        else:
            print_opt('SQLcipher database key:')
            print_opt()
            print(database_key, end='')
            print_opt()
            print_opt()
            print_opt('You can now use "sqlcipher" to decrypt your database: https://stackoverflow.com/a/25132478')
            print_opt('https://sqlitebrowser.org also supports sqlcipher databases.')



if __name__ == '__main__':
    main()
