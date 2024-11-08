mod widget;

use widget::Widget;

use gtk::{gio::SimpleAction, glib::GString, TextView};
use std::sync::Arc;

pub struct Form {
    widget: Arc<Widget>,
}

impl Form {
    // Construct
    pub fn new_arc(action_update: SimpleAction) -> Arc<Self> {
        // Init widget
        let widget = Widget::new_arc(action_update);

        // Result
        Arc::new(Self { widget })
    }

    // Actions
    pub fn focus(&self) {
        self.widget.focus();
    }

    // Getters
    pub fn text(&self) -> GString {
        self.widget.text()
    }

    pub fn gobject(&self) -> &TextView {
        self.widget.gobject()
    }
}
