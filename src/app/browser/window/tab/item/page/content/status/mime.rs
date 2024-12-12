use adw::StatusPage;

const DEFAULT_TITLE: &str = "Oops";
const DEFAULT_ICON_NAME: &str = "help-contents-symbolic";

/// Create new default `GObject` preset for mime issue
/// [StatusPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.StatusPage.html)
pub fn new_gobject() -> StatusPage {
    StatusPage::builder()
        .title(DEFAULT_TITLE)
        .icon_name(DEFAULT_ICON_NAME)
        .build()
}
