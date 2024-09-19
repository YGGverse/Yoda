use gtk::Button;

pub fn new() -> Button
{
    let button = Button::builder()

        .icon_name(
            "go-next-symbolic"
        )

        .tooltip_text(
            "Forward"
        )

        .sensitive(
            false
        )

        .build();

    return button;
}