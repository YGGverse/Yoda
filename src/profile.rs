mod bookmark;
mod database;
//mod history;
mod identity;

use bookmark::Bookmark;
use database::Database;
use identity::Identity;

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
    pub identity: Rc<Identity>,
    pub config_path: PathBuf,
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

        // Init database connection
        let connection = match Connection::open(database_path.as_path()) {
            Ok(connection) => Rc::new(RwLock::new(connection)),
            Err(reason) => panic!("{reason}"),
        };

        // Init profile components
        {
            // Init writable connection
            let mut connection = match connection.write() {
                Ok(connection) => connection,
                Err(reason) => todo!("{reason}"),
            };

            // Init new transaction
            let transaction = match connection.transaction() {
                Ok(transaction) => transaction,
                Err(reason) => todo!("{reason}"),
            };

            // Begin migration
            match migrate(&transaction) {
                Ok(_) => {
                    // Confirm changes
                    if let Err(reason) = transaction.commit() {
                        todo!("{reason}")
                    }
                }
                Err(reason) => todo!("{reason}"),
            }
        } // unlock database

        // Init model
        let database = Rc::new(Database::new(connection.clone()));

        // Get active profile or create new one
        let profile_id = Rc::new(match database.active().unwrap() {
            Some(profile) => profile.id,
            None => match database.add(true, DateTime::now_local().unwrap(), None) {
                Ok(id) => id,
                Err(reason) => todo!("{:?}", reason),
            },
        });

        // Init bookmark component @TODO handle errors
        let bookmark = Rc::new(Bookmark::new(connection.clone(), profile_id.clone()));

        // Init identity component
        let identity = Rc::new(match Identity::new(connection, profile_id) {
            Ok(result) => result,
            Err(reason) => todo!("{:?}", reason.to_string()),
        });

        // Result
        Self {
            bookmark,
            identity,
            database,
            config_path,
        }
    }
}

pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(reason) = database::init(tx) {
        return Err(reason.to_string());
    }

    // Delegate migration to children components
    bookmark::migrate(tx)?;
    identity::migrate(tx)?;
    // @TODO not in use yet
    // history::migrate(tx)?;

    // Success
    Ok(())
}
