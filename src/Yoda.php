<?php

// Load dependencies
require_once __DIR__ .
             DIRECTORY_SEPARATOR . '..'.
             DIRECTORY_SEPARATOR . 'vendor' .
             DIRECTORY_SEPARATOR . 'autoload.php';

// Init filesystem
$filesystem = new \Yggverse\Yoda\Model\Filesystem(
    (
        getenv('HOME') ?? __DIR__ . DIRECTORY_SEPARATOR . '..'
    ) . DIRECTORY_SEPARATOR . '.yoda'
);

// Init database
$database = new \Yggverse\Yoda\Model\Database(
    $filesystem->getAbsolute(
        'database.sqlite'
    )
);

// Init GTK
\Gtk::init();

// Init browser
$browser = new \Yggverse\Yoda\Entity\Browser(
    $database
);

$browser->gtk->connect(
    'destroy',
    function()
    {
        \Gtk::main_quit();
    }
);

$browser->gtk->show_all();

\Gtk::main();