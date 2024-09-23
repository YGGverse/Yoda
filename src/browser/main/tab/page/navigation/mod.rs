mod base;
mod bookmark;
mod history;
mod reload;
mod request;

use gtk::prelude::BoxExt;
use gtk::{Box, Orientation};

use base::Base;
use bookmark::Bookmark;
use history::History;
use reload::Reload;
use request::Request;

pub struct Navigation {
    widget: Box,
}

impl Navigation {
    pub fn new() -> Navigation {
        let base = Base::new();
        let history = History::new();
        let reload = Reload::new();
        let request = Request::new();
        let bookmark = Bookmark::new();

        let widget = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .margin_top(8)
            .margin_start(8)
            .margin_end(8)
            .margin_bottom(8)
            .build();

        widget.append(base.widget());
        widget.append(history.widget());
        widget.append(reload.widget());
        widget.append(request.widget());
        widget.append(bookmark.widget());

        Self { widget }
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }
}
