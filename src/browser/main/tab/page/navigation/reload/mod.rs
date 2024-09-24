use gtk::{prelude::WidgetExt, Button};

pub struct Reload {
    widget: Button,
}

impl Reload {
    // Construct
    pub fn new() -> Reload {
        Self {
            widget: Button::builder()
                .action_name("win.tab_page_reload")
                .icon_name("view-refresh-symbolic")
                .tooltip_text("Reload")
                .sensitive(false)
                .build(),
        }
    }

    // Actions
    pub fn update(&self, is_enabled: bool) {
        self.widget.set_sensitive(is_enabled);
    }

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
    }
}
