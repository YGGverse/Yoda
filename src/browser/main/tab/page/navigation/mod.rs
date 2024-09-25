mod base;
mod bookmark;
mod history;
mod reload;
mod request;

use base::Base;
use bookmark::Bookmark;
use history::History;
use reload::Reload;
use request::Request;

use gtk::{glib::GString, prelude::BoxExt, Box, Orientation};

pub struct Navigation {
    // GTK
    widget: Box,
    // Components
    base: Base,
    history: History,
    reload: Reload,
    request: Request,
    bookmark: Bookmark,
}

impl Navigation {
    pub fn new() -> Self {
        // Init components
        let base = Base::new();
        let history = History::new();
        let reload = Reload::new();
        let request = Request::new();
        let bookmark = Bookmark::new();

        // Init widget
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

        // Result
        Self {
            widget,
            base,
            history,
            reload,
            request,
            bookmark,
        }
    }

    // Actions
    pub fn update(&self) {
        self.base.update();
        self.history.update();
        self.reload.update(!self.request.is_empty());
        self.request.update();
        self.bookmark.update();
    }

    // Setters
    pub fn set_request_text(&self, value: &GString, activate: bool) {
        self.request.set_text(value, activate);
    }

    // Getters
    pub fn widget(&self) -> &Box {
        &self.widget
    }

    pub fn request_text(&self) -> GString {
        self.request.text()
    }
}
