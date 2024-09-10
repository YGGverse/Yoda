#include "page.hpp"
#include "page/content.hpp"
#include "page/navigation.hpp"

using namespace app::browser::main::tab;

Page::Page(
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__REFRESH,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__PAGE_NAVIGATION_HISTORY_BACK,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__PAGE_NAVIGATION_HISTORY_FORWARD,
    const Glib::RefPtr<Gio::SimpleAction> & ACTION__PAGE_NAVIGATION_UPDATE
) {
    // Init extras
    mime = MIME::UNDEFINED;
    title = _("New page");
    subtitle = "";

    // Init components
    pageNavigation = Gtk::make_managed<page::Navigation>(
        ACTION__REFRESH,
        ACTION__PAGE_NAVIGATION_HISTORY_BACK,
        ACTION__PAGE_NAVIGATION_HISTORY_FORWARD,
        ACTION__PAGE_NAVIGATION_UPDATE
    );

        append(
            * pageNavigation
        );

    pageContent = Gtk::make_managed<page::Content>();

        append(
            * pageContent
        );

    // Init widget
    set_orientation(
        Gtk::Orientation::VERTICAL
    );

    refresh(
        title,
        subtitle,
        0
    );
}

// Actions
void Page::refresh(
    const Glib::ustring & TITLE,
    const Glib::ustring & SUBTITLE,
    const double & PROGRESS_FRACTION
) {
    title = TITLE; // @TODO copy
    subtitle = SUBTITLE;

    pageNavigation->refresh(
        PROGRESS_FRACTION
    );
}

void Page::navigation_update(
    const bool & ADD_HISTORY
) {
    // Update navigation history
    if (ADD_HISTORY)
    {
        pageNavigation->history_add(
            pageNavigation->get_request_text(),
            true
        );
    }

    // Update page extras
    refresh(
        pageNavigation->get_request_host(),
        Glib::ustring::sprintf(
            _("load %s.."),
            pageNavigation->get_request_text()
        ), 0
    );

    // Connect scheme driver
    if ("file" == pageNavigation->get_request_scheme())
    {
        // @TODO
    }

    else if ("gemini" == pageNavigation->get_request_scheme())
    {
        // Create new socket connection
        GioSocketClient = Gio::SocketClient::create();

        GioSocketClient->set_tls(
            true
        );

        GioSocketClient->set_tls_validation_flags(
            Gio::TlsCertificateFlags::NO_FLAGS
        );

        GioSocketClient->set_timeout(
            15 // @TODO
        );

        GioSocketClient->connect_to_uri_async(
            pageNavigation->get_request_text(), 1965,
            [this](const Glib::RefPtr<Gio::AsyncResult> & result)
            {
                refresh(
                    pageNavigation->get_request_host(),
                    Glib::ustring::sprintf(
                        _("connect %s.."),
                        pageNavigation->get_request_host()
                    ), .25
                );

                try
                {
                    GioSocketConnection = GioSocketClient->connect_to_uri_finish(
                        result
                    );
                }

                catch (const Glib::Error & EXCEPTION)
                {
                    refresh(
                        pageNavigation->get_request_host(),
                        EXCEPTION.what(), 1
                    );
                }

                // Connection established, begin request
                if (GioSocketConnection != nullptr)
                {
                    const Glib::ustring request = pageNavigation->get_request_text() + "\r\n";

                    GioSocketConnection->get_output_stream()->write_async(
                        request.data(),
                        request.size(),
                        [this](const Glib::RefPtr<Gio::AsyncResult> & result)
                        {
                            refresh(
                                pageNavigation->get_request_host(),
                                Glib::ustring::sprintf(
                                    _("request %s.."),
                                    pageNavigation->get_request_path().empty() ? pageNavigation->get_request_host()
                                                                               : pageNavigation->get_request_path()
                                ), .5
                            );

                            // Response
                            GioSocketConnection->get_input_stream()->read_async( // | read_all_async
                                buffer,
                                sizeof(buffer) - 1,
                                [this](const Glib::RefPtr<Gio::AsyncResult> & result)
                                {
                                    refresh(
                                        pageNavigation->get_request_host(),
                                        Glib::ustring::sprintf(
                                            _("reading %s.."),
                                            pageNavigation->get_request_path().empty() ? pageNavigation->get_request_host()
                                                                                       : pageNavigation->get_request_path()
                                        ), .75
                                    );

                                    // Parse meta
                                    auto meta = Glib::Regex::split_simple(
                                        R"regex(^(\d+)?\s([\w]+\/[\w]+)?)regex",
                                        buffer
                                    );

                                    // Route by status code
                                    if (meta[1] == "20")
                                    {
                                        // Route by mime type or path extension
                                        if (meta[2] == "text/gemini" || Glib::str_has_suffix(pageNavigation->get_request_path(), ".gmi"))
                                        {
                                            mime = MIME::TEXT_GEMINI;

                                            pageContent->set_text_gemini(
                                                buffer // @TODO
                                            );
                                        }

                                        else
                                        {
                                            mime = MIME::UNDEFINED;

                                            title = _("Oops");
                                            subtitle = _("MIME type not supported");

                                            pageContent->set_text_plain( // @TODO
                                                subtitle
                                            );
                                        }
                                    }

                                    else
                                    {
                                        mime = MIME::UNDEFINED;

                                        title = _("Oops");

                                        subtitle = Glib::ustring::sprintf(
                                            _("Response code %s not supported"),
                                            meta[1]
                                        );

                                        pageContent->set_text_plain( // @TODO
                                            subtitle
                                        );
                                    }

                                    GioSocketConnection->close();

                                    refresh(
                                        pageNavigation->get_request_host(), // @TODO title
                                        pageNavigation->get_request_path().empty() ? pageNavigation->get_request_host()
                                                                                   : pageNavigation->get_request_path()
                                        , 1
                                    );
                                }
                            );
                        }
                    );
                }
            }
        );
    }

    // Scheme not found but host provided, redirect to gemini://
    else if (pageNavigation->get_request_scheme().empty() && !pageNavigation->get_request_host().empty())
    {
        pageNavigation->set_request_text(
            "gemini://" + pageNavigation->get_request_text()
        );

        navigation_update(
            false
        );
    }

    else
    {
        // @TODO search request
    }
}

void Page::navigation_history_back()
{
    Glib::ustring request;

    if (pageNavigation->try_history_back(request, true))
    {
        pageNavigation->set_request_text(
            request
        );

        navigation_update(
            false
        );
    }
}

void Page::navigation_history_forward()
{
    Glib::ustring request;

    if (pageNavigation->try_history_forward(request, true))
    {
        pageNavigation->set_request_text(
            request
        );

        navigation_update(
            false
        );
    }
}

// Getters
Page::MIME Page::get_mime()
{
    return mime;
}

Glib::ustring Page::get_title()
{
    return title;
}

Glib::ustring Page::get_subtitle()
{
    return subtitle;
}

Glib::ustring Page::get_navigation_request_text()
{
    return pageNavigation->get_request_text();
}

// Setters
void Page::set_navbar_request_text(
    const Glib::ustring & VALUE
) {
    pageNavigation->set_request_text(
        VALUE
    );
}