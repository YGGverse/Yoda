use gtk::MenuButton;

pub fn new() -> MenuButton
{
    let menu = MenuButton::builder()

        .tooltip_text(
            "Menu"
        )

        .build();

    return menu;
}