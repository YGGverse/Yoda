use adw::ToolbarView;
use gtk::Box;
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

    // Getters
    pub fn gobject(&self) -> &ToolbarView {
        &self.gobject
    }
}
