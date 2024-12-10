mod cancel;
mod choose;
mod open;
mod progress;
mod status;

use cancel::Cancel;
use choose::Choose;
use open::Open;
use progress::Progress;
use status::Status;

use adw::StatusPage;
use gtk::{
    gio::{Cancellable, File},
    prelude::{BoxExt, CancellableExt, WidgetExt},
    Box, FileDialog, FileLauncher, Label, Orientation, Window,
};
use std::rc::Rc;

/// Create new [StatusPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.StatusPage.html)
/// with progress indication and UI controls
/// * applies callback function once on destination [File](https://docs.gtk.org/gio/iface.File.html) selected
/// * requires external IOStream read/write implementation, depending of protocol driver in use
pub fn new(
    initial_filename: &str,
    cancellable: &Cancellable,
    on_choose: impl Fn(File, Label) + 'static,
) -> StatusPage {
    // Init components
    let dialog = FileDialog::builder().initial_name(initial_filename).build();
    let file_launcher = FileLauncher::new(File::NONE);

    let cancel = Rc::new(Cancel::new());
    let choose = Rc::new(Choose::new());
    let open = Rc::new(Open::new());
    let progress = Rc::new(Progress::new());
    let status = Rc::new(Status::new());

    // Init events
    cancel.on_activate({
        let cancellable = cancellable.clone();
        let progress = progress.clone();
        let status = status.clone();
        move |_, button| {
            // cancel all operations
            cancellable.cancel();

            // deactivate `spinner`
            progress.disable();

            // update `status`
            status.set_warning("Operation cancelled");

            // hide self
            button.set_visible(false);
        }
    });

    choose.on_activate({
        // init shared references
        let cancellable = cancellable.clone();
        let cancel = cancel.clone();
        let dialog = dialog.clone();
        let file_launcher = file_launcher.clone();
        let progress = progress.clone();
        let status = status.clone();
        let on_choose = Rc::new(on_choose);
        move |_, button| {
            // lock choose button to prevent double click
            button.set_sensitive(false);
            dialog.save(Window::NONE, Some(&cancellable), {
                // delegate shared references
                let cancel = cancel.clone();
                let file_launcher = file_launcher.clone();
                let progress = progress.clone();
                let status = status.clone();
                let button = button.clone();
                let on_choose = on_choose.clone();
                move |result| {
                    button.set_sensitive(true); // unlock
                    match result {
                        Ok(file) => {
                            // update destination file
                            file_launcher.set_file(Some(&file));

                            // update `status`
                            status.set_default("Loading...");

                            // show `cancel` button
                            cancel.button.set_visible(true);

                            // show `spinner`
                            progress.enable();

                            // hide self
                            button.set_visible(false);

                            // callback
                            on_choose(file, status.label.clone())
                        }
                        Err(e) => {
                            // update destination file
                            file_launcher.set_file(File::NONE);

                            // update `spinner`
                            progress.disable();

                            // update `status`
                            status.set_warning(e.message());
                        }
                    }
                }
            });
        }
    });

    open.on_activate({
        let cancellable = cancellable.clone();
        let file_launcher = file_launcher.clone();
        let status = status.clone();
        move |_, button| {
            button.set_sensitive(false); // lock
            file_launcher.launch(Window::NONE, Some(&cancellable), {
                let status = status.clone();
                let button = button.clone();
                move |result| {
                    if let Err(ref e) = result {
                        status.set_error(e.message())
                    }
                    button.set_sensitive(true); // unlock
                }
            })
        }
    });

    // Init `child` as the container for extra features
    let child = Box::builder().orientation(Orientation::Vertical).build();

    child.append(&progress.spinner);
    child.append(&status.label);
    child.append(&cancel.button);
    child.append(&choose.button);
    child.append(&open.button);

    // Init main widget
    StatusPage::builder()
        .child(&child)
        .icon_name("document-save-symbolic")
        .title("Download")
        .build()
}
