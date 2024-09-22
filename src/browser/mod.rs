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
        width: i32,
        height: i32,
    ) -> Browser {
        // Init widget
        let widget = widget::Browser::new(
            app,
            header::new().widget.as_ref(), // @TODO
            main::new().widget.as_ref(),   // @TODO
            width,
            height,
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
