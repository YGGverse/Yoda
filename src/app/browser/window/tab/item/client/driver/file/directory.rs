use gtk::{gio::File, prelude::FileExt};

pub struct Directory {
    pub file: File,
}

impl Directory {
    pub fn handle(&self, page: &super::Page) {
        page.content.to_directory(&self.file);
        page.set_title(&self.file.parse_name());
        page.set_progress(0.0);
        page.window_action.find.simple_action.set_enabled(false);
    }
}
