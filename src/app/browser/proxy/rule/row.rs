use gtk::{
    Align, Box, Button, Entry, Switch,
    glib::{DateTime, GString},
    prelude::{BoxExt, ButtonExt, EditableExt, WidgetExt},
};

pub struct Row {
    pub id: Option<i64>,
    priority: Entry,
    request: Entry,
    status: Switch,
    url: Entry,
    pub time: DateTime,
    pub widget: Box,
}

impl Row {
    // Constructors

    pub fn build(
        id: Option<i64>,
        time: Option<&DateTime>,
        request: Option<&str>,
        url: Option<&str>,
        priority: Option<i32>,
        is_enabled: bool,
        on_delete: impl Fn() + 'static,
    ) -> Self {
        // Init components

        let status = Switch::builder()
            .active(is_enabled)
            .valign(Align::Center)
            .build();

        let request = Entry::builder()
            .max_width_chars(12)
            .placeholder_text("Request")
            .tooltip_text("Supports regex expressions")
            .text(request.unwrap_or(".*"))
            .build();

        let url = Entry::builder()
            .hexpand(true)
            .placeholder_text("Proxy URL")
            .text(url.unwrap_or_default())
            .tooltip_text("e.g. socks5://127.0.0.1:1080")
            .build();

        let priority = Entry::builder()
            .max_width_chars(1)
            .placeholder_text("Priority")
            .text(priority.unwrap_or(0).to_string())
            .tooltip_text("Apply in priority")
            .build();

        let delete = Button::builder()
            .css_classes(["error"])
            .icon_name("user-trash-symbolic")
            .tooltip_text("Delete")
            .build();

        // Init widget

        let widget = Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(8)
            .build();

        widget.append(&status);
        widget.append(&request);
        widget.append(&url);
        widget.append(&priority);
        widget.append(&delete);

        // Activate

        delete.connect_clicked({
            let c = std::rc::Rc::new(on_delete);
            move |this| {
                use adw::{
                    AlertDialog, ResponseAppearance,
                    prelude::{AdwDialogExt, AlertDialogExt, AlertDialogExtManual},
                };

                const RESPONSE_CONFIRM: (&str, &str) = ("confirm", "Confirm");
                const RESPONSE_CANCEL: (&str, &str) = ("cancel", "Cancel");

                let dialog = AlertDialog::builder()
                    .heading("Delete this rule?")
                    .close_response(RESPONSE_CANCEL.0)
                    .default_response(RESPONSE_CONFIRM.0)
                    .build();

                dialog.add_responses(&[RESPONSE_CANCEL, RESPONSE_CONFIRM]);
                dialog.set_response_appearance(RESPONSE_CONFIRM.0, ResponseAppearance::Destructive);
                dialog.connect_response(None, {
                    let c = c.clone();
                    move |dialog, response| {
                        dialog.set_response_enabled(response, false); // prevent double-click
                        if response == RESPONSE_CONFIRM.0 {
                            c()
                        }
                    }
                });
                dialog.present(Some(this))
            }
        });

        priority.connect_changed({
            let url = url.clone();
            move |this| {
                validate(this, &url);
            }
        });

        url.connect_changed({
            let priority = priority.clone();
            move |this| {
                validate(&priority, this);
            }
        });

        status.connect_state_set({
            let priority = priority.clone();
            let request = request.clone();
            let url = url.clone();
            move |_, state| {
                validate(&priority, &url);

                priority.set_sensitive(state);
                request.set_sensitive(state);
                url.set_sensitive(state);

                gtk::glib::Propagation::Proceed
            }
        });

        Self {
            id,
            priority,
            request,
            status,
            time: time.cloned().unwrap_or(DateTime::now_local().unwrap()),
            url,
            widget,
        }
    }

    // Actions

    pub fn validate(&self) -> bool {
        validate(&self.priority, &self.url)
    }

    // Getters

    pub fn priority(&self) -> i32 {
        self.priority.text().parse::<i32>().unwrap_or_default()
    }

    pub fn request(&self) -> GString {
        self.request.text()
    }

    pub fn url(&self) -> GString {
        self.url.text()
    }

    pub fn is_enabled(&self) -> bool {
        self.status.is_active()
    }
}

pub fn new(on_add: impl Fn() + 'static) -> Box {
    let b = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(8)
        .build();

    b.append(&{
        let add = Button::builder()
            .css_classes(["success"])
            .hexpand(true)
            .icon_name("list-add-symbolic")
            .tooltip_text("Add proxy")
            .build();

        add.connect_clicked(move |_| on_add());
        add
    });
    b
}

fn validate(priority: &Entry, url: &Entry) -> bool {
    fn highlight(entry: &Entry, error: Result<(), String>) {
        const E: &str = "error";
        match error {
            Err(e) => {
                entry.set_css_classes(&[E]);
                entry.set_tooltip_text(Some(&e))
            }
            Ok(()) => {
                entry.remove_css_class(E);
                entry.set_tooltip_text(Some("Value is valid"))
            }
        }
    }

    fn validate_priority(value: &str) -> Result<(), String> {
        if value.parse::<i32>().is_err() {
            Err("Priority value is not valid integer".to_string())
        } else {
            Ok(())
        }
    }

    fn validate_url(value: &str) -> Result<(), String> {
        match gtk::glib::Uri::parse(value, gtk::glib::UriFlags::NONE) {
            Ok(uri) => {
                if uri.scheme().is_empty() {
                    Err("Scheme is empty".to_string())
                } else if uri.host().is_none_or(|h| h.is_empty()) {
                    Err("Host is required".to_string())
                } else if uri.port() == -1 {
                    Err("Port is required".to_string())
                } else if !uri.path().is_empty() {
                    Err("URL should not contain the path part".to_string())
                } else if uri.query().is_some() {
                    Err("URL should not contain the query part".to_string())
                } else if uri.fragment().is_some() {
                    Err("URL should not contain the fragment (anchor) part".to_string())
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }

    let v = validate_priority(&priority.text());
    let is_valid_priority = v.is_ok();
    highlight(priority, v);

    let v = validate_url(&url.text());
    let is_valid_url = v.is_ok();
    highlight(url, v);

    is_valid_priority && is_valid_url
}
