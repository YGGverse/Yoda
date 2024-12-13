mod widget;

use widget::Widget;

use crate::app::browser::window::action::Action as WindowAction;
use gtk::glib::{gformat, GString, Uri};
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
    pub fn update(&self, request: Option<&Uri>) {
        let has_home = match request {
            Some(uri) => {
                self.uri.replace(Some(uri.clone()));
                uri.path().len() > 1
            }
            None => {
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
                return Some(if port.is_positive() {
                    gformat!("{scheme}://{host}:{port}/")
                } else {
                    gformat!("{scheme}://{host}/")
                });
            }
        }
        None
    }
}
