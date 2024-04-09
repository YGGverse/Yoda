<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Box;

class Main
{
    public \GtkBox $box;

    public \Yggverse\Yoda\Box\Menu $menu;
    public \Yggverse\Yoda\Box\Navigation $navigation;
    public \Yggverse\Yoda\Label\Content $content;
    public \Yggverse\Yoda\Label\Tray $tray;

    public function __construct(
        string $name = 'boxMain'
    ) {
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
            0
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

        $this->navigation->address->entry->connect(
            'activate',
            function ($entry)
            {
                global $config;
                global $memory;

                $this->tray->label->set_text(
                    sprintf(
                        'Open %s...',
                        $entry->get_text()
                    )
                );

                $start = microtime(true);

                $host = null;

                if ($config->resolver->enabled)
                {
                    $address = new \Yggverse\Net\Address(
                        $entry->get_text()
                    );

                    $name = $address->getHost();

                    if (!$host = $memory->get($name))
                    {
                        $resolve = new \Yggverse\Net\Resolve(
                            $config->resolver->request->record,
                            $config->resolver->request->host,
                            $config->resolver->request->timeout,
                            $config->resolver->result->shuffle
                        );

                        $resolved = $resolve->address(
                            $address
                        );

                        if ($resolved)
                        {
                            $host = $resolved->getHost();

                            $memory->set(
                                $name,
                                $host
                            );
                        }
                    }
                }

                $request = new \Yggverse\Gemini\Client\Request(
                    $entry->get_text(),
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
        );
    }
}