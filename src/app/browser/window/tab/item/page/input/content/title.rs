mod widget;

use widget::Widget;

use gtk::Label;
use std::sync::Arc;

pub struct Title {
    widget: Arc<Widget>,
}

impl Title {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        // Init widget
        let widget = Widget::new_arc();

        // Result
        Arc::new(Self { widget })
    }

    // Actions
    pub fn set(&self, text: Option<&str>) {
        self.widget.set(text);
    }

    // Getters
    pub fn gobject(&self) -> &Label {
        &self.widget.gobject()
    }
}
