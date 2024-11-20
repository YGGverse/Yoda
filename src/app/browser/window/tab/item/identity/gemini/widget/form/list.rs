mod item;
use item::Item;

use gtk::{
    gio::{
        prelude::{Cast, CastNone},
        ListStore,
    },
    prelude::{BoxExt, ListItemExt, WidgetExt},
    Align, Box, DropDown, Label, ListItem, SignalListItemFactory,
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

        // Setup item factory
        // * wanted only to append items after `DropDown` init
        let factory = SignalListItemFactory::new();

        factory.connect_setup(|_, gobject| {
            // Init row widget for dropdown item
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

            // Update menu item
            gobject
                .downcast_ref::<ListItem>()
                .unwrap()
                .set_child(Some(&widget));
        });

        factory.connect_bind(|_, gobject| {
            // Downcast requirements
            let list_item = gobject.downcast_ref::<ListItem>().unwrap();
            let item = list_item.item().and_downcast::<Item>().unwrap();
            let container = list_item.child().and_downcast::<Box>().unwrap();

            // Update Title (expected as the first child)
            container
                .first_child()
                .unwrap()
                .downcast::<Label>()
                .unwrap()
                .set_label(&item.title());

            // Update Subtitle (expected as the last child)
            container
                .last_child()
                .unwrap()
                .downcast::<Label>()
                .unwrap()
                .set_label(&item.subtitle());
        });

        // Init main `GObject`
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
