use adw::StatusPage;
use gtk::{prelude::ButtonExt, Align, Button};

// Defaults
const DEFAULT_ICON_NAME: &str = "avatar-default-symbolic";
const DEFAULT_TITLE: &str = "Identity";
const DEFAULT_DESCRIPTION: &str = "Client certificate required to continue!";
const DEFAULT_BUTTON_LABEL: &str = "Select";

/// Create new default preset for `Identity`
/// [StatusPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.StatusPage.html)
pub fn new_gobject() -> StatusPage {
    // Init certificate selection
    let button = &Button::builder()
        .label(DEFAULT_BUTTON_LABEL)
        .halign(Align::Center)
        .build();

    // Init events
    button.connect_activate(|_| {}); // @TODO

    // Init status page
    StatusPage::builder()
        .description(DEFAULT_DESCRIPTION)
        .icon_name(DEFAULT_ICON_NAME)
        .title(DEFAULT_TITLE)
        .child(button)
        .build()
}
