use gtk::{
    gio::SimpleAction,
    prelude::{ActionExt, ButtonExt, WidgetExt},
    Button,
};
use std::sync::Arc;

pub struct Reload {
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
        widget.connect_clicked(move |_| {
            action_tab_page_reload.activate(None);
        });

        // Return activated struct
        Self { widget }
    }

    // Actions
    pub fn update(&self, is_enabled: bool) {
        self.widget.set_sensitive(is_enabled);
        // @TODO deactivate action
    }

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
    }
}
