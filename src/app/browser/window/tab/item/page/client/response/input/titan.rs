use gtk::{
    gio::{Cancellable, IOStream},
    glib::{Bytes, Error, Priority},
    prelude::{IOStreamExt, OutputStreamExt},
};

pub struct Titan {
    cancellable: Cancellable,
    stream: IOStream,
}

impl Titan {
    // Actions

    pub fn send(&self, data: Bytes, callback: impl FnOnce(Result<isize, Error>) + 'static) {
        self.stream.output_stream().write_bytes_async(
            &data,
            Priority::DEFAULT,
            Some(&self.cancellable),
            callback,
        )
    }
}
