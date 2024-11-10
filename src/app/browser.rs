mod about;
mod action;
mod database;
mod widget;
mod window;

use about::About;
use action::Action;
use database::Database;
use widget::Widget;
use window::Window;

use crate::profile::Profile;
use adw::ApplicationWindow;
use gtk::{
    gio::{Cancellable, File, SimpleAction},
    prelude::{ActionExt, GtkWindowExt, WidgetExt},
    FileLauncher,
};
use sqlite::Transaction;
use std::rc::Rc;

pub struct Browser {
    action: Rc<Action>,
    widget: Rc<Widget>,
    window: Rc<Window>,
}

impl Browser {
    // Construct
    pub fn new(
        profile: Rc<Profile>,
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
        let action = Rc::new(Action::new());
        let window = Rc::new(Window::new(
            action.clone(),
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
        let widget = Rc::new(Widget::new(
            window.gobject(),
            &[
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

        // Connect actions to browser window
        widget
            .gobject()
            .insert_action_group(action.id(), Some(action.gobject()));

        // Connect events
        action.about().connect_activate({
            let window = window.clone();
            move || {
                About::new().present(Some(window.gobject()));
            }
        });

        action.close().connect_activate({
            let widget = widget.clone();
            move || widget.gobject().close()
        });

        action.debug().connect_activate({
            let widget = widget.clone();
            move || {
                widget.gobject().emit_enable_debugging(true);
            }
        });

        action.profile().connect_activate({
            move || {
                FileLauncher::new(Some(&File::for_path(profile.config_path()))).launch(
                    None::<&gtk::Window>,
                    None::<&Cancellable>,
                    |result| {
                        if let Err(error) = result {
                            println!("{error}")
                        }
                    },
                ); // @TODO move out?
            }
        });

        action.update().connect_activate({
            let window = window.clone();
            move |tab_item_id| window.update(tab_item_id)
        });

        // @TODO
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
                window.tab_page_home(page_position_from_action_state(this));
            }
        });

        action_page_history_back.connect_activate({
            let window = window.clone();
            move |this, _| {
                window.tab_page_history_back(page_position_from_action_state(this));
            }
        });

        action_page_history_forward.connect_activate({
            let window = window.clone();
            move |this, _| {
                window.tab_page_history_forward(page_position_from_action_state(this));
            }
        });

        action_page_reload.connect_activate({
            let window = window.clone();
            move |this, _| window.tab_page_reload(page_position_from_action_state(this))
        });

        action_page_pin.connect_activate({
            let window = window.clone();
            move |this, _| {
                window.tab_pin(page_position_from_action_state(this));
            }
        });

        // Return new activated `Self`
        Self {
            action,
            widget,
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

    pub fn update(&self) {
        self.window.update(None);
    }

    // Getters

    pub fn action(&self) -> &Rc<Action> {
        &self.action
    }

    pub fn gobject(&self) -> &ApplicationWindow {
        self.widget.gobject()
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = Database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    /* @TODO
    header::migrate(tx)?; */
    window::migrate(tx)?;
    widget::migrate(tx)?;

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
