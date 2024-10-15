use adw::StyleManager;
use gtk::{TextTag, WrapMode};

pub struct Link {
    tag: TextTag,
}

impl Link {
    // Construct
    pub fn new() -> Self {
        Self {
            tag: TextTag::builder()
                .foreground_rgba(&StyleManager::default().accent_color_rgba()) // @TODO
                .sentence(true)
                .wrap_mode(WrapMode::Word)
                .build(),
        }
    }

    // Getters
    pub fn gobject(&self) -> &TextTag {
        &self.tag
    }
}
