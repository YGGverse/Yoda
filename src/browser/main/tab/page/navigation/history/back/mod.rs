use gtk::Button;

pub struct Back {
    widget: Button,
}

impl Back {
    // Construct
    pub fn new() -> Back {
        Self {
            widget: Button::builder()
                .action_name("win.tab_page_history_back")
                .icon_name("go-previous-symbolic")
                .tooltip_text("Back")
                .sensitive(false)
                .build(),
        }
    }

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
    }
}
