use gtk::Button;

pub struct Base {
    widget: Button,
}

impl Base {
    // Construct
    pub fn new() -> Base {
        Self {
            widget: Button::builder()
                .action_name("win.tab_page_base")
                .icon_name("go-home-symbolic")
                .tooltip_text("Base")
                .sensitive(false)
                .build(),
        }
    }

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
    }
}
