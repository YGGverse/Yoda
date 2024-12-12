mod image;
mod status;
mod text;

use image::Image;
use status::Status;
use text::Text;

use crate::app::browser::window::{tab::item::Action as TabAction, Action as WindowAction};
use gtk::{
    gdk::Paintable,
    gio::{Cancellable, File},
    glib::Uri,
    prelude::{BoxExt, IsA, WidgetExt},
    Box, Orientation,
};
use std::{rc::Rc, time::Duration};

pub struct Content {
    window_action: Rc<WindowAction>,
    tab_action: Rc<TabAction>,
    pub gobject: Box,
}

impl Content {
    // Construct

    /// Create new container for different components
    pub fn new(action: (Rc<WindowAction>, Rc<TabAction>)) -> Self {
        Self {
            gobject: Box::builder().orientation(Orientation::Vertical).build(),
            window_action: action.0,
            tab_action: action.1,
        }
    }

    // Actions

    /// Set new `content::Image` component for `Self`
    ///
    /// * action removes previous children component from `Self`
    pub fn to_image(&self, paintable: &impl IsA<Paintable>) -> Image {
        self.clean();
        let image = Image::new_from_paintable(paintable);
        self.gobject.append(image.gobject());
        image
    }

    /// Set new `content::Status` component for `Self` with new `status::Download` preset
    ///
    /// * action removes previous children component from `Self`
    pub fn to_status_download(
        &self,
        initial_filename: &str,
        cancellable: &Cancellable,
        on_choose: impl Fn(File, Rc<status::download::Action>) + 'static,
    ) -> Status {
        self.clean();
        let status = Status::new_download(initial_filename, cancellable, on_choose);
        self.gobject.append(status.gobject());
        status
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

    /// Set new `content::Status` component for `Self` with new `status::Mime` issue preset
    ///
    /// * action removes previous children component from `Self`
    pub fn to_status_mime(&self) -> Status {
        self.clean();
        let status = Status::new_mime();
        self.gobject.append(status.gobject());
        status
    }

    /// Set new `content::Status` component for `Self` with new `status::Identity` preset
    ///
    /// * action removes previous children component from `Self`
    pub fn to_status_identity(&self) -> Status {
        self.clean();
        let status = Status::new_identity(self.tab_action.clone());
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
        let text = Text::new_gemini(
            data,
            base,
            (self.window_action.clone(), self.tab_action.clone()),
        );
        self.gobject.append(&text.scrolled_window);
        text
    }

    pub fn to_text_source(&self, data: &str) -> Text {
        self.clean();
        let text = Text::new_source(data);
        self.gobject.append(&text.scrolled_window);
        text
    }

    /// Remove all children components from `Self`
    pub fn clean(&self) {
        while let Some(child) = self.gobject.last_child() {
            self.gobject.remove(&child);
        }
    }
}
