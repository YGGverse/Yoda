mod database;
mod label;
mod page;
mod widget;

use database::Database;
use label::Label;
use page::Page;
use sqlite::{Connection, Transaction};
use widget::Widget;

use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, GString},
    prelude::{ActionExt, WidgetExt},
    GestureClick, Notebook,
};

use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, RwLock},
};

// Common struct for HashMap index
struct TabItem {
    label: Arc<Label>,
    page: Arc<Page>,
}

// Main
pub struct Tab {
    // Extras
    database: Arc<Database>,
    // Actions
    action_tab_page_navigation_base: Arc<SimpleAction>,
    action_tab_page_navigation_history_back: Arc<SimpleAction>,
    action_tab_page_navigation_history_forward: Arc<SimpleAction>,
    action_tab_page_navigation_reload: Arc<SimpleAction>,
    action_update: Arc<SimpleAction>,
    // Dynamically allocated reference index
    index: RefCell<HashMap<GString, TabItem>>,
    // GTK
    widget: Arc<Widget>,
}

impl Tab {
    // Construct
    pub fn new(
        // Extras
        profile_database_connection: Arc<RwLock<Connection>>,
        // Actions
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
    ) -> Self {
        // Init database
        let database = {
            // Init writable database connection
            let mut connection = match profile_database_connection.write() {
                Ok(connection) => connection,
                Err(e) => todo!("{e}"),
            };

            // Init new transaction
            let transaction = match connection.transaction() {
                Ok(transaction) => transaction,
                Err(e) => todo!("{e}"),
            };

            // Init database structure
            match Database::init(&transaction) {
                Ok(database) => match transaction.commit() {
                    Ok(_) => Arc::new(database),
                    Err(e) => todo!("{e}"),
                },
                Err(e) => todo!("{e}"),
            }
        };

        // Init empty HashMap index as no tabs appended yet
        let index = RefCell::new(HashMap::new());

        // Init widget
        let widget = Arc::new(Widget::new());

        // Return non activated struct
        Self {
            database,
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

    pub fn append(&self, page_navigation_request_text: Option<GString>, is_current_page: bool) {
        // Generate unique ID for new page components
        let id = uuid_string_random();

        // Init new tab components
        let label = Arc::new(Label::new(id.clone(), false));
        let page = Arc::new(Page::new(
            id.clone(),
            page_navigation_request_text.clone(),
            self.action_tab_page_navigation_base.clone(),
            self.action_tab_page_navigation_history_back.clone(),
            self.action_tab_page_navigation_history_forward.clone(),
            self.action_tab_page_navigation_reload.clone(),
            self.action_update.clone(),
        ));

        // Register dynamically created tab components in the HashMap index
        self.index.borrow_mut().insert(
            id.clone(),
            TabItem {
                label: label.clone(),
                page: page.clone(),
            },
        );

        // Init additional label actions
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

        // Append new Notebook page
        self.widget
            .append(label.gobject(), page.widget(), is_current_page, true);

        if page_navigation_request_text.is_none() {
            page.navigation_request_grab_focus();
        }
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
                item.label.pin(!item.label.is_pinned()); // toggle
            }
        }
    }

    pub fn page_navigation_base(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page.navigation_base();
            }
        }
    }

    pub fn page_navigation_history_back(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page.navigation_history_back();
            }
        }
    }

    pub fn page_navigation_history_forward(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page.navigation_history_forward();
            }
        }
    }

    pub fn page_navigation_reload(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page.navigation_reload();
            }
        }
    }

    pub fn update(&self) {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page.update();
                if let Some(title) = item.page.title() {
                    item.label.update(Some(&title));
                } else {
                    item.label.update(None);
                }
            }
        }
    }

    pub fn clean(&self, tx: &Transaction, app_browser_window_id: &i64) {
        match self.database.records(tx, app_browser_window_id) {
            Ok(records) => {
                for record in records {
                    match self.database.delete(tx, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
                            // nothing yet..
                        }
                        Err(e) => todo!("{e}"),
                    }
                }
            }
            Err(e) => todo!("{e}"),
        }
    }

    pub fn restore(&self, tx: &Transaction, app_browser_window_id: &i64) {
        match self.database.records(tx, app_browser_window_id) {
            Ok(records) => {
                for record in records {
                    self.append(None, record.is_current);
                    // Delegate restore action to childs
                    // nothing yet..
                }
            }
            Err(e) => todo!("{e}"),
        }
    }

    pub fn save(&self, tx: &Transaction, app_browser_window_id: &i64) {
        let mut page_number = 0;

        for (_, _) in self.index.borrow().iter() {
            match self.database.add(
                tx,
                app_browser_window_id,
                &match self.widget.gobject().current_page() {
                    Some(number) => number == page_number,
                    None => false,
                },
            ) {
                Ok(_) => {
                    // Delegate save action to childs
                    // let id = self.database.last_insert_id(tx);

                    // @TODO
                    // item.label.save()
                    // item.page.save()
                }
                Err(e) => todo!("{e}"),
            }

            page_number += 1;
        }
    }

    // Getters
    pub fn page_title(&self) -> Option<GString> {
        if let Some(id) = self.widget.current_name() {
            if let Some(item) = self.index.borrow().get(&id) {
                return item.page.title();
            }
        }
        None
    }

    pub fn page_description(&self) -> Option<GString> {
        if let Some(id) = self.widget.current_name() {
            // Get page by widget ID
            if let Some(item) = self.index.borrow().get(&id) {
                return item.page.description();
            }
        }
        None
    }

    pub fn gobject(&self) -> &Notebook {
        self.widget.gobject()
    }
}
