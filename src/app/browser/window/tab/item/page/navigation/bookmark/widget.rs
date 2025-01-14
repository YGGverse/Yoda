use gtk::{prelude::ButtonExt, Button};

use super::WindowAction;
use std::rc::Rc;

const ICON_YES: &str = "starred-symbolic";
const ICON_NON: &str = "non-starred-symbolic";

pub struct Widget {
    pub button: Button,
}

impl Widget {
    // Constructors

    /// Build new `Self`
    pub fn build(action: &Rc<WindowAction>) -> Self {
        // Init gobject
        let button = Button::builder()
            .icon_name(ICON_NON)
            .tooltip_text("Bookmark")
            .build();

        // Init events
        button.connect_clicked({
            let action = action.clone();
            move |_| action.bookmark.activate()
        });

        // Return activated `Self`
        Self { button }
    }

    // Actions

    pub fn update(&self, has_bookmark: bool) {
        self.button
            .set_icon_name(if has_bookmark { ICON_YES } else { ICON_NON });
    }
}
