use gtk::Button;

pub fn new() -> Button
{
    let button = Button::builder()

        .icon_name(
            "starred-symbolic"
        )

        .tooltip_text(
            "Toggle bookmark"
        )

        .sensitive(
            false
        )

        .build();

    return button;
}