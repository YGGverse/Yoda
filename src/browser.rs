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

const DEFAULT_HEIGHT: i32 = 480;
const DEFAULT_WIDTH: i32 = 640;

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
        // Actions
        action_debug: Arc<SimpleAction>,
        action_quit: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
        action_tab_append: Arc<SimpleAction>,
        action_tab_close: Arc<SimpleAction>,
        action_tab_close_all: Arc<SimpleAction>,
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_tab_pin: Arc<SimpleAction>,
    ) -> Browser {
        // Init database
        // let db = db::Browser::new(connection); @TODO

        // Init components
        let header = Arc::new(Header::new(
            action_debug.clone(),
            action_quit.clone(),
            action_tab_append.clone(),
            action_tab_close.clone(),
            action_tab_close_all.clone(),
            action_tab_page_navigation_base.clone(),
            action_tab_page_navigation_history_back.clone(),
            action_tab_page_navigation_history_forward.clone(),
            action_tab_page_navigation_reload.clone(),
            action_tab_pin.clone(),
        ));

        let main = Arc::new(Main::new(
            action_tab_page_navigation_base.clone(),
            action_tab_page_navigation_history_back.clone(),
            action_tab_page_navigation_history_forward.clone(),
            action_tab_page_navigation_reload.clone(),
            action_update.clone(),
        ));

        // Init widget
        let widget = ApplicationWindow::builder()
            .application(app)
            .titlebar(header.widget())
            .child(main.widget())
            .default_height(DEFAULT_HEIGHT)
            .default_width(DEFAULT_WIDTH)
            .build();

        widget.add_action(action_debug.as_ref());
        widget.add_action(action_quit.as_ref());
        widget.add_action(action_update.as_ref());

        widget.add_action(action_tab_append.as_ref());
        widget.add_action(action_tab_close.as_ref());
        widget.add_action(action_tab_close_all.as_ref());
        widget.add_action(action_tab_page_navigation_base.as_ref());
        widget.add_action(action_tab_page_navigation_history_back.as_ref());
        widget.add_action(action_tab_page_navigation_history_forward.as_ref());
        widget.add_action(action_tab_page_navigation_reload.as_ref());
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

        action_tab_page_navigation_base.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_page_navigation_base();
            }
        });

        action_tab_page_navigation_history_back.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_page_navigation_history_back();
            }
        });

        action_tab_page_navigation_history_forward.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_page_navigation_history_forward();
            }
        });

        action_tab_page_navigation_reload.connect_activate({
            let main = main.clone();
            move |_, _| {
                main.tab_page_navigation_reload();
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
