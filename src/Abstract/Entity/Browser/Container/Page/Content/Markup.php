<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Content;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Content;

abstract class Markup
{
    public \GtkLabel $gtk;

    // Dependencies
    public Content $content;

    // Defaults
    public const WRAP = 140;

    public function __construct(
        Content $content
    ) {
        // Init dependency
        $this->content = $content;

        // Init markup label
        $this->gtk = new \GtkLabel;

        $this->gtk->set_use_markup(
            true
        );

        $this->gtk->set_selectable(
            true
        );

        $this->gtk->set_can_focus(
            false
        );

        $this->gtk->set_track_visited_links(
            true
        );

        $this->gtk->set_xalign(
            0
        );

        $this->gtk->set_yalign(
            0
        );

        $this->gtk->show();

        // Init events
        $this->gtk->connect(
            'activate-link',
            function(
                \GtkLabel $label,
                string $href
            ) {
                return $this->_onActivateLink(
                    $label,
                    $href
                );
            }
        );

        $this->gtk->connect(
            'button-press-event',
            function()
            {
                // Markup container has focus disabled (to hide cursor position),
                // solution remove selection from request entry on click this area

                // @TODO
            }
        );
    }

    abstract protected function _onActivateLink(
        \GtkLabel $label,
        string $href
    ): bool;

    abstract public function set(
        string $value
    ): void;
}
