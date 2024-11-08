mod widget;

use widget::Widget;

use gtk::WindowControls;
use std::sync::Arc;

pub struct Control {
    widget: Arc<Widget>,
}

impl Control {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self {
            widget: Widget::new_arc(),
        })
    }

    // Getters
    pub fn gobject(&self) -> &WindowControls {
        self.widget.gobject()
    }
}
