use gtk::ScrolledWindow;

pub struct Text {
    widget: ScrolledWindow,
}

impl Text {
    // Construct
    pub fn new() -> Self {
        Self {
            widget: ScrolledWindow::builder().build(),
        }
    }

    // Getters
    pub fn widget(&self) -> &ScrolledWindow {
        &self.widget
    }
}
