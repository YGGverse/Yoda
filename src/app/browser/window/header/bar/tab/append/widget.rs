use gtk::{
    gio::SimpleAction,
    prelude::{ActionExt, ButtonExt},
    Align, Button,
};
use std::rc::Rc;

pub struct Widget {
    gobject: Button,
}

impl Widget {
    // Construct
    pub fn new_rc(action_page_new: SimpleAction) -> Rc<Self> {
        // Init gobject
        let gobject = Button::builder()
            .icon_name("tab-new-symbolic")
            .css_classes(["flat"])
            .valign(Align::Center)
            .tooltip_text("New tab")
            .build();

        // Init events
        gobject.connect_clicked(move |_| {
            action_page_new.activate(None);
        });

        Rc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        &self.gobject
    }
}
