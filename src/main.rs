#[path = "app/browser.rs"] mod browser;

use gtk::prelude::{
    ApplicationExt,
    ApplicationExtManual,
    GtkWindowExt
};

use gtk::{
    Application,
    glib
};

fn main() -> glib::ExitCode
{
    let app = Application::builder().application_id(
        "io.github.yggverse.Yoda.app"
    ).build();

    app.connect_activate(
        |app|
        {
            browser::new(
                app,
                640,
                480
            ).present();
        }
    );

    app.run()
}