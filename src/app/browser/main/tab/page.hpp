#ifndef APP_BROWSER_MAIN_TAB_PAGE_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_HPP

#include <giomm/simpleactiongroup.h>
#include <glibmm/refptr.h>
#include <gtkmm/box.h>
#include <sigc++/functors/mem_fun.h>

#include <giomm/asyncresult.h>
#include <giomm/socketconnection.h>
#include <giomm/socketclient.h>

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

            Glib::RefPtr<Gio::SimpleActionGroup> action_group;
            Glib::RefPtr<Gio::SocketClient> socket_client;

            page::Navbar * navbar;
            page::Content * content;

            void connect(
                const std::string & host,
                int port
            );

        public:

            Page();
            ~Page();

            void update();
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_HPP