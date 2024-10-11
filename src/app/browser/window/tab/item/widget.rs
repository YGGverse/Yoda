mod database;

use database::Database;

use adw::{TabPage, TabView};
use gtk::Box;
use sqlite::Transaction;
use std::sync::Arc;

const DEFAULT_TITLE: &str = "New page";

pub struct Widget {
    gobject: TabPage,
}

impl Widget {
    // Construct
    pub fn new_arc(
        keyword: &str, // ID
        tab_view: &TabView,
        page: &Box,
        title: Option<&str>,
        is_pinned: bool,
        is_selected: bool,
    ) -> Arc<Self> {
        let gobject = tab_view.append(page);

        gobject.set_keyword(keyword);

        gobject.set_title(match title {
            Some(value) => value,
            None => DEFAULT_TITLE,
        });

        tab_view.set_page_pinned(&gobject, is_pinned);

        if is_selected {
            tab_view.set_selected_page(&gobject);
        }

        Arc::new(Self { gobject })
    }

    // Actions
    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_tab_item_id) {
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
        app_browser_window_tab_item_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_tab_item_id) {
            Ok(records) => {
                for record in records {
                    // Record value can be stored as NULL
                    if let Some(title) = record.title {
                        self.gobject.set_title(title.as_str());
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
        app_browser_window_tab_item_id: &i64,
    ) -> Result<(), String> {
        // Keep value in memory until operation complete
        let title = self.gobject.title();

        match Database::add(
            transaction,
            app_browser_window_tab_item_id,
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

    // Getters
    pub fn gobject(&self) -> &TabPage {
        &self.gobject
    }

    // Tools
    pub fn migrate(tx: &Transaction) -> Result<(), String> {
        // Migrate self components
        if let Err(e) = Database::init(&tx) {
            return Err(e.to_string());
        }

        // Delegate migration to childs
        // nothing yet..

        // Success
        Ok(())
    }
}
