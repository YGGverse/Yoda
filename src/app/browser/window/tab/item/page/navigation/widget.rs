use gtk::{
    prelude::{BoxExt, IsA},
    Box, Orientation,
};

const MARGIN: i32 = 6;
const SPACING: i32 = 6;

pub struct Widget {
    pub g_box: Box,
}

impl Widget {
    // Constructors

    /// Build new `Self`
    pub fn build(
        base: &impl IsA<gtk::Widget>,
        history: &impl IsA<gtk::Widget>,
        reload: &impl IsA<gtk::Widget>,
        request: &impl IsA<gtk::Widget>,
        bookmark: &impl IsA<gtk::Widget>,
    ) -> Self {
        let g_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(SPACING)
            .margin_start(MARGIN)
            .margin_end(MARGIN)
            .margin_bottom(MARGIN)
            .build();

        g_box.append(base);
        g_box.append(history);
        g_box.append(reload);
        g_box.append(request);
        g_box.append(bookmark);

        Self { g_box }
    }
}
