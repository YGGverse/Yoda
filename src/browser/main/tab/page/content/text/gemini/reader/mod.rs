mod parser;

use parser::header::Header;
use parser::link::Link;
use parser::plain::Plain;

use gtk::{
    glib::{GString, Propagation, Uri, UriFlags},
    prelude::{StyleContextExt, ToVariant, WidgetExt},
    Align, CssProvider, Label, STYLE_PROVIDER_PRIORITY_APPLICATION,
};

pub struct Reader {
    title: Option<GString>,
    css: CssProvider,
    widget: Label,
}

impl Reader {
    // Construct
    pub fn new(gemtext: &str, base: &Uri) -> Self {
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

            // Is link
            if let Some(link) = Link::from(line, base) {
                // Format
                markup.push_str(link.markup());

                continue;
            }

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
            .halign(Align::Fill)
            .valign(Align::Fill)
            .hexpand(true)
            .vexpand(true)
            .xalign(0.0)
            .yalign(0.0)
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

        // Connect actions
        widget.connect_activate_link(|label, href| {
            // Detect requested protocol
            if let Ok(uri) = Uri::parse(&href, UriFlags::NONE) {
                return match uri.scheme().as_str() {
                    "gemini" => {
                        // Open new page
                        label
                            .activate_action("page.open", Some(&uri.to_str().to_variant()))
                            .expect("Action `page.open` not found");

                        // Prevent link open in external application
                        Propagation::Stop
                    }
                    // Protocol not supported
                    _ => Propagation::Proceed,
                };
            }

            // Delegate unparsable
            Propagation::Proceed
        });

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
