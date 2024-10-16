mod content;
mod widget;

use content::Content;
use widget::Widget;

use adw::ToolbarView;
use std::sync::Arc;

pub struct Input {
    content: Arc<Content>,
    widget: Arc<Widget>,
}

impl Input {
    // Construct
    pub fn new_arc() -> Arc<Self> {
        // Init components
        let content = Content::new_arc();

        // Init widget
        let widget = Widget::new_arc(content.gobject());

        // Result
        Arc::new(Self { content, widget })
    }

    // Actions
    pub fn show(&self, placeholder: &str, sensitive: bool) {
        self.content.set(placeholder, sensitive);
    }

    // Getters
    pub fn gobject(&self) -> &ToolbarView {
        &self.widget.gobject()
    }
}
