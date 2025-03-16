use super::{ItemAction, TabAction, WindowAction};
use crate::app::browser::window::action::Position;
use gtk::{
    Button, GestureClick,
    gdk::BUTTON_MIDDLE,
    prelude::{ActionExt, WidgetExt},
};
use std::rc::Rc;

pub trait Back {
    fn back(action: (&Rc<WindowAction>, &Rc<TabAction>, &Rc<ItemAction>)) -> Self;
}

impl Back for Button {
    fn back(
        (window_action, tab_action, item_action): (
            &Rc<WindowAction>,
            &Rc<TabAction>,
            &Rc<ItemAction>,
        ),
    ) -> Self {
        // Init main widget
        let button = Button::builder()
            .action_name(format!(
                "{}.{}",
                tab_action.id,
                item_action.history.back.name()
            ))
            .icon_name("go-previous-symbolic")
            .tooltip_text("Back")
            .build();

        // Ability to open previous history record in the new tab (without change current page state)
        let new_tab_controller = GestureClick::builder().button(BUTTON_MIDDLE).build();

        new_tab_controller.connect_pressed({
            let item_action = item_action.clone();
            let window_action = window_action.clone();
            move |_, _, _, _| {
                if let Some(request) = item_action.history.back(false) {
                    window_action.append.activate_stateful_once(
                        Position::After,
                        Some(request.to_string()),
                        false,
                        true,
                        false,
                        true,
                    );
                }
            }
        });
        button.add_controller(new_tab_controller);
        button
    }
}
