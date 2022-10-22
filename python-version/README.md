# sparkasse-backup-decrypt

CLI tool to extract and decrypt the SQLite database of a backup ZIP of the [Sparkasse Android application](https://play.google.com/store/apps/details?id=com.starfinanz.smob.android.sfinanzstatus).

## Install
```shell
git clone https://github.com/nikeee/sparkasse-backup-decrypt
pipenv install
```

## Usage
```shell
./decrypt_backup.py <path-to-backup.zip> <path-to-target-plaintext.db>
# (you will be asked for your app password)

# Example call:
./decrypt_backup.py sfinanzstatus55100Auto20200101-155119.zip plaintext.db

# You can check if the out file is in plaintext by using sqlite3:
# sqlite3 plaintext.db .schema
```
