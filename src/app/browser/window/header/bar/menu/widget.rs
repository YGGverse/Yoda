use gtk::{gio::Menu, Align, MenuButton};
use std::sync::Arc;

pub struct Widget {
    gobject: MenuButton,
}

impl Widget {
    // Construct
    pub fn new_arc(model: &Menu) -> Arc<Self> {
        Arc::new(Self {
            gobject: MenuButton::builder()
                .css_classes(["flat"])
                .icon_name("open-menu-symbolic")
                .menu_model(model)
                .tooltip_text("Menu")
                .valign(Align::Center)
                .build(),
        })
    }

    // Getters
    pub fn gobject(&self) -> &MenuButton {
        &self.gobject
    }
}
