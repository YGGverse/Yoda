<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Content;

use \GdkEvent;
use \GtkLabel;

use \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Content\Markup;

class Plain extends Markup
{
    public function set(
        string $source
    ): void
    {
        $this->gtk->set_markup(
            sprintf(
                '<tt>%s</tt>',
                htmlspecialchars(
                    $this->_source = $source
                )
            )
        );
    }

    protected function _onSizeAllocate(
        GtkLabel $label,
        GdkEvent $event
    ): bool
    {
        // @TODO
        return false;
    }
}