use adw::ToolbarView;
use gtk::Box;

pub struct Widget {
    gobject: ToolbarView,
}

impl Widget {
    // Construct
    pub fn new(top_bar: &Box) -> Self {
        let gobject = ToolbarView::builder().build();

        gobject.add_top_bar(top_bar);

        Self { gobject }
    }

    // Getters
    pub fn gobject(&self) -> &ToolbarView {
        &self.gobject
    }
}
