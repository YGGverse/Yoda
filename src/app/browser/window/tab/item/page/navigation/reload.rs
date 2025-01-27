use super::{ItemAction, Request, TabAction, WindowAction};
use crate::app::browser::window::action::Position;
use gtk::{
    gdk::BUTTON_MIDDLE,
    prelude::{ActionExt, WidgetExt},
    Button, Entry, GestureClick,
};
use std::rc::Rc;

pub trait Reload {
    fn reload(
        action: (&Rc<WindowAction>, &Rc<TabAction>, &Rc<ItemAction>),
        request: &Entry,
    ) -> Self;
}

impl Reload for Button {
    fn reload(
        (window_action, tab_action, item_action): (
            &Rc<WindowAction>,
            &Rc<TabAction>,
            &Rc<ItemAction>,
        ),
        request: &Entry,
    ) -> Self {
        let button = Button::builder()
            .action_name(format!("{}.{}", tab_action.id, item_action.reload.name()))
            .icon_name("view-refresh-symbolic")
            .tooltip_text("Reload")
            .build();

        // Navigate home in the new tab (feature)
        let button_middle_controller = GestureClick::builder().button(BUTTON_MIDDLE).build();

        button_middle_controller.connect_pressed({
            let request = request.clone();
            let window_action = window_action.clone();
            move |_, _, _, _| {
                if let Some(uri) = request.home() {
                    window_action.append.activate_stateful_once(
                        Position::After,
                        Some(uri.to_string()),
                        false,
                        true,
                        false,
                        true,
                    );
                }
            }
        });

        button.add_controller(button_middle_controller);
        button
    }
}
