mod action;
mod database;
mod page;
mod widget;

use action::Action;
use database::Database;
use page::Page;
use widget::Widget;

use crate::app::browser::{
    window::action::{Action as WindowAction, Position},
    Action as BrowserAction,
};
use adw::TabView;
use gtk::{
    glib::{uuid_string_random, GString},
    prelude::EditableExt,
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
    pub fn new(
        tab_view: &TabView,
        // Actions
        browser_action: Rc<BrowserAction>,
        window_action: Rc<WindowAction>,
        // Options tuple @TODO struct?
        options: (Position, Option<String>, bool, bool, bool, bool),
    ) -> Self {
        // Get item options from tuple
        let (position, request, is_pinned, is_selected, is_attention, is_load) = options;

        // Generate unique ID for new page components
        let id = uuid_string_random();

        // Init components

        let action = Rc::new(Action::new());

        let page = Rc::new(Page::new(
            id.clone(),
            browser_action,
            window_action,
            action.clone(),
        ));

        let widget = Rc::new(Widget::new(
            id.as_str(),
            tab_view,
            page.widget().gobject(),
            None,
            position,
            (is_pinned, is_selected, is_attention),
        ));

        // Init events

        if let Some(text) = request {
            page.navigation()
                .request()
                .widget()
                .gobject()
                .set_text(&text);

            if is_load {
                page.load(true);
            }
        }

        action.load().connect_activate({
            let page = page.clone();
            move |request, is_history| {
                if let Some(text) = request {
                    page.navigation()
                        .request()
                        .widget()
                        .gobject()
                        .set_text(&text);
                }
                page.load(is_history);
            }
        });

        // Done
        Self { id, page, widget }
    }

    // Actions
    pub fn update(&self) {
        // Update child components
        self.page.update();

        // Update tab loading indicator
        self.widget.gobject().set_loading(self.page().is_loading());
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
        window_action: Rc<WindowAction>,
    ) -> Result<Vec<Rc<Item>>, String> {
        let mut items = Vec::new();

        match Database::records(transaction, app_browser_window_tab_id) {
            Ok(records) => {
                for record in records {
                    // Construct new item object
                    let item = Rc::new(Item::new(
                        tab_view,
                        // Actions
                        browser_action.clone(),
                        window_action.clone(),
                        // Options tuple
                        (
                            Position::End,
                            None,
                            record.is_pinned,
                            record.is_selected,
                            record.is_attention,
                            false,
                        ),
                    ));

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
        is_attention: &bool,
    ) -> Result<(), String> {
        match Database::add(
            transaction,
            app_browser_window_tab_id,
            page_position,
            is_pinned,
            is_selected,
            is_attention,
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

    // Getters

    pub fn id(&self) -> &GString {
        &self.id
    }

    pub fn page(&self) -> &Rc<Page> {
        &self.page
    }

    pub fn widget(&self) -> &Rc<Widget> {
        &self.widget
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
