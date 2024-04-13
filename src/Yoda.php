<?php

// Load dependencies
require_once __DIR__ .
             DIRECTORY_SEPARATOR . '..'.
             DIRECTORY_SEPARATOR . 'vendor' .
             DIRECTORY_SEPARATOR . 'autoload.php';

// Init app
\Gtk::init();

new \Yggverse\Yoda\Entity\App;

\Gtk::main();