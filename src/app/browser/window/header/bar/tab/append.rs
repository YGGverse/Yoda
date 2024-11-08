mod widget;

use widget::Widget;

use gtk::{gio::SimpleAction, Button};
use std::sync::Arc;

pub struct Append {
    pub widget: Arc<Widget>,
}

impl Append {
    // Construct
    pub fn new_arc(action_page_new: SimpleAction) -> Arc<Self> {
        Arc::new(Self {
            widget: Widget::new_arc(action_page_new),
        })
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        self.widget.gobject()
    }
}
