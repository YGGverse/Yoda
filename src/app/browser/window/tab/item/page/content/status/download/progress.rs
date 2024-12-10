use gtk::Spinner; // use adw::Spinner; @TODO adw 1.6 / ubuntu 24.10+

// Defaults

const SIZE: i32 = 32; // 16-64

/// Animate loading process by the [Spinner](https://docs.gtk.org/gtk4/class.Spinner.html)
pub struct Progress {
    pub spinner: Spinner,
}

impl Progress {
    // Constructors

    /// Create new `Self`
    pub fn new() -> Self {
        Self {
            spinner: Spinner::builder()
                .height_request(SIZE)
                .visible(false)
                .width_request(SIZE)
                .build(),
        }
    }
}
