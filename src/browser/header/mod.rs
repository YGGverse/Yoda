mod subject;
mod tray;

use std::sync::Arc;

use gtk::HeaderBar;

pub struct Header {
    pub widget: Arc<gtk::HeaderBar>,
}

pub fn new() -> Header {
    let widget = Arc::new(HeaderBar::builder().build());

    widget.pack_start(&tray::new());
    widget.set_title_widget(Some(&subject::new()));

    Header { widget }
}
