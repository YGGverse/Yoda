mod control;
mod file;
mod header;
mod tab;
mod text;

use control::Control;
use file::File;
use gtk::{
    glib::Bytes,
    prelude::{DisplayExt, WidgetExt},
    Notebook,
};
pub use header::Header;
use tab::Tab;
use text::Text;

pub trait Titan {
    fn titan(callback: impl Fn(Header, Bytes, Box<dyn Fn()>) + 'static) -> Self;
}

impl Titan for gtk::Box {
    fn titan(callback: impl Fn(Header, Bytes, Box<dyn Fn()>) + 'static) -> Self {
        use gtk::{glib::uuid_string_random, prelude::ButtonExt, Label};
        use std::rc::Rc;

        // Init components
        let control = Rc::new(Control::build());
        let file = Rc::new(File::build(&control));
        let text = Rc::new(Text::build(&control));

        let notebook = {
            let notebook = Notebook::builder()
                .name(format!("s{}", uuid_string_random()))
                .show_border(false)
                .build();

            notebook.append_page(&text.text_view, Some(&Label::tab("Text")));
            notebook.append_page(&file.button, Some(&Label::tab("File")));

            notebook.connect_switch_page({
                let control = control.clone();
                let file = file.clone();
                let text = text.clone();
                move |_, _, tab_number| {
                    if tab_number == 0 {
                        control.update(Some(text.len()), Some(text.count()))
                    } else {
                        control.update(file.size(), None)
                    }
                }
            });

            notebook_css_patch(&notebook);
            notebook
        };

        // Init main widget
        let g_box = {
            use gtk::{prelude::BoxExt, Box, Orientation};

            let g_box = {
                const MARGIN: i32 = 8;
                Box::builder()
                    .margin_end(MARGIN)
                    .margin_start(MARGIN)
                    .orientation(Orientation::Vertical)
                    .spacing(MARGIN)
                    .build()
            };

            g_box.append(&notebook);
            g_box.append(&control.g_box);
            g_box
        };

        // Init events
        text.text_view.add_controller({
            const SHORTCUT: &str = "<Primary>Return"; // @TODO optional
            let c = gtk::ShortcutController::new();
            c.add_shortcut(
                gtk::Shortcut::builder()
                    .trigger(&gtk::ShortcutTrigger::parse_string(SHORTCUT).unwrap())
                    .action(&gtk::CallbackAction::new({
                        let u = control.upload.clone();
                        move |_, _| {
                            if u.is_sensitive() {
                                u.emit_activate();
                            } else {
                                u.display().beep();
                            }
                            gtk::glib::Propagation::Stop
                        }
                    }))
                    .build(),
            );
            c
        });

        control.options.connect_clicked({
            let text = text.clone();
            let file = file.clone();
            let notebook = notebook.clone();
            move |this| {
                use gtk::prelude::WidgetExt;
                this.set_sensitive(false); // lock
                let tab_number = notebook.current_page().unwrap();
                match tab_number {
                    0 => text.header(),
                    1 => file.header(),
                    _ => panic!(),
                }
                .dialog(Some(this), {
                    let this = this.clone();
                    let text = text.clone();
                    let file = file.clone();
                    move |header| {
                        match tab_number {
                            0 => text.set_header(header),
                            1 => file.set_header(header),
                            _ => panic!(),
                        };
                        this.set_sensitive(true); // unlock
                    }
                })
            }
        });

        control.upload.connect_clicked({
            move |this| {
                use control::Upload;
                this.set_uploading();
                let tab_number = notebook.current_page().unwrap();
                callback(
                    match tab_number {
                        0 => text.header(),
                        1 => file.header(),
                        _ => panic!(),
                    },
                    match tab_number {
                        0 => text.bytes(),
                        1 => file.bytes().unwrap(),
                        _ => panic!(),
                    },
                    Box::new({
                        let this = this.clone();
                        move || this.set_resend() // re-activate button on failure
                    }),
                )
            }
        });

        g_box
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
