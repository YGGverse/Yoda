mod control;
mod response;
mod title;
mod widget;

use control::Control;
use response::Response;
use title::Title;
use widget::Widget;

use gtk::Box;
use std::sync::Arc;

pub struct Content {
    control: Arc<Control>,
    response: Arc<Response>,
    title: Arc<Title>,
    widget: Arc<Widget>,
}

impl Content {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        // Init components
        let control = Control::new_arc();
        let response = Response::new_arc();
        let title = Title::new_arc();

        // Init widget
        let widget = Widget::new_arc(title.gobject(), response.gobject(), control.gobject());

        // Return activated struct
        Arc::new(Self {
            control,
            response,
            title,
            widget,
        })
    }

    // Actions
    pub fn update(&self, title: Option<&str>, count_limit: Option<&i32>) {
        self.control.update(&0, count_limit); // @TODO
        self.title.update(title);
        self.response.grab_focus();
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.widget.gobject()
    }
}
