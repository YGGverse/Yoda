mod widget;

use widget::Widget;

use crate::app::browser::window::action::Action as WindowAction;
use gtk::glib::{gformat, GString, Uri, UriFlags};
use std::{cell::RefCell, rc::Rc};

pub struct Home {
    action: Rc<WindowAction>,
    uri: RefCell<Option<Uri>>,
    pub widget: Rc<Widget>,
}

impl Home {
    // Construct
    pub fn new(action: Rc<WindowAction>) -> Self {
        Self {
            action: action.clone(),
            uri: RefCell::new(None),
            widget: Rc::new(Widget::new(action)),
        }
    }

    // Actions
    pub fn update(&self, request: &str) {
        let has_home = match Uri::parse(request, UriFlags::NONE) {
            Ok(uri) => {
                let has_home = "/" != uri.path();
                self.uri.replace(Some(uri));
                has_home
            }
            _ => {
                self.uri.replace(None);
                false
            }
        };
        self.action.home.gobject.set_enabled(has_home);
        self.widget.update(has_home);
    }

    // Getters

    pub fn url(&self) -> Option<GString> {
        if let Some(uri) = &*self.uri.borrow() {
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
