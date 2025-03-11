use gtk::{
    gio::subclass::prelude::{DerivedObjectProperties, ObjectImpl, ObjectImplExt, ObjectSubclass},
    glib::{self, GString, Object, Properties},
    prelude::ObjectExt,
};
use std::cell::{Cell, RefCell};

#[derive(Properties, Default)]
#[properties(wrapper_type = super::Item)]
pub struct Item {
    #[property(get, set)]
    title: RefCell<GString>,
    #[property(get, set)]
    subtitle: RefCell<GString>,
    #[property(get, set)]
    request: RefCell<GString>,
    #[property(get, set)]
    has_bookmark: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for Item {
    const NAME: &'static str = "SuggestionItem"; // @TODO make globally unique
    type Type = super::Item;
    type ParentType = Object;
}

#[glib::derived_properties]
impl ObjectImpl for Item {
    fn constructed(&self) {
        self.parent_constructed();
    }
}
