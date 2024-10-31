use adw::{TabBar, TabView};
use gtk::prelude::IsA;
use std::sync::Arc;

pub struct Widget {
    gobject: TabBar,
}

impl Widget {
    // Construct
    pub fn new_arc(view: &TabView, start_action_widget: &impl IsA<gtk::Widget>) -> Arc<Self> {
        Arc::new(Self {
            gobject: TabBar::builder()
                .autohide(false)
                .expand_tabs(false)
                .start_action_widget(start_action_widget)
                .view(view)
                .build(),
        })
    }

    // Getters
    pub fn gobject(&self) -> &TabBar {
        &self.gobject
    }
}
