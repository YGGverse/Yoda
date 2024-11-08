mod append;
mod widget;

use append::Append;
use widget::Widget;

use adw::{TabBar, TabView};
use gtk::gio::SimpleAction;
use std::rc::Rc;

pub struct Tab {
    widget: Rc<Widget>,
}

impl Tab {
    // Construct
    pub fn new_rc(action_page_new: SimpleAction, view: &TabView) -> Rc<Self> {
        Rc::new(Self {
            widget: Widget::new_rc(view, Append::new_rc(action_page_new).gobject()),
        })
    }

    // Getters
    pub fn gobject(&self) -> &TabBar {
        self.widget.gobject()
    }
}
