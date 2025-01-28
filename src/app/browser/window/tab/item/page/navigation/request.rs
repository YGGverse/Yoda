mod database;
mod primary_icon;

use primary_icon::PrimaryIcon;

use super::{ItemAction, Profile};
use gtk::{
    glib::{gformat, GString, Uri, UriFlags},
    prelude::{EditableExt, EntryExt, WidgetExt},
    Entry, EntryIconPosition, StateFlags,
};
use sqlite::Transaction;
use std::{cell::Cell, rc::Rc};

const PREFIX_DOWNLOAD: &str = "download:";
const PREFIX_SOURCE: &str = "source:";

pub trait Request {
    // Constructors

    fn request(item_action: &Rc<ItemAction>, profile: &Rc<Profile>) -> Self;

    // Actions

    fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String>;

    fn restore(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String>;

    fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String>;

    fn update(&self, profile: &Profile);

    // Setters

    fn to_download(&self);
    fn to_source(&self);

    // Getters

    fn strip_prefix(&self) -> GString;
    fn download(&self) -> GString;
    fn source(&self) -> GString;
    fn uri(&self) -> Option<Uri>;
    fn home(&self) -> Option<Uri>;
}

impl Request for Entry {
    // Constructors

    /// Build new `Self`
    fn request(item_action: &Rc<ItemAction>, profile: &Rc<Profile>) -> Self {
        // Init main widget
        let entry = Entry::builder()
            .placeholder_text("URL or search term...")
            .secondary_icon_tooltip_text("Go to the location")
            .hexpand(true)
            .build();

        // Connect events
        entry.connect_icon_release({
            let item_action = item_action.clone();
            move |this, position| match position {
                EntryIconPosition::Primary => item_action.ident.activate(), // @TODO PrimaryIcon impl
                EntryIconPosition::Secondary => item_action.load.activate(Some(&this.text()), true),
                _ => todo!(), // unexpected
            }
        });

        entry.connect_has_focus_notify(|this| {
            if this.focus_child().is_some_and(|text| text.has_focus()) {
                this.set_secondary_icon_name(Some("pan-end-symbolic"));
            } else {
                this.set_secondary_icon_name(None);
                this.select_region(0, 0);
            }
        });

        entry.connect_changed({
            let profile = profile.clone();
            let item_action = item_action.clone();
            move |this| {
                // Update actions
                item_action.reload.set_enabled(!this.text().is_empty());
                item_action
                    .home
                    .set_enabled(uri(&this.text()).is_some_and(|uri| uri.path().len() > 1));

                // Update primary icon
                this.update(&profile)
            }
        });

        entry.connect_activate({
            let item_action = item_action.clone();
            move |entry| {
                item_action.load.activate(Some(&entry.text()), true);
            }
        });

        entry.connect_state_flags_changed({
            // Define last focus state container
            let has_focus = Cell::new(false);
            move |entry, state| {
                // Select entire text on first click (release)
                // this behavior implemented in most web-browsers,
                // to simply overwrite current request with new value
                // Note:
                // * Custom GestureClick is not an option here, as GTK Entry has default controller
                // * This is experimental feature does not follow native GTK behavior @TODO make optional
                if !has_focus.take()
                    && state.contains(StateFlags::ACTIVE | StateFlags::FOCUS_WITHIN)
                    && entry.selection_bounds().is_none()
                {
                    entry.select_region(0, entry.text_length().into());
                }
                // Update last focus state
                has_focus.replace(state.contains(StateFlags::FOCUS_WITHIN));
            }
        });

        entry
    }

