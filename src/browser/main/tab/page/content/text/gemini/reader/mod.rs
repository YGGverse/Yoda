use gtk::{
    prelude::{StyleContextExt, WidgetExt},
    Align, CssProvider, Label, STYLE_PROVIDER_PRIORITY_APPLICATION,
};

pub struct Reader {
    css: CssProvider,
    widget: Label,
}

impl Reader {
    // Construct
    pub fn new(gemtext: &str) -> Self {
        // Init CSS
        let css = CssProvider::new();

        css.load_from_path(
            "src/browser/main/tab/page/content/text/gemini/reader/default.css", // @TODO
        );

        // Init widget
        let widget = Label::builder()
            .halign(Align::Start)
            .valign(Align::Start)
            .vexpand(true)
            .margin_start(8)
            .margin_end(8)
            .wrap(true)
            .selectable(true)
            .use_markup(true)
            .label(gemtext) // @TODO
            .build();

        widget
            .style_context()
            .add_provider(&css, STYLE_PROVIDER_PRIORITY_APPLICATION);

        // Result
        Self { css, widget }
    }

    // Getters
    pub fn widget(&self) -> &Label {
        &self.widget
    }
}
