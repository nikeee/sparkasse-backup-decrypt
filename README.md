# sparkasse-backup-decrypt [![CI](https://github.com/nikeee/sparkasse-backup-decrypt/actions/workflows/CI.yml/badge.svg)](https://github.com/nikeee/sparkasse-backup-decrypt/actions/workflows/CI.yml)

CLI tool to extract and decrypt the SQLite database of a backup ZIP of the [Sparkasse Android application](https://play.google.com/store/apps/details?id=com.starfinanz.smob.android.sfinanzstatus).

This project was rewritten. In case it does not work for you, you can find the previous version in the branch [`python-version`](https://github.com/nikeee/sparkasse-backup-decrypt/tree/python-version).

## Install
TODO

## Usage
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
