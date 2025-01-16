mod image;
mod status;
mod text;

use image::Image;
use text::Text;

use super::{TabAction, WindowAction};
use adw::StatusPage;
use gtk::{
    gdk::Paintable,
    gio::{Cancellable, File},
    glib::{GString, Uri},
    prelude::{BoxExt, IsA, WidgetExt},
    Box, Orientation,
};
use std::{rc::Rc, time::Duration};

pub struct Content {
    window_action: Rc<WindowAction>,
    tab_action: Rc<TabAction>,
    pub g_box: Box,
}

impl Content {
    // Construct

    /// Create new container for different components
    pub fn build((window_action, tab_action): (&Rc<WindowAction>, &Rc<TabAction>)) -> Self {
        Self {
            g_box: Box::builder().orientation(Orientation::Vertical).build(),
            window_action: window_action.clone(),
            tab_action: tab_action.clone(),
        }
    }

    // Actions

    /// Set new `content::Image` component for `Self`
    ///
    /// * action removes previous children component from `Self`
    pub fn to_image(&self, paintable: &impl IsA<Paintable>) -> Image {
        self.clean();
        let image = Image::new_from_paintable(paintable);
        self.g_box.append(&image.picture);
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
    ) -> StatusPage {
        self.clean();
        let status = status::download::build(initial_filename, cancellable, on_choose);
        self.g_box.append(&status);
        status
    }

    /// Set new `content::Status` component for `Self` with new `status::Failure` preset
    ///
    /// * action removes previous children component from `Self`
    pub fn to_status_failure(&self) -> StatusPage {
        self.clean();
        let status = status::failure::new();
        self.g_box.append(&status);
        status
    }

    /// Set new `content::Status` component for `Self` with new `status::Mime` issue preset
    ///
    /// * action removes previous children component from `Self`
    pub fn to_status_mime(
        &self,
        mime: &str,
        download: Option<(Rc<TabAction>, GString)>,
    ) -> StatusPage {
        self.clean();
        let status = status::mime::new(mime, download);
        self.g_box.append(&status);
        status
    }

    /// Set new `content::Status` component for `Self` with new `status::Identity` preset
    ///
    /// * action removes previous children component from `Self`
    pub fn to_status_identity(&self) -> StatusPage {
        self.clean();
        let status = status::identity::new(self.tab_action.clone());
        self.g_box.append(&status);
        status
    }

    /// Set new `content::Status` component for `Self` with new `status::Loading` preset
    ///
    /// * action removes previous children component from `Self`
    pub fn to_status_loading(&self, show_with_delay: Option<Duration>) -> StatusPage {
        self.clean();
        let status = status::loading::new(show_with_delay);
        self.g_box.append(&status);
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
        let text = Text::new_gemini(data, base, (&self.window_action, &self.tab_action));
        self.g_box.append(&text.g_box);
        text
    }

    pub fn to_text_source(&self, data: &str) -> Text {
        self.clean();
        let text = Text::new_source(data);
        self.g_box.append(&text.g_box);
        text
    }

    /// Remove all children components from `Self`
    pub fn clean(&self) {
        while let Some(child) = self.g_box.last_child() {
            self.g_box.remove(&child);
        }
    }
}
