mod menu;
mod tab;
mod widget;

pub struct Tray {
    widget: widget::Tray,
}

impl Tray {
    pub fn new() -> Tray {
        Self {
            widget: widget::Tray::new(
                menu::Menu::new().widget().gtk(),
                tab::Tab::new().widget().gtk(),
            ),
        }
    }

    // Getters
    pub fn widget(&self) -> &widget::Tray {
        &self.widget
    }
}
