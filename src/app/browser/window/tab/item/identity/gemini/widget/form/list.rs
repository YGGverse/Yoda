use gtk::{DropDown, StringList};

pub struct List {
    gobject: DropDown,
}

impl List {
    // Constructors

    /// Create new `Self`
    pub fn new(list_options: &Vec<(Option<i64>, String, bool)>) -> Self {
        // Init empty list model
        let model = StringList::new(&[]);

        // Init `GObject`
        let gobject = DropDown::builder().model(&model).build();

        // Build selection list
        let mut index = 0;

        for (_key, value, is_selected) in list_options {
            model.append(&value);

            if *is_selected {
                gobject.set_selected(index);
            }

            index += 1;
        }

        // Done
        Self { gobject }
    }

    // Getters

    pub fn gobject(&self) -> &DropDown {
        &self.gobject
    }
}
