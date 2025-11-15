use gtk::{
    TextView, WrapMode,
    glib::GString,
    prelude::{TextBufferExt, TextViewExt, WidgetExt},
};
use libspelling::{Checker, TextBufferAdapter};
use sourceview::Buffer;

const MARGIN: i32 = 8;

pub trait Form {
    fn form() -> Self;
    fn text(&self) -> GString;
}

impl Form for TextView {
    // Constructors

    /// Build new `Self`
    fn form() -> Self {
        // Init [SourceView](https://gitlab.gnome.org/GNOME/gtksourceview) type buffer
        let buffer = Buffer::builder().build();

        // Init [libspelling](https://gitlab.gnome.org/GNOME/libspelling)
        let checker = Checker::default();
        let adapter = TextBufferAdapter::builder()
            .buffer(&buffer)
            .checker(&checker)
            .enabled(true)
            .build();

        // Init main widget
        let text_view = TextView::builder()
            .bottom_margin(MARGIN)
            .buffer(&buffer)
            .css_classes(["frame", "view"])
            .extra_menu(&adapter.menu_model())
            .left_margin(MARGIN)
            .margin_bottom(MARGIN / 4)
            .right_margin(MARGIN)
            .top_margin(MARGIN)
            .wrap_mode(WrapMode::Word)
            .build();

        text_view.insert_action_group("spelling", Some(&adapter));

        // Init events
        text_view.connect_realize(|this| {
            this.grab_focus();
        });

        // Return activated `Self`
        text_view
    }

    // Getters

    fn text(&self) -> GString {
        let buffer = self.buffer();
        buffer.text(&buffer.start_iter(), &buffer.end_iter(), true)
    }
}
