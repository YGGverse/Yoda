use gtk::{
    prelude::{BoxExt, IsA},
    Box, Orientation,
};
use std::sync::Arc;

const MARGIN: i32 = 6;
const SPACING: i32 = 6;

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new_arc(
        base: &impl IsA<gtk::Widget>,
        history: &impl IsA<gtk::Widget>,
        reload: &impl IsA<gtk::Widget>,
        request: &impl IsA<gtk::Widget>,
        bookmark: &impl IsA<gtk::Widget>,
    ) -> Arc<Self> {
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

        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
