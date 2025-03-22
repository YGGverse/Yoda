use super::Info;
use adw::{
    ActionRow, PreferencesDialog, PreferencesGroup, PreferencesPage,
    prelude::{ActionRowExt, PreferencesDialogExt, PreferencesGroupExt, PreferencesPageExt},
};
use gtk::glib::gformat;

pub trait Dialog {
    fn info(info: &Info) -> Self;
}

impl Dialog for PreferencesDialog {
    fn info(info: &Info) -> Self {
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
            if info.size.is_some() {
                p.add(&{
                    let g = PreferencesGroup::builder().title("Size").build();
                    if let Some(ref size) = info.size {
                        g.add(&{
                            use crate::tool::Format;
                            ActionRow::builder()
                                .css_classes(["property"])
                                .subtitle_selectable(true)
                                .subtitle(size.bytes())
                                .title_selectable(true)
                                .title("Content")
                                .build()
                        })
                    }
                    g
                });
            } // @TODO header size, total size, etc.
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
                                    .valign(gtk::Align::Center)
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
            }) // @TODO clickable navigation, copy
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
                    for e in &info.event[1..] {
                        g.add(
                            &ActionRow::builder()
                                .subtitle(gformat!(
                                    "{} ms",
                                    e.time().difference(t).as_milliseconds()
                                ))
                                .subtitle_selectable(true)
                                .title_selectable(true)
                                .title(e.name())
                                .build(),
                        )
                    }
                    g
                });
                p
            })
        }
        d
    }
}
