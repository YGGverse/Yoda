use gtk::{gio::Menu, Align, MenuButton};

pub struct Widget {
    gobject: MenuButton,
}

impl Widget {
    // Construct
    pub fn new(model: &Menu) -> Self {
        Self {
            gobject: MenuButton::builder()
                .css_classes(["flat"])
                .icon_name("open-menu-symbolic")
                .menu_model(model)
                .tooltip_text("Menu")
                .valign(Align::Center)
                .build(),
        }
    }

    // Getters
    pub fn gobject(&self) -> &MenuButton {
        &self.gobject
    }
}
