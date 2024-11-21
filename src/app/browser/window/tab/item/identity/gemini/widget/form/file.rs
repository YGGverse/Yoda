use super::Action;
use gtk::{
    prelude::{ButtonExt, WidgetExt},
    Button,
};

use std::rc::Rc;

const LABEL: &str = "Select file..";
const MARGIN: i32 = 8;

pub struct File {
    pub gobject: Button,
}

impl File {
    // Constructors

    /// Create new `Self`
    pub fn new(action: Rc<Action>) -> Self {
        // Init `GObject`
        let gobject = Button::builder()
            .label(LABEL)
            .margin_top(MARGIN)
            .visible(false)
            .build();

        // Init events
        gobject.connect_clicked(move |_| todo!());

        // Return activated `Self`
        Self { gobject }
    }

    // Actions

    /// Change visibility status
    /// * grab focus on `is_visible`
    pub fn show(&self, is_visible: bool) {
        self.gobject.set_visible(is_visible);
        if is_visible {
            self.gobject.grab_focus();
        }
    }
}
