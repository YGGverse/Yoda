use std::sync::Arc;

use gtk::prelude::{ButtonExt, WidgetExt};
use gtk::Button;

pub struct Tab {
    pub widget: Arc<gtk::Button>,
}

pub fn new() -> Tab {
    // Init widget
    let widget = Arc::new(
        Button::builder()
            .icon_name("tab-new-symbolic")
            .tooltip_text("New tab")
            .build(),
    );

    // Init events
    widget.connect_clicked(|this| {
        this.activate_action("win.tab_append", None)
            .expect("The action does not exist");
    });

    // Result
    Tab { widget }
}
