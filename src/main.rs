mod app;
mod profile;
mod tool;

use gtk::glib::ExitCode;
use profile::Profile;

fn main() -> ExitCode {
    use app::App;
    use std::rc::Rc;

    if let Err(e) = gtk::init() {
        eprintln!("Failed to initialize GTK: {e}");
        return ExitCode::FAILURE;
    }

    match Profile::init() {
        Ok(profile) => match App::build(&Rc::new(profile)).run() {
            Ok(app) => return app,
            Err(e) => eprintln!("Failed to initialize application: {e}"),
        },
        Err(e) => eprintln!("Failed to initialize profile: {e}"),
    }

    ExitCode::FAILURE
}
