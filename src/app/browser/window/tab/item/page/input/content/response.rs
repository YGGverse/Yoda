mod widget;

use widget::Widget;

use gtk::TextView;
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
    pub fn grab_focus(&self) {
        self.widget.grab_focus();
    }

    // Getters
    pub fn gobject(&self) -> &TextView {
        &self.widget.gobject()
    }
}
