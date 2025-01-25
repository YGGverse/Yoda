use gtk::{
    gio::SimpleActionGroup,
    glib::{uuid_string_random, GString},
};

/// [SimpleActionGroup](https://docs.gtk.org/gio/class.SimpleActionGroup.html) wrapper for `Tab` actions
pub struct Action {
    pub id: GString,
    pub simple_action_group: SimpleActionGroup,
}

impl Default for Action {
    fn default() -> Self {
        Self::new()
    }
}

impl Action {
    pub fn new() -> Self {
        Self {
            id: uuid_string_random(),
            simple_action_group: SimpleActionGroup::new(),
        }
    }
}
