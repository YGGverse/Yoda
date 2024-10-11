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
        action_tool_debug: Arc<SimpleAction>,
        action_tool_profile: Arc<SimpleAction>,
        action_quit: Arc<SimpleAction>,
        action_tab_append: Arc<SimpleAction>,
        action_tab_close: Arc<SimpleAction>,
        action_tab_close_all: Arc<SimpleAction>,
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_tab_pin: Arc<SimpleAction>,
        // Widgets
        tab_view: &TabView,
    ) -> Arc<Self> {
        // Init components
        let bar = Bar::new_arc(
            action_tool_debug,
            action_tool_profile,
            action_quit,
            action_tab_append,
            action_tab_close,
            action_tab_close_all,
            action_tab_page_navigation_base,
            action_tab_page_navigation_history_back,
            action_tab_page_navigation_history_forward,
            action_tab_page_navigation_reload,
            action_tab_pin,
            tab_view,
        );

        // Return new struct
        Arc::new(Self {
            widget: Widget::new_arc(bar.gobject()),
        })
    }

    // Getters
    pub fn gobject(&self) -> &ToolbarView {
        &self.widget.gobject()
    }
}
