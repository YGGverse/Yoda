mod action;
mod database;
mod item;
mod menu;
mod widget;

use action::Action;
use database::Database;
use item::Item;
use menu::Menu;
use widget::Widget;

use crate::app::browser::action::Action as BrowserAction;
use crate::app::browser::window::action::Action as WindowAction;
use adw::TabView;
use gtk::glib::{GString, Propagation};
use sqlite::Transaction;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

// Main
pub struct Tab {
    // Actions
    browser_action: Rc<BrowserAction>,
    window_action: Rc<WindowAction>,
    // Dynamically allocated reference index
    index: Rc<RefCell<HashMap<GString, Rc<Item>>>>,
    action: Rc<Action>,
    widget: Rc<Widget>,
}

impl Tab {
    // Construct
    pub fn new_rc(browser_action: Rc<BrowserAction>, window_action: Rc<WindowAction>) -> Rc<Self> {
        // Init local actions
        let action = Rc::new(Action::new());

        // Init empty HashMap index as no tabs appended yet
        let index: Rc<RefCell<HashMap<GString, Rc<Item>>>> = Rc::new(RefCell::new(HashMap::new()));

        // Init context menu
        let menu = Menu::new(window_action.clone());

        // Init widget
        let widget = Rc::new(Widget::new(menu.gobject()));

        // Init events

        widget.gobject().connect_setup_menu({
            let window_action = window_action.clone();
            move |tab_view, tab_page| {
                // Set new state for page selected on menu open
                // * this action return default state (`None`) on menu close
                let state = tab_page.map(|this| tab_view.page_position(this));

                // Update actions with new state value
                window_action.close_all().change_state(state);
                window_action.close().change_state(state);
                window_action.history_back().change_state(state);
                window_action.history_forward().change_state(state);
                window_action.home().change_state(state);
                window_action.pin().change_state(state);
                window_action.reload().change_state(state);
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

        action.open().connect_activate({
            let index = index.clone();
            let widget = widget.clone();
            move |request| {
                if let Some(value) = request {
                    if let Some(page) = widget.page(None) {
                        if let Some(id) = page.keyword() {
                            if let Some(item) = index.borrow().get(&id) {
                                item.set_page_navigation_request_text(value.as_str());
                                item.page_reload();
                            }
                        }
                    }
                }
            }
        });

        // Return activated struct
        Rc::new(Self {
            browser_action,
            window_action,
            index,
            action,
            widget,
        })
    }

    // Actions
    pub fn append(&self, position: Option<i32>) -> Rc<Item> {
        // Init new tab item
        let item = Item::new_rc(
            self.gobject(),
            self.browser_action.clone(),
            self.window_action.clone(),
            self.action.clone(),
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

    pub fn update(&self, item_id: Option<GString>) {
        let key = match item_id {
            Some(value) => value,
            None => GString::new(), // @TODO
        };

        match self.index.borrow().get(&key) {
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
    } // @TODO need optimization

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
                        self.browser_action.clone(),
                        self.window_action.clone(),
                        self.action.clone(),
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

    pub fn action(&self) -> &Rc<Action> {
        &self.action
    }

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
