mod display;
mod format;

use display::Display;
use format::Format;

use gtk::{
    gio::FileInfo, pango::EllipsizeMode, Align, ColumnViewColumn, Label, ListItem,
    SignalListItemFactory,
};

pub trait Column {
    fn icon() -> Self;
    fn name(width: i32) -> Self;
    fn size(width: i32) -> Self;
    fn content_type(width: i32) -> Self;
    fn creation_date_time(width: i32) -> Self;
    fn modification_date_time(width: i32) -> Self;
    fn access_date_time(width: i32) -> Self;
}

impl Column for ColumnViewColumn {
    fn icon() -> Self {
        use gtk::{
            gio::FileInfo,
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
                    image.set_tooltip_text(Some(
                        list_item
                            .item()
                            .unwrap()
                            .downcast_ref::<FileInfo>()
                            .unwrap()
                            .file_type()
                            .as_str(),
                    ));
                    let container = gtk::Box::builder().halign(Align::Center).build(); // prevents `gtk::Image` blur
                    container.append(&image);
                    list_item.set_child(Some(&container));
                });
                factory
            })
            .build()
    }

    fn name(width: i32) -> Self {
        ColumnViewColumn::builder()
            .fixed_width(width)
            .resizable(true)
            .title("Name")
            .factory(&{
                let factory = SignalListItemFactory::new();
                factory.connect_bind(|_, this| {
                    use gtk::prelude::{Cast, /*FileExt,*/ ListItemExt};
                    let list_item = this.downcast_ref::<ListItem>().unwrap();
                    let item = list_item.item().unwrap();
                    let file_info = item.downcast_ref::<FileInfo>().unwrap();
                    list_item.set_child(Some(
                        &Label::builder()
                            .halign(Align::Start)
                            .ellipsize(EllipsizeMode::Middle)
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
        ColumnViewColumn::builder()
            .fixed_width(width)
            .resizable(true)
            .title("Size")
            .factory(&{
                let factory = SignalListItemFactory::new();
                factory.connect_bind(|_, this| {
                    use crate::tool::Format;
                    use gtk::prelude::{Cast, ListItemExt};
                    let list_item = this.downcast_ref::<ListItem>().unwrap();
                    let item = list_item.item().unwrap();
                    let file_info = item.downcast_ref::<FileInfo>().unwrap();
                    list_item.set_child(Some(
                        &Label::builder()
                            .halign(Align::Start)
                            .ellipsize(EllipsizeMode::Middle)
                            .label((file_info.size() as usize).bytes())
                            .build(),
                    ));
                });
                factory
            })
            .build()
    }

    fn content_type(width: i32) -> Self {
        ColumnViewColumn::builder()
            .fixed_width(width)
            .resizable(true)
            .title("Content Type")
            .factory(&{
                let factory = SignalListItemFactory::new();
                factory.connect_bind(|_, this| {
                    use gtk::prelude::{Cast, ListItemExt};
                    let list_item = this.downcast_ref::<ListItem>().unwrap();
                    let item = list_item.item().unwrap();
                    let file_info = item.downcast_ref::<FileInfo>().unwrap();
                    list_item.set_child(Some(
                        &Label::builder()
                            .halign(Align::Start)
                            .ellipsize(EllipsizeMode::Middle)
                            .label(file_info.format_content_type())
                            .build(),
                    ));
                });
                factory
            })
            .build()
    }

    fn creation_date_time(width: i32) -> Self {
        ColumnViewColumn::builder()
            .fixed_width(width)
            .resizable(true)
            .title("Created")
            .factory(&{
                let factory = SignalListItemFactory::new();
                factory.connect_bind(|_, this| {
                    use gtk::prelude::{Cast, ListItemExt};
                    let list_item = this.downcast_ref::<ListItem>().unwrap();
                    let item = list_item.item().unwrap();
                    let file_info = item.downcast_ref::<FileInfo>().unwrap();
                    list_item.set_child(Some(
                        &Label::builder()
                            .halign(Align::Start)
                            .ellipsize(EllipsizeMode::Middle)
                            .label(file_info.format_date_time())
                            .build(),
                    ));
                });
                factory
            })
            .build()
    }

    fn modification_date_time(width: i32) -> Self {
        ColumnViewColumn::builder()
            .fixed_width(width)
            .resizable(true)
            .title("Modified")
            .factory(&{
                let factory = SignalListItemFactory::new();
                factory.connect_bind(|_, this| {
                    use gtk::prelude::{Cast, ListItemExt};
                    let list_item = this.downcast_ref::<ListItem>().unwrap();
                    let item = list_item.item().unwrap();
                    let file_info = item.downcast_ref::<FileInfo>().unwrap();
                    list_item.set_child(Some(
                        &Label::builder()
                            .halign(Align::Start)
                            .ellipsize(EllipsizeMode::Middle)
                            .label(file_info.format_date_time())
                            .build(),
                    ));
                });
                factory
            })
            .build()
    }

    fn access_date_time(width: i32) -> Self {
        ColumnViewColumn::builder()
            .fixed_width(width)
            .resizable(true)
            .title("Accessed")
            .factory(&{
                let factory = SignalListItemFactory::new();
                factory.connect_bind(|_, this| {
                    use gtk::prelude::{Cast, ListItemExt};
                    let list_item = this.downcast_ref::<ListItem>().unwrap();
                    let item = list_item.item().unwrap();
                    let file_info = item.downcast_ref::<FileInfo>().unwrap();
                    list_item.set_child(Some(
                        &Label::builder()
                            .halign(Align::Start)
                            .ellipsize(EllipsizeMode::Middle)
                            .label(file_info.format_date_time())
                            .build(),
                    ));
                });
                factory
            })
            .build()
    }
}
