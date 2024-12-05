mod response;
mod sensitive;
mod widget;

use response::Response;
use sensitive::Sensitive;
use widget::Widget;

use crate::app::browser::window::tab::item::Action as TabAction;
use gtk::glib::Uri;
use std::rc::Rc;

pub struct Input {
    pub widget: Rc<Widget>,
}

impl Input {
    // Construct
    pub fn new() -> Self {
        // Init widget
        let widget = Rc::new(Widget::new());

        // Result
        Self { widget }
    }

    // Actions
    pub fn unset(&self) {
        self.widget.update(None);
    }

    // Setters
    pub fn set_new_response(
        &self,
        action: Rc<TabAction>,
        base: Uri,
        title: Option<&str>,
        size_limit: Option<usize>,
    ) {
        self.widget.update(Some(
            &Response::new(action, base, title, size_limit).widget.g_box,
        ));
    }

    pub fn set_new_sensitive(
        &self,
        action: Rc<TabAction>,
        base: Uri,
        title: Option<&str>,
        max_length: Option<i32>,
    ) {
        self.widget.update(Some(
            Sensitive::new(action, base, title, max_length).gobject(),
        ));
    }
}
