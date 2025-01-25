pub mod back;
pub mod forward;

pub use back::Back;
pub use forward::Forward;

use gtk::{prelude::BoxExt, Box, Button, Orientation};

pub trait History {
    fn history(back_action_name: &str, forward_action_name: &str) -> Self;
}

impl History for Box {
    fn history(back_action_name: &str, forward_action_name: &str) -> Self {
        let g_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .css_classes([
                "linked", // merge childs
            ])
            .build();

        g_box.append(&Button::back(back_action_name));
        g_box.append(&Button::forward(forward_action_name));
        g_box
    }
}
