mod subject;
mod tray;
mod widget;

pub struct Header {
    widget: widget::Header,
}

impl Header {
    pub fn new() -> Header {
        Self {
            widget: widget::Header::new(&tray::new(), &subject::new()),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Header {
        &self.widget
    }
}
