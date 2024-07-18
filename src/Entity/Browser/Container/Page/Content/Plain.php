<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Content;

use \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Content\Markup;

class Plain extends Markup
{
    public function set(
        string $value
    ): void
    {
        $this->gtk->set_markup(
            sprintf(
                '<tt>%s</tt>',
                htmlspecialchars(
                    $value
                )
            )
        );
    }
}