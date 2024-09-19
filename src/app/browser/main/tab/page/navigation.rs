#[path = "navigation/base.rs"] mod base;
#[path = "navigation/history.rs"] mod history;
#[path = "navigation/reload.rs"] mod reload;
#[path = "navigation/request.rs"] mod request;
#[path = "navigation/bookmark.rs"] mod bookmark;

use gtk::Box;
use gtk::prelude::BoxExt;

pub fn new() -> Box
{
    let navigation = Box::builder()

        // Tuneup
        .orientation(
            gtk::Orientation::Horizontal
        )

        .spacing(8)

        .margin_top(8)
        .margin_start(8)
        .margin_end(8)
        .margin_bottom(8)

        .build();

        // Compose childs
        navigation.append(
            &base::new()
        );

        navigation.append(
            &history::new()
        );

        navigation.append(
            &reload::new()
        );

        navigation.append(
            &request::new()
        );

        navigation.append(
            &bookmark::new()
        );

    return navigation;
}