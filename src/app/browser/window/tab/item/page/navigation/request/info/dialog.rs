use super::{Info, Profile};
use adw::{
    ActionRow, PreferencesDialog, PreferencesGroup, PreferencesPage,
    prelude::{
        ActionRowExt, ExpanderRowExt, PreferencesDialogExt, PreferencesGroupExt, PreferencesPageExt,
    },
};
use gtk::{Align, glib::gformat};

pub trait Dialog {
    fn info(profile: &Profile, info: &Info) -> Self;
}

impl Dialog for PreferencesDialog {
    fn info(profile: &Profile, info: &Info) -> Self {
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
                        g.add(
                            &ActionRow::builder()
                                .css_classes(["property"])
                                .subtitle_selectable(true)
                                .subtitle(mime)
                                .title_selectable(true)
                                .title("Content type")
                                .build(),
                        )
                    }
                    g
                });
            } // @TODO content language, header size, etc.
            if info.size.is_some() || info.header.is_some() {
                p.add(&{
                    use crate::tool::Format;
                    /// Common `ActionRow` widget pattern
                    fn r(title: &str, subtitle: String) -> ActionRow {
                        ActionRow::builder()
                            .css_classes(["property"])
                            .subtitle_selectable(true)
                            .subtitle(subtitle)
                            .title_selectable(true)
                            .title(title)
                            .build()
                    }
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
                        g.add(&r("Content", c.bytes()))
                    }
                    if i > 1 && t > 0 {
                        g.add(&r("Total", t.bytes()))
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
                /// Common `ActionRow` widget pattern
                fn r(title: &str, subtitle: &str) -> ActionRow {
                    ActionRow::builder()
                        .css_classes(["property"])
                        .subtitle_selectable(true)
                        .subtitle(subtitle)
                        .title_selectable(true)
                        .title(title)
                        .build()
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
                    g.add(&r("Address", &socket.remote_address.to_string()));
                    g.add(&r("Family", f2s(&socket.remote_address.family())));
                    if let Some(location) = l(profile, &socket.remote_address) {
                        g.add(&r("Location", &location));
                    }
                    g
                });
                p.add(&{
                    let g = PreferencesGroup::builder().title("Local").build();
                    g.add(&r("Address", &socket.local_address.to_string()));
                    g.add(&r("Family", f2s(&socket.local_address.family())));
                    if let Some(location) = l(profile, &socket.local_address) {
                        g.add(&r("Location", &location));
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
                    // Collect redirections into the buffer,
                    // to reverse chain before add its members to widget
                    // * capacity optimized for Gemini protocol (as default)
                    let mut b = Vec::with_capacity(5);
                    /// Recursively collect redirection members into the given vector
                    fn chain<'a>(b: &mut Vec<&'a Info>, i: &'a Info) {
                        b.push(i);
                        if let Some(ref r) = i.redirect {
                            chain(b, r)
                        }
                    }
                    chain(&mut b, info);
                    b.reverse();
                    let l = b.len(); // calculate once
                    let t = b[0].event[0].time(); // first event time to count from
                    for (i, r) in b.iter().enumerate() {
                        g.add(&{
                            let a = ActionRow::builder()
                                .subtitle_selectable(true)
                                .title_selectable(true)
                                .title(r.request().unwrap())
                                .build();
                            // show redirections counter
                            a.add_prefix(&{
                                let c = i + 1;
                                gtk::Button::builder()
                                    .css_classes([
                                        "circular",
                                        if c == l { "success" } else { "accent" },
                                    ])
                                    .label(c.to_string())
                                    .sensitive(false)
                                    .valign(Align::Center)
                                    .halign(Align::Center)
                                    .build()
                            });
                            // show total redirection time in ms
                            a.set_subtitle(&if i == 0 {
                                t.format_iso8601().unwrap()
                            } else {
                                gformat!(
                                    "{} ms",
                                    r.event
                                        .last()
                                        .unwrap()
                                        .time()
                                        .difference(t)
                                        .as_milliseconds()
                                )
                            });
                            a
                        });
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
                    g.add(
                        &ActionRow::builder()
                            .subtitle_selectable(true)
                            .subtitle(t.format_iso8601().unwrap())
                            .title_selectable(true)
                            .title(n)
                            .build(),
                    );
                    for (i, e) in info.event[1..].iter().enumerate() {
                        g.add(&{
                            use gtk::{Align, Label};
                            let c = e.time().difference(info.event[i].time()).as_milliseconds();
                            let a = ActionRow::builder()
                                .use_markup(true)
                                .subtitle(gformat!(
                                    "{} ms",
                                    e.time().difference(t).as_milliseconds()
                                ))
                                .subtitle_selectable(true)
                                .title_selectable(true)
                                .title(e.name())
                                .build();
                            a.add_suffix(
                                &Label::builder()
                                    .css_classes([
                                        "flat",
                                        if c == 0 { "success" } else { "warning" },
                                    ])
                                    .halign(Align::End)
                                    .label(if c > 0 {
                                        format!("+{c} ms")
                                    } else {
                                        c.to_string()
                                    })
                                    .sensitive(false)
                                    .valign(Align::Center)
                                    .build(),
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
