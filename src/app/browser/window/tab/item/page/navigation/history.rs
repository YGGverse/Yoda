pub mod back;
pub mod forward;

pub use back::Back;
pub use forward::Forward;

use super::{ItemAction, TabAction, WindowAction};
use gtk::{Box, Button, Orientation, prelude::BoxExt};
use std::rc::Rc;

pub trait History {
    fn history(action: (&Rc<WindowAction>, &Rc<TabAction>, &Rc<ItemAction>)) -> Self;
}

impl History for Box {
    fn history(action: (&Rc<WindowAction>, &Rc<TabAction>, &Rc<ItemAction>)) -> Self {
        let g_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .css_classes([
                "linked", // merge childs
            ])
            .build();

        g_box.append(&Button::back(action));
        g_box.append(&Button::forward(action));
        g_box
    }
}
