mod left;
mod send;
mod widget;

use left::Left;
use send::Send;
use widget::Widget;

use gtk::Box;
use std::sync::Arc;

pub struct Control {
    limit: Arc<Left>,
    send: Arc<Send>,
    widget: Arc<Widget>,
}

impl Control {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        // Init components
        let limit = Left::new_arc();
        let send = Send::new_arc();

        // Init widget
        let widget = Widget::new_arc(limit.gobject(), send.gobject());

        // Return activated struct
        Arc::new(Self {
            limit,
            send,
            widget,
        })
    }

    // Actions
    pub fn update(&self, left: Option<usize>) {
        // Update children components
        self.limit.update(left);
        self.send.update(match left {
            Some(value) => value > 0,
            None => false,
        });
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }
}
