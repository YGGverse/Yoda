pub mod item;
use std::rc::Rc;

use item::Item;

use super::Action;
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
    // profile: Rc<Profile>,
}

impl List {
    // Constructors

    /// Create new `Self`
    pub fn new(action_widget: Rc<Action>, profile: Rc<Profile>, auth_url: &str) -> Self {
        // Init model
        let list_store = ListStore::new::<Item>();

        list_store.remove_all();

        list_store.append(&Item::new_guest_session());
        list_store.append(&Item::new_generate_pem());
        list_store.append(&Item::new_import_pem());

        match profile.identity.gemini.database.records() {
            Ok(identities) => {
                for identity in identities {
                    match Item::new_profile_identity_gemini_id(
                        profile.clone(),
                        identity.id,
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
                    .unwrap_or_default(),
            )
            .factory(&factory)
            .build();

        // Connect events
        dropdown.connect_selected_notify(move |_| action_widget.update.activate(false));

        // Return activated `Self`
        Self {
            dropdown,
            list_store,
            // profile,
        }
    }

    // Actions

    /// Find list item by `profile_identity_gemini_id`
    /// * return `position` found
    pub fn find(&self, profile_identity_gemini_id: i64) -> Option<u32> {
        self.list_store.find_with_equal_func(|this| {
            profile_identity_gemini_id == this.downcast_ref::<Item>().unwrap().value()
        })
    }

    /// Remove list item by `profile_identity_gemini_id` (stores ID)
    /// * return `position` of removed list item
    pub fn remove(&self, profile_identity_gemini_id: i64) -> Option<u32> {
        match self.find(profile_identity_gemini_id) {
            Some(position) => {
                self.list_store.remove(position);
                Some(position)
            }
            None => todo!(),
        }
    }

    /// Update list item by `profile_identity_gemini_id` (stores ID)
    /// * return `position` of updated list item
    pub fn update(&self, profile_identity_gemini_id: i64) -> Option<u32> {
        match self.find(profile_identity_gemini_id) {
            Some(position) => {
                // @TODO
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
