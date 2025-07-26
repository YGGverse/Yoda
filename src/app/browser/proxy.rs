//! Proxy settings dialog

mod ignore;
mod misc;
mod rule;

use super::Profile;
use adw::{
    PreferencesGroup, PreferencesPage,
    prelude::{AdwDialogExt, PreferencesDialogExt, PreferencesGroupExt, PreferencesPageExt},
};
use ignore::Ignore;
use misc::Misc;
use rule::Rule;
use std::rc::Rc;

pub trait Proxy {
    fn proxy(profile: &Rc<Profile>) -> Self;
}

impl Proxy for adw::PreferencesDialog {
    fn proxy(profile: &Rc<Profile>) -> Self {
        // Init components
        let ignore = Ignore::build(profile);
        let misc = Misc::build(profile);
        let rule = Rule::build(profile);

        // Init widget
        let d = adw::PreferencesDialog::builder()
            .search_enabled(true)
            .title("Proxy")
            .build();

        d.add(&{
            let p = PreferencesPage::builder()
                .title("Rules")
                .icon_name("system-run-symbolic")
                .build();
            p.add(&{
                let g = PreferencesGroup::new();
                g.add(&rule.widget);
                g
            });
            p
        });

        d.add(&{
            let p = PreferencesPage::builder()
                .title("Exceptions")
                .icon_name("action-unavailable-symbolic")
                .build();
            p.add(&{
                let g = PreferencesGroup::new();
                g.add(&ignore.widget);
                g
            });
            p
        });

        d.add(&{
            let p = PreferencesPage::builder()
                .title("Interface")
                .icon_name("preferences-desktop-display-symbolic")
                .build();
            p.add(&misc.widget);
            p
        });

        d.connect_closed({
            let profile = profile.clone();
            move |_| {
                profile.proxy.rule.clear();
                for r in rule.take() {
                    if r.validate() {
                        profile.proxy.rule.add(
                            r.id,
                            r.is_enabled(),
                            r.priority(),
                            r.request().to_string(),
                            r.url().to_string(),
                            r.time,
                        )
                    }
                }

                profile.proxy.ignore.clear();
                for i in ignore.take() {
                    if i.validate() {
                        profile
                            .proxy
                            .ignore
                            .add(i.id, i.is_enabled(), i.host().to_string(), i.time)
                    }
                }

                profile
                    .proxy
                    .misc
                    .set_highlight_request_entry(misc.is_highlight_request_entry());
            }
        });
        d
    }
}
