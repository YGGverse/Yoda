#ifndef APP_BROWSER_MAIN_TAB_DATA_HPP
#define APP_BROWSER_MAIN_TAB_DATA_HPP

#include <giomm/simpleactiongroup.h>
#include <glibmm/refptr.h>
#include <gtkmm/box.h>
#include <sigc++/functors/mem_fun.h>

namespace app::browser::main::tab
{
    namespace data
    {
        class Navbar;
        class Content;
    }

    class Data : public Gtk::Box
    {
        private:

            Glib::RefPtr<Gio::SimpleActionGroup> action_group;

            data::Navbar * navbar;
            data::Content * content;

        public:

            Data();
            ~Data();

            void update();
    };
}

#endif // APP_BROWSER_MAIN_TAB_DATA_HPP