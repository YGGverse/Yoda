mod app;

use app::App;
use gtk::glib::{user_config_dir, ExitCode};
use sqlite::Connection;
use std::{fs::create_dir_all, sync::Arc};

const VENDOR: &str = "YGGverse";
const APP_ID: &str = env!("CARGO_PKG_NAME");

fn main() -> ExitCode {
    // Init profile path
    let mut profile_path = user_config_dir();

    profile_path.push(VENDOR);
    profile_path.push(APP_ID);

    if let Err(error) = create_dir_all(&profile_path) {
        panic!("Failed to create profile directory: {error}")
    }

    // Init profile database path
    let mut profile_database_path = profile_path.clone();

    profile_database_path.push("database.sqlite3");

    // Init database connection
    let profile_database_connection = match Connection::open(profile_database_path) {
        Ok(connection) => Arc::new(connection),
        Err(error) => panic!("Failed to connect profile database: {error}"),
    };

    // Init GTK, start application
    match gtk::init() {
        Ok(_) => App::new(profile_database_connection).run(),
        Err(_) => ExitCode::FAILURE,
    }
}
