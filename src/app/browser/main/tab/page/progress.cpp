#include "progress.hpp"

using namespace app::browser::main::tab::page;

Progress::Progress()
{
    set_margin_top(
        MARGIN
    );

    set_margin_bottom(
        MARGIN
    );

    set_pulse_step(
        PULSE_STEP
    );

    set_opacity(0); // fixed height, not hide()
}

// Public actions
void Progress::refresh(
    double fraction
) {
    // Toggle transparency
    set_opacity(
        fraction < 1 ? 1 : 0
    );

    // Reset initial progress
    progress = fraction;

    // Animate progress function
    Glib::signal_timeout().connect(
        [this]() -> bool
        {
            double current = get_fraction();

            if (current < progress)
            {
                set_fraction(
                    current + PULSE_STEP
                );
            }

            return current < 1;
        },
        ANIMATION_TIME
    );
}