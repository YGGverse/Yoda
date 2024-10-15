use adw::StatusPage;

pub struct Error {
    gobject: StatusPage,
}

impl Error {
    pub fn new(title: &str, description: &str) -> StatusPage {
        StatusPage::builder()
            .description(description)
            .icon_name("dialog-error-symbolic")
            .title(title)
            .build()
    }
}
