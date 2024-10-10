mod widget;

use widget::Widget;

use adw::{TabBar, TabView};
use std::sync::Arc;

pub struct Tab {
    widget: Arc<Widget>,
}

impl Tab {
    // Construct
    pub fn new_arc(view: &TabView) -> Arc<Self> {
        Arc::new(Self {
            widget: Widget::new_arc(view),
        })
    }

    // Getters
    pub fn gobject(&self) -> &TabBar {
        &self.widget.gobject()
    }
}
