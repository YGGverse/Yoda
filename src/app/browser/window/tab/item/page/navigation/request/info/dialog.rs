use super::{Info, Profile};
use adw::{
    ActionRow, PreferencesDialog, PreferencesGroup, PreferencesPage,
    prelude::{
        ActionRowExt, ExpanderRowExt, PreferencesDialogExt, PreferencesGroupExt, PreferencesPageExt,
    },
};
use gtk::{Align, glib::GString};

pub trait Dialog {
    fn info(profile: &Profile, info: &Info) -> Self;
}

impl Dialog for PreferencesDialog {
    fn info(profile: &Profile, info: &Info) -> Self {
        /// Common `ActionRow` widget pattern
        fn row(title: impl Into<GString>, subtitle: impl Into<GString>) -> ActionRow {
            ActionRow::builder()
                .css_classes(["property"])
                .subtitle_selectable(true)
                .subtitle(subtitle)
                .title_selectable(true)
                .title(title)
                .use_markup(true)
                .use_markup(true)
                .build()
        }
        let d = PreferencesDialog::builder()
            .search_enabled(true)
            .title("Page info")
            .build();
        d.add(&{
            let p = PreferencesPage::builder()
                .title("General")
                .icon_name("help-about-symbolic")
                .build();
            if info.mime.is_some() {
                p.add(&{
                    let g = PreferencesGroup::builder().title("Meta").build();
                    if let Some(ref mime) = info.mime {
                        g.add(&row("Content type", mime))
                    }
                    g
                });
            } // @TODO content language, header size, etc.
            if info.size.is_some() || info.header.is_some() {
                p.add(&{
                    use crate::tool::Format;
                    let g = PreferencesGroup::builder().title("Size").build();
                    let mut i = 0; // count group members
                    let mut t = 0; // count total size
                    if let Some(ref h) = info.header {
                        let l = h.len();
                        i += 1;
                        t += l;
                        g.add(&{
                            let e = adw::ExpanderRow::builder()
                                .enable_expansion(true)
                                .expanded(false)
                                .subtitle(l.bytes())
                                .title_selectable(true)
                                .title("Header")
                                .build();
                            e.add_row(
                                &ActionRow::builder()
                                    .css_classes(["property"])
                                    .title_selectable(true)
                                    .title(h.escape_default().to_string()) // escape \r\n
                                    .build(),
                            );
                            {
                                use gtk::prelude::{ListBoxRowExt, WidgetExt};
                                e.child().map(|c| {
                                    c.first_child().map(|c| {
                                        c.first_child().map_or_else(
                                            || println!("Deprecated child order!"),
                                            |c| c.add_css_class("property"),
                                        )
                                    })
                                }); // @TODO unstable!
                            }
                            e
                        })
                    }
                    if let Some(ref c) = info.size {
                        i += 1;
                        t += c;
                        g.add(&row("Content", c.bytes()))
                    }
                    if i > 1 && t > 0 {
                        g.add(&row("Total", t.bytes()))
                    }
                    g
                });
            } // @TODO header size, total size, etc.
            p
        });
        d.add(&{
            let p = PreferencesPage::builder()
                .title("Connection")
                .icon_name("network-transmit-receive")
                .build();
            if let Some(ref socket) = info.socket {
                use gtk::{
                    gio::{SocketAddress, SocketFamily},
                    prelude::{SocketAddressExt, SocketConnectableExt},
                };
                /// Convert socket family to string
                fn f2s(socket_family: &SocketFamily) -> &str {
                    match socket_family {
                        SocketFamily::Invalid => "Invalid",
                        SocketFamily::Unix => "Unix",
                        SocketFamily::Ipv4 => "IPv4",
                        SocketFamily::Ipv6 => "IPv6",
                        _ => panic!(),
                    }
                }
                /// Lookup [MaxMind](https://www.maxmind.com) database
                fn l(profile: &Profile, socket_address: &SocketAddress) -> Option<String> {
                    use maxminddb::{
                        MaxMindDBError, Reader,
                        geoip2::{/*City,*/ Country},
                    };
                    if !matches!(
                        socket_address.family(),
                        SocketFamily::Ipv4 | SocketFamily::Ipv6,
                    ) {
                        return None;
                    }
                    let db = {
                        let mut c = profile.config_path.clone();
                        c.push("GeoLite2-Country.mmdb");
                        Reader::open_readfile(c)
                    }
                    .ok()?;
                    let lookup = {
                        let a: std::net::SocketAddr = socket_address.to_string().parse().unwrap();
                        let lookup: std::result::Result<Country, MaxMindDBError> =
                            db.lookup(a.ip());
                        lookup
                    }
                    .ok()?;
                    lookup.country.map(|c| {
                        let mut b = Vec::new();
                        if let Some(iso_code) = c.iso_code {
                            b.push(iso_code)
                        }
                        if let Some(n) = c.names {
                            if let Some(s) = n.get("en") {
                                b.push(s)
                            } // @TODO multi-lang
                        }
                        // @TODO city DB
                        b.join(", ")
                    })
                }
                p.add(&{
                    let g = PreferencesGroup::builder().title("Remote").build();
                    g.add(&row("Address", socket.remote_address.to_string()));
                    g.add(&row("Family", f2s(&socket.remote_address.family())));
                    if let Some(location) = l(profile, &socket.remote_address) {
                        g.add(&row("Location", location))
                    }
                    g
                });
                p.add(&{
                    let g = PreferencesGroup::builder().title("Local").build();
                    g.add(&row("Address", socket.local_address.to_string()));
                    g.add(&row("Family", f2s(&socket.local_address.family())));
                    if let Some(location) = l(profile, &socket.local_address) {
                        g.add(&row("Location", location));
                    }
                    g
                });
            }
            p
        });
        if info.redirect.is_some() {
            d.add(&{
                let g = PreferencesGroup::new();
                let p = PreferencesPage::builder()
                    .title("Redirect")
                    .icon_name("insert-link-symbolic")
                    .build();
                p.add(&{
                    use gtk::Button;
                    /// Common suffix widget pattern
                    fn suffix(
                        icon_name: impl Into<GString>,
                        tooltip_text: impl Into<GString>,
                    ) -> Button {
                        Button::builder()
                            .css_classes(["flat"])
                            .icon_name(icon_name)
                            .tooltip_text(tooltip_text)
                            .sensitive(false)
                            .valign(Align::Center)
                            .halign(Align::Center)
                            .build()
                    }
                    /// Recursively collect redirection members into the given vector
                    fn chain<'a>(b: &mut Vec<&'a Info>, i: &'a Info) {
                        b.push(i);
                        if let Some(ref r) = i.redirect {
                            chain(b, &r.info)
                        }
                    }
                    // Collect redirections into the buffer,
                    // to reverse chain before add its members to widget
                    // * capacity optimized for Gemini protocol (as default)
                    let mut b = Vec::with_capacity(5);
                    chain(&mut b, info);
                    b.reverse();
                    let l = b.len(); // calculate once
                    let t = b[0].event[0].time(); // first event time to count from
                    for (i, r) in b.iter().enumerate() {
                        g.add(&{
                            let a = ActionRow::builder()
                                .css_classes(["property"])
                                .subtitle_selectable(true)
                                .title_selectable(true)
                                .title(r.request().unwrap())
                                .build();
                            a.add_prefix(&{
                                let c = i + 1;
                                let (css_class, tooltip_text) = if r
                                    .redirect
                                    .as_ref()
                                    .is_some_and(|this| this.is_external(r).is_some_and(|v| v))
                                {
                                    if c == l {
                                        ("warning", "Current (External)")
                                    } else {
                                        (
                                            "warning",
                                            if i == 0 {
                                                "Initial request"
                                            } else {
                                                "External redirect"
                                            },
                                        )
                                    }
                                } else if c == l {
                                    ("success", "Current")
                                } else {
                                    (
                                        "accent",
                                        if i == 0 {
                                            "Initial request"
                                        } else {
                                            "Internal redirect"
                                        },
                                    )
                                };
                                Button::builder()
                                    .css_classes(["circular", css_class])
                                    .halign(Align::Center)
                                    .label(c.to_string())
                                    .sensitive(false)
                                    .tooltip_text(tooltip_text)
                                    .valign(Align::Center)
                                    .build()
                            });
                            if let Some(ref redirect) = r.redirect {
                                a.add_suffix(&suffix(
                                    redirect.method.icon_name(),
                                    redirect.method.to_string(),
                                ))
                            }
                            // calculate total redirections time in ms
                            let c = r.event.last().unwrap().time();
                            a.set_subtitle(&if i == 0 {
                                format!("{} ms", c.difference(t).as_milliseconds())
                            } else {
                                format!(
                                    "+{} / {} ms",
                                    c.difference(b[i - 1].event.last().unwrap().time())
                                        .as_milliseconds(),
                                    c.difference(t).as_milliseconds()
                                )
                            });
                            a
                        })
                    }
                    g
                });
                p
            }) // @TODO clickable navigation, test time values
        }
        if !info.event.is_empty() {
            d.add(&{
                let p = PreferencesPage::builder()
                    .title("Events")
                    .icon_name("system-run-symbolic")
                    .build();
                p.add(&{
                    let g = PreferencesGroup::new();
                    let e = &info.event[0];
                    let t = e.time();
                    let n = e.name();
                    g.add(&row(n, t.format_iso8601().unwrap()));
                    for (i, e) in info.event[1..].iter().enumerate() {
                        g.add(&{
                            let c = e.time().difference(info.event[i].time()).as_milliseconds(); // current
                            let s = e.time().difference(t).as_milliseconds(); // sum
                            let a = row(
                                e.name(),
                                if c == s {
                                    format!("+{c} ms")
                                } else {
                                    format!("+{c} / {s} ms")
                                },
                            );
                            a
                        })
                    }
                    g
                });
                p
            })
        }
        d
    }
}
