use gtk::{prelude::BoxExt, Align, Label, Orientation};

const MARGIN: i32 = 8;
const SPACING: i32 = 8;

pub trait File {
    fn file() -> Self;
}

impl File for gtk::Box {
    fn file() -> Self {
        // Init widget
        let g_box = gtk::Box::builder()
            .halign(Align::Center)
            .margin_bottom(MARGIN)
            .margin_end(MARGIN)
            .margin_start(MARGIN)
            .orientation(Orientation::Vertical)
            .spacing(SPACING)
            //.margin_top(MARGIN)
            .build();

        g_box.append(
            &Label::builder()
                .css_classes(["dim-label"])
                .label("Soon..")
                .build(),
        ); // @TODO
        g_box
    }
}
