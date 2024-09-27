use gtk::{gio::SimpleAction, prelude::ActionExt, prelude::ButtonExt, Button};

pub struct Tab {
    pub widget: Button,
}

impl Tab {
    // Construct
    pub fn new(action_tab_append: &SimpleAction) -> Self {
        // Init widget
        let widget = Button::builder()
            .icon_name("tab-new-symbolic")
            .tooltip_text("New tab")
            .build();

        // Init events
        widget.connect_clicked({
            let action_tab_append = action_tab_append.clone();
            move |_| {
                action_tab_append.activate(None);
            }
        });

        // Return activated struct
        Self { widget }
    }

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
    }
}
