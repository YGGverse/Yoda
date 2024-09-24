use gtk::Button;

pub struct Forward {
    widget: Button,
}

impl Forward {
    // Construct
    pub fn new() -> Forward {
        Self {
            widget: Button::builder()
                .action_name("win.tab_page_history_forward")
                .icon_name("go-next-symbolic")
                .tooltip_text("Forward")
                .sensitive(false)
                .build(),
        }
    }

    // Actions
    pub fn update(&self) {
        // @TODO
    }

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
    }
}
