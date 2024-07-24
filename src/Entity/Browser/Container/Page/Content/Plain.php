<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page\Content;

use \GdkEvent;
use \GtkLabel;

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

    protected function _onActivateLink(
        GtkLabel $label,
        string $href
    ): bool
    {
        return true;
    }

    protected function _onButtonPressEvent(
        GtkLabel $label,
        GdkEvent $event
    ): bool
    {
        // Markup container has focus event disabled (hidden cursor position)
        // this solution deactivates Request entry on click Markup area
        $this->content->page->container->tab->gtk->grab_focus();

        return false;
    }
}