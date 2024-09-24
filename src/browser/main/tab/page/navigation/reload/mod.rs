use gtk::Button;

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

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
    }
}
