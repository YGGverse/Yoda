use gtk::{PackType, WindowControls};

const MARGIN: i32 = 4;

pub struct Control {
    pub window_controls: WindowControls,
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
