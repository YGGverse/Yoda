mod subject;
mod tray;

use subject::Subject;
use tray::Tray;

use gtk::{gio::SimpleAction, glib::GString, HeaderBar};

use std::sync::Arc;

pub struct Header {
    widget: HeaderBar,
    subject: Subject,
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
