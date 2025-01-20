pub mod certificate;
pub mod failure;
pub mod input;
pub mod redirect;
pub mod text;

// Local dependencies
pub use certificate::Certificate;
pub use failure::Failure;
pub use input::Input;
pub use redirect::Redirect;
pub use text::Text;

// Global dependencies
use gtk::{
    gio::{Cancellable, IOStream},
    glib::Uri,
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
    Input(Input),
    Redirect(Redirect),
    Stream {
        base: Uri,
        mime: String,
        stream: IOStream,
        cancellable: Cancellable,
    },
    Text(Text),
}
