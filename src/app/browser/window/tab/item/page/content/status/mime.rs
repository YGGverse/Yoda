use super::ItemAction;
use adw::StatusPage;
use gtk::{glib::Uri, prelude::ButtonExt, Align, Button};
use std::rc::Rc;

/// Create new default `GObject` preset for mime issue
/// [StatusPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.StatusPage.html)
pub fn build(mime: &str, download: Option<(&Rc<ItemAction>, &Uri)>) -> StatusPage {
    let status_page = StatusPage::builder()
        .description(format!("Content type `{mime}` not supported!"))
        .icon_name("dialog-question-symbolic")
        .title("Oops")
        .build();

    if let Some((action, request)) = download {
        let button = Button::builder()
            .css_classes(["suggested-action"])
            .halign(Align::Center)
            .label("Download")
            .tooltip_text("Download as file to open with external application")
            .build();

        button.connect_clicked({
            let action = action.clone();
            let request = request.clone();
            move |_| {
                action
                    .load
                    .activate(Some(&format!("download:{}", request)), true);
            }
        });

        status_page.set_child(Some(&button));
    }

    status_page
}
