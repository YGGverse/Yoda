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
        switch (true)
        {
            case ( // is directory
                $list = Filesystem::getList(
                    $address->getPath()
                )
            ):
                // Set MIME
                $this->_connection->setMime(
                    Filesystem::MIME_TEXT_GEMINI
                );

                // Set title
                $this->_connection->setTitle(
                    basename(
                        $address->getPath()
                    )
                );

                // Set subtitle
                $this->_connection->setSubtitle(
                    $address->getPath()
                );

                // Set tooltip
                $this->_connection->setTooltip(
                    $address->getPath()
                );

                // Set data
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
                // Detect MIME type
                switch (true)
                {
                    case $mime = Filesystem::getMimeByPath(
                        $address->getPath()
                    ): break;

                    case $mime = mime_content_type(
                        $address->getPath()
                    ): break;

                    default: $mime = Filesystem::MIME_TEXT_GEMINI;
                }

                // Set MIME
                $this->_connection->setMime(
                    $mime
                );

                // Set title
                $this->_connection->setTitle(
                    basename(
                        $address->getPath()
                    )
                );

                // Set subtitle
                $this->_connection->setSubtitle(
                    $mime
                );

                // Set tooltip
                $this->_connection->setTooltip(
                    $address->getPath()
                );

                // Set data
                $this->_connection->setData(
                    strval(
                        file_get_contents(
                            $address->getPath()
                        )
                    )
                );

            break;

            default:

                // Set MIME
                $this->_connection->setMime(
                    Filesystem::MIME_TEXT_GEMINI
                );

                // Set title
                $this->_connection->setTitle(
                    _('Failure')
                );

                // Set subtitle
                $this->_connection->setSubtitle(
                    $address->getPath()
                );

                // Set tooltip
                $this->_connection->setTooltip(
                    $address->getPath()
                );

                // Set data
                $this->_connection->setData(
                    _('Could not open location')
                );
        }

        $this->_connection->setCompleted(
            true
        );
    }
}