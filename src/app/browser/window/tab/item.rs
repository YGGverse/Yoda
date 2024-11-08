mod database;
mod page;
mod widget;

use database::Database;
use page::Page;
use widget::Widget;

use crate::action::Browser as BrowserAction;
use adw::{TabPage, TabView};
use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, GString},
};
use sqlite::Transaction;
use std::rc::Rc;

pub struct Item {
    // Auto-generated unique item ID
    // useful as widget name in GTK actions callback
    id: GString,
    // Components
    page: Rc<Page>,
    widget: Rc<Widget>,
}

impl Item {
    // Construct
    pub fn new_rc(
        tab_view: &TabView,
        // Global actions
        browser_action: Rc<BrowserAction>,
        // @TODO
        action_tab_open: SimpleAction,
        action_page_home: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
        action_page_reload: SimpleAction,
        // Options
        position: Option<i32>,
        is_pinned: bool,
        is_selected: bool,
    ) -> Rc<Self> {
        // Generate unique ID for new page components
        let id = uuid_string_random();

        // Init components
        let page = Page::new_rc(
            id.clone(),
            // Actions
            browser_action,
            action_tab_open.clone(),
            action_page_home.clone(),
            action_page_history_back.clone(),
            action_page_history_forward.clone(),
            action_page_reload.clone(),
        );

        let widget = Widget::new_rc(
            id.as_str(),
            tab_view,
            page.gobject(),
            None,
            position,
            is_pinned,
            is_selected,
        ); // @TODO

        // Return struct
        Rc::new(Self { id, page, widget })
    }

    // Actions
    pub fn page_home(&self) {
        self.page.home()
    }

    pub fn page_history_back(&self) {
        self.page.history_back()
    }

    pub fn page_history_forward(&self) {
        self.page.history_forward()
    }

    pub fn page_reload(&self) {
        self.page.reload()
    }

    pub fn page_navigation_request_grab_focus(&self) {
        self.page.navigation_request_grab_focus()
    }

    pub fn update(&self) {
        // Update child components
        self.page.update();

        // Update tab loading indicator
        self.widget.gobject().set_loading(self.page_is_loading());
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_id: &i64,
    ) -> Result<(), String> {
        match Database::records(transaction, app_browser_window_tab_id) {
            Ok(records) => {
                for record in records {
                    match Database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to the item childs
                            self.page.clean(transaction, &record.id)?;
                            self.widget.clean(transaction, &record.id)?;
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    // This method does not contain Self context,
    // because child items creating in the runtime (by parent component)
    pub fn restore(
        tab_view: &TabView,
        transaction: &Transaction,
        app_browser_window_tab_id: &i64,
        // Actions
        browser_action: Rc<BrowserAction>,
        action_tab_open: SimpleAction,
        action_page_home: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
        action_page_reload: SimpleAction,
    ) -> Result<Vec<Rc<Item>>, String> {
        let mut items = Vec::new();

        match Database::records(transaction, app_browser_window_tab_id) {
            Ok(records) => {
                for record in records {
                    // Construct new item object
                    let item = Item::new_rc(
                        tab_view,
                        // Actions
                        browser_action.clone(),
                        action_tab_open.clone(),
                        action_page_home.clone(),
                        action_page_history_back.clone(),
                        action_page_history_forward.clone(),
                        action_page_reload.clone(),
                        // Options
                        None,
                        record.is_pinned,
                        record.is_selected,
                    );

                    // Delegate restore action to the item childs
                    item.page.restore(transaction, &record.id)?;
                    item.widget.restore(transaction, &record.id)?;

                    // Result
                    items.push(item);
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(items)
    }

    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_id: &i64,
        page_position: &i32,
        is_pinned: &bool,
        is_selected: &bool,
    ) -> Result<(), String> {
        match Database::add(
            transaction,
            app_browser_window_tab_id,
            page_position,
            is_pinned,
            is_selected,
        ) {
            Ok(_) => {
                let id = Database::last_insert_id(transaction);

                // Delegate save action to childs
                self.page.save(transaction, &id)?;
                self.widget.save(transaction, &id)?;
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    // Setters
    pub fn set_page_navigation_request_text(&self, value: &str) {
        self.page.set_navigation_request_text(value);
    }

    // Getters
    pub fn id(&self) -> GString {
        self.id.clone()
    }

    pub fn page_is_loading(&self) -> bool {
        self.page.is_loading()
    }

    pub fn page_meta_title(&self) -> GString {
        self.page.meta_title()
    }

    pub fn gobject(&self) -> &TabPage {
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
    page::migrate(tx)?;
    widget::migrate(tx)?;

    // Success
    Ok(())
}
