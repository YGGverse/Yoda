use adw::ToolbarView;
use gtk::Box;
use std::rc::Rc;

pub struct Widget {
    gobject: ToolbarView,
}

impl Widget {
    // Construct
    pub fn new_rc(top_bar: &Box) -> Rc<Self> {
        let gobject = ToolbarView::builder().build();

        gobject.add_top_bar(top_bar);

        Rc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &ToolbarView {
        &self.gobject
    }
}
