mod database;
mod error;
mod item;
mod menu;
mod widget;

use error::Error;
use item::Item;
use menu::Menu;
use widget::Widget;

use crate::app::browser::{
    window::action::{Action as WindowAction, Position},
    Action as BrowserAction,
};
use crate::Profile;
use gtk::{
    glib::{GString, Propagation},
    prelude::WidgetExt,
};
use sqlite::Transaction;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

// Main
pub struct Tab {
    profile: Rc<Profile>,
    actions: (Rc<BrowserAction>, Rc<WindowAction>),
    index: Rc<RefCell<HashMap<Rc<GString>, Rc<Item>>>>,
    pub widget: Rc<Widget>,
}

impl Tab {
    // Construct
    pub fn new(profile: Rc<Profile>, action: (Rc<BrowserAction>, Rc<WindowAction>)) -> Self {
        // Init empty HashMap index
        let index: Rc<RefCell<HashMap<Rc<GString>, Rc<Item>>>> =
            Rc::new(RefCell::new(HashMap::new()));

        // Init context menu
        let menu = Menu::new(action.1.clone());

        // Init widget
        let widget = Rc::new(Widget::new(&menu.gobject));

        // Init events

        widget.gobject.connect_setup_menu({
            let action = action.1.clone();
            move |tab_view, tab_page| {
                // Set new state for page selected on menu open
                // * this action return default state (`None`) on menu close
                let state = tab_page.map(|this| tab_view.page_position(this));

                // Update actions with new state value
                action.bookmark.change_state(state);
                action.close_all.change_state(state);
                action.close.change_state(state);
                action.history_back.change_state(state);
                action.history_forward.change_state(state);
                action.home.change_state(state);
                action.pin.change_state(state);
                action.reload.change_state(state);
                action.save_as.change_state(state);
                action.source.change_state(state);
            }
        });

        widget.gobject.connect_close_page({
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

        widget.gobject.connect_selected_page_notify({
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
            profile,
            actions: (action.0, action.1),
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
        let item = Rc::new(Item::new(
            &self.widget.gobject,
            self.profile.clone(),
            // Actions
            (self.actions.0.clone(), self.actions.1.clone()),
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

    /// Close page at given `position`, `None` to close selected page (if available)
    pub fn close(&self, position: Option<i32>) {
        self.widget.close(position);
    }

    // Close all pages
    pub fn close_all(&self) {
        self.widget.close_all();
    }

    // Save page at given `position`, `None` to save selected page (if available)
    pub fn save_as(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.page.navigation.request.to_download();
            item.page.load(true);
        }
    }

    // View source for page at given `position`, `None` to use selected page (if available)
    pub fn source(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.page.navigation.request.to_source();
            item.page.load(true);
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
            item.page.home();
        }
    }

    pub fn page_history_back(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.page.history_back();
        }
    }

    pub fn page_history_forward(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.page.history_forward();
        }
    }

    /// Reload page at `i32` position or selected page on `None` given
    pub fn page_reload(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.page.load(true);
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
                if !item.page.is_loading() {
                    item.widget
                        .gobject
                        .set_title(&item.page.meta.title.borrow())
                }
            }
            // Update all tabs
            None => {
                for (_, item) in self.index.borrow().iter() {
                    // Update item components
                    item.update();

                    // Update tab title on loading indicator inactive
                    if !item.page.is_loading() {
                        item.widget
                            .gobject
                            .set_title(&item.page.meta.title.borrow())
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
        match database::select(transaction, app_browser_window_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, &record.id) {
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
        match database::select(transaction, app_browser_window_id) {
            Ok(records) => {
                for record in records {
                    match Item::restore(
                        &self.widget.gobject,
                        transaction,
                        &record.id,
                        self.profile.clone(),
                        (self.actions.0.clone(), self.actions.1.clone()),
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
        app_browser_window_id: &i64,
    ) -> Result<(), String> {
        match database::insert(transaction, app_browser_window_id) {
            Ok(_) => {
                // Delegate save action to childs
                let id = database::last_insert_id(transaction);

                // Read collected HashMap index
                for (_, item) in self.index.borrow().iter() {
                    item.save(
                        transaction,
                        &id,
                        &self.widget.gobject.page_position(&item.widget.gobject),
                        &item.widget.gobject.is_pinned(),
                        &item.widget.gobject.is_selected(),
                        &item.widget.gobject.needs_attention(),
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
