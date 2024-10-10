mod database;
mod page;

use database::Database;
use page::Page;

use sqlite::Transaction;

use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, GString},
    Box,
};

use std::sync::Arc;

pub struct Item {
    // Auto-generated unique item ID
    // useful as widget name in GTK actions callback
    id: GString,
    // Components
    page: Arc<Page>,
    // Extras, useful for session restore
    is_initially_current: bool,
}

impl Item {
    // Construct
    pub fn new_arc(
        page_navigation_request_text: Option<GString>,
        is_initially_current: bool,
        // Actions
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
    ) -> Arc<Self> {
        // Generate unique ID for new page components
        let id = uuid_string_random();

        // Init components
        let page = Page::new_arc(
            id.clone(),
            page_navigation_request_text.clone(),
            action_tab_page_navigation_base.clone(),
            action_tab_page_navigation_history_back.clone(),
            action_tab_page_navigation_history_forward.clone(),
            action_tab_page_navigation_reload.clone(),
            action_update.clone(),
        );

        // Return struct
        Arc::new(Self {
            id,
            is_initially_current,
            page,
        })
    }

    // Actions
    pub fn pin(&self) {
        //self.label.pin(!self.label.is_pinned()) // toggle
    }

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
        self.page.update();
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
                        None,
                        record.is_initially_current,
                        // Actions
                        action_tab_page_navigation_base.clone(),
                        action_tab_page_navigation_history_back.clone(),
                        action_tab_page_navigation_history_forward.clone(),
                        action_tab_page_navigation_reload.clone(),
                        action_update.clone(),
                    );

                    // Delegate restore action to the item childs

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
        page_number: &u32,
        is_initially_current: &bool,
    ) -> Result<(), String> {
        match Database::add(
            transaction,
            app_browser_window_tab_id,
            page_number,
            is_initially_current,
        ) {
            Ok(_) => {
                let id = Database::last_insert_id(transaction);

                // Delegate save action to childs

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

    pub fn is_initially_current(&self) -> bool {
        self.is_initially_current
    }

    pub fn page(&self) -> &Box {
        &self.page.widget() // @TODO
    }

    pub fn page_title(&self) -> Option<GString> {
        self.page.title()
    }

    pub fn page_description(&self) -> Option<GString> {
        self.page.description()
    }

    // Tools
    pub fn migrate(tx: &Transaction) -> Result<(), String> {
        // Migrate self components
        if let Err(e) = Database::init(&tx) {
            return Err(e.to_string());
        }

        // Delegate migration to childs

        /* @TODO
        Page::migrate(&tx)? */

        // Success
        Ok(())
    }
}
