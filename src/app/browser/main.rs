use gtk::Box;

pub fn new() -> Box
{
    return Box::builder().orientation(
        gtk::Orientation::Vertical
    ).build();
}