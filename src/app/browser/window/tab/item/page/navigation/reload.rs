mod widget;

use widget::Widget;

use super::WindowAction;
use std::rc::Rc;

pub struct Reload {
    action: Rc<WindowAction>,
    pub widget: Rc<Widget>,
}

impl Reload {
    // Constructors

    /// Build new `Self`
    pub fn build(action: &Rc<WindowAction>) -> Self {
        Self {
            action: action.clone(),
            widget: Rc::new(Widget::build(action)),
        }
    }

    // Actions

    pub fn update(&self, is_enabled: bool) {
        // Update actions
        self.action.reload.simple_action.set_enabled(is_enabled);

        // Update child components
        self.widget.update(is_enabled);
    }
}
