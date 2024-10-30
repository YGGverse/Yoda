mod widget;
use widget::Widget;

use adw::StatusPage;
use std::time::Duration;

pub struct Loading {
    widget: Widget,
}

impl Loading {
    pub fn new(
        title: Option<&str>,
        description: Option<&str>,
        show_with_delay: Option<Duration>,
    ) -> Self {
        Self {
            widget: Widget::new(title, description, show_with_delay),
        }
    }

    pub fn gobject(&self) -> &StatusPage {
        &self.widget.gobject()
    }
}
