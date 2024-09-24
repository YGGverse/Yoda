use gtk::Button;

pub struct Bookmark {
    widget: Button,
}

impl Bookmark {
    // Construct
    pub fn new() -> Bookmark {
        Self {
            widget: Button::builder()
                .action_name("win.tab_page_bookmark")
                .icon_name("starred-symbolic")
                .tooltip_text("Bookmark")
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
