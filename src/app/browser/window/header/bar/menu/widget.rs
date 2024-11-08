use gtk::{gio::Menu, Align, MenuButton};
use std::rc::Rc;

pub struct Widget {
    gobject: MenuButton,
}

impl Widget {
    // Construct
    pub fn new_rc(model: &Menu) -> Rc<Self> {
        Rc::new(Self {
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
