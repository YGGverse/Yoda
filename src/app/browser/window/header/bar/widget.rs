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
    pub fn build(
        control: &impl IsA<gtk::Widget>,
        menu: &impl IsA<gtk::Widget>,
        tab: &impl IsA<gtk::Widget>,
    ) -> Self {
        let g_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .build();

        g_box.append(tab);
        g_box.append(menu);
        g_box.append(control);

        Self { g_box }
    }
}
