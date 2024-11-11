mod image;
mod status;
mod text;

use image::Image;
use status::Status;
use text::Text;

use crate::app::browser::window::{tab::item::Action as TabAction, Action as WindowAction};
use gtk::{
    gdk_pixbuf::Pixbuf,
    glib::Uri,
    prelude::{BoxExt, WidgetExt},
    Box, Orientation,
};
use std::{rc::Rc, time::Duration};

pub struct Content {
    window_action: Rc<WindowAction>,
    tab_action: Rc<TabAction>,
    gobject: Box,
}

impl Content {
    // Construct

    /// Create new container for different components
    pub fn new(window_action: Rc<WindowAction>, tab_action: Rc<TabAction>) -> Self {
        Self {
            gobject: Box::builder().orientation(Orientation::Vertical).build(),
            window_action,
            tab_action,
        }
    }

    // Actions

    /// Set new `content::Image` component for `Self`
    ///
    /// * action removes previous children component from `Self`
    pub fn to_image(&self, buffer: &Pixbuf) -> Image {
        self.clean();
        let image = Image::new_from_pixbuf(buffer);
        self.gobject.append(image.gobject());
        image
    }

    /// Set new `content::Status` component for `Self` with new `status::Failure` preset
    ///
    /// * action removes previous children component from `Self`
    pub fn to_status_failure(&self) -> Status {
        self.clean();
        let status = Status::new_failure();
        self.gobject.append(status.gobject());
        status
    }

    /// Set new `content::Status` component for `Self` with new `status::Loading` preset
    ///
    /// * action removes previous children component from `Self`
    pub fn to_status_loading(&self, show_with_delay: Option<Duration>) -> Status {
        self.clean();
        let status = Status::new_loading(show_with_delay);
        self.gobject.append(status.gobject());
        status
    }

    /// Set new `content::Text` component for `Self` with new `text::Gemini` preset
    ///
    /// Useful as reader for [Gemtext](https://geminiprotocol.net/docs/gemtext.gmi)
    ///
    /// * action removes previous children component from `Self`
    ///
    /// **Arguments**
    ///
    /// * `base` - [Uri](https://docs.gtk.org/glib/struct.Uri.html) to resolve relative links in Gemtext
    /// * `data` - Gemtext source to be parsed
    ///
    /// **Return**
    ///
    /// `Text` component with it public API
    /// * could be useful to extract document title parsed from Gemtext
    pub fn to_text_gemini(&self, base: &Uri, data: &str) -> Text {
        self.clean();
        let text = Text::gemini(
            data,
            base,
            (self.window_action.clone(), self.tab_action.clone()),
        );
        self.gobject.append(text.gobject());
        text
    }

    /// Remove all children components from `Self`
    pub fn clean(&self) {
        while let Some(child) = self.gobject.last_child() {
            self.gobject.remove(&child);
        }
    }

    // Getters

    /// Get reference to `Self` gobject
    pub fn gobject(&self) -> &Box {
        &self.gobject
    }
}
