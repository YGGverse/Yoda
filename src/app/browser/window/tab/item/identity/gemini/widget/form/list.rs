pub mod item;
use item::{value::Value, Item};

use gtk::{
    gio::{
        prelude::{Cast, CastNone},
        ListStore,
    },
    prelude::{BoxExt, ListItemExt, WidgetExt},
    Align, Box, DropDown, Label, ListItem, SignalListItemFactory,
};

pub struct List {
    pub dropdown: DropDown,
    list_store: ListStore,
}

impl List {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init `ListStore` with custom `DropDown` properties
        let list_store = ListStore::new::<Item>();

        // Setup item factory
        // * wanted only to append items after `DropDown` init
        let factory = SignalListItemFactory::new();

        factory.connect_setup(|_, dropdown| {
            // Init widget for dropdown item
            let widget = Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .build();

            // Title
            widget.append(&Label::builder().halign(Align::Start).build());

            // Subtitle
            widget.append(
                &Label::builder()
                    .halign(Align::Start)
                    .css_classes(["caption", "dim-label"])
                    .build(),
            );

            // Done
            dropdown
                .downcast_ref::<ListItem>()
                .unwrap()
                .set_child(Some(&widget));
        });

        factory.connect_bind(|_, dropdown| {
            // Downcast requirements
            let list_item = dropdown.downcast_ref::<ListItem>().unwrap();
            let item = list_item.item().and_downcast::<Item>().unwrap();
            let container = list_item.child().and_downcast::<Box>().unwrap();

            // Update `title` (expected as the first child)
            container
                .first_child()
                .unwrap()
                .downcast::<Label>()
                .unwrap()
                .set_label(&item.title());

            // Update `subtitle` (expected as the last child)
            container
                .last_child()
                .unwrap()
                .downcast::<Label>()
                .unwrap()
                .set_label(&item.subtitle());
        });

        // Init main `DropDown`
        let dropdown = DropDown::builder()
            .model(&list_store)
            .factory(&factory)
            .build();

        // Return activated `Self`
        Self {
            list_store,
            dropdown,
        }
    }

    // Actions

    /// Append new item
    pub fn append(&self, value: Value, title: &str, subtitle: &str, is_selected: bool) {
        let item = Item::new(value, title, subtitle);
        self.list_store.append(&item);
        if is_selected {
            self.dropdown
                .set_selected(self.list_store.find(&item).unwrap()); // @TODO panic or handle?
        }
    }

    /// Find list item by `Value`
    /// * return `position` found
    pub fn find(&self, value: i64) -> Option<u32> {
        self.list_store
            .find_with_equal_func(|this| value == this.clone().downcast::<Item>().unwrap().value())
    }

    /// Remove list item by `Value`
    /// * return `position` of removed list item
    pub fn remove(&self, value: i64) -> Option<u32> {
        match self.find(value) {
            Some(position) => {
                self.list_store.remove(position);
                Some(position)
            }
            None => None,
        }
    }

    // Events

    /// Run callback function on `connect_selected_notify` event
    /// * return `Value` enum match selected item
    pub fn on_select(&self, callback: impl Fn(Value) + 'static) {
        self.dropdown.connect_selected_notify(move |list| {
            callback(
                list.selected_item()
                    .and_downcast::<Item>()
                    .unwrap()
                    .value_enum(),
            )
        });
    }

    // Getters

    /// Get formatted `value` match selected item
    pub fn selected(&self) -> Value {
        self.dropdown
            .selected_item()
            .and_downcast::<Item>()
            .unwrap()
            .value_enum()
    }
}
