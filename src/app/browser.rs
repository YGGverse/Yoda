mod about;
mod database;
mod widget;
mod window;

use about::About;
use database::Database;
use widget::Widget;
use window::Window;

use adw::ApplicationWindow;
use gtk::{
    gio::{Cancellable, File, SimpleAction},
    glib::Variant,
    prelude::{ActionExt, GtkWindowExt},
    FileLauncher,
};
use sqlite::Transaction;
use std::{path::PathBuf, sync::Arc};

pub struct Browser {
    // Components
    // header: Arc<Header>,
    window: Arc<Window>,
    widget: Arc<Widget>,
}

impl Browser {
    // Construct
    pub fn new(
        // Extras
        profile_path: PathBuf,
        // Actions
        action_about: SimpleAction,
        action_debug: SimpleAction,
        action_profile: SimpleAction,
        action_quit: SimpleAction,
        action_update: SimpleAction,
        action_page_new: SimpleAction,
        action_page_close: SimpleAction,
        action_page_close_all: SimpleAction,
        action_page_home: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
        action_page_reload: SimpleAction,
        action_page_pin: SimpleAction,
    ) -> Browser {
        // Init components
        let window = Arc::new(Window::new(
            action_about.clone(),
            action_debug.clone(),
            action_profile.clone(),
            action_quit.clone(),
            action_update.clone(),
            action_page_new.clone(),
            action_page_close.clone(),
            action_page_close_all.clone(),
            action_page_home.clone(),
            action_page_history_back.clone(),
            action_page_history_forward.clone(),
            action_page_reload.clone(),
            action_page_pin.clone(),
        ));

        // Init widget
        let widget = Arc::new(Widget::new(
            window.gobject(),
            &[
                action_about.clone(),
                action_debug.clone(),
                action_profile.clone(),
                action_quit.clone(),
                action_update.clone(),
                action_page_new.clone(),
                action_page_close.clone(),
                action_page_close_all.clone(),
                action_page_home.clone(),
                action_page_history_back.clone(),
                action_page_history_forward.clone(),
                action_page_reload.clone(),
                action_page_pin.clone(),
            ],
        ));

        // Init events
        action_about.connect_activate({
            let window = window.clone();
            move |_, _| {
                About::new().present(Some(window.gobject()));
            }
        });

        action_debug.connect_activate({
            let widget = widget.clone();
            move |_, _| {
                widget.gobject().emit_enable_debugging(true);
            }
        });

        action_profile.connect_activate({
            move |_, _| {
                FileLauncher::new(Some(&File::for_path(&profile_path))).launch(
                    None::<&gtk::Window>,
                    None::<&Cancellable>,
                    |result| {
                        if let Err(error) = result {
                            // @TODO
                            println!("Could not delegate launch action: {error}")
                        }
                    },
                );
            }
        });

        action_quit.connect_activate({
            let widget = widget.clone();
            move |_, _| {
                widget.gobject().close();
            }
        });

        action_update.connect_activate({
            let window = window.clone();
            move |_, this| window.update(string_from_variant(this).as_str())
        });

        action_page_new.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_append(None);
            }
        });

        action_page_close.connect_activate({
            let window = window.clone();
            move |this, _| {
                window.tab_close(page_position_from_action_state(this));
            }
        });

        action_page_close_all.connect_activate({
            let window = window.clone();
            move |_, _| {
                window.tab_close_all();
            }
        });

        action_page_home.connect_activate({
            let window = window.clone();
            move |this, _| {
                window.tab_page_navigation_home(page_position_from_action_state(this));
            }
        });

        action_page_history_back.connect_activate({
            let window = window.clone();
            move |this, _| {
                window.tab_page_navigation_history_back(page_position_from_action_state(this));
            }
        });

        action_page_history_forward.connect_activate({
            let window = window.clone();
            move |this, _| {
                window.tab_page_navigation_history_forward(page_position_from_action_state(this));
            }
        });

        action_page_reload.connect_activate({
            let window = window.clone();
            move |this, _| window.tab_page_navigation_reload(page_position_from_action_state(this))
        });

        action_page_pin.connect_activate({
            let window = window.clone();
            move |this, _| {
                window.tab_pin(page_position_from_action_state(this));
            }
        });

        // Return new activated `Self`
        Self {
            widget,
            // header,
            window,
        }
    }

    // Actions
    pub fn clean(&self, transaction: &Transaction, app_id: &i64) -> Result<(), String> {
        match Database::records(transaction, app_id) {
            Ok(records) => {
                for record in records {
                    match Database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
                            self.window.clean(transaction, &record.id)?;
                            self.widget.clean(transaction, &record.id)?;

                            /* @TODO
                            self.header.clean(transaction, &record.id)?; */
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn restore(&self, transaction: &Transaction, app_id: &i64) -> Result<(), String> {
        match Database::records(transaction, app_id) {
            Ok(records) => {
                for record in records {
                    // Delegate restore action to childs
                    self.widget.restore(transaction, &record.id)?;
                    self.window.restore(transaction, &record.id)?;

                    /* @TODO
                    self.header.restore(transaction, &record.id)?; */
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn save(&self, transaction: &Transaction, app_id: &i64) -> Result<(), String> {
        match Database::add(transaction, app_id) {
            Ok(_) => {
                let id = Database::last_insert_id(transaction);

                // Delegate save action to childs
                self.widget.save(transaction, &id)?;
                self.window.save(transaction, &id)?;

                /* @TODO
                self.header.save(transaction, &id)?; */
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn init(&self) {
        self.window.init();
    }

    // Getters
    pub fn gobject(&self) -> &ApplicationWindow {
        &self.widget.gobject()
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = Database::init(&tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    /* @TODO
    header::migrate(&tx)?; */
    window::migrate(&tx)?;
    widget::migrate(&tx)?;

    // Success
    Ok(())
}

// Private helpers @TODO move outside

/// Extract `Optional` page position from C-based
/// [SimpleAction state](https://docs.gtk.org/gio/property.SimpleAction.state.html)
fn page_position_from_action_state(action: &SimpleAction) -> Option<i32> {
    let page_position = action
        .state()
        .expect("Page position required for this action")
        .get::<i32>()
        .expect("Parameter type does not match `i32`");

    if page_position > -1 {
        Some(page_position)
    } else {
        None
    }
}

/// Extract `String` from [Variant](https://docs.gtk.org/glib/struct.Variant.html)
fn string_from_variant(variant: Option<&Variant>) -> String {
    variant
        .expect("Variant required for this action")
        .get::<String>()
        .expect("Parameter type does not match `String`")
}
