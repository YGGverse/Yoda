<?php

// Load dependencies
require_once __DIR__ .
             DIRECTORY_SEPARATOR . '..'.
             DIRECTORY_SEPARATOR . 'vendor' .
             DIRECTORY_SEPARATOR . 'autoload.php';

// Init config
$config = \Yggverse\Yoda\Model\File::getConfig();

// Init memory
$memory = new \Yggverse\Yoda\Model\Memory();

// Init GTK
\Gtk::init();

// Init theme
$css = new \GtkCssProvider();

$css->load_from_data(
    \Yggverse\Yoda\Model\File::getTheme(
        $config->theme
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
    600,
    480
);

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

$main = new \Yggverse\Yoda\Box\Main();

$window->add(
    $main->box
);

$window->show_all();

\Gtk::main();