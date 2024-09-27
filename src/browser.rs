mod db;
mod header;
mod main;

use header::Header;
use main::Main;

use gtk::{
    gio::{ActionEntry, SimpleAction},
    prelude::{ActionMapExt, ActionMapExtManual, GtkWindowExt},
    Application, ApplicationWindow,
};
use std::sync::Arc;

pub struct Browser {
    // Extras
    // db: db::Browser,
    widget: ApplicationWindow,
    // Components
    // header: Arc<Header>,
    // main: Arc<Main>,
}

impl Browser {
    // Construct
    pub fn new(
        app: &Application,
        // connection: Arc<sqlite::Connection>,
        default_width: i32,
        default_height: i32,
    ) -> Browser {
        // Init window actions
        let action_debug = SimpleAction::new("debug", None);
        let action_quit = SimpleAction::new("quit", None);
        let action_update = SimpleAction::new("update", None);

        // Init components
        // let db = db::Browser::new(connection);
        let header = Arc::new(Header::new(&action_debug, &action_quit));

        let main = Arc::new(Main::new(&action_debug, &action_quit, &action_update));

        // Init widget
        let widget = ApplicationWindow::builder()
            .application(app)
            .default_width(default_width)
            .default_height(default_height)
            .titlebar(header.widget())
            .child(main.widget())
            .build();

        widget.add_action(&action_debug);
        widget.add_action(&action_quit);
        widget.add_action(&action_update);

        // Init events
        action_debug.connect_activate({
            let target = widget.clone();
            move |_, _| {
                target.emit_enable_debugging(true);
            }
        });

        action_quit.connect_activate({
            let target = widget.clone();
            move |_, _| {
                target.close();
            }
        });

        action_update.connect_activate({
            let header = header.clone();
            let main = main.clone();
            move |_, _| {
                main.update();
                header.update(main.tab_page_title(), main.tab_page_description());
            }
        });

        // Init actions @TODO
        widget.add_action_entries([
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
            // header,
            // main,
        }
    }

    // Getters
    pub fn widget(&self) -> &ApplicationWindow {
        &self.widget
    }
}
