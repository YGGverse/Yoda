use gtk::Button;

pub trait Form {
    fn form() -> Self;
}

impl Form for Button {
    fn form() -> Self {
        use gtk::prelude::{ButtonExt, WidgetExt};

        const MARGIN: i32 = 8;

        let button = Button::builder()
            .label("Choose a file..")
            .margin_bottom(MARGIN)
            .margin_top(MARGIN)
            .build();

        button.connect_clicked(|this| {
            this.set_sensitive(false); // lock
        });

        button
    }
}
