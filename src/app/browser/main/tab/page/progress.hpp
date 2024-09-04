#ifndef APP_BROWSER_MAIN_TAB_PAGE_PROGRESS_HPP
#define APP_BROWSER_MAIN_TAB_PAGE_PROGRESS_HPP

#include <glibmm/main.h>
#include <gtkmm/progressbar.h>

namespace app::browser::main::tab::page
{
    class Progress : public Gtk::ProgressBar
    {
        const int MARGIN = 2;
        const double PULSE_STEP = .1;
        const int ANIMATION_TIME = 10;

        double progress = 0;

        public:

            Progress();

            void refresh(
                double fraction
            );
    };
}

#endif // APP_BROWSER_MAIN_TAB_PAGE_PROGRESS_HPP