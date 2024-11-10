use crate::app::browser::window::action::Action as WindowAction;
use gtk::{
    prelude::{ButtonExt, WidgetExt},
    Button,
};
use std::rc::Rc;

pub struct Widget {
    gobject: Button,
}

impl Widget {
    // Construct
    pub fn new(window_action: Rc<WindowAction>) -> Self {
        // Init gobject
        let gobject = Button::builder()
            .icon_name("go-previous-symbolic")
            .tooltip_text("Back")
            .sensitive(false)
            .build();

        // Init events
        gobject.connect_clicked(move |_| window_action.history_back().activate());

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
