mod response;
mod send;
mod widget;

use response::Response;
use send::Send;
use widget::Widget;

use gtk::Box;
use std::sync::Arc;

pub struct Content {
    response: Arc<Response>,
    widget: Arc<Widget>,
}

impl Content {
    // Construct
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
        Arc::new(Self { response, widget })
    }

    // Actions
    pub fn set(&self, placeholder: &str, sensitive: bool) {
        self.response.set(placeholder, sensitive);
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }
}
