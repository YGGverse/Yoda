mod action;
mod app;

use app::App;
use gtk::glib::{user_config_dir, ExitCode};
use sqlite::Connection;
use std::{fs::create_dir_all, rc::Rc, sync::RwLock};

const VENDOR: &str = "YGGverse";
const APP_ID: &str = "Yoda"; // env!("CARGO_PKG_NAME");
const BRANCH: &str = "master";

fn main() -> ExitCode {
    // Init profile path
    let mut profile_path = user_config_dir();

    profile_path.push(VENDOR);
    profile_path.push(APP_ID);
    profile_path.push(BRANCH);
    profile_path.push(format!(
        "{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR")
    )); // @TODO remove after auto-migrate feature implementation

    if let Err(e) = create_dir_all(&profile_path) {
        panic!("Failed to create profile directory: {e}")
    }

    // Init profile database path
    let mut profile_database_path = profile_path.clone();

    profile_database_path.push("profile.sqlite3");

    // Init database connection
    let profile_database_connection = match Connection::open(profile_database_path) {
        Ok(connection) => Rc::new(RwLock::new(connection)),
        Err(e) => panic!("Failed to connect profile database: {e}"),
    };

    // Init GTK, start application
    match gtk::init() {
        Ok(_) => App::new(profile_database_connection, profile_path).run(), // @TODO common struct for profile data
        Err(_) => ExitCode::FAILURE,
    }
}
