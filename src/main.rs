#[path = "app/browser.rs"] mod browser;

use gtk::prelude::*;
use gtk::{glib, Application};

fn main() -> glib::ExitCode
{
    let app = Application::builder().application_id(
        "io.github.yggverse.Yoda.app"
    ).build();

    app.connect_activate(
        |app|
        {
            browser::new(
                app
            ).present();
        }
    );

    app.run()
}