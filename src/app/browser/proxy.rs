//! Proxy settings dialog

mod rules;

use super::Profile;
use adw::{
    PreferencesGroup, PreferencesPage,
    prelude::{AdwDialogExt, PreferencesDialogExt, PreferencesGroupExt, PreferencesPageExt},
};
use rules::Rules;
use std::rc::Rc;

pub trait Proxy {
    fn proxy(profile: &Rc<Profile>) -> Self;
}

impl Proxy for adw::PreferencesDialog {
    fn proxy(profile: &Rc<Profile>) -> Self {
        // Init components
        let rules = Rules::build(profile);

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
                g.add(&rules.widget);
                g
            });
            /* @TODO URL entry p.add(&{
                let g = PreferencesGroup::builder().title("Test").build();
                //g.add(&Box::rules(profile));
                g
            });*/
            p
        });

        d.add(
            &PreferencesPage::builder()
                .title("Exceptions")
                .icon_name("action-unavailable-symbolic")
                .build(),
        );

        d.add(
            &PreferencesPage::builder()
                .title("Interface")
                .icon_name("preferences-desktop-display-symbolic")
                .build(),
        );

        d.connect_closed({
            let profile = profile.clone();
            move |_| {
                profile.proxy.clear();
                for rule in rules.take() {
                    if rule.validate() {
                        profile.proxy.add_rule(
                            rule.id,
                            rule.is_enabled(),
                            rule.priority(),
                            rule.request().to_string(),
                            rule.url().to_string(),
                            rule.time,
                        )
                    }
                }
            }
        });
        d
    }
}
