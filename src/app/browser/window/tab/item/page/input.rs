mod response;
mod widget;

use gtk::{gio::SimpleAction, glib::Uri};
use response::Response;
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
    pub fn show(&self) {
        self.widget.show()
    }

    pub fn hide(&self) {
        self.widget.hide()
    }

    pub fn use_response(
        &self,
        action_page_open: Arc<SimpleAction>,
        base: Uri,
        title: Option<&str>,
        size_limit: Option<usize>,
    ) {
        self.widget.set_child(Some(
            &Response::new_arc(action_page_open, base, title, size_limit).gobject(),
        ));
    }

    // Getters
    pub fn gobject(&self) -> &Clamp {
        &self.widget.gobject()
    }
}
