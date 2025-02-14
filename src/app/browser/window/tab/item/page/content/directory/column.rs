pub trait Column {
    fn icon() -> Self;
    fn name() -> Self;
}

impl Column for gtk::ColumnViewColumn {
    fn icon() -> Self {
        use gtk::{
            gio::{FileInfo, FileType},
            prelude::{BoxExt, Cast, ListItemExt, WidgetExt},
            Align, ColumnViewColumn, ListItem, SignalListItemFactory,
        };

        ColumnViewColumn::builder()
            .title("Type")
            .factory(&{
                let factory = SignalListItemFactory::new();
                factory.connect_bind(|_, this| {
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
            .build()
    }

    fn name() -> Self {
        gtk::ColumnViewColumn::builder()
            .expand(true)
            .title("Name")
            .factory(&{
                let factory = gtk::SignalListItemFactory::new();
                factory.connect_bind(|_, this| {
                    use gtk::prelude::{Cast, /*FileExt,*/ ListItemExt};
                    let list_item = this.downcast_ref::<gtk::ListItem>().unwrap();
                    let item = list_item.item().unwrap();
                    let file_info = item.downcast_ref::<gtk::gio::FileInfo>().unwrap();
                    list_item.set_child(Some(
                        &gtk::Label::builder()
                            .halign(gtk::Align::Start)
                            .ellipsize(gtk::pango::EllipsizeMode::Middle)
                            .label(file_info.display_name())
                            /*.tooltip_text(
                                file_info
                                    .attribute_object("standard::file")
                                    .unwrap()
                                    .downcast_ref::<File>()
                                    .unwrap()
                                    .path()
                                    .unwrap()
                                    .to_str()
                                    .unwrap(),
                            ) this feature maybe is not really wanted */
                            .build(),
                    ));
                });
                factory
            })
            .build()
    }
}
