mod content;
mod widget;

use content::Content;
use widget::Widget;

use adw::Clamp;
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
    pub fn show(&self, title: Option<&str>) {
        self.content.set(title);
        self.widget.show(true);
    }

    // Getters
    pub fn gobject(&self) -> &Clamp {
        &self.widget.gobject()
    }
}
