#[path = "header/subject.rs"] mod subject;
#[path = "header/tray.rs"] mod tray;

use gtk::HeaderBar;

pub fn new() -> HeaderBar
{
    let header = HeaderBar::builder().build();

        header.pack_start(
            &tray::new()
        );

        header.set_title_widget(
            Some(
                &subject::new()
            )
        );

    header
}