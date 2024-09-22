mod widget;

pub struct Tab {
    pub widget: widget::Tab,
}

impl Tab {
    pub fn new() -> Tab {
        // Init widget
        let widget = widget::Tab::new();

        // Init events
        /* @TODO
        widget.connect_clicked(|this| {
            this.activate_action("win.tab_append", None)
                .expect("The action does not exist");
        }); */

        // Result
        Self { widget }
    }

    // Getters
    pub fn widget(&self) -> &widget::Tab {
        &self.widget
    }
}
