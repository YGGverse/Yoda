use gtk::{
    gio::SimpleAction,
    glib::GString,
    prelude::{ActionExt, TextBufferExt, TextViewExt, WidgetExt},
    TextView, WrapMode,
};
use libspelling::{Checker, TextBufferAdapter};
use sourceview::Buffer;

const MARGIN: i32 = 8;

pub struct Widget {
    pub text_view: TextView,
}

impl Widget {
    // Construct
    pub fn new(action_update: SimpleAction) -> Self {
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
            .extra_menu(&adapter.menu_model())
            .left_margin(MARGIN)
            .right_margin(MARGIN)
            .top_margin(MARGIN)
            .wrap_mode(WrapMode::Word)
            .build();

        text_view.insert_action_group("spelling", Some(&adapter));

        // Init events
        text_view.buffer().connect_changed(move |_| {
            action_update.activate(None);
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
