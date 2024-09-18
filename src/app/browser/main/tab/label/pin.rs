use gtk::Image;

pub fn new() -> Image
{
    let pin = Image::builder().icon_name(
        "view-pin-symbolic"
    ).build();

    return pin;
}