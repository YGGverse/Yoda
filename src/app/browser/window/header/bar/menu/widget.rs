use gtk::{gio::Menu, Align, MenuButton};

pub struct Widget {
    pub gobject: MenuButton,
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
}
