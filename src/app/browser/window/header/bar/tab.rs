mod append;
mod widget;

use append::Append;
use widget::Widget;

use adw::{TabBar, TabView};
use gtk::gio::SimpleAction;
use std::sync::Arc;

pub struct Tab {
    widget: Arc<Widget>,
}

impl Tab {
    // Construct
    pub fn new_arc(action_page_new: SimpleAction, view: &TabView) -> Arc<Self> {
        Arc::new(Self {
            widget: Widget::new_arc(view, Append::new_arc(action_page_new).gobject()),
        })
    }

    // Getters
    pub fn gobject(&self) -> &TabBar {
        &self.widget.gobject()
    }
}
