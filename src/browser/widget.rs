pub struct Browser {
    gtk: gtk::ApplicationWindow,
}

impl Browser {
    pub fn new(
        application: &gtk::Application,
        titlebar: &gtk::HeaderBar,
        child: &gtk::Box,
        default_width: i32,
        default_height: i32,
    ) -> Browser {
        Self {
            gtk: gtk::ApplicationWindow::builder()
                .application(application)
                .default_width(default_width)
                .default_height(default_height)
                .titlebar(titlebar)
                .child(child)
                .build(),
        }
    }

    pub fn gtk(&self) -> &gtk::ApplicationWindow {
        &self.gtk
    }
}
