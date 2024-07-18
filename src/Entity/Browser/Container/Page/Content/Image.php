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
        $tmp = tmpfile();

        fwrite(
            $tmp,
            $data
        );

        $this->gtk->set_from_file(
            stream_get_meta_data(
                $tmp
            )['uri']
        );

        fclose(
            $tmp
        );
    }
}