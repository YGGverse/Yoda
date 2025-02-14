const DEFAULT: &str = "-";
const DATE_TIME_FORMAT: &str = "%Y.%m.%d %H:%M:%S";

pub trait Column {
    fn icon() -> Self;
    fn name(width: i32) -> Self;
    fn size(width: i32) -> Self;
    fn content_type(width: i32) -> Self;
    fn creation_date_time(width: i32) -> Self;
    fn modification_date_time(width: i32) -> Self;
    fn access_date_time(width: i32) -> Self;
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

    fn name(width: i32) -> Self {
        gtk::ColumnViewColumn::builder()
            .fixed_width(width)
            .resizable(true)
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

    fn size(width: i32) -> Self {
        gtk::ColumnViewColumn::builder()
            .fixed_width(width)
            .resizable(true)
            .title("Size")
            .factory(&{
                let factory = gtk::SignalListItemFactory::new();
                factory.connect_bind(|_, this| {
                    use crate::tool::Format;
                    use gtk::prelude::{Cast, ListItemExt};
                    let list_item = this.downcast_ref::<gtk::ListItem>().unwrap();
                    let item = list_item.item().unwrap();
                    let file_info = item.downcast_ref::<gtk::gio::FileInfo>().unwrap();
                    list_item.set_child(Some(
                        &gtk::Label::builder()
                            .halign(gtk::Align::Start)
                            .ellipsize(gtk::pango::EllipsizeMode::Middle)
                            .label((file_info.size() as usize).bytes())
                            .build(),
                    ));
                });
                factory
            })
            .build()
    }

    fn content_type(width: i32) -> Self {
        gtk::ColumnViewColumn::builder()
            .fixed_width(width)
            .resizable(true)
            .title("Content Type")
            .factory(&{
                let factory = gtk::SignalListItemFactory::new();
                factory.connect_bind(|_, this| {
                    use gtk::prelude::{Cast, ListItemExt};
                    let list_item = this.downcast_ref::<gtk::ListItem>().unwrap();
                    let item = list_item.item().unwrap();
                    let file_info = item.downcast_ref::<gtk::gio::FileInfo>().unwrap();
                    let content_type: gtk::glib::GString = match file_info.content_type() {
                        Some(content_type) => {
                            let display_name = file_info.display_name();
                            if content_type == "text/plain" {
                                if display_name.ends_with(".gmi")
                                    || display_name.ends_with(".gemini")
                                {
                                    "text/gemini".into()
                                } else {
                                    content_type
                                }
                            } else {
                                content_type
                            }
                        }
                        None => DEFAULT.into(),
                    };
                    list_item.set_child(Some(
                        &gtk::Label::builder()
                            .halign(gtk::Align::Start)
                            .ellipsize(gtk::pango::EllipsizeMode::Middle)
                            .label(content_type)
                            .build(),
                    ));
                });
                factory
            })
            .build()
    }

    fn creation_date_time(width: i32) -> Self {
        gtk::ColumnViewColumn::builder()
            .fixed_width(width)
            .resizable(true)
            .title("Created")
            .factory(&{
                let factory = gtk::SignalListItemFactory::new();
                factory.connect_bind(|_, this| {
                    use gtk::prelude::{Cast, ListItemExt};
                    let list_item = this.downcast_ref::<gtk::ListItem>().unwrap();
                    let item = list_item.item().unwrap();
                    let file_info = item.downcast_ref::<gtk::gio::FileInfo>().unwrap();
                    list_item.set_child(Some(
                        &gtk::Label::builder()
                            .halign(gtk::Align::Start)
                            .ellipsize(gtk::pango::EllipsizeMode::Middle)
                            .label(
                                file_info
                                    .creation_date_time()
                                    .unwrap()
                                    .format(DATE_TIME_FORMAT) // @TODO optional
                                    .unwrap_or(DEFAULT.into()),
                            )
                            .build(),
                    ));
                });
                factory
            })
            .build()
    }

    fn modification_date_time(width: i32) -> Self {
        gtk::ColumnViewColumn::builder()
            .fixed_width(width)
            .resizable(true)
            .title("Modified")
            .factory(&{
                let factory = gtk::SignalListItemFactory::new();
                factory.connect_bind(|_, this| {
                    use gtk::prelude::{Cast, ListItemExt};
                    let list_item = this.downcast_ref::<gtk::ListItem>().unwrap();
                    let item = list_item.item().unwrap();
                    let file_info = item.downcast_ref::<gtk::gio::FileInfo>().unwrap();
                    list_item.set_child(Some(
                        &gtk::Label::builder()
                            .halign(gtk::Align::Start)
                            .ellipsize(gtk::pango::EllipsizeMode::Middle)
                            .label(
                                file_info
                                    .modification_date_time()
                                    .unwrap()
                                    .format(DATE_TIME_FORMAT) // @TODO optional
                                    .unwrap_or(DEFAULT.into()),
                            )
                            .build(),
                    ));
                });
                factory
            })
            .build()
    }

    fn access_date_time(width: i32) -> Self {
        gtk::ColumnViewColumn::builder()
            .fixed_width(width)
            .resizable(true)
            .title("Accessed")
            .factory(&{
                let factory = gtk::SignalListItemFactory::new();
                factory.connect_bind(|_, this| {
                    use gtk::prelude::{Cast, ListItemExt};
                    let list_item = this.downcast_ref::<gtk::ListItem>().unwrap();
                    let item = list_item.item().unwrap();
                    let file_info = item.downcast_ref::<gtk::gio::FileInfo>().unwrap();
                    list_item.set_child(Some(
                        &gtk::Label::builder()
                            .halign(gtk::Align::Start)
                            .ellipsize(gtk::pango::EllipsizeMode::Middle)
                            .label(
                                file_info
                                    .access_date_time()
                                    .unwrap()
                                    .format(DATE_TIME_FORMAT) // @TODO optional
                                    .unwrap_or(DEFAULT.into()),
                            )
                            .build(),
                    ));
                });
                factory
            })
            .build()
    }
}
