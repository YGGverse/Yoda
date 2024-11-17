mod list;
mod name;

use list::List;
use name::Name;

use gtk::{
    prelude::{BoxExt, WidgetExt},
    Box, Orientation,
};

pub struct Form {
    gobject: Box,
}

impl Form {
    // Constructors

    /// Create new `Self`
    pub fn new(list_options: Vec<(Option<i64>, String, bool)>) -> Self {
        // Init components
        let list = List::new(&list_options);
        let name = Name::new();

        // Init main container
        let gobject = Box::builder().orientation(Orientation::Vertical).build();

        gobject.append(list.gobject());
        gobject.append(name.gobject());

        // Init events
        list.gobject().connect_selected_notify(move |this| {
            // Get selection ID from vector @TODO use GObject storage instead
            // https://gtk-rs.org/gtk4-rs/stable/latest/book/list_widgets.html
            match list_options.get(this.selected() as usize) {
                // Hide name entry on existing identity selected
                Some((id, _, _)) => name.gobject().set_visible(id.is_none()),
                None => todo!(),
            }
        });

        // Return activated `Self`
        Self { gobject }
    }

    // Getters

    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
