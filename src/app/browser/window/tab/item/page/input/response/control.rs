mod counter;
mod send;
mod widget;

use counter::Counter;
use send::Send;
use widget::Widget;

use gtk::gio::SimpleAction;
use std::rc::Rc;

pub struct Control {
    pub counter: Rc<Counter>,
    pub send: Rc<Send>,
    pub widget: Rc<Widget>,
}

impl Control {
    // Construct
    pub fn new(action_send: SimpleAction) -> Self {
        // Init components
        let counter = Rc::new(Counter::new());
        let send = Rc::new(Send::new(action_send));

        // Init widget
        let widget = Rc::new(Widget::new(&counter.widget.label, &send.widget.button));

        // Return activated struct
        Self {
            counter,
            send,
            widget,
        }
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
}
