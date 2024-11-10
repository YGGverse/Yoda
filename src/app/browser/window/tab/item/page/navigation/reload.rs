mod widget;

use widget::Widget;

use crate::app::browser::window::action::Action as WindowAction;
use gtk::Button;
use std::rc::Rc;

pub struct Reload {
    window_action: Rc<WindowAction>,
    widget: Rc<Widget>,
}

impl Reload {
    // Construct
    pub fn new_rc(window_action: Rc<WindowAction>) -> Rc<Self> {
        Rc::new(Self {
            window_action: window_action.clone(),
            widget: Widget::new_rc(window_action),
        })
    }

    // Actions
    pub fn update(&self, is_enabled: bool) {
        // Update actions
        self.window_action
            .reload()
            .gobject()
            .set_enabled(is_enabled);

        // Update child components
        self.widget.update(is_enabled);
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        self.widget.gobject()
    }
}
