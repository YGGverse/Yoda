use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

fn main() -> glib::ExitCode
{
    let app = Application::builder()
        .application_id("io.github.yggverse.Yoda.app")
        .build();

    app.connect_activate(
        |app|
        {
            let window = ApplicationWindow::builder()
                .application(app)
                .default_width(640)
                .default_height(480)
                .title("Yoda")
                .build();

            window.present();
        }
    );

    app.run()
}