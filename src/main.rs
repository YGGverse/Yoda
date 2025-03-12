mod app;
mod profile;
mod tool;

use gtk::glib::ExitCode;
use profile::Profile;

fn main() -> ExitCode {
    use app::App;

    if let Err(e) = gtk::init() {
        eprintln!("Failed to initialize GTK: {e}");
        return ExitCode::FAILURE;
    }

    match Profile::init() {
        Ok(profile) => match App::build(profile) {
            Ok(app) => match app.run() {
                Ok(run) => return run,
                Err(e) => eprintln!("Failed to run application: {e}"),
            },
            Err(e) => eprintln!("Failed to build application: {e}"),
        },
        Err(e) => eprintln!("Failed to initialize profile: {e}"),
    }

    ExitCode::FAILURE
}
