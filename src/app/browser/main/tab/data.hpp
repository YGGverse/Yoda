#ifndef APP_BROWSER_MAIN_TAB_DATA_HPP
#define APP_BROWSER_MAIN_TAB_DATA_HPP

#include <gtkmm/box.h>
#include <giomm/simpleactiongroup.h>
#include <sigc++/functors/mem_fun.h>

namespace app::browser::main::tab
{
    namespace data
    {
        class Navbar;
    }

    class Data : public Gtk::Box
    {
        private:

            Glib::RefPtr<Gio::SimpleActionGroup> action_group;

            data::Navbar * navbar;

        public:

            Data();

            ~Data();

            void update();
    };
}

#endif // APP_BROWSER_MAIN_TAB_DATA_HPP