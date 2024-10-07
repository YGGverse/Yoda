mod database;
mod label;
mod page;

use database::Database;
use label::Label;
use page::Page;

use sqlite::Transaction;

use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, GString},
    prelude::WidgetExt,
    Box, GestureClick,
};

use std::sync::Arc;

pub struct Item {
    // Auto-generated unique item ID
    // useful as widget name in GTK actions callback
    id: GString,
    // Components
    label: Arc<Label>,
    page: Arc<Page>,
    // Extras, useful for session restore
    is_initially_current: bool,
}

impl Item {
    // Construct
    pub fn new(
        page_navigation_request_text: Option<GString>,
        is_initially_current: bool,
        // Actions
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
    ) -> Self {
        // Generate unique ID for new page components
        let id = uuid_string_random();

        // Init components
        let label = Arc::new(Label::new(id.clone(), false));

        let page = Arc::new(Page::new(
            id.clone(),
            page_navigation_request_text.clone(),
            action_tab_page_navigation_base.clone(),
            action_tab_page_navigation_history_back.clone(),
            action_tab_page_navigation_history_forward.clone(),
            action_tab_page_navigation_reload.clone(),
            action_update.clone(),
        ));

        // Init additional label actions @TODO move to Label?
        let controller = GestureClick::new();

        controller.connect_pressed({
            let label = label.clone();
            move |_, count, _, _| {
                // double click
                if count == 2 {
                    label.pin(!label.is_pinned()); // toggle
                }
            }
        });

        label.gobject().add_controller(controller);

        // Return struct
        Self {
            id,
            is_initially_current,
            label,
            page,
        }
    }

    // Actions
    pub fn pin(&self) {
        self.label.pin(!self.label.is_pinned()) // toggle
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
        if let Some(title) = self.page.title() {
            self.label.update(Some(&title));
        } else {
            self.label.update(None);
        }
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
                            if let Err(e) = self.label.clean(transaction, &record.id) {
                                return Err(e.to_string());
                            }

                            /* @TODO
                            if let Err(e) = self.page.clean(transaction, &record.id) {
                                return Err(e.to_string());
                            } */
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
                    let item = Arc::new(Item::new(
                        None,
                        record.is_initially_current,
                        // Actions
                        action_tab_page_navigation_base.clone(),
                        action_tab_page_navigation_history_back.clone(),
                        action_tab_page_navigation_history_forward.clone(),
                        action_tab_page_navigation_reload.clone(),
                        action_update.clone(),
                    ));

                    // Delegate restore action to the item childs
                    if let Err(e) = item.label.restore(transaction, &record.id) {
                        return Err(e.to_string());
                    }

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
        is_initially_current: &bool,
    ) -> Result<(), String> {
        match Database::add(transaction, app_browser_window_tab_id, is_initially_current) {
            Ok(_) => {
                let id = Database::last_insert_id(transaction);

                // Delegate save action to childs
                if let Err(e) = self.label.save(transaction, &id) {
                    return Err(e.to_string());
                }

                /* @TODO
                if let Err(e) = self.page.save(transaction, &id) {
                    return Err(e.to_string());
                } */
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

    pub fn label(&self) -> &Box {
        &self.label.gobject()
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
        if let Err(e) = Label::migrate(&tx) {
            return Err(e.to_string());
        }

        /* @TODO
        if let Err(e) = Page::migrate(&tx) {
            return Err(e.to_string());
        } */

        // Success
        Ok(())
    }
}