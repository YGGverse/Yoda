mod reader;
mod widget;

use reader::Reader;
use widget::Widget;

use crate::app::browser::window::tab::action::Action as TabAction;
use adw::ClampScrollable;
use gtk::{
    gio::SimpleAction,
    glib::{GString, Uri},
};
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
        tab_action: Rc<TabAction>,
        action_page_open: SimpleAction,
    ) -> Self {
        // Init components
        let reader = Rc::new(Reader::new(gemtext, base, tab_action, action_page_open));
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
