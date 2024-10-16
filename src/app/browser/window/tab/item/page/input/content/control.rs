mod limit;
mod send;
mod widget;

use limit::Limit;
use send::Send;
use widget::Widget;

use gtk::Box;
use std::sync::Arc;

pub struct Control {
    limit: Arc<Limit>,
    send: Arc<Send>,
    widget: Arc<Widget>,
}

impl Control {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        // Init components
        let limit = Limit::new_arc();
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
    pub fn update(&self, count: &i32, count_limit: Option<&i32>) {
        self.limit.update(count, count_limit);
        // @TODO self.send.update(limit);
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }
}
