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
    // Constructors

    /// Build new `Self`
    pub fn build(action_send: SimpleAction) -> Self {
        // Init components
        let counter = Rc::new(Counter::new());
        let send = Rc::new(Send::build(action_send));

        // Init widget
        let widget = Rc::new(Widget::build(&counter.widget.label, &send.widget.button));

        // Return activated struct
        Self {
            counter,
            send,
            widget,
        }
    }

    // Actions
    pub fn update(&self, is_empty: bool, bytes_left: Option<usize>) {
        // Update children components
        self.counter.update(bytes_left);
        self.send.update(match bytes_left {
            Some(left) => !is_empty && left > 0,
            None => false,
        });
    }
}
