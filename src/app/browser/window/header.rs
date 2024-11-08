mod bar;
mod widget;

use bar::Bar;
use widget::Widget;

use adw::{TabView, ToolbarView};
use gtk::gio::SimpleAction;
use std::sync::Arc;

pub struct Header {
    widget: Arc<Widget>,
}

impl Header {
    // Construct
    pub fn new_arc(
        // Actions
        action_about: SimpleAction,
        action_debug: SimpleAction,
        action_profile: SimpleAction,
        action_quit: SimpleAction,
        action_page_new: SimpleAction,
        action_page_close: SimpleAction,
        action_page_close_all: SimpleAction,
        action_page_home: SimpleAction,
        action_page_history_back: SimpleAction,
        action_page_history_forward: SimpleAction,
        action_page_reload: SimpleAction,
        action_page_pin: SimpleAction,
        // Widgets
        tab_view: &TabView,
    ) -> Arc<Self> {
        // Init components
        let bar = Bar::new_arc(
            action_about,
            action_debug,
            action_profile,
            action_quit,
            action_page_new,
            action_page_close,
            action_page_close_all,
            action_page_home,
            action_page_history_back,
            action_page_history_forward,
            action_page_reload,
            action_page_pin,
            tab_view,
        );

        // Return new struct
        Arc::new(Self {
            widget: Widget::new_arc(bar.gobject()),
        })
    }

    // Getters
    pub fn gobject(&self) -> &ToolbarView {
        self.widget.gobject()
    }
}
