<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Content;

use \GdkEvent;
use \GtkLabel;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Content;

abstract class Markup
{
    public GtkLabel $gtk;

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
        $this->gtk = new GtkLabel;

        $this->gtk->set_use_markup(
            true
        );

        $this->gtk->set_selectable(
            true
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
                GtkLabel $label,
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
            function(
                GtkLabel $label,
                GdkEvent $event
            ) {
                return $this->_onButtonPress(
                    $label,
                    $event
                );
            }
        );

        $this->gtk->connect(
            'size-allocate',
            function(
                GtkLabel $label,
                GdkEvent $event
            ) {
                return $this->_onSizeAllocate(
                    $label,
                    $event
                );
            }
        );
    }

    protected function _onActivateLink(
        GtkLabel $label,
        string $href
    ): bool
    {
        return false;
    }

    protected function _onButtonPress(
        GtkLabel $label,
        GdkEvent $event
    ): bool
    {
        return false;
    }

    // Require custom wordwrap implementation on widget resize
    abstract protected function _onSizeAllocate(
        GtkLabel $label,
        GdkEvent $event
    ): bool;

    // Require custom layout implementation
    abstract public function set(
        string $value
    ): void;
}
