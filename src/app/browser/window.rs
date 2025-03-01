mod action;
mod database;
mod header;
pub mod tab;

use action::{Action, Position};
use adw::ToolbarView;
use header::Header;
use sqlite::Transaction;
use tab::Tab;

use super::Action as BrowserAction;
use crate::Profile;
use gtk::{prelude::BoxExt, Box, Orientation};
use std::rc::Rc;

pub struct Window {
    pub action: Rc<Action>,
    pub tab: Rc<Tab>,
    pub g_box: Box,
}

impl Window {
    // Constructors

    /// Build new `Self`
    pub fn build(profile: &Rc<Profile>, browser_action: &Rc<BrowserAction>) -> Self {
        // Init local actions
        let action = Rc::new(Action::new());

        // Init components
        let tab = Rc::new(Tab::build(profile, (browser_action, &action)));

        // Init widget
        let g_box = Box::builder().orientation(Orientation::Vertical).build();

        g_box.append(&ToolbarView::header(
            (browser_action, &action),
            profile,
            &tab.tab_view,
        ));
        g_box.append(&tab.tab_view);

        // Init events
        action.append.connect_activate({
            let tab = tab.clone();
            move |position, request, is_pinned, is_selected, is_attention, is_load| {
                tab.append(
                    position,
                    request.as_deref(),
                    is_pinned,
                    is_selected,
                    is_attention,
                    is_load,
                );
            }
        });

        action.bookmark.connect_activate({
            let tab = tab.clone();
            move |position| {
                if tab.bookmark(position).is_err() {
                    todo!()
                }
            }
        });

        action.pin.connect_activate({
            let tab = tab.clone();
            move |position| tab.pin(position)
        });

        action.reload.connect_activate({
            let tab = tab.clone();
            move |position| tab.reload(position)
        });

        action.home.connect_activate({
            let tab = tab.clone();
            move |position| tab.home(position)
        });

        action.close.connect_activate({
            let tab = tab.clone();
            move |position| tab.close(position)
        });

        action.close_all.connect_activate({
            let tab = tab.clone();
            move |_| tab.close_all()
        });

        action.find.connect_activate({
            let tab = tab.clone();
            move |position| tab.find(position)
        });

        action.save_as.connect_activate({
            let tab = tab.clone();
            move |position| tab.save_as(position)
        });

        action.source.connect_activate({
            let tab = tab.clone();
            move |position| tab.source(position)
        });

        action.history_back.connect_activate({
            let tab = tab.clone();
            move |position| {
                tab.history_back(position);
            }
        });

        action.history_forward.connect_activate({
            let tab = tab.clone();
            move |position| {
                tab.history_forward(position);
            }
        });

        action.load.on_activate({
            let tab = tab.clone();
            move |_, request| {
                tab.append(Position::End, Some(&request), false, true, false, true);
            }
        });

        action.open.on_activate({
            let tab = tab.clone();
            move |position, request| {
                tab.open(position, request, true);
            }
        });

        // Init struct
        Self { action, tab, g_box }
    }

    // Actions
    pub fn escape(&self) {
        self.tab.escape();
    }

    pub fn clean(&self, transaction: &Transaction, app_browser_id: i64) -> Result<(), String> {
        match database::select(transaction, app_browser_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
                            self.tab.clean(transaction, record.id)?;
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }
        Ok(())
    }

    pub fn restore(&self, transaction: &Transaction, app_browser_id: i64) -> Result<(), String> {
        match database::select(transaction, app_browser_id) {
            Ok(records) => {
                for record in records {
                    // Delegate restore action to childs
                    self.tab.restore(transaction, record.id)?;
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn save(&self, transaction: &Transaction, app_browser_id: i64) -> Result<(), String> {
        match database::insert(transaction, app_browser_id) {
            Ok(_) => {
                // Delegate save action to childs
                if let Err(e) = self
                    .tab
                    .save(transaction, database::last_insert_id(transaction))
                {
                    return Err(e.to_string());
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn init(&self) {
        self.tab.init();
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    tab::migrate(tx)?;

    // Success
    Ok(())
}
