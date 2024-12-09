use crate::app::browser::window::tab::item::Action;
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
            .icon_name("avatar-default-symbolic")
            .tooltip_text("Identity")
            //.sensitive(false)
            .build();

        // Init events @TODO dialog window required
        gobject.connect_clicked(move |_| action.ident.activate());

        // Return activated `Self`
        Self { gobject }
    }

    // Actions
    pub fn update(&self, is_auth: bool, is_enabled: bool) {
        self.gobject.set_sensitive(is_enabled);
        self.gobject
            .set_css_classes(if is_auth { &["success"] } else { &[] });
    }
}
