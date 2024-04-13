<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity;

class App
{
    public \GtkWindow $window;
    public \GtkHeaderBar $header;
    public \GtkNotebook $tabs;

    public object $config;

    public function __construct()
    {
        $this->config = \Yggverse\Yoda\Model\File::getConfig()->app; // @TODO

        $this->window = new \GtkWindow();

        $this->window->set_size_request(
            $this->config->width,
            $this->config->height
        );

        if ($this->config->header->enabled)
        {
            $this->header = new \GtkHeaderBar();

            $this->header->set_title(
                $this->config->header->title->default
            );

            $this->header->set_show_close_button(
                $this->config->header->button->close
            );

            $this->window->set_titlebar(
                $this->header
            );
        }

        $this->window->connect(
            'destroy',
            function()
            {
                \Gtk::main_quit();
            }
        );

        $this->tabs = new \GtkNotebook();

        $this->tabs->set_scrollable(
            true
        );

        $this->window->add(
            $this->tabs
        );

        $this->openPage(
            $this->config->tab->page->header->button->home->url // @TODO
        );

        $this->window->show_all();
    }

    public function openPage(
        string $url
    ): void
    {
        $page = new \Yggverse\Yoda\Entity\Tab\Page(
            $this
        );

        $this->tabs->append_page(
            $page->box,
            new \GtkLabel(
                $this->config->tab->page->title->default
            )
        );

        $this->tabs->set_tab_reorderable(
            $page->box,
            true
        );

        $page->open(
            $url
        );
    }

    public function setTitle(
        ?string $value = null
    ): void
    {
        if ($value)
        {
            $title = urldecode(
                strlen($value) > $this->config->header->title->length->max ? substr($value, 0, $this->config->header->title->length->max) . '...'
                                                                           : $value
            );
        }

        else
        {
            $title = $this->config->header->title->default;
        }

        $this->header->set_title(
            $title
        );
    }
}