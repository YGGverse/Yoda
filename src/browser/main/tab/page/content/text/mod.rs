use gtk::{Orientation, ScrolledWindow};

pub struct Text {
    widget: ScrolledWindow,
}

impl Text {
    // Construct
    pub fn new() -> Text {
        Self {
            widget: ScrolledWindow::builder().build(),
        }
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
