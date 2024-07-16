<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Connection;

use \Yggverse\Net\Address;
use \Yggverse\Nex\Client;

use \Yggverse\Yoda\Model\Connection;

class Nex
{
    private Connection $_connection;

    public function __construct(
        Connection $connection
    ) {
        $this->_connection = $connection;
    }

    // @TODO
    public function sync(
        Address $address,
        int $timeout = 5
    ): void
    {
        $response = (new \Yggverse\Nex\Client)->request(
            $address->get(),
            $timeout
        );

        if ($response)
        {
            $this->_connection->setTitle(
                strval(
                    $address->getHost()
                )
            );

            $this->_connection->setData(
                $response
            );

            $this->_connection->setMime(
                $this->_connection::MIME_TEXT_PLAIN
            );
        }

        else
        {
            $this->_connection->setTitle(
                _('Oops!')
            );

            $this->_connection->setData(
                _('Could not open request')
            );

            $this->_connection->setMime(
                $this->_connection::MIME_TEXT_GEMINI
            );
        }

        $this->_connection->setCompleted(
            true
        );
    }
}