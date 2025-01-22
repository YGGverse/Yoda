use gtk::{
    gio::SimpleAction,
    glib::GString,
    prelude::{ActionExt, TextBufferExt, TextViewExt, WidgetExt},
    TextView, WrapMode,
};
use libspelling::{Checker, TextBufferAdapter};
use sourceview::Buffer;

const MARGIN: i32 = 8;

pub struct Form {
    pub text_view: TextView,
}

impl Form {
    // Constructors

    /// Build new `Self`
    pub fn build(action_update: SimpleAction) -> Self {
        // Init [SourceView](https://gitlab.gnome.org/GNOME/gtksourceview) type buffer
        let buffer = Buffer::builder().build();

        // Init [libspelling](https://gitlab.gnome.org/GNOME/libspelling)
        let checker = Checker::default();
        let adapter = TextBufferAdapter::new(&buffer, &checker);
        adapter.set_enabled(true);

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
        text_view.set_size_request(-1, 38); // @TODO [#635](https://gitlab.gnome.org/GNOME/pygobject/-/issues/635)

        // Init events
        text_view.buffer().connect_changed(move |_| {
            action_update.activate(None);
        });

        text_view.connect_realize(|this| {
            this.grab_focus();
        });

        // Return activated `Self`
        Self { text_view }
    }

    // Getters

    pub fn text(&self) -> GString {
        let buffer = self.text_view.buffer();
        buffer.text(&buffer.start_iter(), &buffer.end_iter(), true)
    }
}
