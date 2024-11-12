use gtk::{
    prelude::{ButtonExt, WidgetExt},
    Button,
};

use crate::app::browser::window::action::Action as WindowAction;
use std::rc::Rc;

pub struct Widget {
    gobject: Button,
}

impl Widget {
    // Constructors

    pub fn new(window_action: Rc<WindowAction>) -> Self {
        // Init gobject
        let gobject = Button::builder()
            .icon_name("starred-symbolic")
            .tooltip_text("Bookmark")
            .sensitive(false)
            .build();

        // Init events
        gobject.connect_clicked(move |_| window_action.home().activate());

        // Return activated `Self`
        Self { gobject }
    }

    // Actions

    pub fn update(&self, is_sensitive: bool) {
        self.gobject.set_sensitive(is_sensitive);
    }

    // Getters

    pub fn gobject(&self) -> &Button {
        &self.gobject
    }
}
