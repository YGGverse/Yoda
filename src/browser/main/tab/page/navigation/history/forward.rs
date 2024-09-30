use gtk::{
    prelude::{ActionExt, ButtonExt, WidgetExt},
    {gio::SimpleAction, Button},
};
use std::sync::Arc;

pub struct Forward {
    action_tab_page_navigation_history_forward: Arc<SimpleAction>,
    widget: Button,
}

impl Forward {
    // Construct
    pub fn new(action_tab_page_navigation_history_forward: Arc<SimpleAction>) -> Self {
        // Init widget
        let widget = Button::builder()
            .icon_name("go-next-symbolic")
            .tooltip_text("Forward")
            .sensitive(false)
            .build();

        // Init events
        widget.connect_clicked({
            let action_tab_page_navigation_history_forward =
                action_tab_page_navigation_history_forward.clone();
            move |_| {
                action_tab_page_navigation_history_forward.activate(None);
            }
        });

        // Return activated struct
        Self {
            action_tab_page_navigation_history_forward,
            widget,
        }
    }

    // Actions
    pub fn update(&self, status: bool) {
        self.action_tab_page_navigation_history_forward
            .set_enabled(status);
        self.widget.set_sensitive(status);
    }

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
    }
}
