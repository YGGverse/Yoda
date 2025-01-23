mod database;
mod error;
mod item;
mod menu;
mod widget;

use error::Error;
pub use item::Item;
use menu::Menu;
use widget::Widget;

use crate::app::browser::{
    window::action::{Action as WindowAction, Position},
    Action as BrowserAction,
};
use crate::Profile;
use gtk::{
    glib::{DateTime, GString, Propagation},
    prelude::{EditableExt, WidgetExt},
};
use sqlite::Transaction;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

// Main
pub struct Tab {
    browser_action: Rc<BrowserAction>,
    window_action: Rc<WindowAction>,
    profile: Rc<Profile>,
    index: Rc<RefCell<HashMap<Rc<GString>, Rc<Item>>>>,
    pub widget: Rc<Widget>,
}

impl Tab {
    // Constructors

    /// Build new `Self`
    pub fn build(
        profile: &Rc<Profile>,
        (browser_action, window_action): (&Rc<BrowserAction>, &Rc<WindowAction>),
    ) -> Self {
        // Init empty HashMap index
        let index: Rc<RefCell<HashMap<Rc<GString>, Rc<Item>>>> =
            Rc::new(RefCell::new(HashMap::new()));

        // Init context menu
        let menu = Menu::new(window_action);

        // Init widget
        let widget = Rc::new(Widget::new(&menu.main));

        // Init events
        widget.tab_view.connect_setup_menu({
            let action = window_action.clone();
            let index = index.clone();
            let widget = widget.clone();
            move |tab_view, tab_page| {
                let state = match tab_page {
                    // on menu open
                    Some(this) => {
                        if let Some(id) = this.keyword() {
                            if let Some(item) = index.borrow().get(&id) {
                                item.page.update(); // update window actions using page of tab activated
                            }
                        }
                        Some(tab_view.page_position(this)) // activated tab
                    }
                    // on menu close
                    None => {
                        if let Some(page) = widget.page(None) {
                            if let Some(id) = page.keyword() {
                                if let Some(item) = index.borrow().get(&id) {
                                    item.page.update(); // update window actions using page of current tab
                                }
                            }
                        }
                        None // current tab
                    }
                };

                // Update actions with new state value
                action.bookmark.change_state(state);
                action.close_all.change_state(state);
                action.close.change_state(state);
                action.find.change_state(state);
                action.history_back.change_state(state);
                action.history_forward.change_state(state);
                action.home.change_state(state);
                action.pin.change_state(state);
                action.reload.change_state(state);
                action.save_as.change_state(state);
                action.source.change_state(state);
            }
        });

        widget.tab_view.connect_close_page({
            let index = index.clone();
            let profile = profile.clone();
            move |_, item| {
                // Get index ID by keyword saved
                match item.keyword() {
                    Some(id) => {
                        if id.is_empty() {
                            panic!("Tab index can not be empty!")
                        }
                        // Cleanup HashMap index
                        if let Some(item) = index.borrow_mut().remove(&id) {
                            // Add history record into profile memory pool
                            // * this action allows to recover recently closed tab (e.g. from the main menu)
                            profile
                                .history
                                .memory
                                .tab
                                .add(item, DateTime::now_local().unwrap().to_unix());
                        }
                    }
                    None => panic!("Undefined tab index!"),
                }

                Propagation::Proceed
            }
        });

        widget.tab_view.connect_selected_page_notify({
            let index = index.clone();
            move |this| {
                if let Some(page) = this.selected_page() {
                    if let Some(id) = page.keyword() {
                        if let Some(item) = index.borrow().get(&id) {
                            item.update();
                        }
                    }
                    // Reset attention decorator
                    page.set_needs_attention(false);
                }
            }
        });

        // Return activated `Self`
        Self {
            profile: profile.clone(),
            browser_action: browser_action.clone(),
            window_action: window_action.clone(),
            index,
            widget,
        }
    }

    // Actions
    pub fn append(
        &self,
        position: Position,
        request: Option<String>,
        is_pinned: bool,
        is_selected: bool,
        is_attention: bool,
        is_load: bool,
    ) -> Rc<Item> {
        // Init new tab item
        let item = Rc::new(Item::build(
            &self.widget.tab_view,
            &self.profile,
            // Actions
            (&self.browser_action, &self.window_action),
            // Options
            (
                position,
                request,
                is_pinned,
                is_selected,
                is_attention,
                is_load,
            ),
        ));

        // Register dynamically created tab components in the HashMap index
        self.index
            .borrow_mut()
            .insert(item.id.clone(), item.clone());

        item.page.navigation.request.widget.entry.grab_focus();

        item
    }

