mod database;
mod item;
mod widget;

use database::Database;
use item::Item;
use widget::Widget;

use adw::TabView;
use gtk::{
    gio::SimpleAction,
    glib::{GString, Propagation},
};
use sqlite::Transaction;
use std::{cell::RefCell, collections::HashMap, sync::Arc};

// Main
pub struct Tab {
    // Actions
    action_tab_page_navigation_base: Arc<SimpleAction>,
    action_tab_page_navigation_history_back: Arc<SimpleAction>,
    action_tab_page_navigation_history_forward: Arc<SimpleAction>,
    action_tab_page_navigation_reload: Arc<SimpleAction>,
    action_update: Arc<SimpleAction>,
    // Dynamically allocated reference index
    index: Arc<RefCell<HashMap<GString, Arc<Item>>>>,
    // GTK
    widget: Arc<Widget>,
}

impl Tab {
    // Construct
    pub fn new_arc(
        // Actions
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
    ) -> Arc<Self> {
        // Init empty HashMap index as no tabs appended yet
        let index = Arc::new(RefCell::new(HashMap::new()));

        // Init widget
        let widget = Arc::new(Widget::new());

        // Init events
        widget.gobject().connect_close_page({
            let index = index.clone();
            move |_, item| {
                // Get index ID by keyword saved
                match item.keyword() {
                    Some(id) => {
                        if id.is_empty() {
                            panic!("Tab index can not be empty!")
                        }
                        // Cleanup HashMap index
                        index.borrow_mut().remove(&id);
                    }
                    None => panic!("Undefined tab index!"),
                }

                Propagation::Proceed
            }
        });

        // Return activated struct
        Arc::new(Self {
            // Define action links
            action_tab_page_navigation_base,
            action_tab_page_navigation_history_back,
            action_tab_page_navigation_history_forward,
            action_tab_page_navigation_reload,
            action_update,
            // Init empty HashMap index as no tabs appended yet
            index,
            // GTK
            widget,
        })
    }

    // Actions
    pub fn append(&self) -> Arc<Item> {
        // Init new tab item
        let item = Item::new_arc(
            self.gobject(),
            // Actions
            self.action_tab_page_navigation_base.clone(),
            self.action_tab_page_navigation_history_back.clone(),
            self.action_tab_page_navigation_history_forward.clone(),
            self.action_tab_page_navigation_reload.clone(),
            self.action_update.clone(),
            // Options
            false,
            true,
        );

        // Register dynamically created tab components in the HashMap index
        self.index.borrow_mut().insert(item.id(), item.clone());

        item.page_navigation_request_grab_focus(); // @TODO
        item
    }

    // Close active tab
    pub fn close(&self) {
        self.widget.close();
    }

    // Close all tabs
    pub fn close_all(&self) {
        self.widget.close_all();
    }

    // Toggle pin status for active tab
    pub fn pin(&self) {
        if let Some(page) = self.widget.gobject().selected_page() {
            self.widget
                .gobject()
                .set_page_pinned(&page, !page.is_pinned()); // toggle
        }
    }

    pub fn page_navigation_base(&self) {
        if let Some(id) = self.widget.current_page_keyword() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page_navigation_base();
            }
        }
    }

    pub fn page_navigation_history_back(&self) {
        if let Some(id) = self.widget.current_page_keyword() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page_navigation_history_back();
            }
        }
    }

    pub fn page_navigation_history_forward(&self) {
        if let Some(id) = self.widget.current_page_keyword() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page_navigation_history_forward();
            }
        }
    }

    pub fn page_navigation_reload(&self) {
        if let Some(id) = self.widget.current_page_keyword() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page_navigation_reload();
            }
        }
    }

    pub fn update(&self) {
        if let Some(id) = self.widget.current_page_keyword() {
            if let Some(item) = self.index.borrow().get(&id) {
                // Update item components
                item.update();

                // Update tab title on loading indicator inactive
                if !item.page_is_loading() {
                    if let Some(title) = item.page_meta_title() {
                        item.gobject().set_title(title.as_str())
                    };
                }
            }
        }
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_id) {
            Ok(records) => {
                for record in records {
                    match Database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
                            for (_, item) in self.index.borrow().iter() {
                                item.clean(transaction, &record.id)?
                            }
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
        app_browser_window_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_id) {
            Ok(records) => {
                for record in records {
                    match Item::restore(
                        self.gobject(),
                        transaction,
                        &record.id,
                        self.action_tab_page_navigation_base.clone(),
                        self.action_tab_page_navigation_history_back.clone(),
                        self.action_tab_page_navigation_history_forward.clone(),
                        self.action_tab_page_navigation_reload.clone(),
                        self.action_update.clone(),
                    ) {
                        Ok(items) => {
                            for item in items {
                                // Register dynamically created tab item in the HashMap index
                                self.index.borrow_mut().insert(item.id(), item.clone());
                            }
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_id: &i64,
    ) -> Result<(), String> {
        match Database::add(transaction, app_browser_window_id) {
            Ok(_) => {
                // Delegate save action to childs
                let id = Database::last_insert_id(transaction);

                // Read collected HashMap index
                for (_, item) in self.index.borrow().iter() {
                    item.save(
                        transaction,
                        &id,
                        &self.widget.gobject().page_position(item.gobject()),
                        &item.gobject().is_pinned(),
                        &item.gobject().is_selected(),
                    )?;
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn init(&self) {
        // Append just one blank page if no tabs available after last session restore
        if self.index.borrow().is_empty() {
            self.append();
        }

        // @TODO other/child features..
    }

    // Getters
    pub fn gobject(&self) -> &TabView {
        self.widget.gobject()
    }

    // Tools
    pub fn migrate(tx: &Transaction) -> Result<(), String> {
        // Migrate self components
        if let Err(e) = Database::init(&tx) {
            return Err(e.to_string());
        }

        // Delegate migration to childs
        Item::migrate(&tx)?;

        /* @TODO
        Page::migrate(&tx)?; */

        // Success
        Ok(())
    }
}
