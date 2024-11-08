mod app;
mod profile;

use app::App;
use gtk::glib::ExitCode;

fn main() -> ExitCode {
    match gtk::init() {
        Ok(_) => App::new().run(),
        Err(_) => ExitCode::FAILURE,
    }
}
