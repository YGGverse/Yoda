mod action;
mod client;
mod database;
mod page;

use super::{Action as TabAction, BrowserAction, WindowAction};
use crate::Profile;
use action::Action;
use adw::TabPage;
use client::Client;
use gtk::{
    prelude::{ActionExt, ActionMapExt, BoxExt},
    Box,
};
use page::Page;
use sqlite::Transaction;
use std::rc::Rc;

pub struct Item {
    // Multi-protocol handler
    pub client: Rc<Client>,
    // Components
    pub page: Rc<Page>,
    pub action: Rc<Action>,
    pub tab_page: TabPage,
}

impl Item {
    // Constructors

    /// Build new `Self`
    pub fn build(
        (tab_page, target_child): (&TabPage, &Box),
        profile: &Rc<Profile>,
        (browser_action, window_action, tab_action): (
            &Rc<BrowserAction>,
            &Rc<WindowAction>,
            &Rc<TabAction>,
        ),
        request: Option<&str>,
        is_load: bool,
    ) -> Self {
        // Init components
        let action = Rc::new(Action::new());

        tab_action.simple_action_group.add_action(&action.home);
        tab_action.simple_action_group.add_action(&action.reload);
        tab_action.simple_action_group.add_action(&action.identity);

        tab_action
            .simple_action_group
            .add_action(&action.history.back);

        tab_action
            .simple_action_group
            .add_action(&action.history.forward);

        // Create new `Page` implementation for `TabPage`
        let page = Rc::new(Page::build(
            profile,
            (browser_action, window_action, tab_action, &action),
            tab_page,
        ));

        target_child.append(&page.navigation.g_box);
        target_child.append(&page.content.g_box);
        target_child.append(&page.search.g_box);
        target_child.append(&page.input.clamp);

        // Update tab loading indicator
        let client = Rc::new(Client::init(profile, &page));

        // Connect events
        action.home.connect_enabled_notify({
            let window_action = window_action.clone();
            move |this| {
                window_action
                    .home
                    .simple_action
                    .set_enabled(this.is_enabled())
            }
        });

        action.home.connect_activate({
            let client = client.clone();
            let page = page.clone();
            move |this, _| {
                this.set_enabled(false);
                if let Some(uri) = page.navigation.home() {
                    let request = uri.to_string();
                    page.navigation.set_request(&request);
                    client.handle(&request, true);
                }
            }
        });

        action.load.connect_activate({
            let page = page.clone();
            let client = client.clone();
            move |request, is_history| {
                if let Some(text) = request {
                    page.navigation.set_request(&text);
                    client.handle(&text, is_history);
                }
            }
        });

        action.identity.connect_activate({
            let page = page.clone();
            move |_, _| page.navigation.show_identity_dialog()
        });

        action.reload.connect_activate({
            let page = page.clone();
            let client = client.clone();
            move |_, _| {
                client.handle(&page.navigation.request(), true);
            }
        });

        action.reload.connect_enabled_notify({
            let window_action = window_action.clone();
            move |this| {
                window_action
                    .reload
                    .simple_action
                    .set_enabled(this.is_enabled())
            }
        });

        action.history.back.connect_enabled_notify({
            let window_action = window_action.clone();
            move |this| {
                window_action
                    .history_back
                    .simple_action
                    .set_enabled(this.is_enabled())
            }
        });

        action.history.forward.connect_enabled_notify({
            let window_action = window_action.clone();
            move |this| {
                window_action
                    .history_forward
                    .simple_action
                    .set_enabled(this.is_enabled())
            }
        });

        // Handle immediately on request
        if let Some(text) = request {
            page.navigation.set_request(text);
            if is_load {
                client.handle(text, true);
            }
        }

        Self {
            client,
            page,
            action,
            tab_page: tab_page.clone(),
        }
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
        app_browser_window_tab_id: i64,
        page_position: i32,
    ) -> Result<(), String> {
        match database::insert(
            transaction,
            app_browser_window_tab_id,
            page_position,
            self.tab_page.is_pinned(),
            self.tab_page.is_selected(),
        ) {
            Ok(_) => {
                // Delegate save action to childs
                let id = database::last_insert_id(transaction);
                self.page.save(transaction, id)?;
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

    // Success
    Ok(())
}

// This feature restore require parental implementation
// * see `super::Tab::restore()`
pub fn restore(
    transaction: &Transaction,
    app_browser_window_tab_id: i64,
) -> Result<Vec<database::Table>, String> {
    match database::select(transaction, app_browser_window_tab_id) {
        Ok(records) => Ok(records),
        Err(e) => Err(e.to_string()),
    }
}
