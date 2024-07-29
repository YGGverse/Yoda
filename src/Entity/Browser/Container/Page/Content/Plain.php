<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Content;

use \Yggverse\Yoda\Model\Gtk\Pango\Markup\Plain as Markup;

class Plain extends \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Content\Markup
{
    public function set(
        string $source
    ): void
    {
        $this->gtk->set_markup(
            Markup::format(
                $this->_source = $source
            )
        );
    }
}