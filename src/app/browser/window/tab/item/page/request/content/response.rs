mod widget;

use widget::Widget;

use gtk::Entry;
use std::sync::Arc;

pub struct Response {
    widget: Arc<Widget>,
}

impl Response {
    pub fn new_arc() -> Arc<Self> {
        // Init widget
        let widget = Widget::new_arc();

        // Result
        Arc::new(Self { widget })
    }

    // Getters
    pub fn gobject(&self) -> &Entry {
        &self.widget.gobject()
    }
}
