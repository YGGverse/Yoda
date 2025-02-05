mod file;
mod text;
mod title;

use file::File;
use gtk::{glib::uuid_string_random, prelude::WidgetExt, Label, Notebook};
use text::Text;
use title::Title;

pub trait Titan {
    fn titan(callback: impl Fn(&[u8], Box<dyn Fn()>) + 'static) -> Self;
}

impl Titan for Notebook {
    fn titan(callback: impl Fn(&[u8], Box<dyn Fn()>) + 'static) -> Self {
        let notebook = Notebook::builder()
            .name(format!("s{}", uuid_string_random()))
            .show_border(false)
            .build();

        notebook.append_page(&gtk::Box::text(callback), Some(&Label::title("Text")));
        notebook.append_page(&gtk::Box::file(), Some(&Label::title("File")));

        notebook_css_patch(&notebook);
        notebook
    }
}

// Tools

fn notebook_css_patch(notebook: &Notebook) {
    let name = notebook.widget_name();
    let provider = gtk::CssProvider::new();

    provider.load_from_string(&format!(
        "
            #{name} stack {{
                background-color:rgba(0,0,0,0);
            }}
            #{name} header {{
                border-bottom-color:rgba(0,0,0,0);
            }}
            #{name} tab {{
                opacity:0.9;
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
