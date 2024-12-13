mod database;
mod widget;

use widget::Widget;

use crate::app::browser::{window::tab::item::Action as TabAction, Action as BrowserAction};
use gtk::{
    gio::{Cancellable, NetworkAddress, Resolver},
    glib::{gformat, GString, Uri, UriFlags},
    prelude::{EditableExt, NetworkAddressExt, ResolverExt},
};
use sqlite::Transaction;
use std::rc::Rc;

// Main
pub struct Request {
    pub widget: Rc<Widget>,
}

impl Request {
    // Construct
    pub fn new(action: (Rc<BrowserAction>, Rc<TabAction>)) -> Self {
        Self {
            widget: Rc::new(Widget::new(action)),
        }
    }

    // Actions
    pub fn update(&self, progress_fraction: Option<f64>, is_identity_active: bool) {
        self.widget.update(progress_fraction, is_identity_active);
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_tab_item_page_navigation_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to the item childs
                            self.widget.clean(transaction, &record.id)?;
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
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_tab_item_page_navigation_id) {
            Ok(records) => {
                for record in records {
                    // Delegate restore action to the item childs
                    self.widget.restore(transaction, &record.id)?;
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String> {
        match database::insert(transaction, app_browser_window_tab_item_page_navigation_id) {
            Ok(_) => {
                let id = database::last_insert_id(transaction);

                // Delegate save action to childs
                self.widget.save(transaction, &id)?;
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    // Setters

    pub fn to_download(&self) {
        self.widget.entry.set_text(&self.download());
    }

    pub fn to_source(&self) {
        self.widget.entry.set_text(&self.source());
    }

    /// Asynchronously try replace `Self` entry value with valid, resolvable Gemini request
    /// * callback with `None` if current value does not compatible with Gemini scheme
    pub fn to_gemini_async(
        &self,
        resolver_timeout: u32,
        cancellable: Option<&Cancellable>,
        callback: impl FnOnce(Option<GString>) + 'static,
    ) {
        self.gemini_async(resolver_timeout, cancellable, {
            let entry = self.widget.entry.clone();
            move |result| {
                callback(match result {
                    Some(url) => {
                        entry.set_text(&url);
                        Some(url)
                    }
                    None => None,
                })
            }
        });
    }

    // Getters

    /// Get current request value in [Uri](https://docs.gtk.org/glib/struct.Uri.html) format
    /// * `strip_prefix` on parse
    pub fn uri(&self) -> Option<Uri> {
        match Uri::parse(&strip_prefix(self.widget.entry.text()), UriFlags::NONE) {
            Ok(uri) => Some(uri),
            _ => None,
        }
    }

    /// Get current request value without system prefix
    /// * the `prefix` is not `scheme`
    pub fn strip_prefix(&self) -> GString {
        strip_prefix(self.widget.entry.text())
    }

    /// Get request value in `download:` format
    pub fn download(&self) -> GString {
        gformat!("download:{}", self.strip_prefix())
    }

    /// Get request value in `source:` format
    pub fn source(&self) -> GString {
        gformat!("source:{}", self.strip_prefix())
    }

    /// Asynchronously get valid, resolvable Gemini request for current `Self` entry value
    /// * callback with `None` if current value does not compatible with Gemini scheme
    pub fn gemini_async(
        &self,
        resolver_timeout: u32,
        cancellable: Option<&Cancellable>,
        callback: impl FnOnce(Option<GString>) + 'static,
    ) {
        // suggest scheme
        let url = gformat!("gemini://{}", self.strip_prefix().trim());

        // setup default resolver
        // * wanted to detect value contain **resolvable** hostname
        let resolver = Resolver::default();
        resolver.set_timeout(resolver_timeout);

        // is connectable
        if let Ok(connectable) = NetworkAddress::parse_uri(&url, 1965) {
            // is resolvable
            resolver.lookup_by_name_async(&connectable.hostname(), cancellable, move |result| {
                callback(match result {
                    Ok(_) => Some(url),
                    Err(_) => None,
                })
            })
        }
    }
}

// Tools

/// Strip system prefix from request string
/// * the `prefix` is not `scheme`
pub fn strip_prefix(mut request: GString) -> GString {
    if let Some(postfix) = request.strip_prefix("source:") {
        request = postfix.into()
    };

    if let Some(postfix) = request.strip_prefix("download:") {
        request = postfix.into()
    };

    request
}

pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    widget::migrate(tx)?;

    // Success
    Ok(())
}
