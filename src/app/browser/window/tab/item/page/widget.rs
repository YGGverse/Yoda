use adw::Clamp;
use gtk::{
    gio::{SimpleAction, SimpleActionGroup},
    glib::uuid_string_random,
    prelude::{ActionMapExt, BoxExt, WidgetExt},
    Box, Orientation,
};
use std::sync::Arc;

pub struct Widget {
    gobject: Box,
}

impl Widget {
    // Construct
    pub fn new_arc(
        name: &str,
        // Actions
        action_page_open: SimpleAction,
        // Components
        navigation: &Box,
        content: &Box,
        input: &Clamp,
    ) -> Arc<Self> {
        // Init additional action group
        let action_group = SimpleActionGroup::new();
        action_group.add_action(&action_page_open);

        // Init self
        let gobject = Box::builder()
            .orientation(Orientation::Vertical)
            .name(name)
            .build();

        gobject.append(navigation);
        gobject.append(content);
        gobject.append(input);

        gobject.insert_action_group(&uuid_string_random(), Some(&action_group));

        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
