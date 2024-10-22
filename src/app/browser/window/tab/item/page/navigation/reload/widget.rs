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
    pub fn new_arc(action_tab_page_navigation_reload: Arc<SimpleAction>) -> Arc<Self> {
        // Init gobject
        let gobject = Button::builder()
            .icon_name("view-refresh-symbolic")
            .tooltip_text("Reload")
            .sensitive(false)
            .build();

        // Init events
        gobject.connect_clicked({
            let action_tab_page_navigation_reload = action_tab_page_navigation_reload.clone();
            move |_| {
                action_tab_page_navigation_reload.activate(None);
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
