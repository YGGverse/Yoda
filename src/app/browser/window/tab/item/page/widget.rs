use gtk::{
    prelude::{BoxExt, IsA},
    Box, Orientation,
};

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new(
        name: &str,
        // Components
        navigation: &impl IsA<gtk::Widget>,
        content: &impl IsA<gtk::Widget>,
        input: &impl IsA<gtk::Widget>,
    ) -> Self {
        // Init self
        let gobject = Box::builder()
            .orientation(Orientation::Vertical)
            .name(name)
            .build();

        gobject.append(navigation);
        gobject.append(content);
        gobject.append(input);

        Self { gobject }
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
