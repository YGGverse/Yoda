use adw::ToolbarView;
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
        action_page_open: Arc<SimpleAction>,
        // Components
        navigation: &Box,
        content: &Box,
        request: &ToolbarView,
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
        gobject.append(request);

        gobject.insert_action_group(&uuid_string_random(), Some(&action_group));

        Arc::new(Self { gobject })
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
