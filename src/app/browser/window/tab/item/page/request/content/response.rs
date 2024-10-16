mod widget;

use widget::Widget;

use gtk::Entry;
use std::sync::Arc;

pub struct Response {
    widget: Arc<Widget>,
}

impl Response {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        // Init widget
        let widget = Widget::new_arc();

        // Result
        Arc::new(Self { widget })
    }

    // Actions
    pub fn set(&self, placeholder: &str, sensitive: bool) {
        self.widget.set(placeholder, sensitive);
    }

    // Getters
    pub fn gobject(&self) -> &Entry {
        &self.widget.gobject()
    }
}
