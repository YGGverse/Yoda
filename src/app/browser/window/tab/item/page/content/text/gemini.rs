mod reader;
mod widget;

use reader::Reader;
use widget::Widget;

use crate::app::browser::window::{tab::item::Action as TabAction, Action as WindowAction};
use gtk::glib::Uri;
use std::rc::Rc;

pub struct Gemini {
    pub reader: Rc<Reader>,
    pub widget: Rc<Widget>,
}

impl Gemini {
    // Construct
    pub fn new(
        gemtext: &str,
        base: &Uri,
        (window_action, tab_action): (&Rc<WindowAction>, &Rc<TabAction>),
    ) -> Self {
        // Init components
        let reader = Rc::new(
            Reader::new(gemtext, base, (window_action.clone(), tab_action.clone())).unwrap(),
        ); // @TODO handle errors
        let widget = Rc::new(Widget::new(&reader.widget.text_view));

        // Result
        Self { reader, widget }
    }
}
