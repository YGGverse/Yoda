mod response;
mod send;
mod title;
mod widget;

use response::Response;
use send::Send;
use title::Title;
use widget::Widget;

use gtk::Box;
use std::sync::Arc;

pub struct Content {
    title: Arc<Title>,
    response: Arc<Response>,
    widget: Arc<Widget>,
}

impl Content {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        // Init components
        let title = Title::new_arc();
        let response = Response::new_arc();
        let send = Send::new_arc();

        // Init widget
        let widget = Widget::new_arc(title.gobject(), response.gobject(), send.gobject());

        // Init events
        /* @TODO
        response.gobject().connect_activate(|_| {});
        send.gobject().connect_clicked(|_| {}); */

        // Return activated struct
        Arc::new(Self {
            title,
            response,
            widget,
        })
    }

    // Actions
    pub fn set(&self, title: Option<&str>) {
        self.title.set(title);
        self.response.grab_focus();
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }
}
