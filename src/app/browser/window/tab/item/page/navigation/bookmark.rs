mod widget;

use widget::Widget;

use gtk::Button;
use std::sync::Arc;

pub struct Bookmark {
    widget: Arc<Widget>,
}

impl Bookmark {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self {
            widget: Widget::new_arc(),
        })
    }

    // Actions
    pub fn update(&self) {
        // @TODO
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        self.widget.gobject()
    }
}
