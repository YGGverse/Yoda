#ifndef APP_BROWSER_MAIN_TAB_PAGE_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_HPP

#include <giomm/asyncresult.h>
#include <giomm/inputstream.h>
#include <giomm/outputstream.h>
#include <giomm/simpleactiongroup.h>
#include <giomm/socketclient.h>
#include <giomm/socketconnection.h>
#include <glibmm/i18n.h>
#include <glibmm/refptr.h>
#include <glibmm/regex.h>
#include <glibmm/stringutils.h>
#include <glibmm/ustring.h>
#include <gtkmm/box.h>

namespace app::browser::main::tab
{
    namespace page
    {
        class Content;
        class Navbar;
        class Progressbar;
    }

    class Page : public Gtk::Box
    {
        char buffer[0xfffff]; // 1Mb

        Glib::ustring title;

        Glib::RefPtr<Gio::SocketClient> GioSocketClient_RefPtr;
        Glib::RefPtr<Gio::SocketConnection> GioSocketConnection_RefPtr;

        page::Content * pageContent;
        page::Navbar * pageNavbar;
        page::Progressbar * pageProgressbar;

        public:

            Page(
                const Glib::ustring & TITLE,
                const Glib::ustring & REQUEST = ""
            );

            ~Page();

            // Getters
            Glib::ustring get_title();

            // Actions
            void update();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_HPP