mod widget;

use widget::Widget;

use gtk::{
    gio::SimpleAction,
    glib::{gformat, GString, Uri},
    Button,
};
use std::{cell::RefCell, sync::Arc};

pub struct Base {
    action_page_base: SimpleAction,
    uri: RefCell<Option<Uri>>,
    widget: Arc<Widget>,
}

impl Base {
    // Construct
    pub fn new_arc(action_page_base: SimpleAction) -> Arc<Self> {
        Arc::new(Self {
            action_page_base: action_page_base.clone(),
            uri: RefCell::new(None),
            widget: Widget::new_arc(action_page_base),
        })
    }

    // Actions
    pub fn update(&self, uri: Option<Uri>) {
        // Detect sensitivity value
        let status = match &uri {
            Some(uri) => "/" != uri.path(),
            None => false,
        };

        // Update parsed cache
        self.uri.replace(uri);

        // Update action status
        self.action_page_base.set_enabled(status);

        // Update child components
        self.widget.update(status);
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        &self.widget.gobject()
    }

    pub fn url(&self) -> Option<GString> {
        // Build URL from parsed URI cache
        if let Some(uri) = self.uri.take() {
            let scheme = uri.scheme();
            let port = uri.port();
            if let Some(host) = uri.host() {
                if port.is_positive() {
                    return Some(gformat!("{scheme}://{host}:{port}/"));
                } else {
                    return Some(gformat!("{scheme}://{host}/"));
                } // @TODO auth params
            }
        }
        None
    }
}
