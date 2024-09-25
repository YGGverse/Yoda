use gtk::{Align, Label};

pub struct Reader {
    widget: Label,
}

impl Reader {
    // Construct
    pub fn new() -> Self {
        Self {
            widget: Label::builder()
                .halign(Align::Start)
                .valign(Align::Start)
                .margin_start(8)
                .margin_end(8)
                .wrap(true)
                .selectable(true)
                .use_markup(true)
                .build(),
        }
    }

    // Getters
    pub fn widget(&self) -> &Label {
        &self.widget
    }
}
