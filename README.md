# sparkasse-backup-decrypt [![CI](https://github.com/nikeee/sparkasse-backup-decrypt/actions/workflows/CI.yml/badge.svg)](https://github.com/nikeee/sparkasse-backup-decrypt/actions/workflows/CI.yml)

CLI tool to extract and decrypt the SQLite database of a backup ZIP of the [Sparkasse Android application](https://play.google.com/store/apps/details?id=com.starfinanz.smob.android.sfinanzstatus).

This project was rewritten. In case it does not work for you, you can find the previous version in the branch [`python-version`](https://github.com/nikeee/sparkasse-backup-decrypt/tree/python-version).

## Install
TODO

## Usage

### Prerequisites
You need to have a backup zip file. [Look at the comments on how to get one](https://blog.sparkasse-allgaeu.de/artikel/die-sparkassen-app-auf-ein-neues-handy-uebertragen).
Depending on your Android version, backups are located at:
- `/storage/emulated/0/ebanking`
- `/storage/emulated/0/Android/data/com.starfinanz.smob.android.sfinanzstatus/files`

If you are running Android 11+, chances are that you need to access the files using your PC by connecting your phone to the PC via USB.
You can also use ADB instead of crappy MTP:
```shell
adb pull "/storage/emulated/0/Android/data/com.starfinanz.smob.android.sfinanzstatus/files/"
```

### Decryption
```shell
./sparkasse-backup-decrypt --in-file <path-to-backup.zip> --out-file <path-to-target-plaintext.db>
# (you will be asked for your app password)

# Example call:
./sparkasse-backup-decrypt --in-file sfinanzstatus55100Auto20200101-155119.zip --out-file plaintext.db

# You can check if the out file is in plaintext by using sqlite3:
# sqlite3 plaintext.db .schema
```

## Compile from Source
```shell
git clone https://github.com/nikeee/sparkasse-backup-decrypt.git
cd sparkasse-backup-decrypt
cargo build --release

# binary located at:
# target/release/sparkasse-backup-decrypt
```
