mod response;
mod send;
mod widget;

use response::Response;
use send::Send;
use widget::Widget;

use gtk::Box;
use std::sync::Arc;

pub struct Content {
    widget: Arc<Widget>,
}

impl Content {
    pub fn new_arc() -> Arc<Self> {
        // Init components
        let response = Response::new_arc();
        let send = Send::new_arc();

        // Init widget
        let widget = Widget::new_arc(response.gobject(), send.gobject());

        // Init events
        /* @TODO
        response.gobject().connect_activate(|_| {});
        send.gobject().connect_clicked(|_| {}); */

        // Return activated struct
        Arc::new(Self { widget })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }
}
