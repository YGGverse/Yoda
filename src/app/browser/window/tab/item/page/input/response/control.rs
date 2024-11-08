mod counter;
mod send;
mod widget;

use counter::Counter;
use send::Send;
use widget::Widget;

use gtk::{gio::SimpleAction, Box};
use std::sync::Arc;

pub struct Control {
    counter: Arc<Counter>,
    send: Arc<Send>,
    widget: Arc<Widget>,
}

impl Control {
    // Construct
    pub fn new_arc(action_send: SimpleAction) -> Arc<Self> {
        // Init components
        let counter = Counter::new_arc();
        let send = Send::new_arc(action_send);

        // Init widget
        let widget = Widget::new_arc(counter.gobject(), send.gobject());

        // Return activated struct
        Arc::new(Self {
            counter,
            send,
            widget,
        })
    }

    // Actions
    pub fn update(&self, chars_left: Option<i32>) {
        // Update children components
        self.counter.update(chars_left);
        self.send.update(match chars_left {
            Some(left) => left > 0,
            None => false,
        });
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        self.widget.gobject()
    }
}
