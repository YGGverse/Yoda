use gtk::{Box, HeaderBar};

pub struct Widget {
    gobject: HeaderBar,
}

impl Widget {
    // Construct
    pub fn new(tray: &Box, title_widget: Option<&Box>) -> Self {
        let gobject = HeaderBar::builder().build();

        gobject.pack_start(tray);
        gobject.set_title_widget(title_widget);

        Self { gobject }
    }

    // Getters
    pub fn gobject(&self) -> &HeaderBar {
        &self.gobject
    }
}
