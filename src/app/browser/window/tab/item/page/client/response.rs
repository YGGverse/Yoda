pub mod certificate;
pub mod failure;
pub mod input;

// Local dependencies
pub use certificate::Certificate;
pub use failure::Failure;
pub use input::Input;

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
