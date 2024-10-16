mod default;
mod widget;

use default::Default;
use gtk::glib::Uri;
use widget::Widget;

use adw::Clamp;
use std::sync::Arc;

pub struct Input {
    widget: Arc<Widget>,
}

impl Input {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        // Init widget
        let widget = Widget::new_arc();

        // Result
        Arc::new(Self { widget })
    }

    // Actions
    pub fn use_default(&self, base: Uri, title: Option<&str>, size_limit: Option<usize>) {
        self.widget
            .update(Some(&Default::new_arc(base, title, size_limit).gobject()));
    }

    pub fn show(&self) {
        self.widget.show();
    }

    // Getters
    pub fn gobject(&self) -> &Clamp {
        &self.widget.gobject()
    }
}
