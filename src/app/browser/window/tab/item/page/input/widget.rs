use adw::Clamp;
use gtk::{prelude::WidgetExt, Box};

const MARGIN: i32 = 6;

pub struct Widget {
    pub clamp: Clamp,
}

impl Widget {
    // Construct
    pub fn new() -> Self {
        let clamp = Clamp::builder()
            .margin_bottom(MARGIN)
            .margin_top(MARGIN)
            .maximum_size(800)
            .visible(false)
            .build();

        Self { clamp }
    }

    // Actions
    pub fn update(&self, child: Option<&Box>) {
        if child.is_some() {
            self.clamp.set_visible(true); // widget may be hidden, make it visible to child redraw
            self.clamp.set_child(child);
        } else {
            self.clamp.set_visible(false)
        }
    }
}
