mod redirect;
use redirect::Redirect;

use gtk::glib::{GString, Uri};
use std::{cell::RefCell, sync::Arc};

#[derive(Debug, Clone)]
pub enum Status {
    Complete,
    Connected,
    Connecting,
    Failure,
    Input,
    New,
    ProxyNegotiated,
    ProxyNegotiating,
    Redirect,
    Reload,
    Resolved,
    Resolving,
    Success,
    TlsHandshaked,
    TlsHandshaking,
}

pub struct Meta {
    status: RefCell<Status>,
    title: RefCell<GString>,
    redirect: RefCell<Option<Redirect>>,
}

impl Meta {
    // Constructors

    pub fn new_arc(status: Status, title: GString) -> Arc<Self> {
        Arc::new(Self {
            status: RefCell::new(status),
            title: RefCell::new(title),
            redirect: RefCell::new(None),
        })
    }

    // Setters

    pub fn set_status(&self, status: Status) -> &Self {
        match status {
            Status::Redirect => {
                if self.redirect.borrow().is_none() {
                    panic!("Set `redirect` before use this status")
                }
            }
            _ => {
                self.unset_redirect();
            }
        };

        self.status.replace(status);
        self
    }

    pub fn set_title(&self, title: &str) -> &Self {
        self.title.replace(GString::from(title));
        self
    }

    pub fn set_redirect(&self, count: i8, is_follow: bool, target: Uri) -> &Self {
        self.redirect
            .replace(Some(Redirect::new(count, is_follow, target)));
        self
    }

    pub fn unset_redirect(&self) -> &Self {
        self.redirect.replace(None);
        self
    }

    // Getters

    pub fn status(&self) -> Status {
        self.status.borrow().clone()
    }

    pub fn title(&self) -> GString {
        self.title.borrow().clone()
    }

    pub fn is_redirect(&self) -> bool {
        self.redirect.borrow().is_some()
    }

    pub fn redirect_count(&self) -> Option<i8> {
        match *self.redirect.borrow() {
            Some(ref redirect) => Some(redirect.count().clone()),
            None => None,
        }
    }

    pub fn redirect_target(&self) -> Option<Uri> {
        match *self.redirect.borrow() {
            Some(ref redirect) => Some(redirect.target().clone()),
            None => None,
        }
    }

    pub fn redirect_is_follow(&self) -> Option<bool> {
        match *self.redirect.borrow() {
            Some(ref redirect) => Some(redirect.is_follow().clone()),
            None => None,
        }
    }
}
