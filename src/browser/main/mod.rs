mod tab;

use std::sync::Arc;

use gtk::prelude::BoxExt;
use gtk::Box;

pub struct Main {
    pub widget: Arc<gtk::Box>,
    pub tab: Arc<tab::Tab>,
}

impl Main {
    pub fn tab_append(&self) {
        self.tab.append(true);
    }
}

pub fn new() -> Main {
    // Init components
    let tab = Arc::new(tab::new());

    // Init widget
    let widget = Arc::new(
        Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build(),
    );

    widget.append(tab.widget.as_ref());

    // Init struct
    Main { widget, tab }
}
