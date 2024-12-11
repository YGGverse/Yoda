mod action;
mod database;
mod identity;
mod page;
mod widget;

use action::Action;
use page::Page;
use widget::Widget;

use crate::app::browser::{
    window::action::{Action as WindowAction, Position},
    Action as BrowserAction,
};
use crate::Profile;
use adw::TabView;
use gtk::{
    glib::{uuid_string_random, GString},
    prelude::{Cast, EditableExt},
};
use sqlite::Transaction;
use std::rc::Rc;

pub struct Item {
    // Auto-generated unique item ID
    // useful as widget name in GTK actions callback
    pub id: Rc<GString>,
    // Components
    pub page: Rc<Page>,
    pub widget: Rc<Widget>,
}

impl Item {
    // Construct
    pub fn new(
        tab_view: &TabView,
        profile: Rc<Profile>,
        actions: (Rc<BrowserAction>, Rc<WindowAction>),
        options: (Position, Option<String>, bool, bool, bool, bool),
    ) -> Self {
        // Get item options from tuple
        let (position, request, is_pinned, is_selected, is_attention, is_load) = options;

        // Generate unique ID for new page components
        let id = Rc::new(uuid_string_random());

        // Init components

        let action = Rc::new(Action::new());

        let page = Rc::new(Page::new(
            id.clone(),
            profile.clone(),
            (actions.0.clone(), actions.1.clone(), action.clone()),
        ));

        let widget = Rc::new(Widget::new(
            id.as_str(),
            tab_view,
            page.widget.gobject(),
            None,
            position,
            (is_pinned, is_selected, is_attention),
        ));

        // Init events

        if let Some(text) = request {
            page.navigation.request.widget.entry.set_text(&text);

            if is_load {
                page.load(true);
            }
        }

        // Show identity selection for item
        action.ident.connect_activate({
            let browser_action = actions.0.clone();
            let window_action = actions.1.clone();
            let page = page.clone();
            let parent = tab_view.clone().upcast::<gtk::Widget>();
            move || {
                // Request should match valid URI for all drivers supported
                if let Some(uri) = page.navigation.request.uri() {
                    // Rout by scheme
                    if uri.scheme().to_lowercase() == "gemini" {
                        return identity::new_gemini(
                            (browser_action.clone(), window_action.clone()),
                            profile.clone(),
                            uri,
                        )
                        .present(Some(&parent));
                    }
                }
                // Show dialog with unsupported request message
                identity::new_unsupported().present(Some(&parent));
            }
        });

        // Load new request for item
        action.load.connect_activate({
            let page = page.clone();
            move |request, is_history| {
                if let Some(text) = request {
                    page.navigation.request.widget.entry.set_text(&text);
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
        self.widget.gobject.set_loading(self.page.is_loading());
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_id: &i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_tab_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, &record.id) {
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
        profile: Rc<Profile>,
        // Actions
        action: (Rc<BrowserAction>, Rc<WindowAction>),
    ) -> Result<Vec<Rc<Item>>, String> {
        let mut items = Vec::new();

        match database::select(transaction, app_browser_window_tab_id) {
            Ok(records) => {
                for record in records {
                    // Construct new item object
                    let item = Rc::new(Item::new(
                        tab_view,
                        profile.clone(),
                        // Actions
                        (action.0.clone(), action.1.clone()),
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
        match database::insert(
            transaction,
            app_browser_window_tab_id,
            page_position,
            is_pinned,
            is_selected,
            is_attention,
        ) {
            Ok(_) => {
                let id = database::last_insert_id(transaction);

                // Delegate save action to childs
                self.page.save(transaction, &id)?;
                self.widget.save(transaction, &id)?;
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    page::migrate(tx)?;
    widget::migrate(tx)?;

    // Success
    Ok(())
}
