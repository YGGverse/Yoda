mod column;

use gtk::{gio::File, ScrolledWindow};

pub struct Directory; // @TODO save settings

impl Directory {
    // Constructors

    pub fn for_file(file: &File, callback: impl Fn(&File) + 'static) -> ScrolledWindow {
        use column::Column;
        use gtk::gio::FileInfo;

        // Init children widget
        let column_view = {
            const ATTRIBUTES: &str =
            "standard::display-name,standard::symbolic-icon,standard::size,standard::content-type,standard::modification-date-time";

            let column_view = gtk::ColumnView::builder()
                // @TODO implement profile save .reorderable(true)
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

            let icon = Column::icon();
            let name = Column::name(360);
            let size = Column::size(120);
            let content_type = Column::content_type(180);
            //let modification_date_time = Column::modification_date_time();

            column_view.append_column(&icon);
            column_view.append_column(&name);
            column_view.append_column(&size);
            column_view.append_column(&content_type);
            //column_view.append_column(&modification_date_time);

            column_view.sort_by_column(Some(&name), gtk::SortType::Ascending);
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
            .child(&column_view)
            .vexpand(true)
            .build()
    }
}
