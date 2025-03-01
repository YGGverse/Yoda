mod directory;
mod image;
mod status;
mod text;

use directory::Directory;
use image::Image;
use text::Text;

use super::{ItemAction, TabAction, WindowAction};
use adw::StatusPage;
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
    item_action: Rc<ItemAction>,
    tab_action: Rc<TabAction>,
    pub g_box: Box,
}

impl Content {
    // Construct

    /// Create new container for different components
    pub fn build(
        (window_action, tab_action, item_action): (
            &Rc<WindowAction>,
            &Rc<TabAction>,
            &Rc<ItemAction>,
        ),
    ) -> Self {
        Self {
            g_box: Box::builder().orientation(Orientation::Vertical).build(),
            window_action: window_action.clone(),
            item_action: item_action.clone(),
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
        download: Option<(&Rc<ItemAction>, &Uri)>,
    ) -> StatusPage {
        self.clean();
        let status = status::mime::build(mime, download);
        self.g_box.append(&status);
        status
    }

    /// Set new `content::Status` component for `Self` with new `status::Identity` preset
    ///
    /// * action removes previous children component from `Self`
    pub fn to_status_identity(&self) -> StatusPage {
        self.clean();
        let status = status::identity::build((&self.tab_action, &self.item_action));
        self.g_box.append(&status);
        status
    }

    /// Set new `content::Status` component for `Self` with new `status::Loading` preset
    ///
    /// * action removes previous children component from `Self`
    pub fn to_status_loading(&self, show_with_delay: Option<Duration>) -> StatusPage {
        self.clean();
        let status = status::loading::build(show_with_delay);
        self.g_box.append(&status);
        status
    }

    /// `text/gemini`
    pub fn to_text_gemini(&self, base: &Uri, data: &str) -> Text {
        self.clean();
        let text = Text::gemini((&self.window_action, &self.item_action), base, data);
        self.g_box.append(&text.scrolled_window);
        text
    }

    /// `text/plain`
    pub fn to_text_plain(&self, data: &str) -> Text {
        self.clean();
        let text = Text::plain(data);
        self.g_box.append(&text.scrolled_window);
        text
    }

    pub fn to_directory(
        &self,
        file: &File,
        callback: (impl Fn() + 'static, impl Fn(&File) + 'static),
    ) {
        self.clean();
        self.g_box.append(&Directory::for_file(file, callback))
    }

    /// * system `source:`
    pub fn to_text_source(&self, data: &str) -> Text {
        self.clean();
        let text = Text::source(data);
        self.g_box.append(&text.scrolled_window);
        text
    }

    /// Remove all children components from `Self`
    pub fn clean(&self) {
        while let Some(child) = self.g_box.last_child() {
            self.g_box.remove(&child);
        }
    }
}
