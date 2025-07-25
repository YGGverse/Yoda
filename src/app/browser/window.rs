pub mod action;
mod database;
mod header;
pub mod tab;

use super::Action as BrowserAction;
use crate::Profile;
use action::{Action, Position};
use adw::ToolbarView;
use anyhow::Result;
use gtk::{Box, Orientation, prelude::BoxExt};
use header::Header;
use sqlite::Transaction;
use std::rc::Rc;
use tab::Tab;

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
        let tab = Rc::new(Tab::build(profile, &action));

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
            move |position| tab.bookmark(position)
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
            move |position| tab.history_back(position)
        });

        action.history_forward.connect_activate({
            let tab = tab.clone();
            move |position| tab.history_forward(position)
        });

        action.load.on_activate({
            let tab = tab.clone();
            move |_, request| {
                tab.append(Position::End, Some(&request), false, true, false, true);
            }
        });

        action.open.on_activate({
            let tab = tab.clone();
            move |position, request| tab.open(position, request)
        });

        // Init struct
        Self {
            g_box: {
                let g_box = Box::builder().orientation(Orientation::Vertical).build();
                g_box.append(&ToolbarView::header(
                    (browser_action, &action),
                    &tab.tab_view,
                ));
                g_box.append(&tab.tab_view);
                g_box
            },
            action,
            tab,
        }
    }

    // Actions

    pub fn clean(&self, transaction: &Transaction, app_browser_id: i64) -> Result<()> {
        for record in database::select(transaction, app_browser_id)? {
            database::delete(transaction, record.id)?;
            // Delegate clean action to childs
            self.tab.clean(transaction, record.id)?;
        }
        Ok(())
    }

    pub fn restore(&self, transaction: &Transaction, app_browser_id: i64) -> Result<()> {
        for record in database::select(transaction, app_browser_id)? {
            // Delegate restore action to childs
            self.tab.restore(transaction, record.id)?;
        }
        Ok(())
    }

    pub fn save(&self, transaction: &Transaction, app_browser_id: i64) -> Result<()> {
        self.tab
            .save(transaction, database::insert(transaction, app_browser_id)?)?;
        Ok(())
    }

    pub fn init(&self) {
        self.tab.init();
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<()> {
    // Migrate self components
    database::init(tx)?;

    // Delegate migration to childs
    tab::migrate(tx)?;

    // Success
    Ok(())
}
