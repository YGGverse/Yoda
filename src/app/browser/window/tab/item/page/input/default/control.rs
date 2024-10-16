mod left;
mod send;
mod widget;

use left::Left;
use send::Send;
use widget::Widget;

use gtk::Box;
use std::sync::Arc;

pub struct Control {
    left: Arc<Left>,
    send: Arc<Send>,
    widget: Arc<Widget>,
}

impl Control {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        // Init components
        let left = Left::new_arc();
        let send = Send::new_arc();

        // Init widget
        let widget = Widget::new_arc(left.gobject(), send.gobject());

        // Return activated struct
        Arc::new(Self { left, send, widget })
    }

    // Actions
    pub fn update(&self, chars_left: Option<i32>) {
        // Update children components
        self.left.update(chars_left);
        self.send.update(match chars_left {
            Some(value) => value > 0,
            None => false,
        });
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }
}
