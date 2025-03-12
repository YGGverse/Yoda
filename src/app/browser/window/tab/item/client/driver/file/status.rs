pub enum Status {
    Failure(String),
}

impl Status {
    pub fn handle(&self, page: &super::Page) {
        let (message, widget) = match self {
            Self::Failure(message) => (message, page.content.to_status_failure()),
        };
        widget.set_description(Some(message));
        page.set_title(&widget.title());
        page.set_progress(0.0);
        page.snap_history();
        page.window_action.find.simple_action.set_enabled(false);
    }
}
