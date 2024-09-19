use gtk::Button;

pub fn new() -> Button
{
    let button = Button::builder()

        .icon_name(
            "go-previous-symbolic"
        )

        .tooltip_text(
            "Back"
        )

        .sensitive(
            false
        )

        .build();

    return button;
}