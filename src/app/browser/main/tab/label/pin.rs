use gtk::Image;

pub fn new(
    visible : bool
) -> Image
{
    return Image::builder()

        .icon_name(
            "view-pin-symbolic"
        )

        .visible(
            visible
        )

        .build();
}