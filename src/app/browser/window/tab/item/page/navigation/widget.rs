use gtk::{
    prelude::{BoxExt, IsA},
    Box, Orientation,
};

const MARGIN: i32 = 6;
const SPACING: i32 = 6;

pub struct Widget {
    pub gobject: Box,
}

impl Widget {
    // Construct
    pub fn new(
        auth: &impl IsA<gtk::Widget>,
        base: &impl IsA<gtk::Widget>,
        history: &impl IsA<gtk::Widget>,
        reload: &impl IsA<gtk::Widget>,
        request: &impl IsA<gtk::Widget>,
        bookmark: &impl IsA<gtk::Widget>,
    ) -> Self {
        let gobject = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(SPACING)
            .margin_start(MARGIN)
            .margin_end(MARGIN)
            .margin_bottom(MARGIN)
            .build();

        gobject.append(base);
        gobject.append(history);
        gobject.append(reload);
        gobject.append(request);
        gobject.append(bookmark);
        gobject.append(auth);

        Self { gobject }
    }
}
