mod widget;
use widget::Widget;

use adw::StatusPage;

pub struct Failure {
    widget: Widget,
}

impl Failure {
    pub fn new(title: Option<&str>, description: Option<&str>) -> Self {
        Self {
            widget: Widget::new(title, description),
        }
    }

    pub fn gobject(&self) -> &StatusPage {
        &self.widget.gobject()
    }
}
