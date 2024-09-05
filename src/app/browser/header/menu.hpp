#ifndef APP_BROWSER_HEADER_MENU_HPP
#define APP_BROWSER_HEADER_MENU_HPP

#include <giomm/menu.h>
#include <glibmm/i18n.h>
#include <gtkmm/menubutton.h>

namespace app::browser::header
{
    class Menu : public Gtk::MenuButton
    {
        Glib::RefPtr<Gio::Menu> main,
                                    main_tab,
                                        main_tab_page,
                                            main_tab_page_navigation,
                                                main_tab_page_navigation_history,
                                        main_tab_close,
                                    main_tools;

        public:

            Menu();
    };
}

#endif // APP_BROWSER_HEADER_MENU_HPP