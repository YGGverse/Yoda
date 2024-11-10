mod response;
mod sensitive;
mod widget;

use response::Response;
use sensitive::Sensitive;
use widget::Widget;

use crate::app::browser::window::tab::action::Action as TabAction;
use adw::Clamp;
use gtk::glib::Uri;
use std::rc::Rc;

pub struct Input {
    widget: Rc<Widget>,
}

impl Input {
    // Construct
    pub fn new_rc() -> Rc<Self> {
        // Init widget
        let widget = Widget::new_rc();

        // Result
        Rc::new(Self { widget })
    }

    // Actions
    pub fn unset(&self) {
        self.widget.update(None);
    }

    // Setters
    pub fn set_new_response(
        &self,
        tab_action: Rc<TabAction>,
        base: Uri,
        title: Option<&str>,
        size_limit: Option<usize>,
    ) {
        self.widget.update(Some(
            Response::new_rc(tab_action, base, title, size_limit).gobject(),
        ));
    }

    pub fn set_new_sensitive(
        &self,
        tab_action: Rc<TabAction>,
        base: Uri,
        title: Option<&str>,
        max_length: Option<i32>,
    ) {
        self.widget.update(Some(
            Sensitive::new_rc(tab_action, base, title, max_length).gobject(),
        ));
    }

    // Getters
    pub fn gobject(&self) -> &Clamp {
        self.widget.gobject()
    }
}
