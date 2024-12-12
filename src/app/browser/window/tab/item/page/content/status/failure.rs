use adw::StatusPage;

const DEFAULT_TITLE: &str = "Oops";
const DEFAULT_ICON_NAME: &str = "dialog-error";

/// Create new default `GObject` preset for failure
/// [StatusPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.StatusPage.html)
pub fn new() -> StatusPage {
    StatusPage::builder()
        .title(DEFAULT_TITLE)
        .icon_name(DEFAULT_ICON_NAME)
        .build()
}
