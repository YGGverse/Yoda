mod widget;
use gtk::glib::DateTime;
use widget::Widget;

use crate::profile::Profile;
use adw::ApplicationWindow;
use std::rc::Rc;

pub struct Welcome {
    profile: Rc<Profile>,
    widget: Rc<Widget>,
}

impl Welcome {
    // Construct

    /// Create new `Self` for given Profile
    pub fn new(profile: Rc<Profile>, parent: ApplicationWindow) -> Self {
        // Init widget
        let widget = Rc::new(Widget::new(parent));

        // Init events
        widget.connect_response({
            let profile = profile.clone();
            move |value| {
                match value {
                    Some(id) => {
                        // Activate selected profile by record ID
                        let _ = profile.database.activate(id);
                    }
                    None => {
                        // Create and select new profile
                        let _ = profile
                            .database
                            .add(true, DateTime::now_local().unwrap(), None);
                    }
                } // @TODO handle result
            }
        });

        // Return activated `Self`
        Self { profile, widget }
    }

    // Actions

    /// Show dialog for parent [Window](https://docs.gtk.org/gtk4/class.Window.html)
    pub fn present(&self) {
        // Collect Profile list
        let mut responses = Vec::new();
        for record in self.profile.database.records() {
            responses.push((
                record.id.to_string(),
                record.time.format_iso8601().unwrap().to_string(),
            ))
        }
        // Show dialog
        self.widget.present(responses);
    }
}
