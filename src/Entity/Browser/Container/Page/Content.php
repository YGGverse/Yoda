<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

use \Yggverse\Yoda\Entity\Browser\Container\Page;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Gemtext;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Image;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Plain;
use \Yggverse\Yoda\Entity\Browser\Container\Page\Content\Viewport;

use \Yggverse\Yoda\Model\Filesystem;

class Content
{
    public \GtkScrolledWindow $gtk;

    // Dependencies
    public Page $page;

    // Requirements
    public Viewport $viewport;

    // Defaults
    private int $_margin = 8;

    // Extras
    private ?string $_source = null;

    public function __construct(
        Page $page
    ) {
        $this->page = $page;

        // Init scrolled window container
        $this->gtk = new \GtkScrolledWindow;

        $this->gtk->set_margin_start(
            $this->_margin
        );

        $this->gtk->set_margin_end(
            $this->_margin
        );

        $this->gtk->set_margin_bottom(
            $this->_margin
        );

        // Init scrolled window viewport
        $this->viewport = new Viewport(
            $this
        );

        $this->gtk->add(
            $this->viewport->gtk
        );

        // Render
        $this->gtk->show();
    }

    public function set(
        ?string $data,
        ?string $mime
    ): void
    {
        $this->_source = $data;

        switch ($mime)
        {
            case Filesystem::MIME_TEXT_GEMINI:

                $document = new Gemtext(
                    $this
                );

                $document->setSource(
                    $data
                );

            break;

            case Filesystem::MIME_TEXT_PLAIN:

                $document = new Plain(
                    $this
                );

                $document->setSource(
                    $data
                );

            break;

            /* @TODO
            case 'image/gif':
            case 'image/jpeg':
            case 'image/png':
            case 'image/webp':

            break;
            */

            default:

                $document = new Plain(
                    $this
                );

                $document->setSource(
                    _('MIME type not supported')
                );
        }

        $this->viewport->gtk->add(
            $document->gtk
        );

        //$this->gtk->show_all();
    }

    public function getSource(): ?string
    {
        return $this->_source;
    }

    public function refresh()
    {
        // @TODO
    }
}