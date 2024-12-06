pub mod item;
use std::rc::Rc;

use item::{value::Value, Item};

use crate::profile::Profile;
use gtk::{
    gdk::Cursor,
    gio::{
        prelude::{Cast, CastNone},
        ListStore,
    },
    prelude::{BoxExt, ListItemExt, WidgetExt},
    Align, Box, DropDown, Image, Label, ListItem, Orientation, SignalListItemFactory,
};

pub struct List {
    pub dropdown: DropDown,
    list_store: ListStore,
}

impl List {
    // Constructors

    /// Create new `Self`
    pub fn new(profile: Rc<Profile>, auth_url: &str) -> Self {
        // Init model
        let list_store = ListStore::new::<Item>();

        list_store.append(&Item::new_guest_session());
        list_store.append(&Item::new_generate_pem());
        list_store.append(&Item::new_import_pem());

        // Append identities from profile database
        // * memory cache synced also and could be faster @TODO
        match profile.identity.gemini.database.records() {
            Ok(identities) => {
                for identity in identities {
                    match Item::new_profile_identity_gemini_id(
                        profile.clone(),
                        identity.id,
                        &identity.pem,
                        auth_url,
                    ) {
                        Ok(item) => list_store.append(&item),
                        Err(_) => todo!(),
                    }
                }
            }
            Err(_) => todo!(),
        }

        // Setup item factory
        // * wanted only to append items after `DropDown` init
        let factory = SignalListItemFactory::new();

        factory.connect_setup(|_, this| {
            // Init widget for dropdown item
            let child = Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .build();

            // Title
            child.append(&Label::builder().halign(Align::Start).build());

            // Subtitle
            let subtitle = Box::builder()
                .orientation(Orientation::Horizontal)
                .css_classes(["caption", "dim-label"])
                .halign(Align::Start)
                .build();

            subtitle.append(
                &Image::builder()
                    .css_classes(["accent"])
                    .cursor(&Cursor::from_name("help", None).unwrap())
                    .icon_name("help-about-symbolic")
                    .margin_end(4)
                    .pixel_size(11)
                    .build(),
            );

            subtitle.append(&Label::new(None));

            // Subtitle
            child.append(&subtitle);

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

            // Update `title`
            let title = child.first_child().and_downcast::<Label>().unwrap();

            title.set_label(&item.title());
            title.set_css_classes(if item.is_active() { &["success"] } else { &[] });

            // Update `subtitle`
            let subtitle = child.last_child().and_downcast::<Box>().unwrap();

            subtitle
                .last_child()
                .and_downcast::<Label>()
                .unwrap()
                .set_label(&item.subtitle());

            // Update `tooltip`
            let tooltip = subtitle.first_child().and_downcast::<Image>().unwrap();

            tooltip.set_visible(!item.tooltip().is_empty());
            tooltip.set_tooltip_markup(Some(&item.tooltip()));
        });

        // Init main widget
        let dropdown = DropDown::builder()
            .model(&list_store)
            .selected(
                list_store
                    .find_with_equal_func(|item| {
                        item.dynamic_cast_ref::<Item>().unwrap().is_active()
                    })
                    .unwrap(),
            )
            .factory(&factory)
            .build();

        // Return activated `Self`
        Self {
            list_store,
            dropdown,
        }
    }

    // Actions

    /// Find list item by `value` (stores ID)
    /// * return `position` found
    pub fn find(&self, value: i64) -> Option<u32> {
        self.list_store
            .find_with_equal_func(|this| value == this.clone().downcast::<Item>().unwrap().value())
    }

    /// Remove list item by `value` (stores ID)
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
        self.dropdown.connect_selected_notify(move |this| {
            callback(
                this.selected_item()
                    .and_downcast::<Item>()
                    .unwrap()
                    .value_enum(),
            )
        });
    }

    // Getters

    /// Get selected `Item` GObject
    pub fn selected_item(&self) -> Item {
        self.dropdown
            .selected_item()
            .and_downcast::<Item>()
            .unwrap()
    }
}
