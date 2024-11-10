mod widget;

use widget::Widget;

use crate::app::browser::window::action::Action as WindowAction;
use gtk::Button;
use std::rc::Rc;

pub struct Forward {
    window_action: Rc<WindowAction>,
    widget: Rc<Widget>,
}

impl Forward {
    // Construct
    pub fn new(window_action: Rc<WindowAction>) -> Self {
        Self {
            window_action: window_action.clone(),
            widget: Rc::new(Widget::new(window_action)),
        }
    }

    // Actions
    pub fn update(&self, status: bool) {
        // Update actions
        self.window_action
            .history_forward()
            .gobject()
            .set_enabled(status);

        // Update child components
        self.widget.update(status);
    }

    // Getters
    pub fn gobject(&self) -> &Button {
        self.widget.gobject()
    }
}
