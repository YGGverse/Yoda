use gtk::{gio::File, ScrolledWindow};

pub struct Directory;

impl Directory {
    // Constructors

    pub fn for_file(file: &File) -> ScrolledWindow {
        const ATTRIBUTES:&str = "standard::name,standard::display-name,standard::symbolic-icon,standard::size,standard::content-type";

        ScrolledWindow::builder()
            .child(
                &adw::Clamp::builder()
                    .child(&{
                        let column_view = gtk::ColumnView::builder()
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
                                        let list_item =
                                            this.downcast_ref::<gtk::ListItem>().unwrap();
                                        let image = gtk::Image::from_gicon(
                                            &list_item
                                                .item()
                                                .unwrap()
                                                .downcast_ref::<gtk::gio::FileInfo>()
                                                .unwrap()
                                                .symbolic_icon()
                                                .unwrap(),
                                        );
                                        image.set_tooltip_text(
                                            match list_item
                                                .item()
                                                .unwrap()
                                                .downcast_ref::<gtk::gio::FileInfo>()
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
                                        let container = gtk::Box::builder().build(); // prevents `gtk::Image` blur
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
                                        use gtk::prelude::{Cast, ListItemExt};
                                        let list_item =
                                            this.downcast_ref::<gtk::ListItem>().unwrap();
                                        let item = list_item.item().unwrap();
                                        let file_info =
                                            item.downcast_ref::<gtk::gio::FileInfo>().unwrap();
                                        list_item.set_child(Some(
                                            &gtk::Label::builder()
                                                .halign(gtk::Align::Start)
                                                .ellipsize(gtk::pango::EllipsizeMode::Middle)
                                                .label(file_info.display_name())
                                                .tooltip_text(file_info.display_name())
                                                .build(),
                                        ));
                                    });
                                    factory
                                })
                                .build(),
                        );
                        column_view
                    })
                    .css_classes(["view"])
                    .build(),
            )
            .vexpand(true)
            .build()
    }
}
