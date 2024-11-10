mod database;

use database::Database;

use adw::{TabPage, TabView};
use gtk::prelude::IsA;
use sqlite::Transaction;

const DEFAULT_TITLE: &str = "New page";

pub struct Widget {
    gobject: TabPage,
}

impl Widget {
    // Construct
    pub fn new(
        keyword: &str, // ID
        tab_view: &TabView,
        child: &impl IsA<gtk::Widget>,
        title: Option<&str>,
        position: Option<i32>,
        is_pinned: bool,
        is_selected: bool,
    ) -> Self {
        let gobject = match position {
            Some(value) => {
                // If given `position` match pinned tab, GTK will panic with notice:
                // adw_tab_view_insert: assertion 'position >= self->n_pinned_pages'
                // as the solution, prepend new page after pinned tabs on this case
                if value > tab_view.n_pinned_pages() {
                    tab_view.insert(child, value)
                } else {
                    tab_view.prepend(child)
                }
            }
            None => tab_view.append(child),
        };

        gobject.set_keyword(keyword);

        gobject.set_title(match title {
            Some(value) => value,
            None => DEFAULT_TITLE,
        });

        tab_view.set_page_pinned(&gobject, is_pinned);

        if is_selected {
            tab_view.set_selected_page(&gobject);
        }

        Self { gobject }
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
