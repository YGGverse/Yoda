use gtk::{Spinner, prelude::WidgetExt}; // use adw::Spinner; @TODO adw 1.6 / ubuntu 24.10+

// Defaults

const SIZE: i32 = 32; // 16-64

/// Animate loading process by the [Spinner](https://docs.gtk.org/gtk4/class.Spinner.html)
pub struct Progress {
    pub spinner: Spinner,
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
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

    // Actions

    pub fn enable(&self) {
        self.spinner.set_visible(true);
        self.spinner.start();
    }

    pub fn disable(&self) {
        self.spinner.set_visible(false);
        self.spinner.stop();
    }
}
