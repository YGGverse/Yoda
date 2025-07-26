mod row;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::Profile;
use gtk::{
    Box,
    glib::{GString, uuid_string_random},
    prelude::BoxExt,
};
use row::Row;

pub struct Ignore {
    pub widget: Box,
    rows: Rc<RefCell<HashMap<GString, Row>>>,
}

impl Ignore {
    pub fn build(profile: &Rc<Profile>) -> Self {
        let config = profile.proxy.ignore.all();

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
                let row = Row::build(
                    proxy.id,
                    Some(&proxy.time),
                    Some(&proxy.host),
                    proxy.is_enabled,
                    {
                        let form = form.clone();
                        let key = key.clone();
                        let rows = rows.clone();
                        move || form.remove(&rows.borrow_mut().remove(&key).unwrap().widget)
                    },
                );
                row.validate();
                form.append(&row.widget);
                assert!(r.insert(key, row).is_none())
            }
        }

        let add = {
            let b = Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(8)
                .build();

            b.append(&row::new({
                let form = form.clone();
                let rows = rows.clone();
                move || {
                    let key = uuid_string_random();
                    let row = Row::build(None, None, None, false, {
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
