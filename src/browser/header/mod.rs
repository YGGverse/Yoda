mod subject;
mod tray;
mod widget;

pub struct Header {
    widget: widget::Header,
}

impl Header {
    pub fn new() -> Header {
        let subject = subject::Subject::new();
        let tray = tray::new();
        Self {
            widget: widget::Header::new(&tray, subject.widget().gtk()), // @TODO
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Header {
        &self.widget
    }
}
