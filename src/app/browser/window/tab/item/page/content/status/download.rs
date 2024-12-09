use adw::StatusPage;
use gtk::{
    gio::{Cancellable, File},
    prelude::{BoxExt, ButtonExt, CancellableExt, WidgetExt},
    Align,
    Box,
    Button,
    FileDialog,
    FileLauncher,
    Label,
    Orientation,
    Spinner, // use adw::Spinner; @TODO adw 1.6 / ubuntu 24.10+
    Window,
};
use std::rc::Rc;

const MARGIN: i32 = 16;
const SPINNER_SIZE: i32 = 32; // 16-64

/// Create new [StatusPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.StatusPage.html)
/// with progress indication and UI controls
/// * applies callback function once on destination [File](https://docs.gtk.org/gio/iface.File.html) selected
/// * requires external IOStream read/write implementation, depending of protocol driver in use
pub fn new(
    initial_filename: &str,
    cancellable: &Cancellable,
    on_choose: impl Fn(File, Label) + 'static,
) -> StatusPage {
    // Init file chooser dialog
    let dialog = FileDialog::builder().initial_name(initial_filename).build();

    // Init file launcher dialog
    let file_launcher = FileLauncher::new(File::NONE);

    // Init spinner component
    let spinner = Spinner::builder()
        .height_request(SPINNER_SIZE)
        .visible(false)
        .width_request(SPINNER_SIZE)
        .build();

    // Init `status` feature
    // * indicates current download state in text label
    let status = Label::builder()
        .label("Choose location to download")
        .margin_top(MARGIN)
        .build();

    // Init `cancel` feature
    // * applies shared `Cancellable`
    let cancel = Button::builder()
        .css_classes(["error"])
        .halign(Align::Center)
        .label("Cancel")
        .margin_top(MARGIN)
        .visible(false)
        .build();

    cancel.connect_clicked({
        let cancellable = cancellable.clone();
        let spinner = spinner.clone();
        let status = status.clone();
        move |this| {
            // apply cancellable
            cancellable.cancel();

            // deactivate `spinner`
            spinner.set_visible(false);
            spinner.stop();

            // update `status`
            status.set_css_classes(&["warning"]);
            status.set_label("Operation cancelled");

            // hide self
            this.set_visible(false);
        }
    });

    // Init `open` feature
    // * open selected file on download complete
    let open = Button::builder()
        .css_classes(["accent"])
        .halign(Align::Center)
        .label("Open")
        .margin_top(MARGIN)
        .visible(false)
        .build();

    open.connect_clicked({
        let file_launcher = file_launcher.clone();
        let status = status.clone();
        move |this| {
            this.set_sensitive(false); // lock
            file_launcher.launch(Window::NONE, Cancellable::NONE, {
                let status = status.clone();
                let this = this.clone();
                move |result| {
                    if let Err(ref e) = result {
                        status.set_css_classes(&["error"]);
                        status.set_label(e.message())
                    }
                    this.set_sensitive(true); // unlock
                }
            })
        }
    });

    // Init `choose` feature
    // * select file destination for download
    let choose = Button::builder()
        .css_classes(["accent"])
        .halign(Align::Center)
        .label("Choose..")
        .margin_top(MARGIN)
        .build();

    choose.connect_clicked({
        // init shared references
        let cancel = cancel.clone();
        let dialog = dialog.clone();
        let file_launcher = file_launcher.clone();
        let spinner = spinner.clone();
        let status = status.clone();
        let on_choose = Rc::new(on_choose);
        move |this| {
            // lock choose button to prevent double click
            this.set_sensitive(false);
            dialog.save(Window::NONE, Cancellable::NONE, {
                // delegate shared references
                let cancel = cancel.clone();
                let file_launcher = file_launcher.clone();
                let spinner = spinner.clone();
                let status = status.clone();
                let this = this.clone();
                let on_choose = on_choose.clone();
                move |result| {
                    this.set_sensitive(true); // unlock
                    match result {
                        Ok(file) => {
                            // update destination file
                            file_launcher.set_file(Some(&file));

                            // update `status`
                            status.set_css_classes(&[]);
                            status.set_label("Loading...");

                            // show `cancel` button
                            cancel.set_visible(true);

                            // show `spinner`
                            spinner.set_visible(true);
                            spinner.start();

                            // hide self
                            this.set_visible(false);

                            // callback
                            on_choose(file, status)
                        }
                        Err(e) => {
                            // update destination file
                            file_launcher.set_file(File::NONE);

                            // update `spinner`
                            spinner.set_visible(false);
                            spinner.stop();

                            // update `status`
                            status.set_css_classes(&["warning"]);
                            status.set_label(e.message())
                        }
                    }
                }
            });
        }
    });

    // Init main container
    let child = Box::builder().orientation(Orientation::Vertical).build();

    child.append(&spinner);
    child.append(&status);
    child.append(&cancel);
    child.append(&choose);
    child.append(&open);

    // Done
    StatusPage::builder()
        .child(&child)
        .icon_name("document-save-symbolic")
        .title("Download")
        .build()
}
