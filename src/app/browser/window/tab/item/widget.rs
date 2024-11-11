mod database;

use database::Database;

use crate::app::browser::window::action::Position;
use adw::{TabPage, TabView};
use gtk::prelude::IsA;
use sqlite::Transaction;

const DEFAULT_TITLE: &str = "New page";

pub struct Widget {
    gobject: TabPage,
}

impl Widget {
    // Constructors

    pub fn new(
        keyword: &str, // ID
        tab_view: &TabView,
        child: &impl IsA<gtk::Widget>,
        title: Option<&str>,
        position: Position,
        state: (bool, bool, bool),
    ) -> Self {
        // Define state variables
        let (is_pinned, is_selected, is_attention) = state;

        // Create new `TabPage` GObject in given `TabView`
        let gobject = match position {
            Position::After => match tab_view.selected_page() {
                Some(page) => add(tab_view, child, tab_view.page_position(&page) + 1),
                None => tab_view.append(child),
            },
            Position::End => tab_view.append(child),
            Position::Number(value) => add(tab_view, child, value),
        };

        // Setup `GObject`
        gobject.set_needs_attention(is_attention);
        gobject.set_keyword(keyword);
        gobject.set_title(match title {
            Some(value) => value,
            None => DEFAULT_TITLE,
        });

        tab_view.set_page_pinned(&gobject, is_pinned);

        if is_selected {
            tab_view.set_selected_page(&gobject);
        }

        // Done
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

/// Create new [TabPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabPage.html)
/// in [TabView](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.TabView.html) at given position
///
/// * if given `position` match pinned tab, GTK will panic with notice:
///   adw_tab_view_insert: assertion 'position >= self->n_pinned_pages'\
///   as the solution, prepend new page after pinned tabs in this case
fn add(tab_view: &TabView, child: &impl IsA<gtk::Widget>, position: i32) -> TabPage {
    if position > tab_view.n_pinned_pages() {
        tab_view.insert(child, position)
    } else {
        tab_view.prepend(child)
    }
}
