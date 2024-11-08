mod counter;
mod send;
mod widget;

use counter::Counter;
use send::Send;
use widget::Widget;

use gtk::{gio::SimpleAction, Box};
use std::rc::Rc;

pub struct Control {
    counter: Rc<Counter>,
    send: Rc<Send>,
    widget: Rc<Widget>,
}

impl Control {
    // Construct
    pub fn new_rc(action_send: SimpleAction) -> Rc<Self> {
        // Init components
        let counter = Counter::new_rc();
        let send = Send::new_rc(action_send);

        // Init widget
        let widget = Widget::new_rc(counter.gobject(), send.gobject());

        // Return activated struct
        Rc::new(Self {
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
