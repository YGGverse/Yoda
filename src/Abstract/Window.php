<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Abstract;

abstract class Window
{
    public \GtkWindow $window;
    public \GtkCssProvider $css;
    public \GtkStyleContext $style;

    public object $config;

    public function __construct(
        object $config
    ) {
        $this->config = $config;

        $this->window = new \GtkWindow();

        $this->window->set_size_request(
            $this->config->interface->window->width,
            $this->config->interface->window->height
        );

        if ($this->config->interface->window->header->enabled)
        {
            $header = new \GtkHeaderBar();

            $header->set_show_close_button(
                $this->config->interface->window->header->button->close
            );

            $this->window->set_titlebar(
                $header
            );
        }

        $this->window->set_title(
            'Yoda'
        );

        $this->window->connect(
            'destroy',
            function()
            {
                \Gtk::main_quit();
            }
        );
    }

    public function setTheme(
        string $css
    ): void
    {
        $this->css = new \GtkCssProvider();

        $this->css->load_from_data(
            $css
        );

        $this->style = new \GtkStyleContext();

        $this->style->add_provider_for_screen(
            $this->css,
            600
        );
    }
}