mod app;
mod profile;
mod tool;

use app::App;
use gtk::glib::ExitCode;
use profile::Profile;
use std::rc::Rc;

fn main() -> ExitCode {
    match gtk::init() {
        Ok(_) => App::build(&Rc::new(Profile::new().unwrap())).run(),
        Err(_) => ExitCode::FAILURE,
    }
}
