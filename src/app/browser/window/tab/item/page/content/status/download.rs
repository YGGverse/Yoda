mod action;
mod cancel;
mod choose;
mod open;
mod progress;
mod status;

pub use action::Action;
use cancel::Cancel;
use choose::Choose;
use open::Open;
use progress::Progress;
use status::Status;

use adw::StatusPage;
use gtk::{
    Box, FileDialog, FileLauncher, Orientation, Window,
    gio::{Cancellable, File},
    prelude::{BoxExt, CancellableExt, WidgetExt},
};
use std::rc::Rc;

// Defaults

const ICON_NAME: &str = "document-save-symbolic";
const STATUS_CANCELLED: &str = "Operation cancelled";
const STATUS_LOADING: &str = "Loading...";
const TITLE: &str = "Download";

/// Create new [StatusPage](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/class.StatusPage.html)
/// preset with children widget contain download UI
/// * apply callback function on destination [File](https://docs.gtk.org/gio/iface.File.html) selected
/// * require external IOStream read/write implementation (depending of protocol)
pub fn build(
    initial_filename: &str,
    cancellable: &Cancellable,
    on_choose: impl Fn(File, Rc<Action>) + 'static,
) -> StatusPage {
    // Init components
    let dialog = FileDialog::builder().initial_name(initial_filename).build();
    let file_launcher = FileLauncher::new(File::NONE);

    let action = Rc::new(Action::new()); // public callback API

    let cancel = Rc::new(Cancel::new());
    let choose = Rc::new(Choose::new(true)); // @TODO optional `is_activate_on_release` value
    let open = Rc::new(Open::new());
    let progress = Rc::new(Progress::new());
    let status = Rc::new(Status::new());

    // Init action events
    action.cancel.on_activate({
        let cancel = cancel.clone();
        let cancellable = cancellable.clone();
        let progress = progress.clone();
        let status = status.clone();
        move |_, message| {
            cancellable.cancel();
            progress.disable();
            status.set_error(&message);
            cancel.button.set_visible(false);
        }
    });

    action.complete.on_activate({
        let cancel = cancel.clone();
        let cancellable = cancellable.clone();
        let open = open.clone();
        let progress = progress.clone();
        let status = status.clone();
        move |_, message| {
            cancellable.cancel();
            progress.disable();
            status.set_success(&message);
            cancel.button.set_visible(false);
            open.button.set_visible(true);
        }
    });

    action.update.on_activate({
        let status = status.clone();
        move |_, message| status.set_default(&message)
    });

    // Init widget events
    cancel.on_activate({
        let cancellable = cancellable.clone();
        let progress = progress.clone();
        let status = status.clone();
        move |button| {
            button.set_sensitive(false);
            button.set_visible(false);
            cancellable.cancel();
            progress.disable();
            status.set_warning(STATUS_CANCELLED);
        }
    });

    choose.on_activate({
        let cancellable = cancellable.clone();
        let cancel = cancel.clone();
        let dialog = dialog.clone();
        let file_launcher = file_launcher.clone();
        let progress = progress.clone();
        let status = status.clone();
        let on_choose = Rc::new(on_choose);
        move |button| {
            button.set_sensitive(false); // lock
            dialog.save(Window::NONE, Some(&cancellable), {
                let action = action.clone();
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
                            file_launcher.set_file(Some(&file));
                            status.set_default(STATUS_LOADING);
                            cancel.button.set_visible(true);
                            progress.enable();
                            button.set_visible(false);
                            on_choose(file, action)
                        }
                        Err(e) => {
                            file_launcher.set_file(File::NONE);
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
        move |button| {
            button.set_sensitive(false); // lock
            file_launcher.launch(Window::NONE, Some(&cancellable), {
                let button = button.clone();
                move |_| button.set_sensitive(true) // unlock
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
        .icon_name(ICON_NAME)
        .title(TITLE)
        .build()
}
