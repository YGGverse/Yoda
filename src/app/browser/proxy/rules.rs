mod rule;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::Profile;
use gtk::{
    Box,
    glib::{GString, uuid_string_random},
    prelude::BoxExt,
};
use rule::Rule;

pub struct Rules {
    pub widget: Box,
    rules: Rc<RefCell<HashMap<GString, Rule>>>,
}

impl Rules {
    pub fn build(profile: &Rc<Profile>) -> Self {
        let config = profile.proxy.rules();

        let rules: Rc<RefCell<HashMap<GString, Rule>>> =
            Rc::new(RefCell::new(HashMap::with_capacity(config.len())));

        let form = Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(8)
            .build();

        {
            let mut r = rules.borrow_mut();

            for proxy in config {
                let key = uuid_string_random();
                let rule = Rule::build(
                    proxy.id,
                    Some(&proxy.time),
                    Some(&proxy.request),
                    Some(&proxy.url),
                    Some(proxy.priority),
                    proxy.is_enabled,
                    {
                        let rules = rules.clone();
                        let key = key.clone();
                        let form = form.clone();
                        move || form.remove(&rules.borrow_mut().remove(&key).unwrap().widget)
                    },
                );
                rule.validate();
                form.append(&rule.widget);
                assert!(r.insert(key, rule).is_none())
            }
        }

        let add = {
            let b = Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(8)
                .build();

            b.append(&rule::new({
                let rules = rules.clone();
                let form = form.clone();
                move || {
                    let key = uuid_string_random();
                    let rule = Rule::build(None, None, None, None, None, false, {
                        let rules = rules.clone();
                        let key = key.clone();
                        let form = form.clone();
                        move || form.remove(&rules.borrow_mut().remove(&key).unwrap().widget)
                    });
                    form.append(&rule.widget);
                    assert!(rules.borrow_mut().insert(key, rule).is_none())
                }
            }));
            b
        };

        let widget = Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(8)
            .build();

        widget.append(&form);
        widget.append(&add);

        Self { rules, widget }
    }

    pub fn take(&self) -> Vec<Rule> {
        self.rules.take().into_values().collect()
    }
}
