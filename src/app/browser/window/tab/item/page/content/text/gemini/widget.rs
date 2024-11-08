use adw::ClampScrollable;
use gtk::TextView;
use std::rc::Rc;

pub struct Widget {
    gobject: ClampScrollable,
}

impl Widget {
    // Construct
    pub fn new_rc(child: &TextView) -> Rc<Self> {
        Rc::new(Self {
            gobject: ClampScrollable::builder()
                .child(child)
                .css_classes(["view"])
                .maximum_size(800)
                .build(),
        })
    }

    // Getters
    pub fn gobject(&self) -> &ClampScrollable {
        &self.gobject
    }
}
