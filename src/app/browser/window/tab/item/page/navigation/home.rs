use super::WindowAction;
use gtk::{
    prelude::{ButtonExt, WidgetExt},
    Button,
};
use std::rc::Rc;

pub struct Home {
    action: Rc<WindowAction>,
    pub button: Button,
}

impl Home {
    // Construct
    pub fn build(action: &Rc<WindowAction>) -> Self {
        // Init gobject
        let button = Button::builder()
            .icon_name("go-home-symbolic")
            .tooltip_text("Home")
            .sensitive(false)
            .build();

        // Init events
        button.connect_clicked({
            let action = action.clone();
            move |_| action.home.activate()
        });

        // Return activated `Self`
        Self {
            action: action.clone(),
            button,
        }
    }

    // Actions
    pub fn update(&self, has_home: bool) {
        self.action.home.simple_action.set_enabled(has_home);
        self.button.set_sensitive(has_home);
    }
}
