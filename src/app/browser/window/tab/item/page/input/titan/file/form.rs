use gtk::Button;

pub trait Form {
    fn form() -> Self;
}

impl Form for Button {
    fn form() -> Self {
        use gtk::prelude::{ButtonExt, WidgetExt};

        let button = Button::builder()
            .label("Choose a file..")
            .margin_top(4)
            .build();

        button.connect_clicked(|this| {
            this.set_sensitive(false); // lock
        });

        button
    }
}
