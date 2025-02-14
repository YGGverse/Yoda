mod column;

use gtk::{gio::File, ScrolledWindow};

pub struct Directory;

impl Directory {
    // Constructors

    pub fn for_file(file: &File, callback: impl Fn(&File) + 'static) -> ScrolledWindow {
        use column::Column;
        use gtk::gio::FileInfo;

        // Init children widget
        let column_view = {
            const ATTRIBUTES: &str =
            "standard::display-name,standard::symbolic-icon,standard::size,standard::content-type";

            let column_view = gtk::ColumnView::builder()
                // @TODO enable this option may cause core dumped errors
                // .single_click_activate(true)
                .model(
                    &gtk::SingleSelection::builder()
                        .model(
                            &gtk::DirectoryList::builder()
                                .file(file)
                                .attributes(ATTRIBUTES)
                                .build(),
                        )
                        .build(),
                )
                .build();

            column_view.append_column(&Column::icon());
            column_view.append_column(&Column::name());
            column_view
        };

        // Connect events
        column_view.connect_activate(move |this, i| {
            use gtk::prelude::{Cast, ListModelExt};
            callback(
                this.model()
                    .unwrap()
                    .item(i)
                    .unwrap()
                    .downcast_ref::<FileInfo>()
                    .unwrap()
                    .attribute_object("standard::file")
                    .unwrap()
                    .downcast_ref::<File>()
                    .unwrap(),
            )
        });

        // Build main widget
        ScrolledWindow::builder()
            .child(
                &adw::Clamp::builder()
                    .child(&column_view)
                    .css_classes(["view"])
                    .build(),
            )
            .vexpand(true)
            .build()
    }
}
