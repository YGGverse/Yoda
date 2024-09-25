mod subject;
mod tray;

use subject::Subject;
use tray::Tray;

use gtk::{glib::GString, HeaderBar};
use std::sync::Arc;

pub struct Header {
    widget: HeaderBar,
    subject: Subject,
}

impl Header {
    // Construct
    pub fn new() -> Arc<Header> {
        let tray = Tray::new();
        let subject = Subject::new();

        let widget = HeaderBar::builder().build();
        widget.pack_start(tray.widget());
        widget.set_title_widget(Some(subject.widget()));

        Arc::new(Self { widget, subject })
    }

    // Actions
    pub fn update(&self, title: GString, description: GString) {
        self.subject.update(title, description);
    }

    // Getters
    pub fn widget(&self) -> &HeaderBar {
        &self.widget
    }
}
