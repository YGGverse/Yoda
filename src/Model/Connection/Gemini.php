<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Connection;

use \Yggverse\Gemini\Client\Request;
use \Yggverse\Gemini\Client\Response;
use \Yggverse\Net\Address;

use \Yggverse\Yoda\Model\Connection;

class Gemini
{
    private Connection $_connection;

    public function __construct(
        Connection $connection
    ) {
        $this->_connection = $connection;
    }

    public function request(
        Address $address,
        int $timeout = 5
    ): void
    {
        $request = new Request(
            $address->get()
        );

        $response = new Response(
            $request->getResponse(
                $timeout
            )
        );

        // Route status code
        // https://geminiprotocol.net/docs/protocol-specification.gmi#status-codes
        switch ($response->getCode())
        {
            case 10: // response expected
            case 11: // sensitive input

                $this->_connection->setMime(
                    $this->_connection::MIME_TEXT_GEMINI
                );

                $this->_connection->setRequest(
                    $response->getMeta(),
                    11 !== $response->getCode()
                );

            break;

            case 20: // ok

                $this->_connection->setData(
                    $response->getBody()
                );

                switch (true)
                {
                    case str_contains(
                        $response->getMeta(),
                        self::MIME_TEXT_GEMINI
                    ):

                        $this->_connection->setMime(
                            $this->_connection::MIME_TEXT_GEMINI
                        );

                    break;

                    case str_contains(
                        $response->getMeta(),
                        self::MIME_TEXT_PLAIN
                    ):

                        $this->_connection->setMime(
                            $this->_connection::MIME_TEXT_PLAIN
                        );

                    break;

                    default:

                        throw new \Exception(
                            sprintf(
                                _('MIME type not implemented for %s'),
                                $response->getMeta()
                            )
                        );
                }

            break;

            case 31: // redirect
                     // show link, no follow

                $this->_connection->setTitle(
                    _('Redirect!')
                );

                $this->_connection->setData(
                    sprintf(
                        '=> %s',
                        $response->getMeta()
                    )
                );

                $this->_connection->setMime(
                    $this->_connection::MIME_TEXT_GEMINI
                );

            break;

            default:

                $this->_connection->setTitle(
                    _('Oops!')
                );

                $this->_connection->setData(
                    sprintf(
                        'Could not open request (code: %d)',
                        intval(
                            $response->getCode()
                        )
                    )
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