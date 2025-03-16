mod bookmark;
mod database;
mod history;
mod identity;
mod search;

use anyhow::Result;
use bookmark::Bookmark;
use database::Database;
use gtk::glib::{DateTime, user_config_dir};
use history::History;
use identity::Identity;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use search::Search;
use sqlite::Transaction;
use std::{fs::create_dir_all, path::PathBuf};

const VENDOR: &str = "YGGverse";
const APP_ID: &str = "Yoda";
const BRANCH: &str = "master";

const DB_NAME: &str = "database.sqlite3";

pub struct Profile {
    pub bookmark: Bookmark,
    pub database: Database,
    pub history: History,
    pub identity: Identity,
    pub search: Search,
    pub config_path: PathBuf,
}

impl Profile {
    // Constructors

    pub fn init() -> Result<Self> {
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

        create_dir_all(&config_path)?;

        // Init database path
        let mut database_path = config_path.clone();
        database_path.push(DB_NAME);

        // Init database connection
        let database_pool =
            Pool::new(SqliteConnectionManager::file(database_path.as_path())).unwrap();

        // Init profile components
        {
            // Init writable connection
            let mut connection = database_pool.get()?;

            // Init new transaction
            let tx = connection.transaction()?;

            // Begin migration
            migrate(&tx)?;
            tx.commit()?;
        } // unlock database

        // Init model
        let database = Database::build(&database_pool);

        // Get active profile or create new one
        let profile_id = match database.active()? {
            Some(profile) => profile.id,
            None => database.add(true, DateTime::now_local()?, None)?,
        };

        // Init components
        let bookmark = Bookmark::build(&database_pool, profile_id)?;
        let history = History::build(&database_pool, profile_id)?;
        let search = Search::build(&database_pool, profile_id)?;
        let identity = Identity::build(&database_pool, profile_id)?;

        // Result
        Ok(Self {
            bookmark,
            database,
            history,
            identity,
            search,
            config_path,
        })
    }

    // Actions

    pub fn save(&self) -> Result<()> {
        self.history.save()
    }
}

pub fn migrate(tx: &Transaction) -> Result<()> {
    // Migrate self components
    database::init(tx)?;

    // Delegate migration to children components
    bookmark::migrate(tx)?;
    identity::migrate(tx)?;
    search::migrate(tx)?;
    history::migrate(tx)?;

    // Success
    Ok(())
}
