mod response;
mod sensitive;
mod widget;

use gtk::{gio::SimpleAction, glib::Uri};
use response::Response;
use sensitive::Sensitive;
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
    pub fn unset(&self) {
        self.widget.update(None);
    }

    // Setters
    pub fn set_new_response(
        &self,
        action_page_open: SimpleAction,
        base: Uri,
        title: Option<&str>,
        size_limit: Option<usize>,
    ) {
        self.widget.update(Some(
            &Response::new_arc(action_page_open, base, title, size_limit).gobject(),
        ));
    }

    pub fn set_new_sensitive(
        &self,
        action_page_open: SimpleAction,
        base: Uri,
        title: Option<&str>,
        max_length: Option<i32>,
    ) {
        self.widget.update(Some(
            &Sensitive::new_arc(action_page_open, base, title, max_length).gobject(),
        ));
    }

    // Getters
    pub fn gobject(&self) -> &Clamp {
        self.widget.gobject()
    }
}
