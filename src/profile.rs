mod database;
pub use database::Database;

use gtk::glib::user_config_dir;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

const VENDOR: &str = "YGGverse";
const APP_ID: &str = "Yoda";
const BRANCH: &str = "master";

const DB_NAME: &str = "profile.sqlite3";

pub struct Profile {
    database: Database,
    config_path: PathBuf,
}

impl Profile {
    // Constructors

    pub fn new() -> Self {
        // Init profile path
        let mut config_path = user_config_dir();

        config_path.push(VENDOR);
        config_path.push(APP_ID);
        config_path.push(BRANCH);
        config_path.push(format!(
            "{}.{}",
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR")
        )); // @TODO remove after auto-migrate feature implementation

        if let Err(reason) = create_dir_all(&config_path) {
            panic!("{reason}")
        }

        // Init database path
        let mut database_path = config_path.clone();
        database_path.push(DB_NAME);

        // Result
        Self {
            database: Database::new(database_path.as_path()),
            config_path,
        }
    }

    // Getters

    pub fn database(&self) -> &Database {
        &self.database
    }

    pub fn config_path(&self) -> &Path {
        self.config_path.as_path()
    }
}
