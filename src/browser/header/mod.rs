mod subject;
mod tray;
mod widget;

use std::sync::Arc;

pub struct Header {
    widget: widget::Header,
}

impl Header {
    // Construct
    pub fn new() -> Arc<Header> {
        Arc::new(Self {
            widget: widget::Header::new(
                tray::Tray::new().widget().gtk(),
                subject::Subject::new().widget().gtk(),
            ),
        })
    }

    // Getters
    pub fn widget(&self) -> &widget::Header {
        &self.widget
    }
}
