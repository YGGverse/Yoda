use gtk::gio::{Cancellable, IOStream};

pub struct Titan {
    cancellable: Cancellable,
    stream: IOStream,
}
