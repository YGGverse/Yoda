<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity\Browser\Container\Page;

class Progressbar
{
    public \GtkProgressBar $gtk;

    // Dependencies
    public \Yggverse\Yoda\Entity\Browser\Container\Page $page;

    // Defaults
    private bool $_active = false;

    private float $_step = 0.02;

    public function __construct(
        \Yggverse\Yoda\Entity\Browser\Container\Page $page,
    ) {
        // Init dependencies
        $this->page = $page;

        // Init container
        $this->gtk = new \GtkProgressBar;

        /* Prevent global initiation
        $this->gtk->set_no_show_all(
            true
        );*/
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
        $this->gtk->show(); // | set_opacity(1)
    }

    public function hide(): void
    {
        $this->stop(); // make sure iterator get stopped

        $this->gtk->hide();  // | set_opacity(0)
    }

    public function infinitive(
        int $timeout = 100,
        bool $show = true
    ): void
    {
        // Init visible
        if ($show)
        {
            $this->gtk->show();
        }

        // Activate iterator
        $this->_active = true;

        // Begin iterator
        \Gtk::timeout_add(
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
            $this->gtk->show();
        }

        // Activate iterator
        $this->_active = true;

        // Set initial progress
        $this->gtk->set_fraction(
            $fraction
        );

        // Begin iterator
        \Gtk::timeout_add(
            $timeout,
            function()
            {
                if ($this->_active)
                {
                    // Update fraction step
                    $this->gtk->set_fraction(
                        $fraction = $this->gtk->get_fraction() + $this->_step
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