mod action;
mod database;
mod header;
mod tab;
mod widget;

use action::Action;
use database::Database;
use header::Header;
use sqlite::Transaction;
use tab::Tab;
use widget::Widget;

use crate::app::browser::action::Action as BrowserAction;
use gtk::{glib::GString, Box};
use std::rc::Rc;

pub struct Window {
    //header: Rc<Header>,
    tab: Rc<Tab>,
    action: Rc<Action>,
    widget: Rc<Widget>,
}

impl Window {
    // Construct
    pub fn new(
        // Actions
        browser_action: Rc<BrowserAction>,
    ) -> Self {
        // Init local actions
        let action = Rc::new(Action::new());

        // Init components
        let tab = Rc::new(Tab::new(browser_action.clone(), action.clone()));
        let header = Header::new(browser_action, action.clone(), tab.gobject());

        // GTK
        let widget = Rc::new(Widget::new(header.gobject(), tab.gobject()));

        // Init events
        action.append().connect_activate({
            let tab = tab.clone();
            move || {
                tab.append(None);
            }
        });

        action.pin().connect_activate({
            let tab = tab.clone();
            move |position| tab.pin(position)
        });

        action.reload().connect_activate({
            let tab = tab.clone();
            move |position| tab.page_reload(position)
        });

        action.home().connect_activate({
            let tab = tab.clone();
            move |position| tab.page_home(position)
        });

        action.close().connect_activate({
            let tab = tab.clone();
            move |position| {
                tab.close(position);
            }
        });

        action.close_all().connect_activate({
            let tab = tab.clone();
            move |_| {
                tab.close_all();
            } // @TODO position not in use
        });

        action.history_back().connect_activate({
            let tab = tab.clone();
            move |position| {
                tab.page_history_back(position);
            } // @TODO rename destination method
        });

        action.history_forward().connect_activate({
            let tab = tab.clone();
            move |position| {
                tab.page_history_forward(position);
            } // @TODO rename destination method
        });

        // Init struct
        Self {
            //header,
            tab,
            action,
            widget,
        }
    }

    // Actions
    pub fn update(&self, tab_item_id: Option<GString>) {
        self.tab.update(tab_item_id);
    }

    pub fn clean(&self, transaction: &Transaction, app_browser_id: &i64) -> Result<(), String> {
        match Database::records(transaction, app_browser_id) {
            Ok(records) => {
                for record in records {
                    match Database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to childs
                            self.tab.clean(transaction, &record.id)?;
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn restore(&self, transaction: &Transaction, app_browser_id: &i64) -> Result<(), String> {
        match Database::records(transaction, app_browser_id) {
            Ok(records) => {
                for record in records {
                    // Delegate restore action to childs
                    self.tab.restore(transaction, &record.id)?;
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn save(&self, transaction: &Transaction, app_browser_id: &i64) -> Result<(), String> {
        match Database::add(transaction, app_browser_id) {
            Ok(_) => {
                // Delegate save action to childs
                if let Err(e) = self
                    .tab
                    .save(transaction, &Database::last_insert_id(transaction))
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

    // Getters

    pub fn action(&self) -> &Rc<Action> {
        &self.action
    }

    pub fn tab(&self) -> &Rc<Tab> {
        &self.tab
    }

    pub fn gobject(&self) -> &Box {
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
    tab::migrate(tx)?;

    // Success
    Ok(())
}
