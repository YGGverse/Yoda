mod subject;
mod tray;

use gtk::HeaderBar;
use subject::Subject;
use tray::Tray;

pub struct Header {
    widget: HeaderBar,
}

impl Header {
    // Construct
    pub fn new() -> Header {
        let tray = Tray::new();
        let subject = Subject::new();

        let widget = HeaderBar::builder().build();
        widget.pack_start(tray.widget());
        widget.set_title_widget(Some(subject.widget()));

        Self { widget }
    }

    // Getters
    pub fn widget(&self) -> &HeaderBar {
        &self.widget
    }
}
