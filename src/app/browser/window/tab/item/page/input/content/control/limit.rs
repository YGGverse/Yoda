mod widget;

use widget::Widget;

use gtk::Label;
use std::sync::Arc;

pub struct Limit {
    widget: Arc<Widget>,
}

impl Limit {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        // Init widget
        let widget = Widget::new_arc();

        // Result
        Arc::new(Self { widget })
    }

    // Actions
    pub fn update(&self, count: &i32, limit: Option<&i32>) {
        self.widget.update(count, limit);
    }

    // Getters
    pub fn gobject(&self) -> &Label {
        &self.widget.gobject()
    }
}
