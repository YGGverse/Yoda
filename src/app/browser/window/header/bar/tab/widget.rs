use adw::{TabBar, TabView};
use std::sync::Arc;

pub struct Widget {
    gobject: TabBar,
}

impl Widget {
    // Construct
    pub fn new_arc(view: &TabView) -> Arc<Self> {
        Arc::new(Self {
            gobject: TabBar::builder().view(&view).autohide(false).build(),
        })
    }

    // Getters
    pub fn gobject(&self) -> &TabBar {
        &self.gobject
    }
}
