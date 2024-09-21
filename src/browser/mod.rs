mod header;
mod main;

use std::sync::Arc;

use gtk::{
    gio::ActionEntry,
    prelude::{ActionMapExtManual, GtkWindowExt},
    Application, ApplicationWindow,
};

use sqlite::Connection;
pub struct Browser {
    pub widget: Arc<gtk::ApplicationWindow>,
    pub main: Arc<main::Main>,
}

pub fn new(app: &Application, db: &Connection, width: i32, height: i32) -> Browser {
    // Init components
    let main = Arc::new(main::new());

    // Init widget
    let widget = Arc::new(
        ApplicationWindow::builder()
            .default_width(width)
            .default_height(height)
            .application(app)
            .titlebar(&header::new())
            .child(main.widget.as_ref())
            .build(),
    );

    // Init actions
    let action_tab_append = ActionEntry::builder("tab_append")
        .activate({
            let main = main.clone();
            move |_, _, _| {
                main.tab_append();
            }
        })
        .build();

    let action_debug = ActionEntry::builder("debug")
        .activate(|this: &ApplicationWindow, _, _| {
            this.emit_enable_debugging(true);
        })
        .build();

    let action_quit = ActionEntry::builder("quit")
        .activate(|this: &ApplicationWindow, _, _| {
            this.close();
        })
        .build();

    widget.add_action_entries([action_tab_append, action_debug, action_quit]);

    // Done
    Browser { widget, main }
}
