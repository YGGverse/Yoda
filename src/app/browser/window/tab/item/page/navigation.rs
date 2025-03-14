mod bookmark;
mod database;
mod history;
mod home;
mod reload;
mod request;

use super::{ItemAction, Profile, TabAction, WindowAction};
use anyhow::Result;
use bookmark::Bookmark;
use gtk::{
    glib::{GString, Uri},
    prelude::{BoxExt, EditableExt, EntryExt, WidgetExt},
    Box, Button, Orientation,
};
use history::History;
use home::Home;
use reload::Reload;
use request::Request;
use sqlite::Transaction;
use std::{rc::Rc, sync::Arc};

const MARGIN: i32 = 6;
const SPACING: i32 = 6;

pub struct Navigation {
    request: Rc<Request>,
    pub g_box: Box,
}

impl Navigation {
    pub fn build(
        profile: &Arc<Profile>,
        (window_action, tab_action, item_action): (
            &Rc<WindowAction>,
            &Rc<TabAction>,
            &Rc<ItemAction>,
        ),
    ) -> Self {
        // Init children components
        let history = Box::history((window_action, tab_action, item_action));
        let request = Rc::new(Request::build(item_action, profile));
        let reload = Button::reload((window_action, tab_action, item_action), &request);
        let home = Button::home((window_action, tab_action, item_action), &request);
        let bookmark = Button::bookmark(window_action, profile, &request.entry);

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
        g_box.append(&request.entry);
        g_box.append(&bookmark);

        Self { request, g_box }
    }

    // Actions

    pub fn escape(&self) {
        self.request.escape();
    }

    pub fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_id: &i64,
    ) -> Result<()> {
        for record in database::select(transaction, app_browser_window_tab_item_page_id)? {
            database::delete(transaction, &record.id)?;
            // Delegate clean action to the item childs
            self.request.clean(transaction, &record.id)?;
        }
        Ok(())
    }

    pub fn restore(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_id: &i64,
    ) -> Result<()> {
        for record in database::select(transaction, app_browser_window_tab_item_page_id)? {
            // Delegate restore action to the item childs
            self.request.restore(transaction, &record.id)?;
        }
        Ok(())
    }

    pub fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_id: &i64,
    ) -> Result<()> {
        let id = database::insert(transaction, app_browser_window_tab_item_page_id)?;
        // Delegate save action to childs
        self.request.save(transaction, &id)?;
        Ok(())
    }

    pub fn grab_focus(&self) -> bool {
        self.request.entry.grab_focus()
    }

    pub fn show_identity_dialog(&self) {
        self.request.show_identity_dialog()
    }

    // Setters

    pub fn set_request(&self, value: &str) {
        self.request.entry.set_text(value);
    }

    pub fn set_progress_fraction(&self, value: f64) {
        self.request.entry.set_progress_fraction(value);
    }

    pub fn to_download(&self) {
        self.request.to_download();
    }

    pub fn to_source(&self) {
        self.request.to_source();
    }

    // Getters

    pub fn request(&self) -> GString {
        self.request.entry.text()
    }

    pub fn home(&self) -> Option<Uri> {
        self.request.home()
    }

    pub fn is_file(&self) -> bool {
        self.request.is_file()
    }
}

// Tools
pub fn migrate(tx: &Transaction) -> Result<()> {
    // Migrate self components
    database::init(tx)?;

    // Delegate migration to childs
    request::migrate(tx)?;

    // Success
    Ok(())
}
