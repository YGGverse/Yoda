mod action;
mod db;
mod header;
mod main;
mod widget;

use gtk::prelude::ActionMapExtManual;

pub struct Browser {
    db: db::Browser,
    widget: widget::Browser,
}

impl Browser {
    // Construct new browser
    pub fn new(
        app: &gtk::Application,
        connection: std::sync::Arc<sqlite::Connection>,
        default_width: i32,
        default_height: i32,
    ) -> Browser {
        // Init widget
        let widget = widget::Browser::new(
            app,
            header::Header::new().widget().gtk(),
            main::new().widget.as_ref(), // @TODO
            default_width,
            default_height,
        );

        // Connect actions
        widget
            .gtk()
            .add_action_entries([action::debug(), action::quit()]);

        // Return
        Self {
            db: db::Browser::new(connection),
            widget,
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Browser {
        &self.widget
    }
}
