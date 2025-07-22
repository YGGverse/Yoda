use adw::StatusPage;
use gtk::{
    Align, Button,
    prelude::{BoxExt, ButtonExt, WidgetExt},
};

pub fn build(on_accept: impl Fn() + 'static) -> StatusPage {
    let b = gtk::Box::builder()
        .halign(Align::Center)
        .orientation(gtk::Orientation::Horizontal)
        .spacing(16)
        .build();

    b.append(&gtk::Label::builder().selectable(true).use_markup(true).label(
        "<a href=\"https://geminiprotocol.net/docs/protocol-specification.gmi#tls-server-certificate-validation\" title=\"Gemini protocol specification (HTTP link)\">Read more...</a>"
    ).build());

    b.append(&{
        let b = Button::builder()
            .css_classes(["warning"])
            .label("Accept")
            .tooltip_text("Add an exception")
            .halign(Align::Center)
            .build();

        b.connect_clicked(move |this| {
            this.set_sensitive(false);
            on_accept()
        });

        b
    });

    StatusPage::builder()
        .child(&b)
        .icon_name("security-medium-symbolic")
        .title("Server certificate has been changed")
        .description("it could be a man-in-the-middle attack")
        .build()
}
