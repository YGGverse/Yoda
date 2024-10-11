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

use gtk::{
    gio::SimpleAction,
    glib::GString,
    prelude::{BoxExt, WidgetExt},
    Box, DirectionType, Orientation,
};

use std::sync::Arc;

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
    pub fn new(
        action_tab_page_navigation_base: Arc<SimpleAction>,
        action_tab_page_navigation_history_back: Arc<SimpleAction>,
        action_tab_page_navigation_history_forward: Arc<SimpleAction>,
        action_tab_page_navigation_reload: Arc<SimpleAction>,
        action_update: Arc<SimpleAction>,
    ) -> Self {
        // Init components
        let base = Base::new(action_tab_page_navigation_base);
        let history = History::new(
            action_tab_page_navigation_history_back,
            action_tab_page_navigation_history_forward,
        );
        let reload = Reload::new(action_tab_page_navigation_reload.clone());
        let request = Request::new(
            action_update.clone(),
            action_tab_page_navigation_reload.clone(),
        );
        let bookmark = Bookmark::new();

        // Init widget
        let widget = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .margin_start(6)
            .margin_end(6)
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
    pub fn request_grab_focus(&self) {
        self.request.widget().grab_focus();
    }

    pub fn history_add(&self, request: GString) {
        self.history.add(request, true);
    }

    pub fn history_back(&self, follow_to_index: bool) -> Option<GString> {
        self.history.back(follow_to_index)
    }

    pub fn history_current(&self) -> Option<GString> {
        self.history.current()
    }

    pub fn history_forward(&self, follow_to_index: bool) -> Option<GString> {
        self.history.forward(follow_to_index)
    }

    pub fn update(&self, progress_fraction: Option<f64>) {
        self.base.update(self.request.uri());
        self.history.update();
        self.reload.update(!self.request.is_empty());
        self.request.update(progress_fraction);
        self.bookmark.update();
    }

    // Setters
    pub fn set_request_text(&self, value: &GString) {
        // Focus out from content area on activate the link @TODO
        self.widget.child_focus(DirectionType::Right);

        self.request.set_text(value);
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.widget
    }

    pub fn base_url(&self) -> Option<GString> {
        self.base.url()
    }

    pub fn request_text(&self) -> GString {
        self.request.text()
    }
}
