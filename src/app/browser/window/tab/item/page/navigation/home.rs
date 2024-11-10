mod widget;

use widget::Widget;

use crate::app::browser::window::action::Action as WindowAction;
use gtk::{
    glib::{gformat, GString, Uri},
    Button,
};
use std::{cell::RefCell, rc::Rc};

pub struct Home {
    window_action: Rc<WindowAction>,
    uri: RefCell<Option<Uri>>,
    widget: Rc<Widget>,
}

impl Home {
    // Construct
    pub fn new_rc(window_action: Rc<WindowAction>) -> Rc<Self> {
        Rc::new(Self {
            window_action: window_action.clone(),
            uri: RefCell::new(None),
            widget: Widget::new_rc(window_action),
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
        self.window_action.home().gobject().set_enabled(status);

        // Update child components
        self.widget.update(status);
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        self.widget.gobject()
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
