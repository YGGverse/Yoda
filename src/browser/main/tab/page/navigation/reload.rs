use gtk::{
    gio::SimpleAction,
    prelude::{ActionExt, ButtonExt, WidgetExt},
    Button,
};
use std::sync::Arc;

pub struct Reload {
    action_tab_page_reload: Arc<SimpleAction>,
    widget: Button,
}

impl Reload {
    // Construct
    pub fn new(action_tab_page_reload: Arc<SimpleAction>) -> Self {
        // Init widget
        let widget = Button::builder()
            .icon_name("view-refresh-symbolic")
            .tooltip_text("Reload")
            .sensitive(false)
            .build();

        // Init events
        widget.connect_clicked({
            let action_tab_page_reload = action_tab_page_reload.clone();
            move |_| {
                action_tab_page_reload.activate(None);
            }
        });

        // Return activated struct
        Self {
            action_tab_page_reload,
            widget,
        }
    }

    // Actions
    pub fn update(&self, is_enabled: bool) {
        self.action_tab_page_reload.set_enabled(is_enabled);
        self.widget.set_sensitive(is_enabled);
    }

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
    }
}
