mod app;

use app::App;
use gtk::glib::ExitCode;

fn main() -> ExitCode {
    App::new().run()
}
