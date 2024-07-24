<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\Request\Completion;

use \GObject;
use \GtkListStore;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Navbar\Request\Completion;

class Suggestion
{
    // GTK
    public GtkListStore $gtk;

    // Dependencies
    public Completion $completion;

    public function __construct(
        Completion $completion
    ) {
        // GTK
        $this->gtk = new GtkListStore(
            GObject::TYPE_STRING
        );

        // Dependencies
        $this->completion = $completion;
    }

    public function append(
        string $request
    ): void
    {
        $this->gtk->append(
            [
                $request
            ]
        );
    }

    public function clear(): void
    {
        $this->gtk->clear();
    }
}