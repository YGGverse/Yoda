<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Box;

class Tab
{
    public \GtkBox $box;

    public \Yggverse\Yoda\Box\Menu $menu;
    public \Yggverse\Yoda\Box\Navigation $navigation;
    public \Yggverse\Yoda\Label\Content $content;
    public \Yggverse\Yoda\Label\Tray $tray;

    public \Yggverse\Yoda\Model\Memory $memory;

    public object $config;

    public function __construct(
        string $name = 'boxTab'
    ) {
        // Init config
        $this->config = \Yggverse\Yoda\Model\File::getConfig();

        // Init memory
        $this->memory = new \Yggverse\Yoda\Model\Memory();

        // Init container
        $this->box = new \GtkBox(
            \GtkOrientation::VERTICAL
        );

        $this->box->set_name(
            $name
        );

        // Init dependencies
        $this->menu = new \Yggverse\Yoda\Box\Menu();

        $this->box->pack_start(
            $this->menu->box,
            false,
            true,
            0
        );

        $this->navigation = new \Yggverse\Yoda\Box\Navigation();

        $this->box->pack_start(
            $this->navigation->box,
            false,
            true,
            8
        );

        $this->content = new \Yggverse\Yoda\Label\Content();

        $scroll = new \GtkScrolledWindow();

        $scroll->add(
            $this->content->label
        );

        $this->box->pack_start(
            $scroll,
            true,
            true,
            10
        );

        $this->tray = new \Yggverse\Yoda\Label\Tray();

        $this->box->pack_start(
            $this->tray->label,
            false,
            true,
            0
        );

        // Init listeners
        $this->navigation->address->entry->connect(
            'activate',
            function ($entry)
            {
                $this->navigate(
                    $entry->get_text()
                );
            }
        );

        $this->navigation->go->button->connect(
            'released',
            function ($entry)
            {
                $this->navigate(
                    $this->navigation->address->entry->get_text()
                );
            }
        );

        $this->navigation->reload->button->connect(
            'released',
            function ($entry)
            {
                $this->navigate(
                    $this->navigation->address->entry->get_text()
                );
            }
        );

        if ($this->config->homepage)
        {
            $this->navigation->home->button->connect(
                'released',
                function ($entry)
                {
                    $this->navigation->address->entry->set_text(
                        $this->config->homepage
                    );

                    $this->navigate(
                        $this->config->homepage
                    );
                }
            );
        }

        // @TODO back, forward buttons
    }

    // Actions
    public function navigate(string $url)
    {
        $this->tray->label->set_text(
            sprintf(
                'Open %s...',
                urldecode(
                    $url
                )
            )
        );

        $start = microtime(true);

        $host = null;

        if ($this->config->resolver->enabled)
        {
            $address = new \Yggverse\Net\Address(
                $url
            );

            $name = $address->getHost();

            if (!$host = $this->memory->get($name))
            {
                $resolve = new \Yggverse\Net\Resolve(
                    $this->config->resolver->request->record,
                    $this->config->resolver->request->host,
                    $this->config->resolver->request->timeout,
                    $this->config->resolver->result->shuffle
                );

                $resolved = $resolve->address(
                    $address
                );

                if ($resolved)
                {
                    $host = $resolved->getHost();

                    $this->memory->set(
                        $name,
                        $host
                    );
                }
            }
        }

        $request = new \Yggverse\Gemini\Client\Request(
            $url,
            $host
        );

        $raw = $request->getResponse();

        $end = microtime(true);

        $response = new \Yggverse\Gemini\Client\Response(
            $raw
        );

        $this->content->label->set_markup(
            $response->getBody()
        );

        $this->tray->label->set_text(
            sprintf(
                '%s | %s | %d bytes | %s seconds',
                date('c'),
                $response->getMeta() ? $response->getMeta() : $response->getCode(),
                number_format(
                    mb_strlen(
                        $raw
                    )
                ),
                round(
                    $end - $start, 2
                )
            )
        );
    }
}