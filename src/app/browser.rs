mod header;
mod main;

use header::Header;
use main::Main;

use gtk::{
    gio::SimpleAction,
    prelude::{ActionMapExt, GtkWindowExt},
    ApplicationWindow,
};
use std::sync::Arc;

const DEFAULT_HEIGHT: i32 = 480;
const DEFAULT_WIDTH: i32 = 640;

pub struct Browser {
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
        // Extras
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
            .titlebar(header.widget())
            .child(main.widget())
            .default_height(DEFAULT_HEIGHT)
            .default_width(DEFAULT_WIDTH)
            .build();

        // Return new Browser
        Self {
            // Actions
            action_debug,
            action_quit,
            action_update,
            action_tab_append,
            action_tab_close,
            action_tab_close_all,
            action_tab_page_navigation_base,
            action_tab_page_navigation_history_back,
            action_tab_page_navigation_history_forward,
            action_tab_page_navigation_reload,
            action_tab_pin,
            // db,
            widget,
            // Components
            header,
            main,
        }
    }

    // Actions
    pub fn activate(&self) -> &Self {
        // Assign actions
        self.widget.add_action(self.action_debug.as_ref());
        self.widget.add_action(self.action_quit.as_ref());
        self.widget.add_action(self.action_update.as_ref());
        self.widget.add_action(self.action_tab_append.as_ref());
        self.widget.add_action(self.action_tab_close.as_ref());
        self.widget.add_action(self.action_tab_close_all.as_ref());
        self.widget
            .add_action(self.action_tab_page_navigation_base.as_ref());
        self.widget
            .add_action(self.action_tab_page_navigation_history_back.as_ref());
        self.widget
            .add_action(self.action_tab_page_navigation_history_forward.as_ref());
        self.widget
            .add_action(self.action_tab_page_navigation_reload.as_ref());
        self.widget.add_action(self.action_tab_pin.as_ref());

        // Events
        self.action_debug.connect_activate({
            let widget = self.widget.clone();
            move |_, _| {
                widget.emit_enable_debugging(true);
            }
        });

        self.action_quit.connect_activate({
            let widget = self.widget.clone();
            move |_, _| {
                widget.close();
            }
        });

        self.action_update.connect_activate({
            let header = self.header.clone();
            let main = self.main.clone();
            move |_, _| {
                main.update();
                header.update(main.tab_page_title(), main.tab_page_description());
            }
        });

        self.action_tab_append.connect_activate({
            let main = self.main.clone();
            move |_, _| {
                main.tab_append(None);
            }
        });

        self.action_tab_close.connect_activate({
            let main = self.main.clone();
            move |_, _| {
                main.tab_close();
            }
        });

        self.action_tab_close_all.connect_activate({
            let main = self.main.clone();
            move |_, _| {
                main.tab_close_all();
            }
        });

        self.action_tab_page_navigation_base.connect_activate({
            let main = self.main.clone();
            move |_, _| {
                main.tab_page_navigation_base();
            }
        });

        self.action_tab_page_navigation_history_back
            .connect_activate({
                let main = self.main.clone();
                move |_, _| {
                    main.tab_page_navigation_history_back();
                }
            });

        self.action_tab_page_navigation_history_forward
            .connect_activate({
                let main = self.main.clone();
                move |_, _| {
                    main.tab_page_navigation_history_forward();
                }
            });

        self.action_tab_page_navigation_reload.connect_activate({
            let main = self.main.clone();
            move |_, _| {
                main.tab_page_navigation_reload();
            }
        });

        self.action_tab_pin.connect_activate({
            let main = self.main.clone();
            move |_, _| {
                main.tab_pin();
            }
        });

        &self
    }

    // Getters
    pub fn widget(&self) -> &ApplicationWindow {
        &self.widget
    }
}
