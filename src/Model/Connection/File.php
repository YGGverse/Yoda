<?php

declare(strict_types=1);

namespace Yggverse\Yoda\Model\Connection;

use \Yggverse\Net\Address;

use \Yggverse\Yoda\Model\Connection;
use \Yggverse\Yoda\Model\Filesystem;

class File
{
    private Connection $_connection;

    public function __construct(
        Connection $connection
    ) {
        $this->_connection = $connection;
    }

    public function request(
        Address $address
    ): void
    {
        $this->_connection->setTitle(
            basename(
                $address->getPath()
            )
        );

        $this->_connection->setSubtitle(
            $address->getPath()
        );

        $this->_connection->setTooltip(
            $address->getPath()
        );

        switch (true)
        {
            case ( // is directory
                $list = Filesystem::getList(
                    $address->getPath()
                )
            ):
                $tree = [];

                foreach ($list as $item)
                {
                    $tree[] = trim(
                        sprintf(
                            '=> file://%s %s',
                            $item['path'],
                            $item['name'] . (
                                // append slash indicator
                                $item['file'] ? null : '/'
                            )
                        )
                    );
                }

                $this->_connection->setMime(
                    $this->_connection::MIME_TEXT_GEMINI
                );

                $this->_connection->setData(
                    implode(
                        PHP_EOL,
                        $tree
                    ) . PHP_EOL
                );

            break;

            case file_exists( // is file
                $address->getPath()
            ) && is_readable(
                $address->getPath()
            ):
                $this->_connection->setData(
                    strval(
                        file_get_contents(
                            $address->getPath()
                        )
                    )
                );

                $this->_connection->setMime(
                    strval(
                        mime_content_type(
                            $address->getPath()
                        )
                    )
                );

                if ($this->_connection::MIME_TEXT_PLAIN == $this->_connection->getMime())
                {
                    $extension = pathinfo(
                        strval(
                            $address->getPath()
                        ),
                        PATHINFO_EXTENSION
                    );

                    if (in_array($extension, ['gmi', 'gemini']))
                    {
                        $this->_connection->setMime(
                            $this->_connection::MIME_TEXT_GEMINI
                        );
                    }
                }

            break;

            default:

                $this->_connection->setTitle(
                    _('Failure')
                );

                $this->_connection->setData(
                    _('Could not open location')
                );
        }

        $this->_connection->setCompleted(
            true
        );
    }
}