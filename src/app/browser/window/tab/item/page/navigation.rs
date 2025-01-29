mod bookmark;
mod database;
mod history;
mod home;
mod reload;
mod request;

use super::{ItemAction, Profile, TabAction, WindowAction};
use bookmark::Bookmark;
use gtk::{
    glib::{GString, Uri},
    prelude::{BoxExt, EditableExt, EntryExt, WidgetExt},
    Box, Button, Entry, Orientation,
};
use history::History;
use home::Home;
use reload::Reload;
use request::Request;
use sqlite::Transaction;
use std::rc::Rc;

const MARGIN: i32 = 6;
const SPACING: i32 = 6;

pub struct Navigation {
    profile: Rc<Profile>,
    request: Entry,
    pub g_box: Box,
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
        // Init children components
        let history = Box::history((window_action, tab_action, item_action));
        let request = Entry::request(item_action, profile);
        let reload = Button::reload((window_action, tab_action, item_action), &request);
        let home = Button::home((window_action, tab_action, item_action), &request);
        let bookmark = Button::bookmark(window_action, profile, &request);

        // Init main widget
        let g_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(SPACING)
            .margin_start(MARGIN)
            .margin_end(MARGIN)
            .margin_bottom(MARGIN)
            .build();

        g_box.append(&home);
        g_box.append(&history);
        g_box.append(&reload);
        g_box.append(&request);
        g_box.append(&bookmark);

        Self {
            profile: profile.clone(),
            request,
            g_box,
        }
    }

    // Actions

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

    pub fn grab_focus(&self) -> bool {
        self.request.grab_focus()
    }

    pub fn show_identity_dialog(&self) {
        self.request.show_identity_dialog(&self.profile)
    }

    // Setters

    pub fn set_request(&self, value: &str) {
        self.request.set_text(value);
    }

    pub fn set_progress_fraction(&self, value: f64) {
        self.request.set_progress_fraction(value);
    }

    pub fn to_download(&self) {
        self.request.to_download();
    }

    pub fn to_source(&self) {
        self.request.to_source();
    }

    // Getters

    pub fn request(&self) -> GString {
        self.request.text()
    }

    pub fn uri(&self) -> Option<Uri> {
        self.request.uri()
    }

    pub fn home(&self) -> Option<Uri> {
        self.request.home()
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
