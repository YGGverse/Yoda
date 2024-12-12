use adw::StatusPage;

/// Create new default `GObject` preset for mime issue
/// [StatusPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.StatusPage.html)
pub fn new_gobject(mime: &str) -> StatusPage {
    StatusPage::builder()
        .title("Oops")
        .description(format!("Content type `{mime}` not supported!"))
        .icon_name("dialog-question-symbolic")
        .build()
}
