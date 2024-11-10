use adw::ClampScrollable;
use gtk::TextView;

pub struct Widget {
    gobject: ClampScrollable,
}

impl Widget {
    // Construct
    pub fn new(child: &TextView) -> Self {
        Self {
            gobject: ClampScrollable::builder()
                .child(child)
                .css_classes(["view"])
                .maximum_size(800)
                .build(),
        }
    }

    // Getters
    pub fn gobject(&self) -> &ClampScrollable {
        &self.gobject
    }
}
