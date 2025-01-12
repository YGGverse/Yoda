use gtk::{PackType, WindowControls};

const MARGIN: i32 = 4;

pub struct Control {
    pub window_controls: WindowControls,
}

impl Default for Control {
    fn default() -> Self {
        Self::new()
    }
}

impl Control {
    // Construct
    pub fn new() -> Self {
        Self {
            window_controls: WindowControls::builder()
                .margin_end(MARGIN)
                .side(PackType::End)
                .build(),
        }
    }
}
