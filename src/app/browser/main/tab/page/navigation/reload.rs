use gtk::Button;

pub fn new() -> Button
{
    let button = Button::builder()

        .icon_name(
            "view-refresh-symbolic"
        )

        .tooltip_text(
            "Reload"
        )

        .sensitive(
            false
        )

        .build();

    return button;
}