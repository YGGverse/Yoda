use gtk::{
    gio::{SimpleAction, SimpleActionGroup},
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
        // Actions
        action_page_open: Arc<SimpleAction>,
        // Options
        name: &str,
        // Components
        navigation: &Box,
        content: &Box,
    ) -> Arc<Self> {
        // Init additional action group
        let action_group = SimpleActionGroup::new();
        action_group.add_action(action_page_open.as_ref());

        // Init self
        let gobject = Box::builder()
            .orientation(Orientation::Vertical)
            .name(name)
            .build();

        gobject.append(navigation);
        gobject.append(content);

        gobject.insert_action_group("page", Some(&action_group));

        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
