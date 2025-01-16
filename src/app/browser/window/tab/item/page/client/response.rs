pub mod certificate;
pub mod failure;
pub mod input;

pub use certificate::Certificate;
pub use failure::Failure;
pub use input::Input;

use gtk::{
    gio::{Cancellable, IOStream},
    glib::{GString, Uri},
};

pub enum Response {
    Certificate(Certificate),
    Download {
        base: Uri,
        stream: IOStream,
        cancellable: Cancellable,
    },
    Failure(Failure),
    Gemtext {
        base: Uri,
        source: GString,
        is_source_request: bool,
    },
    Input(Input),
    Redirect {
        request: Uri,
        is_foreground: bool,
    },
    Stream {
        base: Uri,
        mime: String,
        stream: IOStream,
        cancellable: Cancellable,
    },
}
