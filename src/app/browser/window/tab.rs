mod database;
mod item;
mod menu;
mod widget;

use database::Database;
use item::Item;
use menu::Menu;
use widget::Widget;

use adw::TabView;
use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, GString, Propagation},
    prelude::{ActionExt, StaticVariantType, ToVariant},
};
use sqlite::Transaction;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
    index: Rc<RefCell<HashMap<GString, Rc<Item>>>>,
    // GTK
    widget: Rc<Widget>,
}

impl Tab {
    // Construct
    pub fn new_rc(
        action_page_close: SimpleAction,
        action_page_close_all: SimpleAction,
        action_page_home: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
        action_page_pin: SimpleAction,
        action_page_reload: SimpleAction,
        action_update: SimpleAction,
    ) -> Rc<Self> {
        // Init local actions
        let action_tab_open =
            SimpleAction::new(&uuid_string_random(), Some(&String::static_variant_type()));

        // Init empty HashMap index as no tabs appended yet
        let index = Rc::new(RefCell::new(HashMap::new()));

        // Init context menu
        let menu = Menu::new(
            action_page_close_all.clone(),
            action_page_close.clone(),
            action_page_history_back.clone(),
            action_page_history_forward.clone(),
            action_page_home.clone(),
            action_page_pin.clone(),
            action_page_reload.clone(),
        );

        // Init widget
        let widget = Rc::new(Widget::new(menu.gobject()));

        // Init events

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
                let item = Item::new_rc(
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
                    gobject
                        .selected_page()
                        .map(|page| gobject.page_position(&page) + 1),
                    false,
                    false,
                );

                // Register dynamically created tab components in the HashMap index
                index.borrow_mut().insert(item.id(), item.clone());

                // Apply request
                if let Some(variant) = request {
                    if let Some(value) = variant.get::<String>() {
                        item.set_page_navigation_request_text(value.as_str());
                        item.page_reload();
                    }
                }
            }
        });

        widget.gobject().connect_setup_menu({
            // Clone actions to update
            let action_page_close = action_page_close.clone();
            let action_page_history_back = action_page_history_back.clone();
            let action_page_history_forward = action_page_history_forward.clone();
            let action_page_home = action_page_home.clone();
            let action_page_pin = action_page_pin.clone();
            let action_page_reload = action_page_reload.clone();
            move |tab_view, tab_page| {
                // Setup state for selected page
                let state = match tab_page {
                    // Context menu opened
                    Some(this) => tab_view.page_position(this).to_variant(),
                    // Context menu closed (reset state to defaults)
                    None => (-1).to_variant(),
                };

                // Update actions
                action_page_close.change_state(&state);
                action_page_history_back.change_state(&state);
                action_page_history_forward.change_state(&state);
                action_page_home.change_state(&state);
                action_page_pin.change_state(&state);
                action_page_reload.change_state(&state);
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

        widget.gobject().connect_selected_page_notify({
            let index = index.clone();
            move |this| {
                if let Some(page) = this.selected_page() {
                    if let Some(id) = page.keyword() {
                        if let Some(item) = index.borrow().get(&id) {
                            item.update();
                        }
                    }
                }
            }
        });

        // Return activated struct
        Rc::new(Self {
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
    pub fn append(&self, position: Option<i32>) -> Rc<Item> {
        // Init new tab item
        let item = Item::new_rc(
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

    /// Close page at given `position`, `None` to close selected page (if available)
    pub fn close(&self, position: Option<i32>) {
        self.widget.close(position);
    }

    // Close all pages
    pub fn close_all(&self) {
        self.widget.close_all();
    }

    // Toggle pin status for active tab
    pub fn pin(&self, page_position: Option<i32>) {
        self.widget.pin(page_position);
    }

    pub fn page_home(&self, page_position: Option<i32>) {
        if let Some(page) = self.widget.page(page_position) {
            if let Some(id) = page.keyword() {
                if let Some(item) = self.index.borrow().get(&id) {
                    item.page_home();
                }
            }
        }
    }

    pub fn page_history_back(&self, page_position: Option<i32>) {
        if let Some(page) = self.widget.page(page_position) {
            if let Some(id) = page.keyword() {
                if let Some(item) = self.index.borrow().get(&id) {
                    item.page_history_back();
                }
            }
        }
    }

    pub fn page_history_forward(&self, page_position: Option<i32>) {
        if let Some(page) = self.widget.page(page_position) {
            if let Some(id) = page.keyword() {
                if let Some(item) = self.index.borrow().get(&id) {
                    item.page_history_forward();
                }
            }
        }
    }

    /// Reload page at `i32` position or selected page on `None` given
    pub fn page_reload(&self, page_position: Option<i32>) {
        if let Some(page) = self.widget.page(page_position) {
            if let Some(id) = page.keyword() {
                if let Some(item) = self.index.borrow().get(&id) {
                    item.page_reload();
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
    if let Err(e) = Database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    item::migrate(tx)?;

    // Success
    Ok(())
}
