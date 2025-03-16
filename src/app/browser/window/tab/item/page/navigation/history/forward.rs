use super::{ItemAction, TabAction, WindowAction};
use crate::app::browser::window::action::Position;
use gtk::{
    Button, GestureClick,
    gdk::BUTTON_MIDDLE,
    prelude::{ActionExt, WidgetExt},
};
use std::rc::Rc;

pub trait Forward {
    fn forward(action: (&Rc<WindowAction>, &Rc<TabAction>, &Rc<ItemAction>)) -> Self;
}

impl Forward for Button {
    fn forward(
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
                item_action.history.forward.name()
            ))
            .icon_name("go-next-symbolic")
            .tooltip_text("Forward")
            .build();

        // Ability to open next history record in the new tab (without change current page state)
        let new_tab_controller = GestureClick::builder().button(BUTTON_MIDDLE).build();

        new_tab_controller.connect_pressed({
            let item_action = item_action.clone();
            let window_action = window_action.clone();
            move |_, _, _, _| {
                if let Some(request) = item_action.history.forward(false) {
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
