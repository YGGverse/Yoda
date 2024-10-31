use adw::StatusPage;

const DEFAULT_TITLE: &str = "Oops";
const DEFAULT_DESCRIPTION: Option<&str> = None;
const DEFAULT_ICON_NAME: &str = "dialog-error";

pub struct Widget {
    gobject: StatusPage,
}

impl Widget {
    // Constructors

    /// Create new default widget configuration with options
    pub fn new(title: Option<&str>, description: Option<&str>) -> Self {
        let gobject = StatusPage::new();

        gobject.set_title(match title {
            Some(value) => value,
            None => DEFAULT_TITLE,
        });

        gobject.set_description(match description {
            Some(value) => Some(value),
            None => DEFAULT_DESCRIPTION,
        });

        gobject.set_icon_name(Some(DEFAULT_ICON_NAME));

        Self { gobject }
    }

    // Getters

    pub fn gobject(&self) -> &StatusPage {
        &self.gobject
    }
}
