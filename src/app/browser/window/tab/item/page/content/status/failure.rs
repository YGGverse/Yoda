use adw::StatusPage;

const DEFAULT_TITLE: &str = "Oops";
const DEFAULT_DESCRIPTION: Option<&str> = None;
const DEFAULT_ICON_NAME: Option<&str> = Some("dialog-error");

pub struct Failure {
    gobject: StatusPage,
}

impl Failure {
    pub fn new(title: Option<&str>, description: Option<&str>, icon_name: Option<&str>) -> Self {
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

        Self { gobject }
    }

    pub fn gobject(&self) -> &StatusPage {
        &self.gobject
    }
}
