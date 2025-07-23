use gtk::{TextView, WrapMode, prelude::WidgetExt};
//use libspelling::{Checker, TextBufferAdapter};
use sourceview::Buffer;

pub trait Form {
    fn form() -> Self;
}

impl Form for TextView {
    // Constructors

    /// Build new `Self`
    fn form() -> Self {
        // Init [SourceView](https://gitlab.gnome.org/GNOME/gtksourceview) type buffer
        let buffer = Buffer::builder().build();

        // Init [libspelling](https://gitlab.gnome.org/GNOME/libspelling)
        /*let checker = Checker::default();
        let adapter = TextBufferAdapter::new(&buffer, &checker);
        adapter.set_enabled(true);*/

        // Init main widget

        let text_view = {
            const MARGIN: i32 = 8;
            TextView::builder()
                .bottom_margin(MARGIN)
                .buffer(&buffer)
                .css_classes(["frame", "view"])
                //.extra_menu(&adapter.menu_model())
                .left_margin(MARGIN)
                .right_margin(MARGIN)
                .top_margin(MARGIN)
                .wrap_mode(WrapMode::Word)
                .build()
        };

        //text_view.insert_action_group("spelling", Some(&adapter));
        text_view.set_size_request(-1, 38); // @TODO [#635](https://gitlab.gnome.org/GNOME/pygobject/-/issues/635)

        // Init events
        text_view.connect_realize(|this| {
            this.grab_focus();
        });

        // Return activated `Self`
        text_view
    }
}
