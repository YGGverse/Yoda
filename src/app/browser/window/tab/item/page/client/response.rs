pub mod certificate;
pub mod failure;
pub mod input;
pub mod redirect;

// Local dependencies
pub use certificate::Certificate;
pub use failure::Failure;
pub use input::Input;
pub use redirect::Redirect;

// Global dependencies
use gtk::{
    gio::{Cancellable, IOStream},
    glib::{GString, Uri},
};

/// Single `Client` response API for all protocol drivers
pub enum Response {
    Certificate(Certificate),
    Download {
        base: Uri,
        stream: IOStream,
        cancellable: Cancellable,
    },
    Failure(Failure),
    TextGemini {
        base: Uri,
        source: GString,
        is_source_request: bool,
    },
    Input(Input),
    Redirect(Redirect),
    Stream {
        base: Uri,
        mime: String,
        stream: IOStream,
        cancellable: Cancellable,
    },
}
