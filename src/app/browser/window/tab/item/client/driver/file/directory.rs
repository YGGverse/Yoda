use gtk::{gio::File, prelude::FileExt};
use std::rc::Rc;

pub struct Directory {
    pub file: File,
}

impl Directory {
    pub fn handle(&self, page: &Rc<super::Page>, is_snap_history: bool) {
        page.set_progress(1.0);
        page.content.to_directory(
            &self.file,
            (
                // on ready
                {
                    let page = page.clone();
                    move || {
                        page.navigation
                            .request
                            .info
                            .borrow_mut()
                            .add_event("Build directory tree".to_string());
                        page.set_progress(0.0)
                    }
                },
                // on activate
                {
                    let page = page.clone();
                    move |file| {
                        page.item_action.load.activate(
                            Some(&format!(
                                "file://{}",
                                file.path().unwrap().to_str().unwrap()
                            )),
                            is_snap_history,
                            false,
                        )
                    }
                },
            ),
        );
        page.set_title(&self.file.parse_name());
        if is_snap_history {
            page.snap_history();
        }
        page.window_action.find.simple_action.set_enabled(false);
        page.window_action.save_as.simple_action.set_enabled(false);
    }
}
