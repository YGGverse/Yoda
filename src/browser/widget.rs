use gtk::{Application, ApplicationWindow, Box, HeaderBar};

pub struct Browser {
    window: ApplicationWindow,
}

impl Browser {
    pub fn new(
        application: &Application,
        titlebar: &HeaderBar,
        child: &Box,
        default_width: i32,
        default_height: i32,
    ) -> Browser {
        Self {
            window: ApplicationWindow::builder()
                .application(application)
                .default_width(default_width)
                .default_height(default_height)
                .titlebar(titlebar)
                .child(child)
                .build(),
        }
    }

    pub fn window(&self) -> &ApplicationWindow {
        &self.window
    }
}
