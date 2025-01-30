pub mod item;

use crate::profile::Profile;
use gtk::{
    gio::{
        prelude::{Cast, CastNone},
        ListStore,
    },
    prelude::{BoxExt, ListItemExt, ObjectExt, WidgetExt},
    Align, Box, DropDown, Label, ListItem, Orientation, SignalListItemFactory,
};
pub use item::Item;
use std::rc::Rc;

pub struct List {
    pub dropdown: DropDown,
    list_store: ListStore,
}

impl List {
    // Constructors

    /// Create new `Self`
    pub fn build(profile: &Rc<Profile>) -> Self {
        // Init dropdown items
        let new_search_provider = Item::add();

        // Init model
        let list_store = ListStore::new::<Item>();

        list_store.append(&new_search_provider);
        for record in profile.search.records() {
            list_store.append(&Item::profile_search_id(
                record.id,
                &record.query,
                record.is_default,
            ))
        }

        // Setup item factory
        // * wanted only to append items after `DropDown` init
        let factory = SignalListItemFactory::new();

        factory.connect_setup(|_, this| {
            // Init widget for dropdown item
            // * legacy container, exists because maybe some other elements will be added later
            let child = Box::builder()
                .orientation(Orientation::Vertical)
                .valign(Align::Center)
                .build();

            // Title
            child.append(&Label::new(None));

            // Done
            this.downcast_ref::<ListItem>()
                .unwrap()
                .set_child(Some(&child));
        });

        factory.connect_bind(|_, this| {
            // Downcast requirements
            let list_item = this.downcast_ref::<ListItem>().unwrap();
            let item = list_item.item().and_downcast::<Item>().unwrap();
            let child = list_item.child().and_downcast::<Box>().unwrap();

            // Bind `title`
            match child.first_child().and_downcast::<Label>() {
                Some(label) => {
                    label.set_label(&item.title());
                    label.set_css_classes(if item.is_default() { &["accent"] } else { &[] });
                    item.bind_property("title", &label, "label").build(); // sync label
                    item.bind_property("is-default", &label, "css-classes")
                        .transform_to(|_, is_default| {
                            if is_default {
                                Some(vec!["accent".to_string()])
                            } else {
                                Some(vec![])
                            }
                        })
                        .build(); // sync class by status
                }
                None => todo!(),
            }
        });

        // Init main widget
        let dropdown = DropDown::builder()
            .model(&list_store)
            .selected(
                list_store
                    .find_with_equal_func(|item| {
                        item.dynamic_cast_ref::<Item>().unwrap().is_default()
                    })
                    .unwrap_or_default(),
            )
            .factory(&factory)
            .build();

        // Return activated `Self`
        Self {
            dropdown,
            list_store,
        }
    }

    // Actions

    /// Find list item by `profile_search_id`
    /// * return `position` found
    pub fn find(&self, profile_search_id: i64) -> Option<u32> {
        self.list_store.find_with_equal_func(|this| {
            profile_search_id == this.downcast_ref::<Item>().unwrap().value()
        })
    }

    /// Remove list item by `profile_search_id`
    /// * return `position` of removed list item
    pub fn remove(&self, profile_search_id: i64) -> Option<u32> {
        match self.find(profile_search_id) {
            Some(position) => {
                self.list_store.remove(position);
                Some(position)
            }
            None => todo!(),
        }
    }

    // Getters

    /// Get selected `Item` GObject
    pub fn selected(&self) -> Item {
        self.dropdown
            .selected_item()
            .and_downcast::<Item>()
            .unwrap()
    }
}
