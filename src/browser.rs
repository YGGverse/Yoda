mod db;
mod header;
mod main;

use header::Header;
use main::Main;

use gtk::{
    gio::SimpleAction,
    prelude::{ActionMapExt, GtkWindowExt},
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
        let action_debug = Arc::new(SimpleAction::new("debug", None));
        let action_quit = Arc::new(SimpleAction::new("quit", None));
        let action_update = Arc::new(SimpleAction::new("update", None));

        let action_tab_append = Arc::new(SimpleAction::new("tab_append", None));
        let action_tab_close = Arc::new(SimpleAction::new("tab_close", None));
        let action_tab_close_all = Arc::new(SimpleAction::new("tab_close_all", None));
        let action_tab_page_reload = Arc::new(SimpleAction::new("tab_page_reload", None));
        let action_tab_pin = Arc::new(SimpleAction::new("tab_pin", None));

        // Init components
        // let db = db::Browser::new(connection);
        let header = Arc::new(Header::new(
            action_debug.clone(),
            action_quit.clone(),
            action_tab_append.clone(),
            action_tab_close.clone(),
            action_tab_close_all.clone(),
            action_tab_page_reload.clone(),
            action_tab_pin.clone(),
        ));

        let main = Arc::new(Main::new(
            action_tab_page_reload.clone(),
            action_update.clone(),
        ));

        // Init widget
        let widget = ApplicationWindow::builder()
            .application(app)
            .default_width(default_width)
            .default_height(default_height)
            .titlebar(header.widget())
            .child(main.widget())
            .build();

        widget.add_action(action_debug.as_ref());
        widget.add_action(action_quit.as_ref());
        widget.add_action(action_update.as_ref());

        widget.add_action(action_tab_append.as_ref());
        widget.add_action(action_tab_close.as_ref());
        widget.add_action(action_tab_close_all.as_ref());
        widget.add_action(action_tab_page_reload.as_ref());
        widget.add_action(action_tab_pin.as_ref());

        // Init events
        action_debug.connect_activate({
            let widget = widget.clone();
            move |_, _| {
                widget.emit_enable_debugging(true);
            }
        });

        action_quit.connect_activate({
            let widget = widget.clone();
            move |_, _| {
                widget.close();
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

        action_tab_append.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_append(None);
            }
        });

        action_tab_close.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_close();
            }
        });

        action_tab_close_all.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_close_all();
            }
        });

        action_tab_page_reload.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_page_reload();
            }
        });

        action_tab_pin.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_pin();
            }
        });

        // Return activated browser struct
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
