use gtk::{prelude::ButtonExt, Button};

use crate::app::browser::window::Action;
use std::rc::Rc;

const ICON_YES: &str = "starred-symbolic";
const ICON_NON: &str = "non-starred-symbolic";

pub struct Widget {
    pub gobject: Button,
}

impl Widget {
    // Constructors

    pub fn new(action: Rc<Action>) -> Self {
        // Init gobject
        let gobject = Button::builder()
            .icon_name(ICON_NON)
            .tooltip_text("Bookmark")
            .build();

        // Init events
        gobject.connect_clicked(move |_| action.bookmark.activate());

        // Return activated `Self`
        Self { gobject }
    }

    // Actions

    pub fn update(&self, has_bookmark: bool) {
        self.gobject
            .set_icon_name(if has_bookmark { ICON_YES } else { ICON_NON });
    }
}
