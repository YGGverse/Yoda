mod about;
mod action;
mod bookmarks;
mod database;
mod history;
mod proxy;
mod widget;
pub mod window;

use about::About;
use action::Action;
use bookmarks::Bookmarks;
use history::History;
use proxy::Proxy;
use widget::Widget;
use window::Window;

use crate::Profile;
use adw::{AboutDialog, Application, PreferencesDialog, prelude::AdwDialogExt};
use anyhow::Result;
use gtk::{
    FileLauncher,
    gio::{Cancellable, File},
    prelude::GtkWindowExt,
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
            &window,
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
            let w = window.clone();
            move || AboutDialog::about().present(Some(&w.g_box))
        });

        action.close.connect_activate({
            let w = widget.clone();
            move || w.application_window.close()
        });

        action.debug.connect_activate({
            let w = widget.clone();
            move || {
                w.application_window.emit_enable_debugging(true);
            }
        });

        action.profile.connect_activate({
            let p = profile.clone();
            move || {
                FileLauncher::new(Some(&File::for_path(p.config_path.as_path()))).launch(
                    adw::Window::NONE,
                    Cancellable::NONE,
                    |r| {
                        if let Err(e) = r {
                            println!("{e}")
                        }
                    },
                ); // @TODO move out?
            }
        });

        action.history.connect_activate({
            let p = profile.clone();
            let w = window.clone();
            move || PreferencesDialog::history(&w.action, &p).present(Some(&w.g_box))
        });

        action.bookmarks.connect_activate({
            let p = profile.clone();
            let w = window.clone();
            move || PreferencesDialog::bookmarks(&w.action, &p).present(Some(&w.g_box))
        });

        action.proxy.connect_activate({
            let p = profile.clone();
            let w = window.clone();
            move || {
                PreferencesDialog::proxy(&p, {
                    let w = w.clone();
                    move || {
                        w.tab
                            .items()
                            .into_iter()
                            .for_each(|i| i.page.navigation.request.refresh_proxy_resolver())
                    }
                })
                .present(Some(&w.g_box))
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

    pub fn clean(&self, transaction: &Transaction, app_id: i64) -> Result<()> {
        for r in database::select(transaction, app_id)? {
            database::delete(transaction, r.id)?;
            // Delegate clean action to childs
            self.window.clean(transaction, r.id)?;
            self.widget.clean(transaction, r.id)?;
            /* @TODO
            self.header.clean(transaction, &record.id)?; */
        }
        Ok(())
    }

    pub fn restore(&self, transaction: &Transaction, app_id: i64) -> Result<()> {
        for r in database::select(transaction, app_id)? {
            // Delegate restore action to childs
            self.widget.restore(transaction, r.id)?;
            self.window.restore(transaction, r.id)?;
            /* @TODO
            self.header.restore(transaction, &r.id)?; */
        }
        Ok(())
    }

    pub fn save(&self, transaction: &Transaction, app_id: i64) -> Result<()> {
        let id = database::insert(transaction, app_id)?;
        // Delegate save action to childs
        self.widget.save(transaction, id)?;
        self.window.save(transaction, id)?;
        /* @TODO
        self.header.save(transaction, &id)?; */
        Ok(())
    }

    pub fn init(&self, application: Option<&Application>) -> &Self {
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
pub fn migrate(tx: &Transaction) -> Result<()> {
    // Migrate self components
    database::init(tx)?;

    // Delegate migration to childs
    /* @TODO
    header::migrate(tx)?; */
    window::migrate(tx)?;
    widget::migrate(tx)?;

    // Success
    Ok(())
}
