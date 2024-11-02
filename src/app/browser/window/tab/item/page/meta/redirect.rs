use gtk::glib::Uri;

/// # Redirection data holder
///
/// This component does nothing,
/// but useful as the container for temporary redirection data
/// operated by external controller
///
/// ## Members
///
/// * `count` - to limit redirect attempts
/// * `is_follow` - indicates how to process this redirect exactly
/// * `target` - destination address
pub struct Redirect {
    count: i8,
    is_follow: bool,
    target: Uri,
}

impl Redirect {
    pub fn new(count: i8, is_follow: bool, target: Uri) -> Self {
        Self {
            count,
            is_follow,
            target,
        }
    }

    pub fn count(&self) -> &i8 {
        &self.count
    }

    pub fn is_follow(&self) -> &bool {
        &self.is_follow
    }

    pub fn target(&self) -> &Uri {
        &self.target
    }
}
