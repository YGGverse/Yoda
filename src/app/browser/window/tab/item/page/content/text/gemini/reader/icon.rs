use gtk::{gdk::Display, IconLookupFlags, IconPaintable, IconTheme, TextDirection};

const SIZE: i32 = 16;

/// Indication icons asset (for tag blocks decoration)
pub struct Icon {
    pub quote: IconPaintable,
    // @TODO other tags..
}

impl Icon {
    pub fn new() -> Option<Self> {
        Display::default().map(|display| {
            let theme = IconTheme::for_display(&display);
            Self {
                quote: icon(&theme, "mail-forward-symbolic"),
            }
        })
    }
}

fn icon(theme: &IconTheme, name: &str) -> IconPaintable {
    theme.lookup_icon(
        name,
        &[], // @TODO
        SIZE,
        SIZE,
        TextDirection::None,
        IconLookupFlags::FORCE_SYMBOLIC,
    )
}
