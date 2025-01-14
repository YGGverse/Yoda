use gtk::{
    prelude::{BoxExt, IsA},
    Box, Orientation,
};

pub struct Widget {
    pub g_box: Box,
}

impl Widget {
    // Constructors

    /// Build new `Self`
    pub fn build(back: &impl IsA<gtk::Widget>, forward: &impl IsA<gtk::Widget>) -> Self {
        let g_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .css_classes([
                "linked", // merge childs
            ])
            .build();

        g_box.append(back);
        g_box.append(forward);

        Self { g_box }
    }
}
