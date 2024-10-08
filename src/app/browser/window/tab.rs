mod database;
mod item;
mod widget;

use database::Database;
use item::Item;
use sqlite::Transaction;
use widget::Widget;

use gtk::{
    gio::SimpleAction,
    glib::GString,
    prelude::{ActionExt, WidgetExt},
    Notebook,
};

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
    index: RefCell<HashMap<GString, Arc<Item>>>,
    // GTK
    widget: Arc<Widget>,
}

impl Tab {
    // Construct
    pub fn new(
        // Actions
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
    ) -> Self {
        // Init empty HashMap index as no tabs appended yet
        let index = RefCell::new(HashMap::new());

        // Init widget
        let widget = Arc::new(Widget::new());

        // Return non activated struct
        Self {
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
        }
    }

    // Actions
    pub fn activate(&self, tab: Arc<Self>) {
        self.widget
            .gobject()
            .connect_page_removed(move |_, widget, _| {
                // Cleanup HashMap index
                let id = widget.widget_name();

                // Check for required value as raw access to gobject @TODO
                if id.is_empty() {
                    panic!("Undefined tab index!")
                }

                tab.index.borrow_mut().remove(&id);
            });

        // Switch page post-event (`connect_switch_page` activates before `page_number` get updated)
        self.widget.gobject().connect_page_notify({
            let action_update = self.action_update.clone();
            // Update window header with current page title
            move |_| action_update.activate(None)
        });
    }

    pub fn append(
        &self,
        page_navigation_request_text: Option<GString>,
        is_initially_current: bool,
    ) -> Arc<Item> {
        // Init new tab item
        let item = Item::new(
            page_navigation_request_text.clone(),
            is_initially_current,
            // Actions
            self.action_tab_page_navigation_base.clone(),
            self.action_tab_page_navigation_history_back.clone(),
            self.action_tab_page_navigation_history_forward.clone(),
            self.action_tab_page_navigation_reload.clone(),
            self.action_update.clone(),
        );

        // Register dynamically created tab components in the HashMap index
        self.index.borrow_mut().insert(item.id(), item.clone());

        // Append new Notebook page
        self.widget
            .append(item.label(), item.page(), item.is_initially_current());

        if page_navigation_request_text.is_none() {
            item.page_navigation_request_grab_focus(); // @TODO
        }

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
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.pin(); // toggle
            }
        }
    }

    pub fn page_navigation_base(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page_navigation_base();
            }
        }
    }

    pub fn page_navigation_history_back(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page_navigation_history_back();
            }
        }
    }

    pub fn page_navigation_history_forward(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page_navigation_history_forward();
            }
        }
    }

    pub fn page_navigation_reload(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page_navigation_reload();
            }
        }
    }

    pub fn update(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.update();
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

                                // Append new Notebook page
                                self.widget.append(
                                    item.label(),
                                    item.page(),
                                    item.is_initially_current(),
                                );
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

                // Read HashMap index collected
                let mut page_number = 0;

                for (_, item) in self.index.borrow().iter() {
                    item.save(
                        transaction,
                        &id,
                        &match self.widget.gobject().current_page() {
                            Some(number) => number == page_number,
                            None => false,
                        },
                    )?;

                    page_number += 1;
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    // Getters
    pub fn page_title(&self) -> Option<GString> {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                return item.page_title();
            }
        }
        None
    }

    pub fn page_description(&self) -> Option<GString> {
        if let Some(id) = self.widget.current_name() {
            // Get page by widget ID
            if let Some(item) = self.index.borrow().get(&id) {
                return item.page_description();
            }
        }
        None
    }

    pub fn gobject(&self) -> &Notebook {
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
