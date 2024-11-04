mod database;
mod item;
mod widget;

use database::Database;
use item::Item;
use widget::Widget;

use adw::TabView;
use gtk::{
    gio::{Menu, SimpleAction},
    glib::{gformat, uuid_string_random, GString, Propagation},
    prelude::{ActionExt, StaticVariantType, ToVariant},
};
use sqlite::Transaction;
use std::{cell::RefCell, collections::HashMap, sync::Arc};

// Main
pub struct Tab {
    // Local actions
    action_tab_open: SimpleAction,
    // Global actions
    action_page_home: SimpleAction,
    action_page_history_back: SimpleAction,
    action_page_history_forward: SimpleAction,
    action_page_reload: SimpleAction,
    action_update: SimpleAction,
    // Dynamically allocated reference index
    index: Arc<RefCell<HashMap<GString, Arc<Item>>>>,
    // GTK
    widget: Arc<Widget>,
}

impl Tab {
    // Construct
    pub fn new_arc(
        // Actions
        action_page_home: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
        action_page_reload: SimpleAction,
        action_update: SimpleAction,
    ) -> Arc<Self> {
        // Init local actions
        let action_tab_open =
            SimpleAction::new(&uuid_string_random(), Some(&String::static_variant_type()));

        // Init empty HashMap index as no tabs appended yet
        let index = Arc::new(RefCell::new(HashMap::new()));

        // @TODO move to mod
        let menu = Menu::new();

        menu.append(
            Some("Reload"),
            Some(&gformat!("win.{}", action_page_reload.name())),
        ); // @TODO resolve namespace outside

        // Init widget
        let widget = Arc::new(Widget::new(&menu));

        // Init events

        // Setup actions for context menu
        widget.gobject().connect_setup_menu({
            let action_page_reload = action_page_reload.clone();
            move |tab_view, tab_page| {
                // Enable actions by default
                action_page_reload.set_enabled(true);

                match tab_page {
                    // Menu opened:
                    // setup actions to operate with page selected only
                    Some(this) => {
                        // Set state
                        let state = tab_view.page_position(this).to_variant();

                        // Update related actions
                        action_page_reload.change_state(&state);
                    }
                    // Menu closed:
                    // return actions to default values
                    None => {
                        // Set state
                        let state = &(-1).to_variant();

                        // Update related actions
                        action_page_reload.change_state(&state);
                    }
                }
            }
        });

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

        action_tab_open.connect_activate({
            let index = index.clone();
            let gobject = widget.gobject().clone();
            // Actions
            let action_tab_open = action_tab_open.clone();
            let action_page_home = action_page_home.clone();
            let action_page_history_back = action_page_history_back.clone();
            let action_page_history_forward = action_page_history_forward.clone();
            let action_page_reload = action_page_reload.clone();
            let action_update = action_update.clone();
            move |_, request| {
                // Init new tab item
                let item = Item::new_arc(
                    &gobject,
                    // Local actions
                    action_tab_open.clone(),
                    // Global actions
                    action_page_home.clone(),
                    action_page_history_back.clone(),
                    action_page_history_forward.clone(),
                    action_page_reload.clone(),
                    action_update.clone(),
                    // Options
                    match gobject.selected_page() {
                        Some(page) => Some(gobject.page_position(&page) + 1),
                        None => None,
                    },
                    false,
                    false,
                );

                // Register dynamically created tab components in the HashMap index
                index.borrow_mut().insert(item.id(), item.clone());

                // Apply request
                if let Some(variant) = request {
                    if let Some(value) = variant.get::<String>() {
                        item.set_page_navigation_request_text(value.as_str());
                        item.page_navigation_reload();
                    }
                }
            }
        });

        // Return activated struct
        Arc::new(Self {
            // Local actions
            action_tab_open,
            // Global actions
            action_page_home,
            action_page_history_back,
            action_page_history_forward,
            action_page_reload,
            action_update,
            // Init empty HashMap index as no tabs appended yet
            index,
            // GTK
            widget,
        })
    }

    // Actions
    pub fn append(&self, position: Option<i32>) -> Arc<Item> {
        // Init new tab item
        let item = Item::new_arc(
            self.gobject(),
            // Local actions
            self.action_tab_open.clone(),
            // Global actions
            self.action_page_home.clone(),
            self.action_page_history_back.clone(),
            self.action_page_history_forward.clone(),
            self.action_page_reload.clone(),
            self.action_update.clone(),
            // Options
            position,
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

    pub fn page_navigation_home(&self) {
        if let Some(id) = self.widget.current_page_keyword() {
            if let Some(item) = self.index.borrow().get(&id) {
                item.page_navigation_home();
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

    /// Reload page at `i32` position or selected page on `None` given
    pub fn page_navigation_reload(&self, page_position: Option<i32>) {
        match page_position {
            Some(value) => {
                if let Some(id) = self.widget.gobject().nth_page(value).keyword() {
                    if let Some(item) = self.index.borrow().get(&id) {
                        item.page_navigation_reload();
                    }
                }
            }
            None => {
                if let Some(id) = self.widget.current_page_keyword() {
                    if let Some(item) = self.index.borrow().get(&id) {
                        item.page_navigation_reload();
                    }
                }
            }
        }
    }

    pub fn update(&self, id: &str) {
        match self.index.borrow().get(id) {
            Some(item) => {
                // Update item components
                item.update();

                // Update tab title on loading indicator inactive
                if !item.page_is_loading() {
                    item.gobject().set_title(item.page_meta_title().as_str())
                }
            }
            // Update all tabs on ID not found @TODO change initial update method
            None => {
                for (_, item) in self.index.borrow().iter() {
                    // Update item components
                    item.update();

                    // Update tab title on loading indicator inactive
                    if !item.page_is_loading() {
                        item.gobject().set_title(item.page_meta_title().as_str())
                    }
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
                        self.action_tab_open.clone(),
                        self.action_page_home.clone(),
                        self.action_page_history_back.clone(),
                        self.action_page_history_forward.clone(),
                        self.action_page_reload.clone(),
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
            self.append(None);
        }

        // @TODO other/child features..
    }

    // Getters
    pub fn gobject(&self) -> &TabView {
        self.widget.gobject()
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = Database::init(&tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    item::migrate(&tx)?;

    // Success
    Ok(())
}
