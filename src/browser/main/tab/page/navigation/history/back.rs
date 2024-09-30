use gtk::{
    gio::SimpleAction,
    prelude::{ActionExt, ButtonExt, WidgetExt},
    Button,
};
use std::sync::Arc;

pub struct Back {
    action_tab_page_navigation_history_back: Arc<SimpleAction>,
    widget: Button,
}

impl Back {
    // Construct
    pub fn new(action_tab_page_navigation_history_back: Arc<SimpleAction>) -> Self {
        // Init widget
        let widget = Button::builder()
            .icon_name("go-previous-symbolic")
            .tooltip_text("Back")
            .sensitive(false)
            .build();

        // Init events
        widget.connect_clicked({
            let action_tab_page_navigation_history_back =
                action_tab_page_navigation_history_back.clone();
            move |_| {
                action_tab_page_navigation_history_back.activate(None);
            }
        });

        // Return activated struct
        Self {
            action_tab_page_navigation_history_back,
            widget,
        }
    }

    // Actions
    pub fn update(&self, status: bool) {
        self.action_tab_page_navigation_history_back
            .set_enabled(status);
        self.widget.set_sensitive(status);
    }

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
    }
}
