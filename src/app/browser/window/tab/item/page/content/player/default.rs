mod widget;
use widget::Widget;

use gtk::{MediaStream, Video};

pub struct Default {
    widget: Widget,
}

impl Default {
    pub fn new(media_stream: &MediaStream) -> Self {
        Self {
            widget: Widget::new(media_stream),
        }
    }

    pub fn gobject(&self) -> &Video {
        &self.widget.gobject()
    }
}
