mod database;
mod redirect;

use database::Database;
use redirect::Redirect;

use gtk::glib::GString;
use sqlite::Transaction;
use std::{cell::RefCell, sync::Arc};

#[derive(Debug, Clone)]
pub enum Status {
    Complete,
    Connected,
    Connecting,
    Failure,
    Input,
    New,
    ProxyNegotiated,
    ProxyNegotiating,
    Redirect,
    Reload,
    Resolved,
    Resolving,
    SessionRestore,
    SessionRestored,
    Success,
    TlsHandshaked,
    TlsHandshaking,
}

pub struct Meta {
    status: RefCell<Status>,
    title: RefCell<GString>,
    redirect: RefCell<Option<Redirect>>,
    redirect_count: RefCell<Option<i8>>,
}

impl Meta {
    // Constructors

    pub fn new_arc(status: Status, title: GString) -> Arc<Self> {
        Arc::new(Self {
            status: RefCell::new(status),
            title: RefCell::new(title),
            redirect: RefCell::new(None),
            redirect_count: RefCell::new(None),
        })
    }

    // Setters

    pub fn set_status(&self, status: Status) -> &Self {
        self.status.replace(status);
        self
    }

    pub fn set_title(&self, title: &str) -> &Self {
        self.title.replace(GString::from(title));
        self
    }

    pub fn set_redirect(&self, request: GString, is_foreground: bool) -> &Self {
        self.redirect
            .replace(Some(Redirect::new(request, is_foreground)));
        self
    }

    pub fn set_redirect_count(&self, redirect_count: Option<i8>) -> &Self {
        self.redirect_count.replace(redirect_count);
        self
    }

    pub fn unset_redirect_count(&self) -> &Self {
        if self.redirect_count.borrow().is_some() {
            self.set_redirect_count(None);
        }
        self
    }

    /* @TODO not in use
    pub fn unset_redirect(&self) -> &Self {
        self.redirect.replace(None);
        self
    } */

    // Getters

    pub fn status(&self) -> Status {
        self.status.borrow().clone()
    }

    pub fn title(&self) -> GString {
        self.title.borrow().clone()
    }

    pub fn redirect_count(&self) -> Option<i8> {
        self.redirect_count.borrow().clone()
    }

    /// WARNING!
    ///
    /// This function **take** the `Redirect` without clone semantics
    pub fn take_redirect(&self) -> Option<Redirect> {
        self.redirect.take()
    }

    // Actions

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_page_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_tab_page_id) {
            Ok(records) => {
                for record in records {
                    match Database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to the item childs
                            // nothing yet..
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn restore(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_page_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_tab_page_id) {
            Ok(records) => {
                for record in records {
                    // Record value can be stored as NULL
                    if let Some(title) = record.title {
                        self.set_title(title.as_str());
                    }

                    // Delegate restore action to the item childs
                    // nothing yet..
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_page_id: &i64,
    ) -> Result<(), String> {
        // Keep value in memory until operation complete
        let title = self.title();

        match Database::add(
            transaction,
            app_browser_window_tab_page_id,
            match title.is_empty() {
                true => None,
                false => Some(title.as_str()),
            },
        ) {
            Ok(_) => {
                // let id = Database::last_insert_id(transaction);

                // Delegate save action to childs
                // nothing yet..
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = Database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    // nothing yet..

    // Success
    Ok(())
}
