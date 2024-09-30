use gtk::{
    gio::SimpleAction,
    glib::{gformat, GString, Uri},
    prelude::{ActionExt, ButtonExt, WidgetExt},
    Button,
};
use std::{cell::RefCell, sync::Arc};

pub struct Base {
    // Actions
    action_tab_page_navigation_base: Arc<SimpleAction>,
    // Mutable URI cache (parsed on update)
    uri: RefCell<Option<Uri>>,
    // GTK
    widget: Button,
}

impl Base {
    // Construct
    pub fn new(action_tab_page_navigation_base: Arc<SimpleAction>) -> Self {
        // Init widget
        let widget = Button::builder()
            .icon_name("go-home-symbolic")
            .tooltip_text("Base")
            .sensitive(false)
            .build();

        // Init events
        widget.connect_clicked({
            let action_tab_page_navigation_base = action_tab_page_navigation_base.clone();
            move |_| {
                action_tab_page_navigation_base.activate(None);
            }
        });

        // Return activated struct
        Self {
            action_tab_page_navigation_base,
            uri: RefCell::new(None),
            widget,
        }
    }

    // Actions
    pub fn update(&self, uri: Option<Uri>) {
        // Update sensitivity
        let status = match &uri {
            Some(uri) => "/" != uri.path(),
            None => false,
        };

        self.action_tab_page_navigation_base.set_enabled(status);
        self.widget.set_sensitive(status);

        // Update parsed cache
        self.uri.replace(uri);
    }

    // Getters
    pub fn widget(&self) -> &Button {
        &self.widget
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
