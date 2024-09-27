mod db;
mod header;
mod main;

use header::Header;
use main::Main;

use gtk::{
    gio::ActionEntry,
    prelude::{ActionMapExtManual, GtkWindowExt},
    Application, ApplicationWindow,
};
use std::sync::Arc;

pub struct Browser {
    // Extras
    // db: db::Browser,
    widget: ApplicationWindow,
    // Components
    header: Arc<Header>,
    main: Arc<Main>,
}

impl Browser {
    // Construct
    pub fn new(
        app: &Application,
        // connection: Arc<sqlite::Connection>,
        default_width: i32,
        default_height: i32,
    ) -> Browser {
        // Init components
        // let db = db::Browser::new(connection);
        let header = Arc::new(header::Header::new());
        let main = Arc::new(main::Main::new());

        let widget = ApplicationWindow::builder()
            .application(app)
            .default_width(default_width)
            .default_height(default_height)
            .titlebar(header.widget())
            .child(main.widget())
            .build();

        // Init actions
        widget.add_action_entries([
            ActionEntry::builder("update")
                .activate({
                    let header = header.clone();
                    let main = main.clone();
                    move |_, _, _| {
                        main.update();
                        header.update(main.tab_page_title(), main.tab_page_description());
                    }
                })
                .build(),
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
                        main.tab_append(None);
                    }
                })
                .build(),
            ActionEntry::builder("tab_page_reload")
                .activate({
                    let main = main.clone();
                    move |_, _, _| {
                        main.tab_page_reload();
                    }
                })
                .build(),
            ActionEntry::builder("tab_close")
                .activate({
                    let main = main.clone();
                    move |_, _, _| {
                        main.tab_close();
                    }
                })
                .build(),
            ActionEntry::builder("tab_close_all")
                .activate({
                    let main = main.clone();
                    move |_, _, _| {
                        main.tab_close_all();
                    }
                })
                .build(),
            ActionEntry::builder("tab_pin")
                .activate({
                    let main = main.clone();
                    move |_, _, _| {
                        main.tab_pin();
                    }
                })
                .build(),
        ]);

        // Return
        Self {
            // db,
            widget,
            header,
            main,
        }
    }

    // Getters
    pub fn widget(&self) -> &ApplicationWindow {
        &self.widget
    }
}
