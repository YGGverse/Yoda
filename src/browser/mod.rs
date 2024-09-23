mod db;
mod header;
mod main;
mod widget;

use gtk::gio::ActionEntry;
use gtk::{Application, ApplicationWindow};
use std::sync::Arc;

use gtk::prelude::{ActionMapExtManual, GtkWindowExt};

pub struct Browser {
    // Extras
    db: db::Browser,
    widget: widget::Browser,
    // Components
    header: Arc<header::Header>,
    main: Arc<main::Main>,
}

impl Browser {
    // Construct
    pub fn new(
        app: &Application,
        connection: Arc<sqlite::Connection>,
        default_width: i32,
        default_height: i32,
    ) -> Arc<Browser> {
        // Init components
        let db = db::Browser::new(connection);
        let header = header::Header::new();
        let main = main::Main::new();

        let widget = widget::Browser::new(
            app,
            header.widget().gtk(),
            main.widget().gtk(),
            default_width,
            default_height,
        );

        // Init actions @TODO separated module
        widget.window().add_action_entries([
            ActionEntry::builder("debug")
                .activate(|this: &ApplicationWindow, _, _| {
                    this.emit_enable_debugging(true);
                })
                .build(),
            ActionEntry::builder("quit")
                .activate(|this: &ApplicationWindow, _, _| {
                    this.close();
                })
                .build(),
            ActionEntry::builder("tab_append")
                .activate({
                    let main = main.clone();
                    move |_, _, _| {
                        main.tab_append();
                    }
                })
                .build(),
        ]);

        // Return
        Arc::new(Self {
            db,
            widget,
            header,
            main,
        })
    }

    // Getters
    pub fn widget(&self) -> &widget::Browser {
        &self.widget
    }
}
