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
        let widget = Rc::new(Widget::build(&counter.label, &send.button));

        // Return activated struct
        Self {
            counter,
            send,
            widget,
        }
    }

    // Actions
    pub fn update(&self, bytes_total: Option<usize>) {
        // Update children components
        self.counter.update(bytes_total);
        self.send.update(match bytes_total {
            Some(total) => total > 0,
            None => false,
        });
    }
}
