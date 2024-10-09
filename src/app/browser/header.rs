mod title;
mod tray;
mod widget;

use title::Title;
use tray::Tray;
use widget::Widget;

use adw::HeaderBar;
use gtk::gio::SimpleAction;
use std::sync::Arc;

pub struct Header {
    title: Arc<Title>,
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

        let title = Arc::new(Title::new());

        // Init widget
        let widget = Arc::new(Widget::new(tray.gobject(), Some(title.gobject())));

        // Return new struct
        Self { title, widget }
    }

    // Actions
    pub fn update(&self, title: Option<&str>, description: Option<&str>) {
        self.title.update(title, description);
    }

    // Getters
    pub fn gobject(&self) -> &HeaderBar {
        &self.widget.gobject()
    }
}
