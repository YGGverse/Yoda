mod subject;
mod tray;
mod widget;

use subject::Subject;
use tray::Tray;
use widget::Widget;

use adw::HeaderBar;
use gtk::{gio::SimpleAction, glib::GString};
use std::sync::Arc;

pub struct Header {
    subject: Arc<Subject>,
    // tray: Arc<Subject>,
    widget: Arc<Widget>,
}

impl Header {
    // Construct
    pub fn new(
        action_tool_debug: Arc<SimpleAction>,
        action_tool_profile_directory: Arc<SimpleAction>,
        action_quit: Arc<SimpleAction>,
        action_tab_append: Arc<SimpleAction>,
        action_tab_close: Arc<SimpleAction>,
        action_tab_close_all: Arc<SimpleAction>,
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_tab_pin: Arc<SimpleAction>,
    ) -> Self {
        // Init components
        let tray = Tray::new(
            action_tool_debug,
            action_tool_profile_directory,
            action_quit,
            action_tab_append,
            action_tab_close,
            action_tab_close_all,
            action_tab_page_navigation_base,
            action_tab_page_navigation_history_back,
            action_tab_page_navigation_history_forward,
            action_tab_page_navigation_reload,
            action_tab_pin,
        );

        let subject = Arc::new(Subject::new());

        // Init widget
        let widget = Arc::new(Widget::new(tray.gobject(), Some(subject.gobject())));

        // Return new struct
        Self { subject, widget }
    }

    // Actions
    pub fn update(&self, title: Option<GString>, description: Option<GString>) {
        self.subject.update(title, description);
    }

    // Getters
    pub fn gobject(&self) -> &HeaderBar {
        &self.widget.gobject()
    }
}
