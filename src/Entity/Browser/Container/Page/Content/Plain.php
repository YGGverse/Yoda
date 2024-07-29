<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Content;

use \GdkEvent;
use \GtkLabel;

use \Yggverse\Yoda\Model\Gtk\Pango\Markup;

class Plain extends \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Content\Markup
{
    public function set(
        string $source
    ): void
    {
        $this->gtk->set_markup(
            Markup::code( // @TODO
                $this->_source = $source
            )
        );
    }
}