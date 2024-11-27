use crate::app::browser::window::Action;
use gtk::{
    prelude::{ButtonExt, WidgetExt},
    Button,
};
use std::rc::Rc;

pub struct Widget {
    pub gobject: Button,
}

impl Widget {
    // Construct
    pub fn new(action: Rc<Action>) -> Self {
        // Init gobject
        let gobject = Button::builder()
            .icon_name("go-home-symbolic")
            .tooltip_text("Home")
            .sensitive(false)
            .build();

        // Init events
        gobject.connect_clicked(move |_| action.home.activate());

        // Return activated `Self`
        Self { gobject }
    }

    // Actions
    pub fn update(&self, is_sensitive: bool) {
        self.gobject.set_sensitive(is_sensitive);
    }
}
