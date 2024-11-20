mod item;
use item::Item;

use adw::ActionRow;
use gtk::{
    gio::{
        prelude::{Cast, CastNone},
        ListStore,
    },
    prelude::ListItemExt,
    DropDown, ListItem, SignalListItemFactory,
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
            list_item.set_child(Some(
                &ActionRow::builder()
                    .title(item.title())
                    .subtitle(item.subtitle())
                    .build(),
            ));
        });

        // Init list `GObject`
        let gobject = DropDown::builder().model(&model).factory(&factory).build();

        // Return activated `Self`
        Self { model, gobject }
    }

    // Actions

    /// Append new item
    pub fn append(&self, profile_identity_gemini_id: Option<i64>, title: &str, subtitle: &str) {
        self.model
            .append(&Item::new(profile_identity_gemini_id, title, subtitle));
    }

    // Events

    /// Run callback function on `connect_selected_notify` event
    /// * return formatted `profile_identity_gemini_id` match selected item
    pub fn on_select(&self, callback: impl Fn(Option<i64>) + 'static) {
        self.gobject.connect_selected_notify(move |list| {
            callback(
                list.selected_item()
                    .and_downcast::<Item>()
                    .unwrap()
                    .profile_identity_gemini_id_option(),
            )
        });
    }

    // Getters

    /// Get formatted `profile_identity_gemini_id` **option** match selected item
    pub fn selected(&self) -> Option<i64> {
        self.gobject
            .selected_item()
            .and_downcast::<Item>()
            .unwrap()
            .profile_identity_gemini_id_option()
    }
}
