mod file;
mod header;
mod text;
mod title;

use file::File;
use gtk::{
    glib::{uuid_string_random, Bytes},
    Label, Notebook,
};
pub use header::Header;
use text::Text;
use title::Title;

pub trait Titan {
    fn titan(callback: impl Fn(Header, Bytes, Box<dyn Fn()>) + 'static) -> Self;
}

impl Titan for Notebook {
    fn titan(callback: impl Fn(Header, Bytes, Box<dyn Fn()>) + 'static) -> Self {
        use gtk::Box;
        use std::{cell::Cell, rc::Rc};

        let notebook = Notebook::builder()
            .name(format!("s{}", uuid_string_random()))
            .show_border(false)
            .build();

        let header = Rc::new(Cell::new(Header::new()));

        notebook.append_page(&Box::text(&header, callback), Some(&Label::title("Text")));
        notebook.append_page(&Box::file(), Some(&Label::title("File")));

        notebook_css_patch(&notebook);
        notebook
    }
}

// Tools

fn notebook_css_patch(notebook: &Notebook) {
    use gtk::prelude::WidgetExt;

    let name = notebook.widget_name();
    let provider = gtk::CssProvider::new();

    provider.load_from_string(&format!(
        "
            #{name} stack {{
                background: transparent;
            }}
            #{name} header {{
                border-bottom-color: transparent;
            }}
            #{name} tab {{
                opacity: 0.9;
            }}
        "
    ));

    gtk::style_context_add_provider_for_display(
        &notebook.display(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
} // @TODO replace `Notebook` with `ToggleGroup` in Adw 1.7 / Ubuntu 26.04
  // https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.ToggleGroup.html
