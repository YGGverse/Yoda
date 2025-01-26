mod bookmark;
mod database;
mod history;
mod home;
mod reload;
mod request;
mod widget;

use super::{ItemAction, Profile, TabAction, WindowAction};
use bookmark::Bookmark;
use gtk::{Box, Button};
use history::History;
use home::Home;
use reload::Reload;
use request::Request;
use sqlite::Transaction;
use std::rc::Rc;
use widget::Widget;

pub struct Navigation {
    pub profile: Rc<Profile>,
    pub home: Button,
    pub reload: Button,
    pub bookmark: Button,
    pub request: Rc<Request>,
    pub widget: Rc<Widget>,
}

impl Navigation {
    pub fn build(
        profile: &Rc<Profile>,
        (window_action, tab_action, item_action): (
            &Rc<WindowAction>,
            &Rc<TabAction>,
            &Rc<ItemAction>,
        ),
    ) -> Self {
        // init children components

        let history = Box::history((window_action, tab_action, item_action));
        let request = Rc::new(Request::build(item_action, profile));
        let reload = Button::reload((window_action, tab_action, item_action), &request);
        let home = Button::home((window_action, tab_action, item_action), &request);
        let bookmark = Button::bookmark(window_action);

        // init main widget
        let widget = Rc::new(Widget::build(
            &home,
            &history,
            &reload,
            &request.entry, // @TODO
            &bookmark,
        ));

        // done
        Self {
            profile: profile.clone(),
            home,
            request,
            reload,
            bookmark,
            widget,
        }
    }

    // Actions

    pub fn update(&self) {
        // init shared request value
        let request = self.request.strip_prefix();

        // update children components
        self.bookmark
            .update(self.profile.bookmark.get(&request).is_ok());
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_id: &i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_tab_item_page_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to the item childs
                            self.request.clean(transaction, &record.id)?;
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
        app_browser_window_tab_item_page_id: &i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_tab_item_page_id) {
            Ok(records) => {
                for record in records {
                    // Delegate restore action to the item childs
                    self.request.restore(transaction, &record.id)?;
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_id: &i64,
    ) -> Result<(), String> {
        match database::insert(transaction, app_browser_window_tab_item_page_id) {
            Ok(_) => {
                let id = database::last_insert_id(transaction);

                // Delegate save action to childs
                self.request.save(transaction, &id)?;
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
    request::migrate(tx)?;

    // Success
    Ok(())
}
