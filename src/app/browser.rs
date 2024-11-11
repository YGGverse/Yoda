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
    gio::{Cancellable, File},
    prelude::GtkWindowExt,
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
    pub fn new(profile: Rc<Profile>) -> Browser {
        // Init components
        let action = Rc::new(Action::new());
        let window = Rc::new(Window::new(action.clone()));

        // Init widget
        let widget = Rc::new(Widget::new(
            window.widget().gobject(),
            &[
                // Connect action groups (to apply accels)
                (
                    // Browser
                    action.id(),
                    action.gobject().clone(),
                ),
                (
                    // Window
                    window.action().id(),
                    window.action().gobject().clone(),
                ),
            ],
        ));

        // Connect events
        action.about().connect_activate({
            let window = window.clone();
            move || {
                About::new().present(Some(window.widget().gobject()));
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

    pub fn window(&self) -> &Rc<Window> {
        &self.window
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
