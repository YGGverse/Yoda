mod database;
pub use database::Database;

use gtk::glib::user_config_dir;
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

const DB_NAME: &str = "profile.sqlite3";

pub struct Profile {
    database: Database,
    config_path: PathBuf,
}

impl Profile {
    // Constructors

    pub fn new(vendor: &str, app_id: &str, branch: &str, version: &str) -> Self {
        // Init profile path
        let mut config_path = user_config_dir();

        config_path.push(vendor);
        config_path.push(app_id);
        config_path.push(branch);
        config_path.push(version); // @TODO remove after auto-migrate feature implementation

        if let Err(e) = create_dir_all(&config_path) {
            panic!("Failed to create profile directory: {e}")
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
