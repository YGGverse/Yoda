#ifndef APP_BROWSER_HPP
#define APP_BROWSER_HPP

#include <glibmm/i18n.h>
#include <gtkmm/applicationwindow.h>

namespace lib
{
    class Database;
}

namespace app
{
    namespace browser
    {
        class Header;
        class Main;
    }

    class Browser : public Gtk::ApplicationWindow
    {
        private:

            app::browser::Header * header;
            app::browser::Main * main;

        public:

            Browser(
                //const Glib::RefPtr<Gtk::Application> & app,
                //const std::shared_ptr<lib::Database> & db
            );

            ~Browser();

            void main_tab_append();
            void main_tab_update();

            void main_tab_close();
            void main_tab_close_left();
            void main_tab_close_right();
            void main_tab_close_all();

            void debug();
    };
}

#endif // APP_BROWSER_HPP