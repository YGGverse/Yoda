<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity;

abstract class Entry
{
    public \GtkEntry $gtk;

    private int $_length = 1024;
    private string $_placeholder = '';
    private string $_value = '';

    public function __construct()
    {
        $this->gtk = new \GtkEntry;

        $this->gtk->set_placeholder_text(
            $this->_placeholder
        );

        $this->gtk->set_max_length(
            $this->_length
        );

        $this->gtk->set_text(
            $this->_value
        );

        $this->gtk->connect(
            'activate',
            function(
                \GtkEntry $entry
            ) {
                $this->_onActivate(
                    $entry
                );
            }
        );

        $this->gtk->connect(
            'key-release-event',
            function (
                \GtkEntry $entry,
                \GdkEvent $event
            ) {
                $this->_onKeyRelease(
                    $entry,
                    $event
                );
            }
        );
    }

    abstract protected function _onActivate(
        \GtkEntry $entry
    ): void;

    abstract protected function _onKeyRelease(
        \GtkEntry $entry,
        \GdkEvent $event
    ): void;
}