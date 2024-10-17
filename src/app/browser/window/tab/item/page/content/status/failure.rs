use adw::StatusPage;

pub struct Failure {
    // nothing yet..
}

impl Failure {
    pub fn new(title: &str, description: &str) -> StatusPage {
        StatusPage::builder()
            .description(description)
            .icon_name("dialog-error-symbolic")
            .title(title)
            .build()
    }
}
