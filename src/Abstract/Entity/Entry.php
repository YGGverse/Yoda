<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity;

abstract class Entry
{
    public \GtkEntry $gtk;

    protected int $_length = 1024;
    protected string $_placeholder = '';
    protected string $_value = '';

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

    public function setValue(
        ?string $value = null
    ): void
    {
        $this->gtk->set_text(
            is_null($value) ? $this->_value : trim(
                strval(
                    $value
                )
            )
        );
    }

    public function getValue(): ?string
    {
        return $this->gtk->get_text();
    }
}