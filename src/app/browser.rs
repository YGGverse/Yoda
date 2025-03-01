mod about;
mod action;
mod database;
mod widget;
pub mod window;

use about::About;
use action::Action;
use widget::Widget;
use window::Window;

use crate::Profile;
use adw::{prelude::AdwDialogExt, AboutDialog};
use gtk::{
    gio::{Cancellable, File},
    prelude::{GtkWindowExt, IsA},
    Application, FileLauncher,
};
use sqlite::Transaction;
use std::rc::Rc;

pub struct Browser {
    pub action: Rc<Action>,
    pub widget: Rc<Widget>,
    pub window: Rc<Window>,
}

impl Browser {
    // Constructors

    /// Build new `Self`
    pub fn build(profile: &Rc<Profile>) -> Browser {
        // Init components
        let action = Rc::new(Action::new());
        let window = Rc::new(Window::build(profile, &action));

        // Init widget
        let widget = Rc::new(Widget::new(
            &window.g_box,
            &[
                // action groups
                (
                    // browser
                    &action.id,
                    action.simple_action_group.clone(),
                ),
                (
                    // window
                    &window.action.id,
                    window.action.simple_action_group.clone(),
                ),
                (
                    // tab
                    &window.tab.action.id,
                    window.tab.action.simple_action_group.clone(),
                ),
            ],
        ));

        // Connect events
        action.about.connect_activate({
            let window = window.clone();
            move || AboutDialog::about().present(Some(&window.g_box))
        });

        action.close.connect_activate({
            let widget = widget.clone();
            move || widget.application_window.close()
        });

        action.debug.connect_activate({
            let widget = widget.clone();
            move || {
                widget.application_window.emit_enable_debugging(true);
            }
        });

        action.escape.connect_activate({
            let widget = widget.clone();
            let window = window.clone();
            move |_, _| {
                window.escape();
                widget.application_window.set_focus(gtk::Window::NONE);
            }
        });

        action.profile.connect_activate({
            let profile = profile.clone();
            move || {
                FileLauncher::new(Some(&File::for_path(profile.config_path.as_path()))).launch(
                    gtk::Window::NONE,
                    Cancellable::NONE,
                    |result| {
                        if let Err(error) = result {
                            println!("{error}")
                        }
                    },
                ); // @TODO move out?
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

    pub fn clean(&self, transaction: &Transaction, app_id: i64) -> Result<(), String> {
        match database::select(transaction, app_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
                            self.window.clean(transaction, record.id)?;
                            self.widget.clean(transaction, record.id)?;

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

    pub fn restore(&self, transaction: &Transaction, app_id: i64) -> Result<(), String> {
        match database::select(transaction, app_id) {
            Ok(records) => {
                for record in records {
                    // Delegate restore action to childs
                    self.widget.restore(transaction, record.id)?;
                    self.window.restore(transaction, record.id)?;

                    /* @TODO
                    self.header.restore(transaction, &record.id)?; */
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn save(&self, transaction: &Transaction, app_id: i64) -> Result<(), String> {
        match database::insert(transaction, app_id) {
            Ok(_) => {
                let id = database::last_insert_id(transaction);

                // Delegate save action to childs
                self.widget.save(transaction, id)?;
                self.window.save(transaction, id)?;

                /* @TODO
                self.header.save(transaction, &id)?; */
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn init(&self, application: Option<&impl IsA<Application>>) -> &Self {
        // Assign browser window to this application
        self.widget.application_window.set_application(application); // @TODO

        // Init main window
        self.window.init();
        self
    }

    pub fn present(&self) -> &Self {
        self.widget.application_window.present();
        self
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
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
