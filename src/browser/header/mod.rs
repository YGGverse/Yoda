mod subject;
mod tray;
mod widget;

pub struct Header {
    widget: widget::Header,
}

impl Header {
    pub fn new() -> Header {
        Self {
            widget: widget::Header::new(
                tray::Tray::new().widget().gtk(),
                subject::Subject::new().widget().gtk(),
            ),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Header {
        &self.widget
    }
}
