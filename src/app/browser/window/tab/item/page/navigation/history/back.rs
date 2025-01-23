use super::WindowAction;
use gtk::{
    prelude::{ButtonExt, WidgetExt},
    Button,
};
use std::rc::Rc;

pub struct Back {
    action: Rc<WindowAction>,
    pub button: Button,
}

impl Back {
    // Constructors

    /// Build new `Self`
    pub fn build(action: &Rc<WindowAction>) -> Self {
        // Init gobject
        let button = Button::builder()
            .icon_name("go-previous-symbolic")
            .tooltip_text("Back")
            .sensitive(false)
            .build();

        // Init events
        button.connect_clicked({
            let action = action.clone();
            move |_| action.history_back.activate()
        });

        // Return activated `Self`
        Self {
            action: action.clone(),
            button,
        }
    }

    // Actions

    pub fn update(&self, status: bool) {
        self.action.history_back.simple_action.set_enabled(status);
        self.button.set_sensitive(status);
    }
}
