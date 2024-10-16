mod widget;

use widget::Widget;

use gtk::Label;
use std::sync::Arc;

pub struct Left {
    widget: Arc<Widget>,
}

impl Left {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        // Init widget
        let widget = Widget::new_arc();

        // Result
        Arc::new(Self { widget })
    }

    // Actions
    pub fn update(&self, left: Option<usize>) {
        self.widget.update(left);
    }

    // Getters
    pub fn gobject(&self) -> &Label {
        &self.widget.gobject()
    }
}
