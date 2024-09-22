use gtk::prelude::BoxExt;

pub struct Navigation {
    gtk: gtk::Box,
}

impl Navigation {
    // Construct
    pub fn new(
        base: &gtk::Button,
        history: &gtk::Box,
        reload: &gtk::Button,
        request: &gtk::Entry,
        bookmark: &gtk::Button,
    ) -> Navigation {
        let gtk = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(8)
            .margin_top(8)
            .margin_start(8)
            .margin_end(8)
            .margin_bottom(8)
            .build();

        gtk.append(base);
        gtk.append(history);
        gtk.append(reload);
        gtk.append(request);
        gtk.append(bookmark);

        Self { gtk }
    }

    // Getters
    pub fn gtk(&self) -> &gtk::Box {
        &self.gtk
    }
}
