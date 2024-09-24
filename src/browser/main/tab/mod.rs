mod label;
mod page;

use label::Label;
use page::Page;

use gtk::{prelude::WidgetExt, GestureClick, Notebook};
use std::{cell::RefCell, collections::HashMap, sync::Arc};

pub struct Tab {
    widget: Notebook,
    // Dynamically allocated reference index
    labels: RefCell<HashMap<u32, Arc<Label>>>,
    pages: RefCell<HashMap<u32, Arc<Page>>>,
}

impl Tab {
    // Construct
    pub fn new() -> Tab {
        Self {
            // Init widget
            widget: Notebook::builder().scrollable(true).build(),
            // Init empty hashmap as no tabs yet
            labels: RefCell::new(HashMap::new()),
            pages: RefCell::new(HashMap::new()),
        }
    }

    // Actions
    pub fn append(&self, is_current_page: bool) -> u32 {
        // Init new tab components
        let label = Arc::new(Label::new(false));
        let page = Arc::new(Page::new());

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

        // Append new Notebook page
        let page_number = self.widget.append_page(page.widget(), Some(label.widget()));

        // Additional setup for Notebook tab created
        self.widget.set_tab_reorderable(page.widget(), true);

        if is_current_page {
            self.widget.set_current_page(Some(page_number));
        }

        // Register dynamically created tab components in the HashMap index
        // @TODO static key on tab reorder
        // @TODO cleanup on tab remove
        self.labels
            .borrow_mut()
            .insert(page_number.try_into().unwrap(), label);

        self.pages
            .borrow_mut()
            .insert(page_number.try_into().unwrap(), page);

        // Result
        page_number
    }

    // Close active tab
    pub fn close(&self) {
        self.widget.remove_page(self.widget.current_page());
    }

    // Close all tabs
    pub fn close_all(&self) {
        // @TODO skip pinned or make confirmation alert (GTK>=4.10)
        while let Some(page_number) = self.widget.current_page() {
            self.widget.remove_page(Some(page_number));
        }
    }

    // Toggle pin status for active tab
    pub fn pin(&self) {
        if let Some(page_number) = self.widget.current_page() {
            let label = self.labels.borrow();
            label
                .get(&page_number)
                .unwrap()
                .pin(!label.get(&page_number).unwrap().is_pinned()); // toggle
        }
    }

    // Getters
    pub fn widget(&self) -> &Notebook {
        &self.widget
    }
}