    /// Close page at given `page_position`, `None` to close selected page (if available)
    pub fn close(&self, page_position: Option<i32>) {
        self.widget.close(page_position);
    }

    // Close all pages
    pub fn close_all(&self) {
        self.widget.close_all();
    }

    // Toggle escape action for specified or current item
    pub fn escape(&self, item_id: Option<GString>) {
        match item_id {
            Some(id) => {
                if let Some(item) = self.index.borrow().get(&id) {
                    item.page.escape()
                }
            }
            None => {
                if let Some(item) = self.item(None) {
                    item.page.escape();
                }
            }
        }
    }

    // Toggle search widget
    pub fn find(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.page.find();
        }
    }

    // Save page at given `position`, `None` to save selected page (if available)
    pub fn save_as(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.page.navigation.request.to_download();
            self.window_action.reload.activate();
        }
    }

    // View source for page at given `position`, `None` to use selected page (if available)
    pub fn source(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.page.navigation.request.to_source();
            self.window_action.reload.activate();
        }
    }

    /// Toggle `Bookmark` in current `Profile` for `Page` at given `position` (current page on `None`)
    /// * return `true` on bookmark created, `false` on deleted; `Error` otherwise.
    pub fn bookmark(&self, page_position: Option<i32>) -> Result<bool, Error> {
        if let Some(item) = self.item(page_position) {
            return match item.page.bookmark() {
                Ok(result) => Ok(result),
                Err(_) => Err(Error::Bookmark),
            };
        }
        Err(Error::PageNotFound)
    }

    // Toggle pin status for active tab
    pub fn pin(&self, page_position: Option<i32>) {
        self.widget.pin(page_position);
    }

    pub fn page_home(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            if let Some(home) = item.page.navigation.request.home() {
                let home = home.to_string();
                item.page.navigation.request.widget.entry.set_text(&home);
                item.client.handle(&home, true);
            }
        }
    }

    pub fn page_history_back(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            if let Some(back) = item.page.navigation.history.back(true) {
                item.page.navigation.request.widget.entry.set_text(&back);
                item.client.handle(&back, false);
            }
        }
    }

    pub fn page_history_forward(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            if let Some(forward) = item.page.navigation.history.forward(true) {
                item.page.navigation.request.widget.entry.set_text(&forward);
                item.client.handle(&forward, false);
            }
        }
    }

    /// Reload page at `i32` position or selected page on `None` given
    pub fn page_reload(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.client
                .handle(&item.page.navigation.request.widget.entry.text(), true);
        }
    }

    pub fn update(&self, item_id: Option<GString>) {
        let key = item_id.unwrap_or_default();

        match self.index.borrow().get(&key) {
            Some(item) => {
                item.update();
            }
            None => {
                // update all tabs
                for (_, item) in self.index.borrow().iter() {
                    item.update();
                }
            }
        }
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_id: i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
                            for (_, item) in self.index.borrow().iter() {
                                item.clean(transaction, record.id)?
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
        app_browser_window_id: i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_id) {
            Ok(records) => {
                for record in records {
                    match Item::restore(
                        &self.widget.tab_view,
                        transaction,
                        record.id,
                        &self.profile,
                        (&self.browser_action, &self.window_action),
                    ) {
                        Ok(items) => {
                            for item in items {
                                // Register dynamically created tab item in the HashMap index
                                self.index
                                    .borrow_mut()
                                    .insert(item.id.clone(), item.clone());
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
        app_browser_window_id: i64,
    ) -> Result<(), String> {
        match database::insert(transaction, app_browser_window_id) {
            Ok(_) => {
                // Delegate save action to childs
                let id = database::last_insert_id(transaction);

                // Read collected HashMap index
                for (_, item) in self.index.borrow().iter() {
                    item.save(
                        transaction,
                        id,
                        self.widget.tab_view.page_position(&item.widget.tab_page),
                        item.widget.tab_page.is_pinned(),
                        item.widget.tab_page.is_selected(),
                        item.widget.tab_page.needs_attention(),
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
            self.append(Position::End, None, false, true, false, false);
        }

        // @TODO other/child features..
    }

    fn item(&self, position: Option<i32>) -> Option<Rc<Item>> {
        if let Some(page) = self.widget.page(position) {
            if let Some(id) = page.keyword() {
                if let Some(item) = self.index.borrow().get(&id) {
                    return Some(item.clone());
                }
            }
        }
        None
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    item::migrate(tx)?;

    // Success
    Ok(())
}
