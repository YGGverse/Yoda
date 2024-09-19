#[path = "page/navigation.rs"] mod navigation;
#[path = "page/content.rs"] mod content;

use gtk::Box;
use gtk::prelude::BoxExt;

pub fn new() -> Box
{
    let page = Box::builder().orientation(
        gtk::Orientation::Vertical
    ).build();

    page.append(
        &navigation::new()
    );

    page.append(
        &content::new()
    );

    return page;
}