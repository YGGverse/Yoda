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
    prelude::{ActionExt, WidgetExt},
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
    index: RefCell<HashMap<GString, Arc<Item>>>,
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
        let index = RefCell::new(HashMap::new());

        // Init widget
        let widget = Arc::new(Widget::new());

        // Init events
        widget.gobject().connect_close_page(move |_, tab_page| {
            /* @TODO
            // Cleanup HashMap index
            let id = tab_page.widget_name();

            // Check for required value as raw access to gobject @TODO
            if id.is_empty() {
                panic!("Undefined tab index!")
            }

            tab.index.borrow_mut().remove(&id); */
            Propagation::Proceed
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
            self.action_tab_page_navigation_base.clone(),
            self.action_tab_page_navigation_history_back.clone(),
            self.action_tab_page_navigation_history_forward.clone(),
            self.action_tab_page_navigation_reload.clone(),
            self.action_update.clone(),
        );

        // Register dynamically created tab components in the HashMap index
        self.index.borrow_mut().insert(item.id(), item.clone());

        // Append new page
        self.widget.gobject().add_page(item.gobject(), None);

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
                                /* @TODO
                                self.widget.append(
                                    item.page(),
                                ); */
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

                // At least one active page wanted to continue
                /* @TODO
                if let Some(current_page) = self.widget.gobject().current_page() {
                    // Read collected HashMap index
                    for (_, item) in self.index.borrow().iter() {
                        // Get page number as HashMap does not keep order, no page_reorder listener also
                        match self.widget.gobject().page_num(item.page()) {
                            Some(page_number) => {
                                item.save(
                                    transaction,
                                    &id,
                                    &page_number,
                                    &(current_page == page_number),
                                )?;
                            }
                            None => panic!(), // page number expected at this point @TODO Err?
                        }
                    }
                }; */
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
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
