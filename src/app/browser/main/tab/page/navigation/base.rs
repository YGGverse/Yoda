use gtk::Button;

pub fn new() -> Button
{
    let button = Button::builder()

        .icon_name(
            "go-home-symbolic"
        )

        .tooltip_text(
            "Base"
        )

        .sensitive(
            false
        )

        .build();

    return button;
}