mod content;
mod widget;

use content::Content;
use widget::Widget;

use adw::ToolbarView;
use std::sync::Arc;

pub struct Request {
    widget: Arc<Widget>,
}

impl Request {
    pub fn new_arc() -> Arc<Self> {
        // Init components
        let content = Content::new_arc();

        // Init widget
        let widget = Widget::new_arc(content.gobject());

        // Result
        Arc::new(Self { widget })
    }

    // Getters
    pub fn gobject(&self) -> &ToolbarView {
        &self.widget.gobject()
    }
}
