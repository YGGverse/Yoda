mod action;
mod database;
mod error;
mod item;
mod menu;
mod widget;

use action::Action;
use adw::{TabPage, TabView};
use error::Error;
pub use item::Item;
use menu::Menu;
use widget::Widget;

use super::{Action as WindowAction, BrowserAction, Position};
use crate::Profile;
use gtk::{
    glib::{DateTime, Propagation},
    prelude::{ActionExt, EditableExt, WidgetExt},
};
use sqlite::Transaction;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

// Main
pub struct Tab {
    browser_action: Rc<BrowserAction>,
    window_action: Rc<WindowAction>,
    profile: Rc<Profile>,
    index: Rc<RefCell<HashMap<TabPage, Rc<Item>>>>,
    pub action: Rc<Action>,
    pub widget: Rc<Widget>,
}

impl Tab {
    // Constructors

    /// Build new `Self`
    pub fn build(
        profile: &Rc<Profile>,
        (browser_action, window_action): (&Rc<BrowserAction>, &Rc<WindowAction>),
    ) -> Self {
        let action = Rc::new(Action::new());

        // Init empty HashMap index
        let index = Rc::new(RefCell::new(HashMap::new()));

        // Init context menu
        let menu = Menu::new(window_action);

        // Init widget
        let widget = Rc::new(Widget::new(&menu.main));

        // Init events
        widget.tab_view.connect_setup_menu({
            let index = index.clone();
            let window_action = window_action.clone();
            move |tab_view, tab_page| {
                // by documentation:
                // * `tab_page` == `Some` - popover open
                // * `tab_page` == `None` - popover closed
                update_actions(tab_view, tab_page.cloned(), &index, &window_action);
            }
        });

        widget.tab_view.connect_close_page({
            let index = index.clone();
            let profile = profile.clone();
            let window_action = window_action.clone();
            move |tab_view, tab_page| {
                // cleanup HashMap index
                // add history record into profile memory pool
                // * this action allows to recover recently closed tab (e.g. from the main menu)
                profile.history.memory.tab.add(
                    index.borrow_mut().remove(tab_page).unwrap(),
                    DateTime::now_local().unwrap().to_unix(),
                );

                update_actions(tab_view, tab_view.selected_page(), &index, &window_action);
                Propagation::Proceed
            }
        });

        widget.tab_view.connect_page_attached({
            let window_action = window_action.clone();
            let index = index.clone();
            move |tab_view, _, _| {
                update_actions(tab_view, tab_view.selected_page(), &index, &window_action)
            }
        });

        widget.tab_view.connect_selected_page_notify({
            let window_action = window_action.clone();
            let index = index.clone();
            move |tab_view| {
                if let Some(tab_page) = tab_view.selected_page() {
                    tab_page.set_needs_attention(false);
                }
                update_actions(tab_view, tab_view.selected_page(), &index, &window_action)
            }
        });

        // Return activated `Self`
        Self {
            profile: profile.clone(),
            browser_action: browser_action.clone(),
            window_action: window_action.clone(),
            index,
            widget,
            action,
        }
    }

    // Actions
    pub fn append(
        &self,
        position: Position,
        request: Option<&str>,
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
            (&self.browser_action, &self.window_action, &self.action),
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

        // Expect user input on tab appended has empty request entry
        // * this action initiated here because should be applied on tab appending event only
        if request.is_none() || request.is_some_and(|value| value.is_empty()) {
            item.page.navigation.request.entry.grab_focus();
        }

        // Register dynamically created tab components in the HashMap index
        self.index
            .borrow_mut()
            .insert(item.widget.tab_page.clone(), item.clone());

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
    pub fn escape(&self) {
        if let Some(item) = self.item(None) {
            item.page.escape();
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
                item.page.navigation.request.entry.set_text(&home);
                item.client.handle(&home, true);
            }
        }
    }

    pub fn page_history_back(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.action.history.back(true);
        }
    }

    pub fn page_history_forward(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.action.history.forward(true);
        }
    }

    /// Reload page at `i32` position or selected page on `None` given
    pub fn page_reload(&self, page_position: Option<i32>) {
        if let Some(item) = self.item(page_position) {
            item.client
                .handle(&item.page.navigation.request.entry.text(), true);
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
                        (&self.browser_action, &self.window_action, &self.action),
                    ) {
                        Ok(items) => {
                            for item in items {
                                // Register dynamically created tab item in the HashMap index
                                self.index
                                    .borrow_mut()
                                    .insert(item.widget.tab_page.clone(), item.clone());
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
        if let Some(tab_page) = self.widget.page(position) {
            if let Some(item) = self.index.borrow().get(&tab_page) {
                return Some(item.clone());
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

fn update_actions(
    tab_view: &TabView,
    tab_page: Option<TabPage>,
    index: &Rc<RefCell<HashMap<TabPage, Rc<Item>>>>,
    window_action: &Rc<WindowAction>,
) {
    match tab_page {
        Some(tab_page) => {
            if let Some(item) = index.borrow().get(&tab_page) {
                window_action
                    .home
                    .simple_action
                    .set_enabled(item.action.home.is_enabled());
                window_action
                    .reload
                    .simple_action
                    .set_enabled(item.action.reload.is_enabled());
                window_action
                    .history_back
                    .simple_action
                    .set_enabled(item.action.history.back.is_enabled());
                window_action
                    .history_forward
                    .simple_action
                    .set_enabled(item.action.history.forward.is_enabled());

                window_action.change_state(Some(tab_view.page_position(&tab_page)));
            } // @TODO incorrect index init implementation, tabs refactory wanted
        }
        None => {
            // Reset to defaults
            window_action.home.simple_action.set_enabled(false);
            window_action.reload.simple_action.set_enabled(false);
            window_action.history_back.simple_action.set_enabled(false);
            window_action
                .history_forward
                .simple_action
                .set_enabled(false);

            window_action.change_state(None);
        }
    }
}
