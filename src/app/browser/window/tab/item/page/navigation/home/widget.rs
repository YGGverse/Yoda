use gtk::{
    gio::SimpleAction,
    prelude::{ActionExt, ButtonExt, WidgetExt},
    Button,
};
use std::rc::Rc;

pub struct Widget {
    gobject: Button,
}

impl Widget {
    // Construct
    pub fn new_rc(action_page_home: SimpleAction) -> Rc<Self> {
        // Init gobject
        let gobject = Button::builder()
            .icon_name("go-home-symbolic")
            .tooltip_text("Home")
            .sensitive(false)
            .build();

        // Init events
        gobject.connect_clicked({
            let action_page_home = action_page_home.clone();
            move |_| {
                action_page_home.activate(None);
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
