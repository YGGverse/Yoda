mod reader;
mod widget;

use reader::Reader;
use widget::Widget;

use crate::app::browser::window::{tab::item::Action as TabAction, Action as WindowAction};
use adw::ClampScrollable;
use gtk::glib::{GString, Uri};
use std::rc::Rc;

pub struct Gemini {
    reader: Rc<Reader>,
    widget: Rc<Widget>,
}

impl Gemini {
    // Construct
    pub fn new(
        gemtext: &str,
        base: &Uri,
        window_action: Rc<WindowAction>,
        tab_action: Rc<TabAction>,
    ) -> Self {
        // Init components
        let reader = Rc::new(Reader::new(gemtext, base, window_action, tab_action));
        let widget = Rc::new(Widget::new(reader.gobject()));

        // Result
        Self { reader, widget }
    }

    // Getters
    pub fn reader_title(&self) -> &Option<GString> {
        self.reader.title()
    }

    pub fn gobject(&self) -> &ClampScrollable {
        self.widget.gobject()
    }
}
