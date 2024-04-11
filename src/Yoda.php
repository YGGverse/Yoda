<?php

// Load dependencies
require_once __DIR__ .
             DIRECTORY_SEPARATOR . '..'.
             DIRECTORY_SEPARATOR . 'vendor' .
             DIRECTORY_SEPARATOR . 'autoload.php';

// Init app
\Gtk::init();

$config = \Yggverse\Yoda\Model\File::getConfig(); // @TODO

$window = new \GtkWindow();

$window->set_size_request(
    $config->window->width,
    $config->window->height
);

if ($config->window->header->enabled)
{
    $header = new \GtkHeaderBar();

    $header->set_title(
        $config->window->title
    );

    $header->set_show_close_button(
        $config->window->header->button->close
    );

    $window->set_titlebar(
        $header
    );
}

$window->connect(
    'destroy',
    function()
    {
        \Gtk::main_quit();
    }
);

$page = new \Yggverse\Yoda\Tab\Page();

$page->open(
    'yoda://welcome'
);

$tab = new \GtkNotebook();

$tab->add(
    $page->box
);

$window->add(
    $tab
);

$window->show_all();

\Gtk::main();