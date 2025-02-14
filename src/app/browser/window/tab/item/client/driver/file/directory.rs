use gtk::{gio::File, prelude::FileExt};
use std::rc::Rc;

pub struct Directory {
    pub file: File,
}

impl Directory {
    pub fn handle(&self, page: &Rc<super::Page>) {
        page.content.to_directory(&self.file, {
            let page = page.clone();
            move |file| {
                page.item_action.load.activate(
                    Some(&format!(
                        "file://{}",
                        file.path().unwrap().to_str().unwrap()
                    )),
                    true,
                )
            }
        });
        page.set_title(&self.file.parse_name());
        page.set_progress(0.0);
        page.window_action.find.simple_action.set_enabled(false);
    }
}
