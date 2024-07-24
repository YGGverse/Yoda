<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

use \Gtk;
use \GtkProgressBar;

use \Yggverse\Yoda\Entity\Browser\Container\Page;

class Progressbar
{
    // GTK
    public GtkProgressBar $gtk;

    // Dependencies
    public Page $page;

    // Defaults
    private bool $_active = false;

    public const MARGIN = 8;
    public const STEP = 0.02;

    public function __construct(
        Page $page,
    ) {
        // Init dependencies
        $this->page = $page;

        // Init container
        $this->gtk = new GtkProgressBar;

        $this->gtk->set_margin_start(
            $this::MARGIN
        );

        $this->gtk->set_margin_end(
            $this::MARGIN
        );

        $this->gtk->set_margin_bottom(
            $this::MARGIN
        );

        $this->gtk->show(); // fixed block height, show always

        $this->gtk->set_opacity(0); // init transparently
    }

    public function start(): void
    {
        $this->_active = true;
    }

    public function stop(): void
    {
        $this->_active = false;
    }

    public function show(): void
    {
        $this->gtk->set_opacity(1); // fixed block height, do not show()
    }

    public function hide(): void
    {
        $this->stop(); // make sure iterator get stopped

        $this->gtk->set_opacity(0); // fixed block height, do not hide()
    }

    public function infinitive(
        int $timeout = 100,
        bool $show = true
    ): void
    {
        // Init visible
        if ($show)
        {
            $this->show();
        }

        // Activate iterator
        $this->_active = true;

        // Reset initial progress
        $this->gtk->set_fraction(0);

        // Begin iterator
        Gtk::timeout_add(
            $timeout,
            function()
            {
                if ($this->_active)
                {
                    $this->gtk->pulse();
                }

                else return false; // stop
            }
        );
    }

    public function progressive(
        float $fraction = 0,
        int $timeout = 100,
        bool $show = true
    ): void
    {
        // Init visible
        if ($show)
        {
            $this->show();
        }

        // Activate iterator
        $this->_active = true;

        // Set initial progress
        $this->gtk->set_fraction(
            $fraction
        );

        // Begin iterator
        Gtk::timeout_add(
            $timeout,
            function()
            {
                if ($this->_active)
                {
                    // Update fraction step
                    $this->gtk->set_fraction(
                        $fraction = $this->gtk->get_fraction() + $this::STEP
                    );

                    // Deactivate loop on progress complete
                    if ($fraction >= 1)
                    {
                        $this->_active = false;
                    }
                }

                else return false; // stop
            }
        );
    }
}