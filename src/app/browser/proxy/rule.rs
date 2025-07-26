mod row;

use super::Profile;
use gtk::{
    Box,
    glib::{GString, uuid_string_random},
    prelude::BoxExt,
};
use row::Row;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Rule {
    pub widget: Box,
    rows: Rc<RefCell<HashMap<GString, Row>>>,
}

impl Rule {
    pub fn build(profile: &Rc<Profile>) -> Self {
        let config = profile.proxy.rule.all();

        let rows: Rc<RefCell<HashMap<GString, Row>>> =
            Rc::new(RefCell::new(HashMap::with_capacity(config.len())));

        let form = Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(8)
            .build();

        {
            let mut r = rows.borrow_mut();

            for proxy in config {
                let key = uuid_string_random();
                let rule = Row::build(
                    proxy.id,
                    Some(&proxy.time),
                    Some(&proxy.request),
                    Some(&proxy.url),
                    Some(proxy.priority),
                    proxy.is_enabled,
                    {
                        let rows = rows.clone();
                        let key = key.clone();
                        let form = form.clone();
                        move || form.remove(&rows.borrow_mut().remove(&key).unwrap().widget)
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

            b.append(&row::new({
                let rows = rows.clone();
                let form = form.clone();
                move || {
                    let key = uuid_string_random();
                    let row = Row::build(None, None, None, None, None, false, {
                        let rows = rows.clone();
                        let key = key.clone();
                        let form = form.clone();
                        move || form.remove(&rows.borrow_mut().remove(&key).unwrap().widget)
                    });
                    form.append(&row.widget);
                    assert!(rows.borrow_mut().insert(key, row).is_none())
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

        Self { rows, widget }
    }

    pub fn take(&self) -> Vec<Row> {
        self.rows.take().into_values().collect()
    }
}
