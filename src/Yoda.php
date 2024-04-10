<?php

// Load dependencies
require_once __DIR__ .
             DIRECTORY_SEPARATOR . '..'.
             DIRECTORY_SEPARATOR . 'vendor' .
             DIRECTORY_SEPARATOR . 'autoload.php';

// Init config
$config = \Yggverse\Yoda\Model\File::getConfig();

// Init GTK
\Gtk::init();

// Init theme
$css = new \GtkCssProvider();

$css->load_from_data(
    \Yggverse\Yoda\Model\File::getTheme(
        $config->interface->theme
    )
);

$style = new \GtkStyleContext();

$style->add_provider_for_screen(
    $css,
    600
);

// Init app window
$window = new \GtkWindow();

$window->set_size_request(
    $config->interface->window->width,
    $config->interface->window->height
);

if ($config->interface->window->header->enabled)
{
    $header = new \GtkHeaderBar();

    $header->set_show_close_button(
        $config->interface->window->header->button->close
    );

    $window->set_titlebar(
        $header
    );
}

$window->set_title(
    'Yoda'
);

$window->connect(
    'destroy',
    function()
    {
        \Gtk::main_quit();
    }
);

$tab = new \Yggverse\Yoda\Box\Tab();

$window->add(
    $tab->box
);

$window->show_all();

\Gtk::main();