use gtk::{
    prelude::{ButtonExt, WidgetExt},
    Button,
};

use super::WindowAction;
use std::rc::Rc;

pub struct Forward {
    action: Rc<WindowAction>,
    pub button: Button,
}

impl Forward {
    // Constructors

    /// Build new `Self`
    pub fn build(action: &Rc<WindowAction>) -> Self {
        // Init gobject
        let button = Button::builder()
            .icon_name("go-next-symbolic")
            .tooltip_text("Forward")
            .sensitive(false)
            .build();

        // Init events
        button.connect_clicked({
            let action = action.clone();
            move |_| action.history_forward.activate()
        });

        // Return activated `Self`
        Self {
            action: action.clone(),
            button,
        }
    }

    // Actions
    pub fn update(&self, status: bool) {
        self.action
            .history_forward
            .simple_action
            .set_enabled(status);

        self.button.set_sensitive(status);
    }
}
