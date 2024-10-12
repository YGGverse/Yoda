use adw::ClampScrollable;
use gtk::TextView;
use std::sync::Arc;

pub struct Widget {
    gobject: ClampScrollable,
}

impl Widget {
    // Construct
    pub fn new_arc(child: &TextView) -> Arc<Self> {
        Arc::new(Self {
            gobject: ClampScrollable::builder().child(child).build(),
        })
    }

    // Getters
    pub fn gobject(&self) -> &ClampScrollable {
        &self.gobject
    }
}