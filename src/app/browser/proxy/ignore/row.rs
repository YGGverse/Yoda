use gtk::{
    Align, Box, Button, Entry, Switch,
    glib::{DateTime, GString},
    prelude::{BoxExt, ButtonExt, EditableExt, WidgetExt},
};

pub struct Row {
    pub id: Option<i64>,
    host: Entry,
    status: Switch,
    pub time: DateTime,
    pub widget: Box,
}

impl Row {
    // Constructors

    pub fn build(
        id: Option<i64>,
        time: Option<&DateTime>,
        host: Option<&str>,
        is_enabled: bool,
        on_delete: impl Fn() + 'static,
    ) -> Self {
        // Init components

        let status = Switch::builder()
            .active(is_enabled)
            .valign(Align::Center)
            .build();

        let host = Entry::builder()
            .hexpand(true)
            .placeholder_text("Hostname, *pattern or IP address")
            .text(host.unwrap_or_default())
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
        widget.append(&host);
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
                    .heading("Delete this exception?")
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

        host.connect_changed(move |this| {
            validate(this);
        });

        status.connect_state_set({
            let host = host.clone();
            move |_, state| {
                validate(&host);

                host.set_sensitive(state);

                gtk::glib::Propagation::Proceed
            }
        });

        Self {
            id,
            status,
            time: time.cloned().unwrap_or(DateTime::now_local().unwrap()),
            host,
            widget,
        }
    }

    // Actions

    pub fn validate(&self) -> bool {
        validate(&self.host)
    }

    // Getters

    pub fn host(&self) -> GString {
        self.host.text()
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
            .tooltip_text("Add hostname or IP address exception")
            .build();

        add.connect_clicked(move |_| on_add());
        add
    });
    b
}

fn validate(host: &Entry) -> bool {
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

    fn validate_host(value: &str) -> Result<(), String> {
        // https://docs.gtk.org/gio/property.SimpleProxyResolver.ignore-hosts.html
        match gtk::gio::NetworkAddress::parse(
            value.trim_start_matches('.').trim_start_matches('*'),
            0,
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Valid hostname or IP address is required: `{e}`")),
        }
    }

    let v = validate_host(&host.text());
    let is_valid_host = v.is_ok();
    highlight(host, v);

    is_valid_host
}
