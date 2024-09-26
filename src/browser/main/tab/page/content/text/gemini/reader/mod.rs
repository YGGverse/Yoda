mod parser;

use parser::header::Header;
use parser::plain::Plain;

use gtk::{
    glib::GString,
    prelude::{StyleContextExt, WidgetExt},
    Align, CssProvider, Label, STYLE_PROVIDER_PRIORITY_APPLICATION,
};

pub struct Reader {
    title: Option<GString>,
    css: CssProvider,
    widget: Label,
}

impl Reader {
    // Construct
    pub fn new(gemtext: &str) -> Self {
        // Init title
        let mut title = None;

        // Init markup
        let mut markup = String::new();

        for line in gemtext.lines() {
            // Is header
            if let Some(header) = Header::from(line) {
                // Format
                markup.push_str(header.markup());

                // Set title from first document header tag
                if title == None {
                    title = Some(header.text().clone());
                }

                continue;
            }

            // Is link @TODO

            // Nothing match, escape string just
            markup.push_str(Plain::from(line).markup())
        }

        // Init CSS
        let css = CssProvider::new();

        css.load_from_path(
            "src/browser/main/tab/page/content/text/gemini/reader/default.css", // @TODO
        );

        // Init widget
        let widget = Label::builder()
            .halign(Align::Start)
            .valign(Align::Start)
            .hexpand(true) // @TODO
            .vexpand(true)
            .margin_start(8)
            .margin_end(8)
            .wrap(true)
            .selectable(true)
            .use_markup(true)
            .label(markup)
            .build();

        widget
            .style_context()
            .add_provider(&css, STYLE_PROVIDER_PRIORITY_APPLICATION);

        // Result
        Self { title, css, widget }
    }

    // Getters
    pub fn title(&self) -> &Option<GString> {
        &self.title
    }

    pub fn widget(&self) -> &Label {
        &self.widget
    }
}
