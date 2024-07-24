<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Connection;

use \Yggverse\Net\Address;
use \Yggverse\Nex\Client;

use \Yggverse\Yoda\Model\Connection;
use \Yggverse\Yoda\Model\Filesystem;

class Nex
{
    private Connection $_connection;

    public function __construct(
        Connection $connection
    ) {
        $this->_connection = $connection;
    }

    // @TODO
    public function request(
        Address $address,
        int $timeout = 15
    ): void
    {
        $response = (new Client)->request(
            $address->get(),
            $timeout
        );

        // Success
        if ($response)
        {
            // Detect MIME type
            switch (true)
            {
                case $mime = Filesystem::getMimeByPath(
                    $address->getPath()
                ): break;

                case $mime = Filesystem::getMimeByData(
                    $response
                ): break;

                default: $mime = Filesystem::MIME_TEXT_PLAIN;
            }

            // Set MIME
            $this->_connection->setMime(
                $mime
            );

            // Set title
            $this->_connection->setTitle(
                $address->getHost()
            );

            // Set subtitle
            $this->_connection->setSubtitle(
                $address->getHost()
            );

            // Set tooltip
            $this->_connection->setTooltip(
                $address->get()
            );

            // Set data
            $this->_connection->setData(
                $response
            );

            // Cache
            $this->_connection->cache(
                $address->get()
            );
        }

        // Try cache
        if ($cache = $this->_connection->getCache())
        {
            $this->_connection->setTitle(
                $cache->title
            );

            $this->_connection->setSubtitle(
                date(
                    'c',
                    $cache->time
                ) # $cache->subtitle
            );

            $this->_connection->setTooltip(
                $cache->tooltip
            );

            $this->_connection->setData(
                $cache->data
            );

            $this->_connection->setMime(
                $cache->mime
            );
        }

        else
        {
            $this->_connection->setMime(
                Filesystem::MIME_TEXT_PLAIN
            );

            $this->_connection->setTitle(
                _('Oops!')
            );

            $this->_connection->setSubtitle(
                $address->getHost()
            );

            $this->_connection->setTooltip(
                $address->get()
            );

            $this->_connection->setData(
                _('Could not open request')
            );
        }

        $this->_connection->setCompleted(
            true
        );
    }
}