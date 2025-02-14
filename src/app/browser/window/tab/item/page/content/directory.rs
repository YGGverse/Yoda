use gtk::{gio::File, ScrolledWindow};

pub struct Directory;

impl Directory {
    // Constructors

    pub fn for_file(file: &File, callback: impl Fn(&File) + 'static) -> ScrolledWindow {
        use gtk::{gio::FileInfo, Align, ListItem};

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

            column_view.append_column(
                &gtk::ColumnViewColumn::builder()
                    .title("Type")
                    .factory(&{
                        use gtk::prelude::{BoxExt, Cast, ListItemExt, WidgetExt};
                        let factory = gtk::SignalListItemFactory::new();
                        factory.connect_bind(|_, this| {
                            use gtk::gio::FileType;
                            let list_item = this.downcast_ref::<ListItem>().unwrap();
                            let image = gtk::Image::from_gicon(
                                &list_item
                                    .item()
                                    .unwrap()
                                    .downcast_ref::<FileInfo>()
                                    .unwrap()
                                    .symbolic_icon()
                                    .unwrap(),
                            );
                            image.set_tooltip_text(
                                match list_item
                                    .item()
                                    .unwrap()
                                    .downcast_ref::<FileInfo>()
                                    .unwrap()
                                    .file_type()
                                {
                                    FileType::Unknown => Some("Unknown"),
                                    FileType::Regular => Some("File"),
                                    FileType::Directory => Some("Directory"),
                                    FileType::SymbolicLink => Some("SymbolicLink"),
                                    FileType::Special => Some("Special"),
                                    FileType::Shortcut => Some("Shortcut"),
                                    FileType::Mountable => Some("Mountable"),
                                    _ => None,
                                },
                            );
                            let container = gtk::Box::builder().halign(Align::Center).build(); // prevents `gtk::Image` blur
                            container.append(&image);
                            list_item.set_child(Some(&container));
                        });
                        factory
                    })
                    .build(),
            );

            column_view.append_column(
                &gtk::ColumnViewColumn::builder()
                    .expand(true)
                    .title("Name")
                    .factory(&{
                        let factory = gtk::SignalListItemFactory::new();
                        factory.connect_bind(|_, this| {
                            use gtk::prelude::{Cast, FileExt, ListItemExt};
                            let list_item = this.downcast_ref::<ListItem>().unwrap();
                            let item = list_item.item().unwrap();
                            let file_info = item.downcast_ref::<FileInfo>().unwrap();
                            list_item.set_child(Some(
                                &gtk::Label::builder()
                                    .halign(Align::Start)
                                    .ellipsize(gtk::pango::EllipsizeMode::Middle)
                                    .label(file_info.display_name())
                                    .tooltip_text(
                                        file_info
                                            .attribute_object("standard::file")
                                            .unwrap()
                                            .downcast_ref::<File>()
                                            .unwrap()
                                            .path()
                                            .unwrap()
                                            .to_str()
                                            .unwrap(),
                                    )
                                    .build(),
                            ));
                        });
                        factory
                    })
                    .build(),
            );

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
