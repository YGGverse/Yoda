mod widget;

use widget::Widget;

use gtk::Button;
use std::sync::Arc;

pub struct Send {
    widget: Arc<Widget>,
}

impl Send {
    pub fn new_arc() -> Arc<Self> {
        // Init widget
        let widget = Widget::new_arc();

        // Result
        Arc::new(Self { widget })
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        &self.widget.gobject()
    }
}
