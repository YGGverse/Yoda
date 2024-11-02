mod redirect;
use redirect::Redirect;

use gtk::glib::GString;
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
    redirect_count: RefCell<i8>,
}

impl Meta {
    // Constructors

    pub fn new_arc(status: Status, title: GString) -> Arc<Self> {
        Arc::new(Self {
            status: RefCell::new(status),
            title: RefCell::new(title),
            redirect: RefCell::new(None),
            redirect_count: RefCell::new(0),
        })
    }

    // Setters

    pub fn set_status(&self, status: Status) -> &Self {
        self.status.replace(status);
        self
    }

    pub fn set_title(&self, title: &str) -> &Self {
        self.title.replace(GString::from(title));
        self
    }

    pub fn set_redirect(&self, request: GString, is_foreground: bool) -> &Self {
        self.redirect
            .replace(Some(Redirect::new(request, is_foreground)));
        self
    }

    pub fn set_redirect_count(&self, redirect_count: i8) -> &Self {
        self.redirect_count.replace(redirect_count);
        self
    }

    /* @TODO not in use
    pub fn unset_redirect(&self) -> &Self {
        self.redirect.replace(None);
        self
    } */

    // Getters

    pub fn status(&self) -> Status {
        self.status.borrow().clone()
    }

    pub fn title(&self) -> GString {
        self.title.borrow().clone()
    }

    pub fn redirect_count(&self) -> i8 {
        self.redirect_count.borrow().clone()
    }

    /// WARNING!
    ///
    /// This function **take** the `Redirect` without clone semantics
    pub fn take_redirect(&self) -> Option<Redirect> {
        self.redirect.take()
    }
}
