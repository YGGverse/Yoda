use gtk::{
    gio::SimpleAction,
    prelude::{ActionExt, ButtonExt, WidgetExt},
    Button,
};
use std::sync::Arc;

pub struct Widget {
    gobject: Button,
}

impl Widget {
    // Construct
    pub fn new_arc(action_page_reload: SimpleAction) -> Arc<Self> {
        // Init gobject
        let gobject = Button::builder()
            .icon_name("view-refresh-symbolic")
            .tooltip_text("Reload")
            .sensitive(false)
            .build();

        // Init events
        gobject.connect_clicked({
            let action_page_reload = action_page_reload.clone();
            move |_| {
                action_page_reload.activate(None);
            }
        });

        // Return activated struct
        Arc::new(Self { gobject })
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
