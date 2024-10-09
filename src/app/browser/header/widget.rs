use adw::{HeaderBar, WindowTitle};
use gtk::Box;

pub struct Widget {
    gobject: HeaderBar,
}

impl Widget {
    // Construct
    pub fn new(pack_start: &Box, title_widget: Option<&WindowTitle>) -> Self {
        let gobject = HeaderBar::builder().build();

        gobject.pack_start(pack_start);
        gobject.set_title_widget(title_widget);

        Self { gobject }
    }

    // Getters
    pub fn gobject(&self) -> &HeaderBar {
        &self.gobject
    }
}
