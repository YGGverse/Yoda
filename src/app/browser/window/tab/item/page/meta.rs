mod database;
mod redirect;

use redirect::Redirect;

use gtk::glib::GString;
use sqlite::Transaction;
use std::cell::{Cell, RefCell};

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
    pub status: RefCell<Status>,
    pub title: RefCell<GString>,
    pub redirect: RefCell<Vec<Redirect>>,
}

impl Meta {
    // Constructors

    pub fn new(status: Status, title: GString) -> Self {
        Self {
            status: RefCell::new(status),
            title: RefCell::new(title),
            redirect: RefCell::new(Vec::new()),
        }
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

    pub fn add_redirect(
        &self,
        request: GString,
        referrer: Option<GString>,
        is_foreground: bool,
    ) -> &Self {
        self.redirect.borrow_mut().push(Redirect {
            request,
            referrer,
            is_foreground,
            is_processed: Cell::new(false),
        });
        self
    }

    // Getters

    pub fn status(&self) -> Status {
        self.status.borrow().clone()
    }

    pub fn title(&self) -> GString {
        self.title.borrow().clone()
    }

    pub fn redirects(&self) -> usize {
        self.redirect.borrow().len() + 1
    }

    pub fn redirect(&self) -> Option<Redirect> {
        if let Some(redirect) = self.redirect.borrow().last() {
            if !redirect.is_processed.replace(true) {
                return Some(redirect.clone());
            }
        }
        None
    }

    // Actions

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_page_id: &i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_tab_page_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, &record.id) {
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
        match database::select(transaction, app_browser_window_tab_page_id) {
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

        match database::insert(
            transaction,
            app_browser_window_tab_page_id,
            match title.is_empty() {
                true => None,
                false => Some(title.as_str()),
            },
        ) {
            Ok(_) => {
                // let id = database::last_insert_id(transaction);

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
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    // nothing yet..

    // Success
    Ok(())
}
