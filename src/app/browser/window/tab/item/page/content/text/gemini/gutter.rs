use gtk::{
    Align, Label, TextView, TextWindowType,
    glib::{Uri, timeout_add_local_once},
    pango::EllipsizeMode,
    prelude::{TextViewExt, WidgetExt},
};
use std::{cell::Cell, rc::Rc, time::Duration};

pub struct Gutter {
    pub label: Label,
    is_active: Rc<Cell<bool>>,
}

impl Gutter {
    pub fn build(text_view: &TextView) -> Self {
        const MARGIN_X: i32 = 8;
        const MARGIN_Y: i32 = 2;
        let label = Label::builder()
            .css_classes(["caption", "dim-label"])
            .ellipsize(EllipsizeMode::Middle)
            .halign(Align::Start)
            .margin_bottom(MARGIN_Y)
            .margin_end(MARGIN_X)
            .margin_start(MARGIN_X)
            .margin_top(MARGIN_Y)
            .visible(false)
            .build();

        text_view.set_gutter(TextWindowType::Bottom, Some(&label));
        text_view
            .gutter(TextWindowType::Bottom)
            .unwrap()
            .set_css_classes(&["view"]); // @TODO unspecified patch

        Self {
            is_active: Rc::new(Cell::new(false)),
            label,
        }
    }

    pub fn set_uri(&self, uri: Option<&Uri>) {
        match uri {
            Some(uri) => {
                if !self.label.is_visible() {
                    if !self.is_active.replace(true) {
                        timeout_add_local_once(Duration::from_millis(250), {
                            let label = self.label.clone();
                            let is_active = self.is_active.clone();
                            let uri = uri.clone();
                            move || {
                                if is_active.replace(false) {
                                    label.set_label(&uri.to_string());
                                    label.set_visible(true)
                                }
                            }
                        });
                    }
                } else {
                    self.label.set_label(&uri.to_string())
                }
            }
            None => {
                self.is_active.replace(false);
                self.label.set_visible(false)
            }
        }
    }
}
