mod database;
mod page;
mod widget;

use database::Database;
use page::Page;
use widget::Widget;

use adw::{TabPage, TabView};
use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, GString},
};
use sqlite::Transaction;
use std::sync::Arc;

pub struct Item {
    // Auto-generated unique item ID
    // useful as widget name in GTK actions callback
    id: GString,
    // Components
    page: Arc<Page>,
    widget: Arc<Widget>,
}

impl Item {
    // Construct
    pub fn new_arc(
        tab_view: &TabView,
        // Actions
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
        // Options
        is_pinned: bool,
        is_selected: bool,
    ) -> Arc<Self> {
        // Generate unique ID for new page components
        let id = uuid_string_random();

        // Init components
        let page = Page::new_arc(
            id.clone(),
            action_tab_page_navigation_base.clone(),
            action_tab_page_navigation_history_back.clone(),
            action_tab_page_navigation_history_forward.clone(),
            action_tab_page_navigation_reload.clone(),
            action_update.clone(),
        );

        let widget = Widget::new_arc(
            id.as_str(),
            tab_view,
            page.gobject(),
            None,
            is_pinned,
            is_selected,
        ); // @TODO

        // Return struct
        Arc::new(Self { id, page, widget })
    }

    // Actions
    pub fn page_navigation_base(&self) {
        self.page.navigation_base()
    }

    pub fn page_navigation_history_back(&self) {
        self.page.navigation_history_back()
    }

    pub fn page_navigation_history_forward(&self) {
        self.page.navigation_history_forward()
    }

    pub fn page_navigation_reload(&self) {
        self.page.navigation_reload()
    }

    pub fn page_navigation_request_grab_focus(&self) {
        self.page.navigation_request_grab_focus()
    }

    pub fn update(&self) {
        // Update child components
        self.page.update();

        // Update tab loading indicator
        self.widget.gobject().set_loading(self.page_is_loading());
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_tab_id) {
            Ok(records) => {
                for record in records {
                    match Database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to the item childs
                            self.widget.clean(transaction, &record.id)?;

                            /* @TODO
                            self.page.clean(transaction, &record.id)?;*/
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    // This method does not contain Self context,
    // because child items creating in the runtime (by parent component)
    pub fn restore(
        tab_view: &TabView,
        transaction: &Transaction,
        app_browser_window_tab_id: &i64,
        // Actions
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
    ) -> Result<Vec<Arc<Item>>, String> {
        let mut items = Vec::new();

        match Database::records(transaction, app_browser_window_tab_id) {
            Ok(records) => {
                for record in records {
                    // Construct new item object
                    let item = Item::new_arc(
                        tab_view,
                        // Actions
                        action_tab_page_navigation_base.clone(),
                        action_tab_page_navigation_history_back.clone(),
                        action_tab_page_navigation_history_forward.clone(),
                        action_tab_page_navigation_reload.clone(),
                        action_update.clone(),
                        // Options
                        record.is_pinned,
                        record.is_selected,
                    );

                    // Delegate restore action to the item childs
                    item.widget.restore(transaction, &record.id)?;

                    /* @TODO
                    self.page.restore(transaction, &id)?; */

                    // Result
                    items.push(item);
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(items)
    }

    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_id: &i64,
        page_position: &i32,
        is_pinned: &bool,
        is_selected: &bool,
    ) -> Result<(), String> {
        match Database::add(
            transaction,
            app_browser_window_tab_id,
            page_position,
            is_pinned,
            is_selected,
        ) {
            Ok(_) => {
                let id = Database::last_insert_id(transaction);

                // Delegate save action to childs
                self.widget.save(transaction, &id)?;

                /* @TODO
                self.page.save(transaction, &id)?; */
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    // Getters
    pub fn id(&self) -> GString {
        self.id.clone()
    }

    pub fn page_is_loading(&self) -> bool {
        self.page.is_loading()
    }

    pub fn page_meta_title(&self) -> Option<GString> {
        self.page.meta_title()
    }

    pub fn gobject(&self) -> &TabPage {
        &self.widget.gobject()
    }

    // Tools
    pub fn migrate(tx: &Transaction) -> Result<(), String> {
        // Migrate self components
        if let Err(e) = Database::init(&tx) {
            return Err(e.to_string());
        }

        // Delegate migration to childs
        Widget::migrate(&tx)?;

        /* @TODO
        Page::migrate(&tx)? */

        // Success
        Ok(())
    }
}
