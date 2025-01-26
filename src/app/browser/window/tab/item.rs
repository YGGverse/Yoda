mod action;
mod client;
mod database;
mod identity;
pub mod page;
mod widget;

use super::{Action as TabAction, BrowserAction, Position, WindowAction};
use crate::Profile;
use action::Action;
use adw::TabView;
use client::Client;
use gtk::{
    glib::{uuid_string_random, GString},
    prelude::{ActionMapExt, Cast, EditableExt},
};
use page::Page;
use sqlite::Transaction;
use std::rc::Rc;
use widget::Widget;

pub struct Item {
    // Auto-generated unique item ID
    // useful as widget name in GTK actions callback
    pub id: Rc<GString>,
    // Multi-protocol handler
    pub client: Rc<Client>,
    // Components
    pub page: Rc<Page>,
    pub widget: Rc<Widget>,
    pub action: Rc<Action>,
}

impl Item {
    // Constructors

    /// Build new `Self`
    pub fn build(
        tab_view: &TabView,
        profile: &Rc<Profile>,
        (browser_action, window_action, tab_action): (
            &Rc<BrowserAction>,
            &Rc<WindowAction>,
            &Rc<TabAction>,
        ),
        (position, request, is_pinned, is_selected, is_attention, is_load): (
            Position,
            Option<&str>,
            bool,
            bool,
            bool,
            bool,
        ),
    ) -> Self {
        // Generate unique ID for new page components
        let id = Rc::new(uuid_string_random());

        // Init components
        let action = Rc::new(Action::new());

        tab_action.simple_action_group.add_action(&action.home);

        tab_action
            .simple_action_group
            .add_action(&action.history.back);

        tab_action
            .simple_action_group
            .add_action(&action.history.forward);

        let page = Rc::new(Page::build(
            &id,
            profile,
            (browser_action, window_action, tab_action, &action),
        ));

        let widget = Rc::new(Widget::build(
            id.as_str(),
            tab_view,
            &page.widget.g_box,
            None,
            position,
            (is_pinned, is_selected, is_attention),
        ));

        // Update tab loading indicator
        let client = Rc::new(Client::init(&page, &widget.tab_page));

        // Connect events
        action.home.connect_activate({
            let client = client.clone();
            let page = page.clone();
            move |this, _| {
                this.set_enabled(false);
                if let Some(uri) = page.navigation.request.home() {
                    let request = uri.to_string();
                    page.navigation.request.widget.entry.set_text(&request);
                    client.handle(&request, true);
                }
            }
        });

        action.ident.connect_activate({
            let browser_action = browser_action.clone();
            let page = page.clone();
            let parent = tab_view.clone().upcast::<gtk::Widget>();
            let profile = profile.clone();
            let window_action = window_action.clone();
            move || {
                if let Some(uri) = page.navigation.request.uri() {
                    let scheme = uri.scheme();
                    if scheme == "gemini" || scheme == "titan" {
                        return identity::default(
                            (&browser_action, &window_action),
                            &profile,
                            &uri,
                        )
                        .present(Some(&parent));
                    }
                }
                identity::unsupported().present(Some(&parent));
            }
        });

        action.load.connect_activate({
            let page = page.clone();
            let client = client.clone();
            move |request, is_history| {
                if let Some(text) = request {
                    page.navigation.request.widget.entry.set_text(&text);
                    client.handle(&text, is_history);
                }
            }
        });

        // Handle immediately on request
        if let Some(text) = request {
            page.navigation.request.widget.entry.set_text(text);
            if is_load {
                client.handle(text, true);
            }
        }
        // Done
        Self {
            id,
            client,
            page,
            widget,
            action,
        }
    }

    // Actions
    pub fn update(&self) {
        // Update self actions
        self.action
            .home
            .set_enabled(self.page.navigation.request.home().is_some_and(|home| {
                home.to_string() != self.page.navigation.request.widget.entry.text()
            }));

        // Update child components
        self.page.update();
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_id: i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_tab_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, record.id) {
                        Ok(_) => {
                            // Delegate clean action to the item childs
                            self.page.clean(transaction, record.id)?;
                            self.widget.clean(transaction, record.id)?;
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
        app_browser_window_tab_id: i64,
        profile: &Rc<Profile>,
        // Actions
        (browser_action, window_action, item_action): (
            &Rc<BrowserAction>,
            &Rc<WindowAction>,
            &Rc<super::Action>,
        ),
    ) -> Result<Vec<Rc<Item>>, String> {
        let mut items = Vec::new();

        match database::select(transaction, app_browser_window_tab_id) {
            Ok(records) => {
                for record in records {
                    // Construct new item object
                    let item = Rc::new(Item::build(
                        tab_view,
                        profile,
                        // Actions
                        (browser_action, window_action, item_action),
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
                    item.page.restore(transaction, record.id)?;
                    item.widget.restore(transaction, record.id)?;

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
        app_browser_window_tab_id: i64,
        page_position: i32,
        is_pinned: bool,
        is_selected: bool,
        is_attention: bool,
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
                self.page.save(transaction, id)?;
                self.widget.save(transaction, id)?;
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
