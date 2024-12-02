use adw::ClampScrollable;
use gtk::prelude::IsA;

pub struct Widget {
    pub clamp_scrollable: ClampScrollable,
}

impl Widget {
    // Construct
    pub fn new(child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            clamp_scrollable: ClampScrollable::builder()
                .child(child)
                .css_classes(["view"])
                .maximum_size(800)
                .build(),
        }
    }
}
