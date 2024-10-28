mod widget;
use widget::Widget;

use adw::StatusPage;

pub struct Loading {
    widget: Widget,
}

impl Loading {
    pub fn new(title: Option<&str>, description: Option<&str>) -> Self {
        Self {
            widget: Widget::new(title, description),
        }
    }

    pub fn gobject(&self) -> &StatusPage {
        &self.widget.gobject()
    }
}
