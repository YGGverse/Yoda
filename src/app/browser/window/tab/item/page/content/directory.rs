mod column;

use gtk::{gio::File, ScrolledWindow};

pub struct Directory; // @TODO save settings

impl Directory {
    // Constructors

    pub fn for_file(
        file: &File,
        (on_ready, on_activate): (impl Fn() + 'static, impl Fn(&File) + 'static),
    ) -> ScrolledWindow {
        use column::Column;
        use gtk::gio::FileInfo;

        // Init model
        const ATTRIBUTES: &str =
        "standard::type,standard::display-name,standard::symbolic-icon,standard::size,standard::content-type,time::modified,time::created,time::access";

        let directory_list = gtk::DirectoryList::builder()
            .file(file)
            .attributes(ATTRIBUTES)
            .build();

        // Init children widget
        let column_view = {
            let column_view = gtk::ColumnView::builder()
                .halign(gtk::Align::Center)
                // @TODO implement profile save .reorderable(true)
                // @TODO enable this option may cause core dumped errors
                // .single_click_activate(true)
                .model(
                    &gtk::SingleSelection::builder()
                        .model(&directory_list)
                        .build(),
                )
                .build();

            let icon = Column::icon();
            let name = Column::name(360);
            let size = Column::size(120);
            let content_type = Column::content_type(180);
            let creation_date_time = Column::creation_date_time(220);
            let modification_date_time = Column::modification_date_time(220);
            let access_date_time = Column::access_date_time(220);

            column_view.append_column(&icon);
            column_view.append_column(&name);
            column_view.append_column(&content_type);
            column_view.append_column(&size);
            column_view.append_column(&creation_date_time);
            column_view.append_column(&modification_date_time);
            column_view.append_column(&access_date_time);

            column_view.sort_by_column(Some(&name), gtk::SortType::Ascending);
            column_view
        };

        // Connect events
        directory_list.connect_loading_notify(move |this| {
            if !this.is_loading() {
                on_ready()
            }
        });

        column_view.connect_activate(move |this, i| {
            use gtk::prelude::{Cast, ListModelExt};
            on_activate(
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
            .css_classes(["view"])
            .child(&column_view)
            .vexpand(true)
            .build()
    }
}
