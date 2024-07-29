<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity\Browser\Container\Page\Content;

use \GdkEvent;
use \GtkLabel;

use \Yggverse\Yoda\Entity\Browser\Container\Page\Content;

use \Yggverse\Net\Address;

abstract class Markup
{
    public GtkLabel $gtk;

    // Dependencies
    public Content $content;

    // Extras
    protected ?string $_source = null;

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

    // Custom wordwrap on widget resize
    protected function _onSizeAllocate(
        GtkLabel $label,
        GdkEvent $event
    ): bool
    {
        $this->set( // @TODO Gtk::timeout_add
            $this->_source
        );

        return false;
    }

    // Require custom layout implementation
    abstract public function set(
        string $value
    ): void;

    // Tools
    protected function _url(
        string $link
    ): ?string
    {
        $address = new Address(
            $link
        );

        if ($address->isRelative())
        {
            $address->toAbsolute(
                new Address(
                    $this->content->page->navbar->request->getValue()
                )
            );
        }

        return $address->get();
    }
}
