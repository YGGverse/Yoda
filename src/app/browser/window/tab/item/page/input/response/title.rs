mod widget;

use widget::Widget;

use gtk::Label;
use std::sync::Arc;

pub struct Title {
    widget: Arc<Widget>,
}

impl Title {
    // Construct
    pub fn new_arc(title: Option<&str>) -> Arc<Self> {
        // Init widget
        let widget = Widget::new_arc(title);

        // Result
        Arc::new(Self { widget })
    }

    // Getters
    pub fn gobject(&self) -> &Label {
        &self.widget.gobject()
    }
}
