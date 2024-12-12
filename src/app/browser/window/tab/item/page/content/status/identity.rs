use crate::app::browser::window::tab::item::Action;
use adw::StatusPage;
use gtk::{prelude::ButtonExt, Align, Button};
use std::rc::Rc;

// Defaults
const DEFAULT_ICON_NAME: &str = "avatar-default-symbolic";
const DEFAULT_TITLE: &str = "Identity";
const DEFAULT_DESCRIPTION: &str = "Client certificate required to continue!";
const DEFAULT_BUTTON_LABEL: &str = "Select";
const DEFAULT_BUTTON_TOOLTIP_TEXT: &str = "Select certificate";
const DEFAULT_BUTTON_CLASS: &str = "suggested-action";

/// Create new default preset for `Identity`
/// [StatusPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.StatusPage.html)
pub fn new(action: Rc<Action>) -> StatusPage {
    // Init certificate selection
    let button = &Button::builder()
        .css_classes([DEFAULT_BUTTON_CLASS])
        .label(DEFAULT_BUTTON_LABEL)
        .tooltip_text(DEFAULT_BUTTON_TOOLTIP_TEXT)
        .halign(Align::Center)
        .build();

    // Init events
    button.connect_clicked(move |_| action.ident.activate());

    // Init status page
    StatusPage::builder()
        .description(DEFAULT_DESCRIPTION)
        .icon_name(DEFAULT_ICON_NAME)
        .title(DEFAULT_TITLE)
        .child(button)
        .build()
}
