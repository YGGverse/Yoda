#ifndef APP_BROWSER_MAIN_TAB_PAGE_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_HPP

#include <giomm/asyncresult.h>
#include <giomm/inputstream.h>
#include <giomm/outputstream.h>
#include <giomm/simpleactiongroup.h>
#include <giomm/socketclient.h>
#include <giomm/socketconnection.h>
#include <glibmm/refptr.h>
#include <gtkmm/box.h>

#include <string>

namespace app::browser::main::tab
{
    namespace page
    {
        class Navbar;
        class Content;
    }

    class Page : public Gtk::Box
    {
        private:

            char buffer[0xfffff];

            Glib::RefPtr<Gio::SimpleActionGroup> action_group;
            Glib::RefPtr<Gio::SocketClient> socket_client;
            Glib::RefPtr<Gio::SocketConnection> socket_connection;

            page::Navbar * navbar;
            page::Content * content;

        public:

            Page();
            ~Page();

            void update();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_HPP