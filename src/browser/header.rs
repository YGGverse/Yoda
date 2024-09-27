mod subject;
mod tray;

use subject::Subject;
use tray::Tray;

use gtk::{gio::SimpleAction, glib::GString, HeaderBar};

pub struct Header {
    widget: HeaderBar,
    subject: Subject,
}

impl Header {
    // Construct
    pub fn new(action_debug: &SimpleAction, action_quit: &SimpleAction) -> Self {
        // Init components
        let tray = Tray::new(action_debug, action_quit);

        let subject = Subject::new();

        // Init widget
        let widget = HeaderBar::builder().build();
        widget.pack_start(tray.widget());
        widget.set_title_widget(Some(subject.widget()));

        // Return new struct
        Self { widget, subject }
    }

    // Actions
    pub fn update(&self, title: Option<GString>, description: Option<GString>) {
        self.subject.update(title, description);
    }

    // Getters
    pub fn widget(&self) -> &HeaderBar {
        &self.widget
    }
}
