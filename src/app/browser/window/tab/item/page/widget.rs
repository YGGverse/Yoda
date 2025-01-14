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
        name: &str,
        // Components
        navigation: &impl IsA<gtk::Widget>,
        content: &impl IsA<gtk::Widget>,
        search: &impl IsA<gtk::Widget>,
        input: &impl IsA<gtk::Widget>,
    ) -> Self {
        // Init self
        let g_box = Box::builder()
            .orientation(Orientation::Vertical)
            .name(name)
            .build();

        g_box.append(navigation);
        g_box.append(content);
        g_box.append(search);
        g_box.append(input);

        Self { g_box }
    }
}