    // Actions
    fn clean(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_tab_item_page_navigation_id) {
            Ok(records) => {
                for record in records {
                    match database::delete(transaction, &record.id) {
                        Ok(_) => {
                            // Delegate clean action to the item childs
                            // nothing yet..
                        }
                        Err(e) => return Err(e.to_string()),
                    }
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    fn restore(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String> {
        match database::select(transaction, app_browser_window_tab_item_page_navigation_id) {
            Ok(records) => {
                for record in records {
                    if let Some(text) = record.text {
                        self.set_text(&text);
                    }

                    // Delegate restore action to the item childs
                    // nothing yet..
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    fn save(
        &self,
        transaction: &Transaction,
        app_browser_window_tab_item_page_navigation_id: &i64,
    ) -> Result<(), String> {
        // Keep value in memory until operation complete
        let text = self.text();

        match database::insert(
            transaction,
            app_browser_window_tab_item_page_navigation_id,
            match text.is_empty() {
                true => None,
                false => Some(text.as_str()),
            },
        ) {
            Ok(_) => {
                // let id = database::last_insert_id(transaction);

                // Delegate save action to childs
                // nothing yet..
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }

    fn update(&self, profile: &Profile) {
        // Update primary icon
        self.first_child().unwrap().remove_css_class("success"); // @TODO handle

        self.set_primary_icon_activatable(false);
        self.set_primary_icon_sensitive(false);

        match primary_icon::from(&self.text()) {
            PrimaryIcon::Download { name, tooltip } => {
                self.set_primary_icon_name(Some(name));
                self.set_primary_icon_tooltip_text(Some(tooltip));
            }
            PrimaryIcon::Gemini { name, tooltip } | PrimaryIcon::Titan { name, tooltip } => {
                self.set_primary_icon_activatable(true);
                self.set_primary_icon_sensitive(true);
                self.set_primary_icon_name(Some(name));
                if profile.identity.get(&strip_prefix(self.text())).is_some() {
                    self.first_child().unwrap().add_css_class("success"); // @TODO handle
                    self.set_primary_icon_tooltip_text(Some(tooltip.1));
                } else {
                    self.set_primary_icon_tooltip_text(Some(tooltip.0));
                }
            }
            PrimaryIcon::Search { name, tooltip } => {
                self.set_primary_icon_name(Some(name));
                self.set_primary_icon_tooltip_text(Some(tooltip));
            }
            PrimaryIcon::Source { name, tooltip } => {
                self.set_primary_icon_name(Some(name));
                self.set_primary_icon_tooltip_text(Some(tooltip));
            }
        }
    }

    // Setters

    fn to_download(&self) {
        self.set_text(&self.download());
    }

    fn to_source(&self) {
        self.set_text(&self.source());
    }

    // Getters

    /// Get current request value without system prefix
    /// * the `prefix` is not `scheme`
    fn strip_prefix(&self) -> GString {
        strip_prefix(self.text())
    }

    /// Get request value with formatted `download` prefix
    fn download(&self) -> GString {
        gformat!("{PREFIX_DOWNLOAD}{}", self.strip_prefix())
    }

    /// Get request value with formatted `source` prefix
    fn source(&self) -> GString {
        gformat!("{PREFIX_SOURCE}{}", self.strip_prefix())
    }

    /// Try get current request value as [Uri](https://docs.gtk.org/glib/struct.Uri.html)
    /// * `strip_prefix` on parse
    fn uri(&self) -> Option<Uri> {
        uri(&strip_prefix(self.text()))
    }

    /// Try build home [Uri](https://docs.gtk.org/glib/struct.Uri.html) for `Self`
    fn home(&self) -> Option<Uri> {
        home(self.uri())
    }
}

// Tools

pub fn migrate(tx: &Transaction) -> Result<(), String> {
    // Migrate self components
    if let Err(e) = database::init(tx) {
        return Err(e.to_string());
    }

    // Delegate migration to childs
    // nothing yet..

    // Success
    Ok(())
}

/// Strip system prefix from request string
/// * the `prefix` is not `scheme`
fn strip_prefix(mut request: GString) -> GString {
    if let Some(postfix) = request.strip_prefix(PREFIX_SOURCE) {
        request = postfix.into()
    };

    if let Some(postfix) = request.strip_prefix(PREFIX_DOWNLOAD) {
        request = postfix.into()
    };

    request
} // @TODO move prefix features to page client

fn uri(value: &str) -> Option<Uri> {
    match Uri::parse(value, UriFlags::NONE) {
        Ok(uri) => Some(uri),
        _ => None,
    }
}

/// Parse home [Uri](https://docs.gtk.org/glib/struct.Uri.html) for `subject`
fn home(subject: Option<Uri>) -> Option<Uri> {
    subject.map(|uri| {
        Uri::build(
            UriFlags::NONE,
            &if uri.scheme() == "titan" {
                GString::from("gemini")
            } else {
                uri.scheme()
            },
            uri.userinfo().as_deref(),
            uri.host().as_deref(),
            uri.port(),
            "/",
            None,
            None,
        )
    })
}
