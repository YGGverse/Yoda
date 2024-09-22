use gtk::prelude::{ButtonExt, WidgetExt};

pub struct Tab {
    gtk: gtk::Button,
}

impl Tab {
    // Construct
    pub fn new() -> Tab {
        // Init widget
        let gtk = gtk::Button::builder()
            .icon_name("tab-new-symbolic")
            .tooltip_text("New tab")
            .build();

        // Init events
        gtk.connect_clicked(|this| {
            this.activate_action("win.tab_append", None)
                .expect("The action does not exist");
        });

        Self { gtk }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Button {
        &self.gtk
    }
}
