use super::{Request, WindowAction};
use gtk::{
    prelude::{ButtonExt, WidgetExt},
    Button,
};
use std::rc::Rc;

pub struct Home {
    action: Rc<WindowAction>,
    request: Rc<Request>,
    pub button: Button,
}

impl Home {
    // Construct
    pub fn build(action: &Rc<WindowAction>, request: &Rc<Request>) -> Self {
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
            request: request.clone(),
            button,
        }
    }

    // Actions
    pub fn update(&self) {
        let has_home = self.request.home().is_some();
        self.action.home.simple_action.set_enabled(has_home);
        self.button.set_sensitive(has_home);
    }
}
