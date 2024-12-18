use gtk::Separator;

const MARGIN: i32 = 6;

pub fn new() -> Separator {
    Separator::builder()
        .margin_bottom(MARGIN)
        .margin_end(MARGIN)
        .margin_start(MARGIN)
        .margin_top(MARGIN)
        .build()
}
