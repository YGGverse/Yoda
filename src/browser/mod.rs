mod db;
mod header;
mod main;
mod widget;

use std::sync::Arc;

use gtk::prelude::{ActionMapExtManual, GtkWindowExt};

pub struct Browser {
    db: db::Browser,
    header: header::Header,
    main: Arc<main::Main>,
    widget: widget::Browser,
}

impl Browser {
    // Construct new browser
    pub fn new(
        app: &gtk::Application,
        connection: std::sync::Arc<sqlite::Connection>, // @TODO glib clone macro?
        default_width: i32,
        default_height: i32,
    ) -> Browser {
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
        widget.gtk().add_action_entries([
            gtk::gio::ActionEntry::builder("debug")
                .activate(|this: &gtk::ApplicationWindow, _, _| {
                    this.emit_enable_debugging(true);
                })
                .build(),
            gtk::gio::ActionEntry::builder("quit")
                .activate(|this: &gtk::ApplicationWindow, _, _| {
                    this.close();
                })
                .build(),
            gtk::gio::ActionEntry::builder("tab_append")
                .activate({
                    let main = main.clone();
                    move |_, _, _| {
                        main.tab_append();
                    }
                })
                .build(),
        ]);

        // Return
        Self {
            db,
            header,
            main,
            widget,
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Browser {
        &self.widget
    }
}
