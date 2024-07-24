<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract\Entity;

use \GdkEvent;
use \GtkEntry;

abstract class Entry
{
    public GtkEntry $gtk;

    public const LENGTH = 1024;
    public const PLACEHOLDER = '';
    public const VALUE = '';
    public const VISIBLE = true;

    public function __construct()
    {
        $this->gtk = new GtkEntry;

        $this->gtk->set_placeholder_text(
            _($this::PLACEHOLDER)
        );

        $this->gtk->set_max_length(
            $this::LENGTH
        );

        $this->gtk->set_text(
            _($this::VALUE)
        );

        $this->gtk->set_visibility(
            $this::VISIBLE
        );

        // Render
        $this->gtk->show();

        // Init events
        $this->gtk->connect(
            'activate',
            function(
                GtkEntry $entry
            ) {
                $this->_onActivate(
                    $entry
                );
            }
        );

        $this->gtk->connect(
            'key-release-event',
            function (
                GtkEntry $entry,
                GdkEvent $event
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
                GtkEntry $entry
            ) {
                $this->_onChanged(
                    $entry
                );
            }
        );

        $this->gtk->connect(
            'focus-out-event',
            function (
                GtkEntry $entry,
                GdkEvent $event
            ) {
                $this->_onFocusOut(
                    $entry,
                    $event
                );
            }
        );
    }

    abstract protected function _onActivate(
        GtkEntry $entry
    ): void;

    abstract protected function _onKeyRelease(
        GtkEntry $entry,
        GdkEvent $event
    ): void;

    abstract protected function _onChanged(
        GtkEntry $entry
    ): void;

    abstract protected function _onFocusOut(
        GtkEntry $entry,
        GdkEvent $event
    ): void;

    public function setLength(
        ?int $length = null
    ): void
    {
        $this->gtk->set_max_length(
            is_null($length) ? $this::LENGTH : $length
        );
    }

    public function setPlaceholder(
        ?string $placeholder = null
    ): void
    {
        $this->gtk->set_placeholder_text(
            is_null($placeholder) ? $this::PLACEHOLDER : trim(
                strval(
                    $placeholder
                )
            )
        );
    }

    public function setValue(
        ?string $value = null
    ): void
    {
        $this->gtk->set_text(
            is_null($value) ? $this::VALUE : trim(
                strval(
                    $value
                )
            )
        );
    }

    public function setVisible(
        ?bool $visible = null
    ): void
    {
        $this->gtk->set_visibility(
            is_null($visible) ? $this::VISIBLE : $visible
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