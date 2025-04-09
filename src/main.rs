use clap::{Parser, ValueEnum};
use std::{fs::File, io::BufReader, path::PathBuf};

mod decryption;
mod key;
mod prefs;

#[derive(Debug, Clone, ValueEnum)]
pub enum ApplicationDatabase {
    OldDb,
    NewDb,
}

impl Default for ApplicationDatabase {
    fn default() -> Self {
        Self::NewDb
    }
}

impl ApplicationDatabase {
    pub fn database_file(&self) -> &'static str {
        match self {
            ApplicationDatabase::OldDb => "data.db",
            ApplicationDatabase::NewDb => "mkabankingapplication.db",
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source path of the backup ZIP file.
    #[arg(short, long)]
    in_file: PathBuf,

    /// Target path for the plaintext (decrypted) SQLite file.
    #[arg(short, long)]
    out_file: PathBuf,

    /// Database from backup that should be decrypted
    #[arg(short, long)]
    db_file: ApplicationDatabase,

    /// App password. Is used to decrypt the SQLite database. If not provided, will be asked upon command invocation.
    #[arg(short, long)]
    password: Option<String>,
}

fn main() {
    let args = Args::parse();

    let password = args.password.unwrap_or_else(|| {
        rpassword::prompt_password("App password (won't be printed): ").unwrap()
    });

    let backup_file = File::open(&args.in_file).unwrap();
    let backup_reader = BufReader::new(backup_file);

    let mut zip_file = zip::ZipArchive::new(backup_reader).unwrap();

    let key = match zip_file.by_name("StarMoneyPrefs") {
        Err(_) => panic!("Could not find StarMoneyPrefs in backup. Most likely, the provided ZIP file is not a valid backup."),
        Ok(mut sp) => decryption::decrypt_sqlcipher_key(&mut sp, &password).unwrap_or_else(|e| panic!("{}", e.message))
    };

    let mut encrypted_database = zip_file.by_name(&args.db_file.database_file())
        .unwrap_or_else(|_| panic!("Could not find database file in backup. Most likely, the provided ZIP file is not a valid backup or the database path changed."));

    decryption::decrypt_database_file_to(&mut encrypted_database, &key, &args.out_file)
        .unwrap_or_else(|e| panic!("{}", e.message))
}
