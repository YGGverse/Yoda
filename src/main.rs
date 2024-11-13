mod app;
mod profile;

use app::App;
use profile::Profile;

use gtk::glib::ExitCode;
use std::rc::Rc;

fn main() -> ExitCode {
    match gtk::init() {
        Ok(_) => App::new(Rc::new(Profile::new())).run(),
        Err(_) => ExitCode::FAILURE,
    }
}
