mod control;

use super::Header;
use gtk::Box;

pub trait File {
    fn file() -> Self;
}

impl File for Box {
    fn file() -> Self {
        use control::Control;
        use std::{cell::Cell, rc::Rc};

        // Init components
        let header = Rc::new(Cell::new(Header {
            mime: None,
            token: None,
        }));
        let control = Box::control(&header);
        let form = {
            const MARGIN: i32 = 8;
            gtk::Button::builder()
                .label("Choose a file..")
                .margin_bottom(MARGIN)
                .margin_top(MARGIN)
                .build()
        };

        // Init main widget
        {
            use gtk::{prelude::BoxExt, Orientation};

            const MARGIN: i32 = 8;

            let g_box = Box::builder()
                .margin_bottom(MARGIN)
                .margin_end(MARGIN)
                .margin_start(MARGIN)
                .orientation(Orientation::Vertical)
                .spacing(MARGIN)
                .build();

            g_box.append(&form);
            g_box.append(&control);
            g_box
        }
    }
}
