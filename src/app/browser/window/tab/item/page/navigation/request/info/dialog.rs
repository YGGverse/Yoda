use super::Info;
use adw::{
    ActionRow, PreferencesDialog, PreferencesGroup, PreferencesPage,
    prelude::{PreferencesDialogExt, PreferencesGroupExt, PreferencesPageExt},
};

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
                        g.add(&{
                            let r = ActionRow::builder()
                                .css_classes(["property"])
                                .subtitle(mime)
                                .title("Content type")
                                .build();
                            r
                        })
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
                            let r = ActionRow::builder()
                                .css_classes(["property"])
                                .subtitle(size.bytes())
                                .title("Content")
                                .build();
                            r
                        })
                    }
                    g
                });
            } // @TODO header size, total size, etc.
            p
        });
        if let Some(ref redirect) = info.redirect {
            d.add(&{
                PreferencesPage::builder()
                    .title("Redirect")
                    .icon_name("insert-link-symbolic")
                    .build()
            });
            // @TODO recursive lookup
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
                    g.add(&{
                        let r = ActionRow::builder()
                            .subtitle(t.format_iso8601().unwrap())
                            .title(n)
                            .build();
                        r
                    });
                    for e in &info.event[1..] {
                        g.add(&{
                            let r = ActionRow::builder()
                                .subtitle(gtk::glib::gformat!(
                                    "{} ms",
                                    e.time().difference(t).as_milliseconds()
                                ))
                                .title(e.name())
                                .build();
                            r
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
