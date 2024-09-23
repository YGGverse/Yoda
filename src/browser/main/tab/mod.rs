mod label;
mod page;

use std::sync::Arc;

use gtk::prelude::WidgetExt;
use gtk::{GestureClick, Notebook};
use label::Label;
use page::Page;

pub struct Tab {
    widget: Notebook,
}

impl Tab {
    // Construct
    pub fn new() -> Tab {
        Self {
            widget: Notebook::builder().scrollable(true).build(),
        }
    }

    // Actions
    pub fn append(&self, is_active: bool) -> u32 {
        // Init new tab components
        let label = Arc::new(Label::new(false));
        let page = Page::new();

        // Init additional label actions
        let controller = GestureClick::new();

        controller.connect_pressed({
            let label = label.clone();
            move |_, n: i32, _, _| {
                // double click
                if n == 2 {
                    label.pin(!label.is_pinned()); // toggle
                }
            }
        });

        label.widget().add_controller(controller);

        // Append new page
        let page_number = self.widget.append_page(page.widget(), Some(label.widget()));

        self.widget.set_tab_reorderable(page.widget(), true);

        // Follow?
        if is_active {
            self.widget.set_current_page(Some(page_number));
        }

        // Result
        page_number
    }

    pub fn close(&self) {
        todo!()
    }

    /* @TODO
    pub fn close_all(&self) {
        todo!()
    }*/

    pub fn pin(&self) -> bool {
        todo!()
    }

    // Getters
    pub fn widget(&self) -> &Notebook {
        &self.widget
    }
}
