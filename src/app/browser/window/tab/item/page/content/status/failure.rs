use adw::StatusPage;

const DEFAULT_TITLE: &str = "Oops";
const DEFAULT_DESCRIPTION: Option<&str> = None;
const DEFAULT_ICON_NAME: Option<&str> = Some("dialog-error");

/// Create new `GObject` preset for failure [StatusPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.StatusPage.html)
pub fn new_gobject_from(
    title: Option<&str>,
    description: Option<&str>,
    icon_name: Option<&str>,
) -> StatusPage {
    let gobject = StatusPage::new();

    gobject.set_title(match title {
        Some(value) => value,
        None => DEFAULT_TITLE,
    });

    gobject.set_description(match description {
        Some(value) => Some(value),
        None => DEFAULT_DESCRIPTION,
    });

    gobject.set_icon_name(match icon_name {
        Some(value) => Some(value),
        None => DEFAULT_ICON_NAME,
    });

    gobject
}
