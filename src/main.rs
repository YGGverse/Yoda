mod app;
mod profile;
mod tool;

use app::App;
use gtk::glib::ExitCode;
use profile::Profile;
use std::rc::Rc;

fn main() -> ExitCode {
    match Profile::new() {
        Ok(profile) => {
            if let Err(e) = gtk::init() {
                eprintln!("Failed to initialize GTK: {e}");
                return ExitCode::FAILURE;
            }
            match App::build(&Rc::new(profile)).run() {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Failed to initialize application: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize profile: {e}");
            ExitCode::FAILURE
        }
    }
}
