<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity;

abstract class Entry
{
    public \GtkEntry $gtk;

    protected int $_length = 1024;
    protected string $_placeholder = '';
    protected string $_value = '';
    protected bool $_visible = true;

    public function __construct()
    {
        $this->gtk = new \GtkEntry;

        $this->gtk->set_placeholder_text(
            _($this->_placeholder)
        );

        $this->gtk->set_max_length(
            $this->_length
        );

        $this->gtk->set_text(
            _($this->_value)
        );

        $this->gtk->set_visibility(
            $this->_visible
        );

        // Render
        $this->gtk->show();

        // Init events
        $this->gtk->connect(
            'activate',
            function(
                \GtkEntry $entry,
                \GdkEvent $event
            ) {
                $this->_onActivate(
                    $entry,
                    $event
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

        $this->gtk->connect(
            'changed',
            function (
                \GtkEntry $entry
            ) {
                $this->_onChanged(
                    $entry
                );
            }
        );

        $this->gtk->connect(
            'focus-out-event',
            function (
                \GtkEntry $entry,
                \GdkEvent $event
            ) {
                $this->_onFocusOut(
                    $entry,
                    $event
                );
            }
        );
    }

    abstract protected function _onActivate(
        \GtkEntry $entry,
        \GdkEvent $event
    ): void;

    abstract protected function _onKeyRelease(
        \GtkEntry $entry,
        \GdkEvent $event
    ): void;

    abstract protected function _onChanged(
        \GtkEntry $entry
    ): void;

    abstract protected function _onFocusOut(
        \GtkEntry $entry,
        \GdkEvent $event
    ): void;

    public function setLength(
        ?int $value = null
    ): void
    {
        $this->gtk->set_max_length(
            is_null($value) ? $this->_length : $value
        );
    }

    public function setPlaceholder(
        ?string $value = null
    ): void
    {
        $this->gtk->set_placeholder_text(
            is_null($value) ? $this->_value : trim(
                strval(
                    $value
                )
            )
        );
    }

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

    public function setVisible(
        ?bool $value = null
    ): void
    {
        $this->gtk->set_visibility(
            is_null($value) ? $this->_visible : $value
        );
    }

    public function getLength(): ?int
    {
        return $this->gtk->get_max_length();
    }

    public function getPlaceholder(): ?string
    {
        return $this->gtk->get_placeholder_text();
    }

    public function getValue(): ?string
    {
        return $this->gtk->get_text();
    }

    public function isVisible(): ?bool
    {
        return $this->gtk->get_visibility();
    }

    public function focus(): void
    {
        $this->gtk->grab_focus();
    }
}