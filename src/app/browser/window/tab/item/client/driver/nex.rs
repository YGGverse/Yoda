//! https://nightfall.city/nex/info/specification.txt

use super::{Feature, Page};
use gtk::gio::{MemoryInputStream, SocketConnection};
use gtk::prelude::{
    Cast, IOStreamExt, InputStreamExtManual, OutputStreamExtManual, SocketClientExt,
};
use gtk::{
    gdk::Texture,
    gdk_pixbuf::Pixbuf,
    gio::{Cancellable, IOStream, SocketClient, SocketClientEvent, SocketProtocol},
    glib::{Priority, Uri},
};
use sourceview::prelude::FileExt;
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub struct Nex {
    page: Rc<Page>,
}

impl Nex {
    pub fn init(page: &Rc<Page>) -> Self {
        Self { page: page.clone() }
    }

    pub fn handle(
        &self,
        uri: Uri,
        feature: Rc<Feature>,
        cancellable: Cancellable,
        is_snap_history: bool,
    ) {
        {
            self.page
                .navigation
                .request
                .info
                .borrow_mut()
                .set_request(Some(uri.to_string()));
        }

        let path = uri.path(); // copy once

        if path.is_empty() {
            // auto-append trailing slash to the root locations
            let mut r = uri.to_string();
            r.push('/');
            // apply the permanent redirection
            let mut i = self.page.navigation.request.info.take();
            i.add_event("Canonicalize root request".to_string());
            self.page
                .navigation
                .request
                .info
                .replace(i.into_permanent_redirect());
            self.page.navigation.set_request(&r);
            self.page.item_action.load.activate(Some(&r), false, true);
            return; // prevents operation cancelled message on redirect
        }

        if is_snap_history {
            self.page.snap_history();
        }

        let socket = SocketClient::new();
        socket.set_protocol(SocketProtocol::Tcp);
        socket.set_timeout(30); // @TODO optional

        socket.connect_event({
            let p = self.page.clone();
            move |_, e, _, _| {
                let mut i = p.navigation.request.info.borrow_mut();
                p.set_progress(match e {
                    // 0.1 reserved for handle begin
                    SocketClientEvent::Resolving => {
                        i.add_event("Resolving".to_string());
                        0.2
                    }
                    SocketClientEvent::Resolved => {
                        i.add_event("Resolved".to_string());
                        0.3
                    }
                    SocketClientEvent::Connecting => {
                        i.add_event("Connecting".to_string());
                        0.4
                    }
                    SocketClientEvent::Connected => {
                        i.add_event("Connected".to_string());
                        0.5
                    }
                    SocketClientEvent::ProxyNegotiating => {
                        i.add_event("Proxy negotiating".to_string());
                        0.6
                    }
                    SocketClientEvent::ProxyNegotiated => {
                        i.add_event("Proxy negotiated".to_string());
                        0.7
                    }
                    SocketClientEvent::TlsHandshaking => {
                        i.add_event("TLS handshaking".to_string());
                        0.8
                    }
                    SocketClientEvent::TlsHandshaked => {
                        i.add_event("TLS handshaked".to_string());
                        0.9
                    }
                    SocketClientEvent::Complete => {
                        i.add_event("Receiving".to_string());
                        1.0
                    }
                    _ => panic!(),
                })
            }
        });

        socket.connect_to_uri_async(&uri.to_string(), 1900, Some(&cancellable.clone()), {
            let p = self.page.clone();
            move |result| match result {
                Ok(c) => {
                    {
                        use gtk::prelude::SocketConnectionExt;
                        let mut i = p.navigation.request.info.borrow_mut();
                        i.set_socket(Some((
                            c.local_address().unwrap(),
                            c.remote_address().unwrap(),
                        )));
                        // * unwrap fails only on `connection.socket_connection.is_closed()`
                        //   panic as unexpected.
                    }
                    c.output_stream().write_all_async(
                        format!("{path}\r\n"),
                        Priority::DEFAULT,
                        Some(&cancellable.clone()),
                        move |r| match r {
                            Ok((_, size, _)) => {
                                // Is download feature request,
                                // delegate this task to the separated handler function.
                                if matches!(*feature, Feature::Download) {
                                    return download(c, (p, uri), cancellable);
                                }

                                // Navigate to the download gateway on content type is not supported
                                if !is_renderable(&path) {
                                    p.content
                                        .to_status_mime(&path, Some((&p.item_action, &uri)));
                                    p.set_progress(0.0);
                                    c.close_async(Priority::DEFAULT, Some(&cancellable), {
                                        let p = p.clone();
                                        move |r| {
                                            event(
                                                &p,
                                                &match r {
                                                    Ok(()) => "Disconnected".to_string(),
                                                    Err(e) => e.to_string(),
                                                },
                                                Some(size),
                                            )
                                        }
                                    });
                                    return;
                                }

                                // Is renderable types..

                                // Show loading status page if awaiting time > 1 second
                                // * the RefCell is just to not init the loading widget before timeout and prevent bg blinks
                                let loading: RefCell<Option<adw::StatusPage>> = RefCell::new(None);
                                let loading_total: Cell<usize> = Cell::new(0);

                                // Nex is the header-less protocol, final content size is never known,
                                // borrow ggemini::gio wrapper api to preload the buffer swap-safely,
                                // by using the chunks controller.
                                ggemini::gio::memory_input_stream::from_stream_async(
                                    c.clone().upcast::<IOStream>(),
                                    Priority::DEFAULT,
                                    cancellable.clone(),
                                    ggemini::gio::memory_input_stream::Size {
                                        chunk: 0x400,    // 1024 bytes chunk
                                        limit: 0xA00000, // 10M limit
                                        total: 0,        // initial totals
                                    },
                                    (
                                        {
                                            let p = p.clone();
                                            move |_, t| {
                                                if loading_total.replace(t) > 102400 {
                                                    let mut l = loading.borrow_mut();
                                                    match *l {
                                                        Some(ref this) => this.set_description(
                                                            Some(&format!("Preload: {t} bytes")),
                                                        ),
                                                        None => {
                                                            l.replace(
                                                                p.content.to_status_loading(None),
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        move |r| match r {
                                            Ok((m, s)) => {
                                                c.close_async(
                                                    Priority::DEFAULT,
                                                    Some(&cancellable),
                                                    {
                                                        let p = p.clone();
                                                        move |r| {
                                                            event(
                                                                &p,
                                                                &match r {
                                                                    Ok(()) => {
                                                                        "Disconnected".to_string()
                                                                    }
                                                                    Err(e) => e.to_string(),
                                                                },
                                                                Some(s),
                                                            )
                                                        }
                                                    },
                                                );
                                                render((m, s), (p, feature, uri), cancellable);
                                            }
                                            Err(e) => failure(&p, &e.to_string()),
                                        },
                                    ),
                                )
                            }
                            Err((_, e)) => failure(&p, &e.to_string()),
                        },
                    )
                }
                Err(e) => failure(&p, &e.to_string()),
            }
        })
    }
}

fn event(p: &Page, e: &str, s: Option<usize>) {
    let mut i = p.navigation.request.info.borrow_mut();
    i.add_event(e.to_string()).set_size(s);
    p.navigation.request.update_secondary_icon(&i)
}

fn failure(p: &Page, d: &str) {
    let s = p.content.to_status_failure();
    s.set_description(Some(d));
    p.set_progress(0.0);
    p.set_title(&s.title())
}

fn render(
    (m, s): (MemoryInputStream, usize),
    (p, f, u): (Rc<Page>, Rc<Feature>, Uri),
    c: Cancellable,
) {
    use crate::tool::uri_to_title;
    let q = u.to_string();
    if is_image(&q) {
        p.window_action.find.simple_action.set_enabled(false);
        Pixbuf::from_stream_async(&m, Some(&c), move |r| match r {
            Ok(b) => {
                p.set_title(&uri_to_title(&u));
                p.content.to_image(&Texture::for_pixbuf(&b));
                p.set_progress(0.0);
                event(&p, "Completed", Some(s))
            }
            Err(e) => failure(&p, &e.to_string()),
        })
    } else if is_text(&q) {
        p.window_action.find.simple_action.set_enabled(true);
        match *f {
            Feature::Default | Feature::Source => {
                m.read_all_async(vec![0; s], Priority::DEFAULT, Some(&c), move |r| match r {
                    Ok((b, s, ..)) => match std::str::from_utf8(&b) {
                        Ok(d) => {
                            let t = if matches!(*f, Feature::Source) {
                                p.content.to_text_source(d)
                            } else if q.ends_with(".gmi") || q.ends_with(".gemini") {
                                p.content.to_text_gemini(&u, d)
                            } else if q.ends_with(".log") {
                                p.content.to_text_plain(d)
                            } else {
                                p.content.to_text_nex(&u, d)
                            };
                            event(&p, "Parsed", Some(s));
                            p.search.set(Some(t.text_view));
                            p.set_title(&match t.meta.title {
                                Some(t) => t.into(), // @TODO
                                None => uri_to_title(&u),
                            });
                            p.set_progress(0.0);
                            event(&p, "Completed", Some(s))
                        }
                        Err(e) => failure(&p, &e.to_string()),
                    },
                    Err((_, e)) => failure(&p, &e.to_string()),
                })
            }
            Feature::Download => panic!(), // unexpected
        }
    } else {
        panic!() // unexpected
    }
}

fn download(s: SocketConnection, (p, u): (Rc<Page>, Uri), c: Cancellable) {
    use crate::tool::Format;
    use ggemini::gio::file_output_stream;
    event(&p, "Download begin", None);
    let t = crate::tool::uri_to_title(&u)
        .trim_matches(std::path::MAIN_SEPARATOR)
        .to_string();
    p.content.to_status_download(&t, &c, {
        let c = c.clone();
        let p = p.clone();
        let t = t.clone();
        move |f, a| match f.replace(None, false, gtk::gio::FileCreateFlags::NONE, Some(&c)) {
            Ok(file_output_stream) => {
                file_output_stream::from_stream_async(
                    s.clone().upcast::<IOStream>(),
                    file_output_stream,
                    c.clone(),
                    Priority::DEFAULT,
                    file_output_stream::Size {
                        chunk: 0x100000, // 1M bytes per chunk
                        limit: None,     // unlimited
                        total: 0,        // initial totals
                    },
                    (
                        // on chunk
                        {
                            let a = a.clone();
                            let p = p.clone();
                            move |_, total| {
                                const T: &str = "Received";
                                let t = format!("{T} {}...", total.bytes());
                                event(&p, T, Some(total));
                                p.set_title(&t);
                                a.update.activate(&t)
                            }
                        },
                        // on complete
                        {
                            let a = a.clone();
                            let p = p.clone();
                            let t = t.clone();
                            let c = c.clone();
                            let s = s.clone();
                            move |result| match result {
                                Ok((_, total)) => {
                                    a.complete.activate(&format!(
                                        "Saved to {} ({} total)",
                                        f.parse_name(),
                                        total.bytes()
                                    ));
                                    p.set_progress(0.0);
                                    p.set_title(&t);
                                    s.close_async(Priority::DEFAULT, Some(&c), {
                                        let p = p.clone();
                                        move |r| {
                                            event(
                                                &p,
                                                &match r {
                                                    Ok(()) => "Disconnected".to_string(),
                                                    Err(e) => e.to_string(),
                                                },
                                                Some(total),
                                            )
                                        }
                                    })
                                }
                                Err(e) => a.cancel.activate(&e.to_string()),
                            }
                        },
                    ),
                )
            }
            Err(e) => a.cancel.activate(&e.to_string()),
        }
    });
}

fn is_image(q: &str) -> bool {
    q.ends_with(".gif")
        || q.ends_with(".jpeg")
        || q.ends_with(".jpg")
        || q.ends_with(".png")
        || q.ends_with(".webp")
}

fn is_text(q: &str) -> bool {
    q.ends_with(".txt")
        || q.ends_with(".log")
        || q.ends_with(".gmi")
        || q.ends_with(".gemini")
        || q.ends_with("/")
        || !q.contains(".")
}

fn is_renderable(q: &str) -> bool {
    is_text(q) || is_image(q)
}
