use super::{ItemAction, TabAction};
use adw::StatusPage;
use gtk::{prelude::ActionExt, Align, Button};
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
pub fn build((tab_action, item_action): (&Rc<TabAction>, &Rc<ItemAction>)) -> StatusPage {
    // Init certificate selection
    let button = &Button::builder()
        .action_name(format!("{}.{}", tab_action.id, item_action.identity.name()))
        .css_classes([DEFAULT_BUTTON_CLASS])
        .label(DEFAULT_BUTTON_LABEL)
        .tooltip_text(DEFAULT_BUTTON_TOOLTIP_TEXT)
        .halign(Align::Center)
        .build();

    // Init status page
    StatusPage::builder()
        .description(DEFAULT_DESCRIPTION)
        .icon_name(DEFAULT_ICON_NAME)
        .title(DEFAULT_TITLE)
        .child(button)
        .build()
}
