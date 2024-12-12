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

    pub fn to_gemini(&self, resolver_timeout: u32) -> Option<GString> {
        self.gemini(resolver_timeout).and_then(|url| {
            self.widget.entry.set_text(&url);
            Some(url)
        })
    }

    // Getters

    pub fn uri(&self) -> Option<Uri> {
        match Uri::parse(&self.widget.entry.text(), UriFlags::NONE) {
            Ok(uri) => Some(uri),
            _ => None,
        }
    }

    pub fn strip_prefix(&self) -> GString {
        let mut text = self.widget.entry.text();

        if let Some(postfix) = text.strip_prefix("source:") {
            text = postfix.into()
        };

        if let Some(postfix) = text.strip_prefix("download:") {
            text = postfix.into()
        };

        if let Some(postfix) = text.strip_prefix("file://") {
            text = postfix.into()
        };

        if let Some(postfix) = text.strip_prefix("gemini://") {
            text = postfix.into()
        };

        text
    }

    pub fn download(&self) -> GString {
        gformat!("download:{}", self.strip_prefix())
    }

    pub fn source(&self) -> GString {
        gformat!("source:{}", self.strip_prefix())
    }

    pub fn gemini(&self, resolver_timeout: u32) -> Option<GString> {
        // suggest scheme
        let url = gformat!("gemini://{}", self.strip_prefix().trim());

        // setup default resolver
        // * wanted to detect value contain **resolvable** hostname
        let resolver = Resolver::default();
        resolver.set_timeout(resolver_timeout);

        // is connectable
        if let Ok(connectable) = NetworkAddress::parse_uri(&url, 1965) {
            // is resolvable @TODO async
            if resolver
                .lookup_by_name(&connectable.hostname(), Cancellable::NONE)
                .is_ok()
            {
                return Some(url);
            }
        }
        None
    }
}

// Tools
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
