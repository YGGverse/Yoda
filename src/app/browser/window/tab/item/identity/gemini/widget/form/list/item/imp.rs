//! Custom `GObject` implementation for dropdown
//! [ListStore](https://docs.gtk.org/gio/class.ListStore.html) menu item

use gtk::{
    gio::subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectImplExt, ObjectSubclass},
    glib::{self, Object, Properties},
    prelude::ObjectExt,
};
use std::cell::{Cell, RefCell};

#[derive(Properties, Default)]
#[properties(wrapper_type = super::Item)]
pub struct Item {
    #[property(get, set)]
    value: Cell<i64>,
    #[property(get, set)]
    title: RefCell<String>,
    #[property(get, set)]
    subtitle: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for Item {
    const NAME: &str = "Item"; // @TODO make globally unique
    type Type = super::Item;
    type ParentType = Object;
}

#[glib::derived_properties]
impl ObjectImpl for Item {
    fn constructed(&self) {
        self.parent_constructed();
    }
}
