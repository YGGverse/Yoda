<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Content;

class Plain extends \Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Content\Markup
{
    public function setSource(
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