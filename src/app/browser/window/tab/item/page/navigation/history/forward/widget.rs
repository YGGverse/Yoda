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
    pub fn new_rc(window_action: Rc<WindowAction>) -> Rc<Self> {
        // Init gobject
        let gobject = Button::builder()
            .icon_name("go-next-symbolic")
            .tooltip_text("Forward")
            .sensitive(false)
            .build();

        // Init events
        gobject.connect_clicked({
            let window_action = window_action.clone();
            move |_| {
                window_action.history_forward().activate();
            }
        });

        // Return activated struct
        Rc::new(Self { gobject })
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
