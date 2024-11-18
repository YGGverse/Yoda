mod item;
use item::Item;

use gtk::{
    gio::{
        prelude::{Cast, CastNone},
        ListStore,
    },
    prelude::ListItemExt,
    DropDown, Label, ListItem, SignalListItemFactory,
};

pub struct List {
    pub gobject: DropDown,
    model: ListStore,
}

impl List {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        // Init model with custom `GObject` properties
        let model = ListStore::new::<Item>();

        // Setup item factory to append items after `DropDown` init
        let factory = SignalListItemFactory::new();

        // @TODO factory.connect_setup(move |_, gobject| {});
        factory.connect_bind(move |_, gobject| {
            // Cast components
            let list_item = gobject.downcast_ref::<ListItem>().unwrap();
            let item = list_item.item().and_downcast::<Item>().unwrap();

            // Update menu item
            list_item.set_child(Some(&Label::new(Some(&item.label()))));

            // @TODO
            println!("{:?}", item.profile_identity_gemini_id_option());
            println!("{:?}", item.label());
            println!("{:?}", item.is_enabled());
        });

        // Init list `GObject`
        let gobject = DropDown::builder().model(&model).factory(&factory).build();

        // Return activated `Self`
        Self { model, gobject }
    }

    // Actions

    /// Append new item with `profile_identity_gemini_id` as `key` and label as `value`
    pub fn append(&self, profile_identity_gemini_id: Option<i64>, label: &str, is_enabled: bool) {
        self.model
            .append(&Item::new(profile_identity_gemini_id, label, is_enabled));
    }

    // Events

    /// Run callback function on `connect_selected_notify` event
    /// * return formatted `profile_identity_gemini_id` match selected
    pub fn connect_selected_notify(&self, callback: impl Fn(Option<i64>) + 'static) {
        self.gobject.connect_selected_notify(move |list| {
            callback(
                list.selected_item()
                    .and_downcast::<Item>()
                    .unwrap()
                    .profile_identity_gemini_id_option(),
            )
        });
    }
}
