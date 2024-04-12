<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Entity;

class App
{
    public \GtkWindow $window;
    public \GtkNotebook $tab;

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
            $header = new \GtkHeaderBar();

            $header->set_title(
                $this->config->title
            );

            $header->set_show_close_button(
                $this->config->header->button->close
            );

            $this->window->set_titlebar(
                $header
            );
        }

        $this->window->connect(
            'destroy',
            function()
            {
                \Gtk::main_quit();
            }
        );

        $page = new \Yggverse\Yoda\Entity\Tab\Page(
            $this
        );

        $page->open(
            $this->config->tab->page->header->button->home->url
        );

        $this->tab = new \GtkNotebook();

        $this->tab->set_scrollable(
            true
        );

        $this->tab->append_page(
            $page->box,
            new \GtkLabel(
                'New page' // @TODO
            )
        );

        $this->tab->set_menu_label(
            $page->box,
            new \GtkLabel(
                '2' // @TODO
            )
        );

        $this->tab->set_tab_reorderable(
            $page->box,
            true
        );

        $this->window->add(
            $this->tab
        );

        $this->window->show_all();
    }
}