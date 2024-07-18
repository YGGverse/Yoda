<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Content;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Content;

class Image
{
    public \GtkImage $gtk;

    // Dependencies
    public Content $content;

    public function __construct(
        Content $content
    ) {
        // Init dependency
        $this->content = $content;

        // Init image object
        $this->gtk = new \GtkImage;
    }

    public function set(
        string $data
    ): void
    {
        $this->gtk->set_from_resource(
            $data
        );
    }
}