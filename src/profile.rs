mod bookmark;
mod database;
mod history;
mod identity;
mod search;

use bookmark::Bookmark;
use database::Database;
use history::History;
use identity::Identity;
use search::Search;

use gtk::glib::{user_config_dir, DateTime};
use sqlite::{Connection, Transaction};
use std::{fs::create_dir_all, path::PathBuf, rc::Rc, sync::RwLock};

const VENDOR: &str = "YGGverse";
const APP_ID: &str = "Yoda";
const BRANCH: &str = "master";

const DB_NAME: &str = "database.sqlite3";

pub struct Profile {
    pub bookmark: Rc<Bookmark>,
    pub database: Rc<Database>,
    pub history: Rc<History>,
    pub identity: Rc<Identity>,
    pub search: Rc<Search>,
    pub config_path: PathBuf,
}

impl Default for Profile {
    fn default() -> Self {
        Self::new()
    }
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

        if let Err(e) = create_dir_all(&config_path) {
            panic!("{e}")
        }

        // Init database path
        let mut database_path = config_path.clone();
        database_path.push(DB_NAME);

        // Init database connection
        let connection = match Connection::open(database_path.as_path()) {
            Ok(connection) => Rc::new(RwLock::new(connection)),
            Err(e) => panic!("{e}"),
        };

        // Init profile components
        {
            // Init writable connection
            let mut connection = match connection.write() {
                Ok(connection) => connection,
                Err(e) => todo!("{e}"),
            };

            // Init new transaction
            let transaction = match connection.transaction() {
                Ok(transaction) => transaction,
                Err(e) => todo!("{e}"),
            };

            // Begin migration
            match migrate(&transaction) {
                Ok(_) => {
                    // Confirm changes
                    if let Err(e) = transaction.commit() {
                        todo!("{e}")
                    }
                }
                Err(e) => todo!("{e}"),
            }
        } // unlock database

        // Init model
        let database = Rc::new(Database::build(&connection));

        // Get active profile or create new one
        let profile_id = Rc::new(match database.active().unwrap() {
            Some(profile) => profile.id,
            None => match database.add(true, DateTime::now_local().unwrap(), None) {
                Ok(id) => id,
                Err(e) => todo!("{:?}", e),
            },
        });

        // Init components
        let bookmark = Rc::new(Bookmark::build(&connection, &profile_id));
        let history = Rc::new(History::build(&connection, &profile_id));
        let search = Rc::new(Search::build(&connection, &profile_id).unwrap()); // @TODO handle
        let identity = Rc::new(match Identity::build(&connection, &profile_id) {
            Ok(result) => result,
            Err(e) => todo!("{:?}", e.to_string()),
        });

        // Result
        Self {
            bookmark,
            database,
            history,
            identity,
            search,
            config_path,
        }
    }
}

pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to children components
    bookmark::migrate(tx)?;
    identity::migrate(tx)?;
    search::migrate(tx)?;
    // @TODO not in use yet
    // history::migrate(tx)?;

    // Success
    Ok(())
}
