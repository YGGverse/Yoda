mod action;
mod app;
mod profile;

use crate::profile::Profile;
use app::App;
use gtk::glib::ExitCode;
use std::rc::Rc;

const VENDOR: &str = "YGGverse";
const APP_ID: &str = "Yoda";
const BRANCH: &str = "master";

fn main() -> ExitCode {
    match gtk::init() {
        Ok(_) => App::new(Rc::new(Profile::new(
            VENDOR,
            APP_ID,
            BRANCH,
            format!(
                "{}.{}",
                env!("CARGO_PKG_VERSION_MAJOR"),
                env!("CARGO_PKG_VERSION_MINOR")
            )
            .as_str(),
        )))
        .run(),
        Err(_) => ExitCode::FAILURE,
    }
}
