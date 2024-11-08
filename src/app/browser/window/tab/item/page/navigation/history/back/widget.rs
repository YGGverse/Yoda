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
    pub fn new_rc(action_page_history_back: SimpleAction) -> Rc<Self> {
        // Init gobject
        let gobject = Button::builder()
            .icon_name("go-previous-symbolic")
            .tooltip_text("Back")
            .sensitive(false)
            .build();

        // Init events
        gobject.connect_clicked({
            let action_page_history_back = action_page_history_back.clone();
            move |_| {
                action_page_history_back.activate(None);
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
