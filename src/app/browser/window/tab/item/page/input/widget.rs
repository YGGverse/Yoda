use adw::ToolbarView;
use gtk::{prelude::WidgetExt, Box};
use std::sync::Arc;

pub struct Widget {
    gobject: ToolbarView,
}

impl Widget {
    // Construct
    pub fn new_arc(content: &Box) -> Arc<Self> {
        let gobject = ToolbarView::builder()
            .content(content)
            .visible(false)
            .build();

        Arc::new(Self { gobject })
    }

    // Actions
    pub fn show(&self, visible: bool) {
        self.gobject.set_visible(visible);
    }

    // Getters
    pub fn gobject(&self) -> &ToolbarView {
        &self.gobject
    }
}
