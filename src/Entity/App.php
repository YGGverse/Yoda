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
        // Init config
        $this->config = \Yggverse\Yoda\Model\File::getConfig()->app; // @TODO

        // Init window
        $this->window = new \GtkWindow;

        $this->window->set_size_request(
            $this->config->width,
            $this->config->height
        );

        if ($this->config->header->enabled)
        {
            $this->header = new \GtkHeaderBar;

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

        // Init tabs
        $this->tabs = new \GtkNotebook;

        $this->tabs->set_scrollable(
            true
        );

        // + button
        $blank = new \GtkLabel;

        $this->tabs->append_page(
            $blank,
            new \GtkLabel(
                '+'
            )
        );

        $this->tabs->set_tab_reorderable(
            $blank,
            true
        );

        // Append blank page
        $page = $this->blankPage();

        $page->open(
            $this->config->tab->page->header->button->home->url // @TODO
        );

        // Render
        $this->window->add(
            $this->tabs
        );

        $this->window->show_all();

        // Init event listener
        $this->tabs->connect(
            'switch-page',
            function ($tabs, $child, $position)
            {
                if ('+' == $tabs->get_tab_label_text($child))
                {
                    $page = $this->blankPage();

                    $this->tabs->show_all();

                    $this->tabs->set_current_page(
                        $this->tabs->page_num(
                            $page->box
                        )
                    );
                }
            }
        );

        $this->window->connect(
            'destroy',
            function()
            {
                \Gtk::main_quit();
            }
        );
    }

    public function blankPage(): \Yggverse\Yoda\Entity\Tab\Page
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

        $this->tabs->show_all();

        $this->tabs->set_current_page(
            $this->tabs->page_num(
                $page->box
            )
        );

        return $page;
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