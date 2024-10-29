mod image;
mod status;
mod text;

use image::Image;
use status::Status;
use text::Text;

use gtk::{
    gdk_pixbuf::Pixbuf,
    gio::SimpleAction,
    glib::{GString, Uri},
    prelude::{BoxExt, WidgetExt},
    Box, Orientation,
};
pub struct Content {
    // GTK
    gobject: Box,
    // Actions
    action_tab_open: SimpleAction,
    action_page_open: SimpleAction,
}

impl Content {
    // Construct
    pub fn new(action_tab_open: SimpleAction, action_page_open: SimpleAction) -> Self {
        Self {
            gobject: Box::builder().orientation(Orientation::Vertical).build(),
            action_tab_open,
            action_page_open,
        }
    }

    // Actions
    pub fn set_image(&self, buffer: &Pixbuf) {
        self.clean();

        let image = Image::new_from_pixbuf(buffer);

        self.gobject.append(image.gobject());
    }

    pub fn set_status_failure(&self, title: Option<&str>, description: Option<&str>) {
        self.clean();

        let status_default = Status::new_failure(title, description);

        self.gobject.append(status_default.gobject());
    }

    /// Loading placeholder
    pub fn set_status_loading(&self, title: Option<&str>, description: Option<&str>) {
        self.clean();

        let status_default = Status::new_loading(title, description);

        self.gobject.append(status_default.gobject());
    }

    /// Default reading widget for [Gemtext](https://geminiprotocol.net/docs/gemtext.gmi),
    /// removes previous children widget.
    ///
    /// **Arguments**
    ///
    /// * `base` - [Uri](https://docs.gtk.org/glib/struct.Uri.html) to resolve relative links in Gemtext
    /// * `data` - Gemtext source to be parsed
    ///
    /// **Return**
    ///
    /// `GString` copy of any header found in `data` parsed, `None` otherwise
    /// * header useful as window, tab title or any other UI related to content loaded
    pub fn set_text_gemini(&self, base: &Uri, data: &str) -> Option<GString> {
        self.clean();

        let text_gemini = Text::gemini(
            data,
            base,
            self.action_tab_open.clone(),
            self.action_page_open.clone(),
        );

        self.gobject.append(text_gemini.gobject());

        text_gemini.meta_title().clone()
    }

    pub fn clean(&self) {
        while let Some(child) = self.gobject.last_child() {
            self.gobject.remove(&child);
        }
    }

    // Getters
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
