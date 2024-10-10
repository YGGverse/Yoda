use adw::ToolbarView;
use gtk::Box;
use std::sync::Arc;

pub struct Widget {
    gobject: ToolbarView,
}

impl Widget {
    // Construct
    pub fn new_arc(top_bar: &Box) -> Arc<Self> {
        let gobject = ToolbarView::builder().build();

        gobject.add_top_bar(top_bar);

        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &ToolbarView {
        &self.gobject
    }
}
