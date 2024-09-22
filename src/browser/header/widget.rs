pub struct Header {
    gtk: gtk::HeaderBar,
}

impl Header {
    pub fn new(tray: &gtk::Box, subject: &gtk::Box) -> Header {
        let gtk = gtk::HeaderBar::builder().build();

        gtk.pack_start(tray);
        gtk.set_title_widget(Some(subject));

        Self { gtk }
    }

    pub fn gtk(&self) -> &gtk::HeaderBar {
        &self.gtk
    }
}
